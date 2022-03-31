use crate::constants::{COLS, ROWS};

use std::sync::mpsc::{Receiver, SyncSender};
use std::{fmt, thread};

use rand::{thread_rng, Rng};
use rusqlite::{params, Connection, Error as RusqliteError};

pub type GetIDSenderType = (u32, Option<bool>);
/// first bool is player exists,
/// second bool is if paired,
/// third bool is if cyan player
pub type CheckPairingType = (bool, bool, bool);

pub type BoardStateType = (DBGameState, Option<String>);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DBGameState {
    CyanTurn,
    MagentaTurn,
    CyanWon,
    MagentaWon,
    Draw,
    NotPaired,
    OpponentDisconnected,
    UnknownID,
    InternalError,
}

impl fmt::Display for DBGameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            DBGameState::CyanTurn => write!(f, "cyan_turn"),
            DBGameState::MagentaTurn => write!(f, "magenta_turn"),
            DBGameState::CyanWon => write!(f, "cyan_won"),
            DBGameState::MagentaWon => write!(f, "magenta_won"),
            DBGameState::Draw => write!(f, "draw"),
            DBGameState::NotPaired => write!(f, "not_paired"),
            DBGameState::OpponentDisconnected => write!(f, "opponent_disconnected"),
            DBGameState::UnknownID => write!(f, "unknown_id"),
            DBGameState::InternalError => write!(f, "internal_error"),
        }
    }
}

impl From<i64> for DBGameState {
    fn from(value: i64) -> Self {
        match value {
            0 => DBGameState::CyanTurn,
            1 => DBGameState::MagentaTurn,
            2 => DBGameState::CyanWon,
            3 => DBGameState::MagentaWon,
            4 => DBGameState::Draw,
            _ => DBGameState::InternalError,
        }
    }
}

#[derive(Clone, Debug)]
pub enum DBHandlerRequest {
    GetID(SyncSender<GetIDSenderType>),
    CheckPairing {
        id: u32,
        response_sender: SyncSender<CheckPairingType>,
    },
    GetGameState {
        id: u32,
        response_sender: SyncSender<BoardStateType>,
    },
    DisconnectID {
        id: u32,
        response_sender: SyncSender<bool>,
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

                // don't stop server on send fail, may have timed out and
                // dropped the receiver
                player_tx.send((player_id, is_cyan_player_opt)).ok();
            }
            DBHandlerRequest::CheckPairing {
                id,
                response_sender,
            } => {
                if let Ok((exists, is_paired, is_cyan)) = self.check_if_player_is_paired(None, id) {
                    // don't stop server on send fail, may have timed out and
                    // dropped the receiver
                    response_sender.send((exists, is_paired, is_cyan)).ok();
                } else {
                    // On error, just respond that the given player_id doesn't
                    // exist
                    response_sender.send((false, false, true)).ok();
                }
            }
            DBHandlerRequest::GetGameState {
                id,
                response_sender,
            } => {
                let get_board_result = self.get_board_state(None, id);
                if get_board_result.is_err() {
                    // don't stop server on send fail, may have timed out and
                    // dropped the receiver
                    response_sender.send((DBGameState::UnknownID, None)).ok();
                    return false;
                }
                // don't stop server on send fail, may have timed out and
                // dropped the receiver
                response_sender.send(get_board_result.unwrap()).ok();
            }
            DBHandlerRequest::DisconnectID {
                id,
                response_sender,
            } => {
                // don't stop server on send fail, may have timed out and
                // dropped the receiver
                response_sender
                    .send(self.disconnect_player(None, id).is_ok())
                    .ok();
            }
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
                    FOREIGN KEY(cyan_player) REFERENCES players (id) ON DELETE SET NULL,
                    FOREIGN KEY(magenta_player) REFERENCES players (id) ON DELETE SET NULL);
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
        if conn.is_none() {
            return self.pair_up_players(Some(&self.get_conn(DBFirstRun::NotFirstRun)?));
        }
        let conn = conn.unwrap();
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
    }

    fn create_game(&self, conn: Option<&Connection>, players: &[u32; 2]) -> Result<u32, String> {
        if conn.is_none() {
            return self.create_game(Some(&self.get_conn(DBFirstRun::NotFirstRun)?), players);
        }
        let conn = conn.unwrap();
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
    }

    fn check_if_player_is_paired(
        &self,
        conn: Option<&Connection>,
        player_id: u32,
    ) -> Result<CheckPairingType, String> {
        {
            let player_exists_result = self.check_if_player_exists(None, player_id);
            if player_exists_result.is_err() || !player_exists_result.unwrap() {
                // player doesn't exist
                return Ok((false, false, true));
            }
        }

        if conn.is_none() {
            return self.check_if_player_is_paired(
                Some(&self.get_conn(DBFirstRun::NotFirstRun)?),
                player_id,
            );
        }
        let conn = conn.unwrap();

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
            // either does not exist or is not paired
            let exists_check_result = self.check_if_player_exists(Some(conn), player_id);
            if let Ok(exists) = exists_check_result {
                if exists {
                    Ok((true, false, true))
                } else {
                    Ok((false, false, true))
                }
            } else {
                // pass the error contained in result, making sure the Ok type
                // is the expected type
                exists_check_result.map(|_| (false, false, false))
            }
        } else if let Err(e) = check_player_row {
            Err(format!("check_if_player_is_paired: {:?}", e))
        } else {
            unreachable!("All possible Ok and Err results are already checked");
        }
    }

    fn check_if_player_exists(
        &self,
        conn: Option<&Connection>,
        player_id: u32,
    ) -> Result<bool, String> {
        if conn.is_none() {
            return self
                .check_if_player_exists(Some(&self.get_conn(DBFirstRun::NotFirstRun)?), player_id);
        }
        let conn = conn.unwrap();
        let check_player_row =
            conn.query_row("SELECT id FROM players WHERE id = ?;", [player_id], |row| {
                row.get::<usize, u32>(0)
            });
        if let Ok(_id) = check_player_row {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn get_board_state(
        &self,
        conn: Option<&Connection>,
        player_id: u32,
    ) -> Result<BoardStateType, String> {
        if conn.is_none() {
            return self.get_board_state(Some(&self.get_conn(DBFirstRun::NotFirstRun)?), player_id);
        }
        let conn = conn.unwrap();

        // TODO maybe handle "opponent_disconnected" case
        let row_result: Result<(String, i64, Option<u32>, Option<u32>), RusqliteError> =
            conn.query_row(
                "SELECT games.board, games.status, games.cyan_player, games.magenta_player FROM games JOIN players WHERE players.id = ? AND games.id = players.game_id;",
                [player_id],
                |row| {
                    let board_result = row.get(0);
                    let status_result = row.get(1);
                    let cyan_player = row.get(2);
                    let magenta_player = row.get(3);
                    if board_result.is_ok() && status_result.is_ok() && cyan_player.is_ok() && magenta_player.is_ok() {
                        if let (Ok(board), Ok(status), Ok(cyan_id), Ok(magenta_id)) = (board_result, status_result, cyan_player, magenta_player) {
                            Ok((board, status, cyan_id, magenta_id))
                        } else {
                            unreachable!("Both row items should be Ok");
                        }
                    } else if board_result.is_err() {
                        board_result
                            .map(|_| (String::from("this value should never be returned"), 0, None, None))
                    } else if status_result.is_err() {
                        status_result
                            .map(|_| (String::from("this value should never be returned"), 0, None, None))
                    } else if cyan_player.is_err() {
                       cyan_player
                            .map(|_| (String::from("this value should never be returned"), 0, None, None))
                    } else {
                      magenta_player
                            .map(|_| (String::from("this value should never be returned"), 0, None, None))
                    }
                }
            );
        if let Ok((board, status, cyan_opt, magenta_opt)) = row_result {
            if board.len() != (ROWS * COLS) as usize {
                // board is invalid size
                Ok((DBGameState::InternalError, None))
            } else if cyan_opt.is_none() || magenta_opt.is_none() {
                // One player disconnected
                let player_remove_result = self.disconnect_player(Some(conn), player_id);
                if player_remove_result.is_err() {
                    // Failed to disconnect remaining player
                    Ok((DBGameState::InternalError, None))
                } else {
                    // Remove the game(s) with disconnected players
                    if self.clear_empty_games(Some(conn)).is_err() {
                        Ok((DBGameState::InternalError, None))
                    } else {
                        Ok((DBGameState::OpponentDisconnected, Some(board)))
                    }
                }
            } else {
                // Game in progress, or other state depending on "status"
                Ok((DBGameState::from(status), Some(board)))
            }
        } else if let Err(RusqliteError::QueryReturnedNoRows) = row_result {
            // No rows is either player doesn't exist or not paired
            let (exists, is_paired, _is_cyan) =
                self.check_if_player_is_paired(Some(conn), player_id)?;
            if !exists {
                Ok((DBGameState::UnknownID, None))
            } else if !is_paired {
                Ok((DBGameState::NotPaired, None))
            } else {
                unreachable!("either exists or is_paired must be false");
            }
        } else {
            // TODO use internal error enum instead of string
            Err(String::from("internal_error"))
        }
    }

    fn disconnect_player(&self, conn: Option<&Connection>, player_id: u32) -> Result<(), String> {
        if conn.is_none() {
            return self
                .disconnect_player(Some(&self.get_conn(DBFirstRun::NotFirstRun)?), player_id);
        }
        let conn = conn.unwrap();

        let stmt_result = conn.execute("DELETE FROM players WHERE id = ?;", [player_id]);
        if let Ok(1) = stmt_result {
            Ok(())
        } else {
            Err(String::from("id not found"))
        }
    }

    fn clear_empty_games(&self, conn: Option<&Connection>) -> Result<(), String> {
        if conn.is_none() {
            return self.clear_empty_games(Some(&self.get_conn(DBFirstRun::NotFirstRun)?));
        }
        let conn = conn.unwrap();

        // Only fails if no rows were removed, and that is not an issue
        conn.execute(
            "DELETE FROM games WHERE cyan_player ISNULL AND magenta_player ISNULL;",
            [],
        )
        .ok();

        Ok(())
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
