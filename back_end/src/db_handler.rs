use std::sync::mpsc::{Receiver, SyncSender};
use std::thread;

use rand::{thread_rng, Rng};
use rusqlite::{params, Connection};

pub type GetIDSenderType = (u32, Option<bool>);
/// first bool is player exists,
/// second bool is if paired,
/// third bool is if cyan player
pub type CheckPairingType = (bool, bool, bool);

#[derive(Clone, Debug)]
pub enum DBHandlerRequest {
    GetID(SyncSender<GetIDSenderType>),
    CheckPairing {
        id: u32,
        response_sender: SyncSender<CheckPairingType>,
    },
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum DBFirstRun {
    FirstRun,
    NotFirstRun,
}

struct DBHandler {
    rx: Receiver<DBHandlerRequest>,
    sqlite_path: String,
    shutdown_tx: SyncSender<()>,
}

impl DBHandler {
    /// Returns true if should break out of outer loop
    fn handle_request(&mut self) -> bool {
        let rx_recv_result = self.rx.recv();
        if let Err(e) = rx_recv_result {
            println!("Failed to get DBHandlerRequest: {:?}", e);
            self.shutdown_tx.send(()).ok();
            return false;
        }
        let db_request = rx_recv_result.unwrap();
        match db_request {
            DBHandlerRequest::GetID(player_tx) => {
                // got request to create new player, create new player
                let mut player_id: u32 = thread_rng().gen();
                let conn_result = self.get_conn(DBFirstRun::NotFirstRun);
                if let Err(e) = conn_result {
                    println!("Failed to get sqlite db connection: {:?}", e);
                    self.shutdown_tx.send(()).ok();
                    return false;
                }
                let conn = conn_result.unwrap();
                loop {
                    let exists_result = self.check_if_player_exists(Some(&conn), player_id);
                    if let Ok(exists) = exists_result {
                        if exists {
                            player_id = thread_rng().gen();
                        } else {
                            break;
                        }
                    } else {
                        let error = exists_result.unwrap_err();
                        println!("Failed to check if player exists in db: {:?}", error);
                        self.shutdown_tx.send(()).ok();
                        return true;
                    }
                }
                let insert_result = conn.execute(
                    "INSERT INTO players (id, date_added) VALUES (?, datetime());",
                    [player_id],
                );
                if let Err(e) = insert_result {
                    println!("Failed to insert into sqlite db: {:?}", e);
                    self.shutdown_tx.send(()).ok();
                    return true;
                }

                let pair_up_result = self.pair_up_players(Some(&conn));
                if let Err(e) = pair_up_result {
                    println!("Failed to pair up players: {}", e);
                    return true;
                }

                // Check if current player has been paired
                let mut is_cyan_player_opt: Option<bool> = None;
                let check_player_row = conn.query_row("SELECT games.cyan_player FROM players JOIN games WHERE games.id = players.game_id AND players.id = ?;", [player_id], |row| row.get::<usize, u32>(0));
                if let Ok(cyan_player) = check_player_row {
                    if cyan_player == player_id {
                        // is paired, is cyan_player
                        is_cyan_player_opt = Some(true);
                    } else {
                        // is paired, not cyan_player
                        is_cyan_player_opt = Some(false);
                    }
                } else if check_player_row.is_err() {
                    // not paired, can do nothing here
                }

                let send_result = player_tx.send((player_id, is_cyan_player_opt));
                if let Err(e) = send_result {
                    println!("Failed to send back player id: {:?}", e);
                    self.shutdown_tx.send(()).ok();
                    return true;
                }
                send_result.unwrap();
            }
            DBHandlerRequest::CheckPairing {
                id,
                response_sender,
            } => {
                let check_result = self.check_if_player_is_paired(id);
                if let Ok((exists, is_paired, is_cyan)) = check_result {
                    let send_result = response_sender.send((exists, is_paired, is_cyan));
                    if let Err(e) = send_result {
                        println!("Failed to send back check pairing status: {:?}", e);
                        self.shutdown_tx.send(()).ok();
                        return true;
                    }
                    send_result.unwrap();
                } else {
                }
            } // DBHandlerRequest::GetID(player_tx)
        } // match db_request

        false
    }

    fn get_conn(&self, first_run: DBFirstRun) -> Result<Connection, String> {
        if let Ok(conn) = Connection::open(&self.sqlite_path) {
            conn.execute("PRAGMA foreign_keys = ON;", [])
                .map_err(|e| format!("Should be able to handle \"foreign_keys\": {:?}", e))?;
            let result = conn.execute(
                "
                CREATE TABLE players (id INTEGER PRIMARY KEY NOT NULL,
                    date_added TEXT NOT NULL,
                    game_id INTEGER,
                    FOREIGN KEY(game_id) REFERENCES games(id) ON DELETE CASCADE);
            ",
                [],
            );
            if result.is_ok() {
                if first_run == DBFirstRun::FirstRun {
                    println!("Created \"players\" table");
                }
            } else if first_run == DBFirstRun::FirstRun {
                println!("\"players\" table exists");
            }

            let result = conn.execute(
                "
                CREATE TABLE games (id INTEGER PRIMARY KEY NOT NULL,
                    cyan_player INTEGER UNIQUE,
                    magenta_player INTEGER UNIQUE,
                    date_added TEXT NOT NULL,
                    board TEXT NOT NULL,
                    status INTEGER NOT NULL,
                    FOREIGN KEY(cyan_player) REFERENCES players (id),
                    FOREIGN KEY(magenta_player) REFERENCES players (id));
            ",
                [],
            );
            if result.is_ok() {
                if first_run == DBFirstRun::FirstRun {
                    println!("Created \"games\" table");
                }
            } else if first_run == DBFirstRun::FirstRun {
                println!("\"games\" table exists");
            }
            Ok(conn)
        } else {
            Err(String::from("Failed to open connection"))
        }
    }

    fn pair_up_players(&self, conn: Option<&Connection>) -> Result<(), String> {
        if let Some(conn) = conn {
            let mut to_pair: Option<u32> = None;
            let mut unpaired_players_stmt = conn
                .prepare("SELECT id FROM players WHERE game_id ISNULL ORDER BY date_added;")
                .map_err(|e| format!("{:?}", e))?;
            let mut unpaired_players_rows = unpaired_players_stmt
                .query([])
                .map_err(|e| format!("{:?}", e))?;
            while let Some(row) = unpaired_players_rows
                .next()
                .map_err(|e| format!("{:?}", e))?
            {
                if to_pair.is_none() {
                    to_pair = Some(row.get(0).map_err(|e| format!("{:?}", e))?);
                } else {
                    let players: [u32; 2] = [
                        to_pair.take().unwrap(),
                        row.get(0).map_err(|e| format!("{:?}", e))?,
                    ];
                    self.create_game(Some(conn), &players)?;
                }
            }

            Ok(())
        } else {
            let conn = self.get_conn(DBFirstRun::NotFirstRun)?;
            self.pair_up_players(Some(&conn))
        }
    }

    fn create_game(&self, conn: Option<&Connection>, players: &[u32; 2]) -> Result<u32, String> {
        if let Some(conn) = conn {
            let mut game_id: u32 = thread_rng().gen();
            {
                let mut get_game_stmt = conn
                    .prepare("SELECT id FROM games WHERE id = ?;")
                    .map_err(|e| format!("{:?}", e))?;
                while get_game_stmt.query_row([game_id], |_row| Ok(())).is_ok() {
                    game_id = thread_rng().gen();
                }
            }

            // TODO randomize players (or first-come-first-serve ok to do?)
            conn.execute(
                "INSERT INTO games (id, cyan_player, magenta_player, date_added, board, status) VALUES (?, ?, ?, datetime(), ?, 0);",
                params![game_id, players[0], players[1], new_board()]
            )
            .map_err(|e| format!("{:?}", e))?;
            conn.execute(
                "UPDATE players SET game_id = ? WHERE id = ?",
                [game_id, players[0]],
            )
            .map_err(|e| format!("{:?}", e))?;
            conn.execute(
                "UPDATE players SET game_id = ? WHERE id = ?",
                [game_id, players[1]],
            )
            .map_err(|e| format!("{:?}", e))?;

            Ok(game_id)
        } else {
            let conn = self.get_conn(DBFirstRun::NotFirstRun)?;
            self.create_game(Some(&conn), players)
        }
    }

    fn check_if_player_is_paired(&self, player_id: u32) -> Result<CheckPairingType, String> {
        {
            let player_exists_result = self.check_if_player_exists(None, player_id);
            if player_exists_result.is_err() || !player_exists_result.unwrap() {
                // player doesn't exist
                return Ok((false, false, true));
            }
        }

        let conn = self.get_conn(DBFirstRun::NotFirstRun)?;

        let check_player_row = conn.query_row("SELECT games.cyan_player FROM players JOIN games where games.id = players.game_id AND players.id = ?;", [player_id], |row| row.get::<usize, u32>(0));
        if let Ok(cyan_player) = check_player_row {
            if cyan_player == player_id {
                // is cyan player
                Ok((true, true, true))
            } else {
                // is magenta player
                Ok((true, true, false))
            }
        } else if let Err(rusqlite::Error::QueryReturnedNoRows) = check_player_row {
            // is not paired
            Ok((true, false, true))
        } else if let Err(e) = check_player_row {
            Err(format!("check_if_player_is_paired: {:?}", e))
        } else {
            unreachable!();
        }
    }

    fn check_if_player_exists(
        &self,
        conn: Option<&Connection>,
        player_id: u32,
    ) -> Result<bool, String> {
        if let Some(conn) = conn {
            let check_player_row =
                conn.query_row("SELECT id FROM players WHERE id = ?;", [player_id], |row| {
                    row.get::<usize, u32>(0)
                });
            if let Ok(_id) = check_player_row {
                Ok(true)
            } else {
                Ok(false)
            }
        } else {
            let conn = self.get_conn(DBFirstRun::NotFirstRun)?;
            self.check_if_player_exists(Some(&conn), player_id)
        }
    }
}

pub fn start_db_handler_thread(
    rx: Receiver<DBHandlerRequest>,
    sqlite_path: String,
    shutdown_tx: SyncSender<()>,
) {
    let mut handler = DBHandler {
        rx,
        sqlite_path,
        shutdown_tx,
    };
    thread::spawn(move || {
        // temporarily get conn which should initialize on first setup of db
        if let Ok(_conn) = handler.get_conn(DBFirstRun::FirstRun) {
        } else {
            println!("ERROR: Failed init sqlite db connection");
            handler.shutdown_tx.send(()).ok();
            return;
        }

        'outer: loop {
            if handler.handle_request() {
                handler.shutdown_tx.send(()).ok();
                break 'outer;
            }
        }
    });
}

fn new_board() -> String {
    let mut board = String::with_capacity(56);
    for _i in 0..56 {
        board.push('a');
    }
    board
}
