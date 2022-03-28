use std::sync::mpsc::{Receiver, SyncSender};
use std::thread;

use rand::{thread_rng, Rng};
use rusqlite::Connection;

#[derive(Clone, Debug)]
pub enum DBHandlerRequest {
    GetID(SyncSender<u32>),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum DBFirstRun {
    FirstRun,
    NotFirstRun,
}

fn init_conn(sqlite_path: &str, first_run: DBFirstRun) -> Result<Connection, String> {
    if let Ok(conn) = Connection::open(sqlite_path) {
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
                cyan_player INTEGER UNIQUE NOT NULL,
                magenta_player INTEGER UNIQUE NOT NULL,
                date_added TEXT NOT NULL,
                date_changed TEXT NOT NULL,
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

pub fn start_db_handler_thread(
    rx: Receiver<DBHandlerRequest>,
    sqlite_path: String,
    shutdown_tx: SyncSender<()>,
) {
    thread::spawn(move || {
        // temporarily get conn which should initialize on first setup of db
        if let Ok(_conn) = init_conn(&sqlite_path, DBFirstRun::FirstRun) {
        } else {
            println!("ERROR: Failed init sqlite db connection");
            shutdown_tx.send(()).ok();
            return;
        }

        'outer: loop {
            let rx_recv_result = rx.recv();
            if let Err(e) = rx_recv_result {
                println!("Failed to get DBHandlerRequest: {:?}", e);
                shutdown_tx.send(()).ok();
                break;
            }
            let db_request = rx_recv_result.unwrap();
            match db_request {
                DBHandlerRequest::GetID(player_tx) => {
                    // got request to create new player, create new player
                    let mut player_id: u32 = thread_rng().gen();
                    let conn_result = init_conn(&sqlite_path, DBFirstRun::NotFirstRun);
                    if let Err(e) = conn_result {
                        println!("Failed to get sqlite db connection: {:?}", e);
                        shutdown_tx.send(()).ok();
                        break;
                    }
                    let conn = conn_result.unwrap();
                    loop {
                        let stmt_result = conn.prepare("SELECT id FROM players WHERE id = ?;");
                        if let Err(e) = stmt_result {
                            println!("Failed to create sqlite statement: {:?}", e);
                            shutdown_tx.send(()).ok();
                            break 'outer;
                        }
                        let mut stmt = stmt_result.unwrap();
                        match stmt.query_row([player_id], |_row| Ok(())) {
                            Ok(_) => {
                                player_id = thread_rng().gen();
                            }
                            Err(_) => break,
                        }
                    }
                    let insert_result = conn.execute(
                        "INSERT INTO players (id, date_added) VALUES (?, datetime());",
                        [player_id],
                    );
                    if let Err(e) = insert_result {
                        println!("Failed to insert into sqlite db: {:?}", e);
                        shutdown_tx.send(()).ok();
                        break 'outer;
                    }
                    let send_result = player_tx.send(player_id);
                    if let Err(e) = send_result {
                        println!("Failed to send back player id: {:?}", e);
                        shutdown_tx.send(()).ok();
                        break 'outer;
                    }
                    send_result.unwrap();
                }
            }

            // Pair up players
            // TODO
        } // loop end
    });
}
