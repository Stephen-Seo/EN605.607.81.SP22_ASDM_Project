use std::sync::mpsc::{Receiver, SyncSender};
use std::thread;

use rand::{thread_rng, Rng};
use rusqlite::Connection;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum DBFirstRun {
    FirstRun,
    NotFirstRun,
}

fn init_conn(sqlite_path: &str, first_run: DBFirstRun) -> Result<Connection, String> {
    if let Ok(conn) = Connection::open(sqlite_path) {
        conn.execute("PRAGMA foreign_keys = ON;", [])
            .expect("Should be able to enable \"foreign_keys\"");
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
        Err(String::from("Failed to get connection"))
    }
}

pub fn start_db_handler_thread(rx: Receiver<SyncSender<u32>>, sqlite_path: String) {
    thread::spawn(move || {
        // temporarily get conn which should initialize on first setup of db
        if let Ok(_conn) = init_conn(&sqlite_path, DBFirstRun::FirstRun) {
        } else {
            println!("ERROR: Failed init sqlite db connection");
            return;
        }

        loop {
            let result = rx.recv();
            //println!("db_handler: Got result from rx");

            if let Ok(player_tx) = result {
                //println!("db_handler: Got player_tx from rx");
                // got request to create new player, create new player
                let mut player_id: u32 = thread_rng().gen();
                let conn = init_conn(&sqlite_path, DBFirstRun::NotFirstRun)
                    .expect("DB connection should be available");
                loop {
                    let mut stmt = conn
                        .prepare("SELECT id FROM players WHERE id = ?;")
                        .expect("Should be able to prepare DB statement");
                    match stmt.query_row([player_id], |_row| Ok(())) {
                        Ok(_) => {
                            player_id = thread_rng().gen();
                        }
                        Err(_) => break,
                    }
                }
                conn.execute(
                    "INSERT INTO players (id, date_added) VALUES (?, datetime());",
                    [player_id],
                )
                .unwrap_or_else(|_| {
                    panic!("Should be able to insert new player with id {}", player_id)
                });
                player_tx
                    .send(player_id)
                    .expect("Should be able to send back valid player id");
            } else {
                println!("db_handler: Failed to get player_tx");
            }
            // Pair up players
            // TODO
        } // loop end
    });
}
