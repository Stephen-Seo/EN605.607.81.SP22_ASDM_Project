//Four Line Dropper Backend - A server enabling networked multiplayer for Four Line Dropper
//Copyright (C) 2022 Stephen Seo
//
//This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
//
//This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
//
//You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
use crate::ai::{get_ai_choice, AIDifficulty};
use crate::constants::{
    BACKEND_CLEANUP_INTERVAL_SECONDS, COLS, GAME_CLEANUP_TIMEOUT, PLAYER_CLEANUP_TIMEOUT,
    PLAYER_COUNT_LIMIT, ROWS, TURN_SECONDS,
};
use crate::state::{board_from_string, new_string_board, string_from_board, BoardState, Turn, EmoteEnum};

use std::collections::HashMap;
use std::sync::mpsc::{Receiver, RecvTimeoutError, SyncSender};
use std::time::{Duration, Instant};
use std::{fmt, thread};

use rand::{thread_rng, Rng};
use rusqlite::{params, Connection, Error as RusqliteError};

/// first value is ID, None if too many players
/// second value is true if player is cyan_player, None if not paired yet
pub type GetIDSenderType = (Option<u32>, Option<bool>);
/// first bool is player exists,
/// second bool is if paired,
/// third bool is if cyan player
pub type CheckPairingType = (bool, bool, bool);

/// second String is board string, third String is received emote type
pub type BoardStateType = (DBGameState, Option<String>, Option<EmoteEnum>);

pub type PlaceResultType = Result<(DBPlaceStatus, Option<String>), DBPlaceError>;

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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DBPlaceStatus {
    Accepted,
    GameEndedDraw,
    GameEndedCyanWon,
    GameEndedMagentaWon,
}

impl fmt::Display for DBPlaceStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            DBPlaceStatus::Accepted => write!(f, "accepted"),
            DBPlaceStatus::GameEndedDraw => write!(f, "game_ended_draw"),
            DBPlaceStatus::GameEndedCyanWon => write!(f, "game_ended_cyan_won"),
            DBPlaceStatus::GameEndedMagentaWon => write!(f, "game_ended_magenta_won"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DBPlaceError {
    NotPairedYet,
    NotYourTurn,
    Illegal,
    OpponentDisconnected,
    UnknownID,
    InternalError,
}

impl fmt::Display for DBPlaceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            DBPlaceError::NotPairedYet => write!(f, "not_paired_yet"),
            DBPlaceError::NotYourTurn => write!(f, "not_your_turn"),
            DBPlaceError::Illegal => write!(f, "illegal"),
            DBPlaceError::OpponentDisconnected => write!(f, "opponent_disconnected"),
            DBPlaceError::UnknownID => write!(f, "unknown_id"),
            DBPlaceError::InternalError => write!(f, "internal_error"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum DBHandlerRequest {
    GetID {
        response_sender: SyncSender<GetIDSenderType>,
        phrase: Option<String>,
    },
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
    PlaceToken {
        id: u32,
        pos: usize,
        response_sender: SyncSender<PlaceResultType>,
    },
    SendEmote {
        id: u32,
        emote_type: EmoteEnum,
        response_sender: SyncSender<Result<(), ()>>,
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
        let rx_recv_result = self.rx.recv_timeout(Duration::from_secs(1));
        if let Err(RecvTimeoutError::Timeout) = rx_recv_result {
            return false;
        } else if let Err(e) = rx_recv_result {
            println!("Failed to get DBHandlerRequest: {:?}", e);
            self.shutdown_tx.send(()).ok();
            return false;
        }
        let db_request = rx_recv_result.unwrap();
        match db_request {
            DBHandlerRequest::GetID {
                response_sender,
                phrase,
            } => {
                // got request to create new player, create new player
                let conn_result = self.get_conn(DBFirstRun::NotFirstRun);
                if let Err(e) = conn_result {
                    println!("{}", e);
                    return true;
                }
                let conn = conn_result.unwrap();

                let create_player_result = self.create_new_player(Some(&conn), phrase);
                if let Err(e) = create_player_result {
                    println!("{}", e);
                    response_sender.send((None, None)).ok();
                    // don't stop server because player limit may have been reached
                    return false;
                }
                let player_id = create_player_result.unwrap();

                let pair_up_result = self.pair_up_players(Some(&conn));
                if let Err(e) = pair_up_result {
                    println!("{}", e);
                    return true;
                }

                // Check if current player has been paired
                let paired_check_result = self.check_if_player_is_paired(Some(&conn), player_id);
                if let Err(e) = paired_check_result {
                    println!("{}", e);
                    return true;
                } else if let Ok((exists, paired, is_cyan)) = paired_check_result {
                    if exists {
                        if paired {
                            // don't stop server on send fail, may have timed
                            // out and dropped the receiver
                            response_sender.send((Some(player_id), Some(is_cyan))).ok();
                        } else {
                            // don't stop server on send fail, may have timed
                            // out and dropped the receiver
                            response_sender.send((Some(player_id), None)).ok();
                        }
                    } else {
                        println!("Internal error, created player doesn't exist");
                        return true;
                    }
                } else {
                    unreachable!();
                }
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
                if let Err(e) = get_board_result {
                    println!("{}", e);
                    // don't stop server on send fail, may have timed out and
                    // dropped the receiver
                    response_sender
                        .send((DBGameState::UnknownID, None, None))
                        .ok();
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
            DBHandlerRequest::PlaceToken {
                id,
                pos,
                response_sender,
            } => {
                let place_result = self.place_token(None, id, pos);
                // don't stop server on send fail, may have timed out and
                // dropped the receiver
                response_sender.send(place_result).ok();
            }
            DBHandlerRequest::SendEmote {
                id,
                emote_type,
                response_sender,
            } => {
                let result = self.create_new_sent_emote(None, id, emote_type);
                if let Err(error_string) = result {
                    println!("{}", error_string);
                    // don't stop server on send fail, may have timed
                    // out and dropped the receiver
                    response_sender.send(Err(())).ok();
                } else {
                    // don't stop server on send fail, may have timed
                    // out and dropped the receiver
                    response_sender.send(Ok(())).ok();
                }
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
                    date_added TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                    game_id INTEGER,
                    phrase TEXT,
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
                if let Err(e) = self.db_check_migration(&conn) {
                    println!("{}", e);
                }
            }

            let result = conn.execute(
                "
                CREATE TABLE games (id INTEGER PRIMARY KEY NOT NULL,
                    cyan_player INTEGER UNIQUE,
                    magenta_player INTEGER UNIQUE,
                    date_added TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                    board TEXT NOT NULL,
                    status INTEGER NOT NULL,
                    turn_time_start TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
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

            let result = conn.execute(
                "
                CREATE TABLE emotes (id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                    type TEXT NOT NULL,
                    date_added TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                    receiver_id INTEGER NOT NULL,
                    FOREIGN KEY(receiver_id) REFERENCES players (id) ON DELETE CASCADE);
            ",
                [],
            );
            if result.is_ok() {
                if first_run == DBFirstRun::FirstRun {
                    println!("Created \"emotes\" table");
                }
            } else if first_run == DBFirstRun::FirstRun {
                println!("\"emotes\" table exists");
            }

            Ok(conn)
        } else {
            Err(String::from("Failed to open connection"))
        }
    }

    fn db_check_migration(&self, conn: &Connection) -> Result<(), String> {
        let mut table_entries_stmt = conn
            .prepare("PRAGMA table_info(players);")
            .map_err(|e| format!("{:?}", e))?;
        let mut table_entries_rows = table_entries_stmt
            .query([])
            .map_err(|e| format!("{:?}", e))?;
        // check if "phrase" column exists
        let mut phrase_exists = false;
        while let Some(row) = table_entries_rows.next().map_err(|e| format!("{:?}", e))? {
            let column_name: String = row.get(1).map_err(|e| format!("{:?}", e))?;
            if column_name.contains("phrase") {
                phrase_exists = true;
            }
        }
        if !phrase_exists {
            conn.execute("ALTER TABLE players ADD COLUMN phrase TEXT;", [])
                .map_err(|e| format!("{:?}", e))?;
            println!("Added \"phrase\" column to \"players\" in db.");
        }
        Ok(())
    }

    fn create_new_player(
        &self,
        conn: Option<&Connection>,
        phrase: Option<String>,
    ) -> Result<u32, String> {
        let mut _conn_result = Err(String::new());
        let conn = if let Some(c) = conn {
            c
        } else {
            _conn_result = self.get_conn(DBFirstRun::NotFirstRun);
            _conn_result.as_ref().unwrap()
        };

        let row_result: Result<usize, _> =
            conn.query_row("SELECT count(id) FROM players;", [], |row| row.get(0));
        if let Ok(count) = row_result {
            if count > PLAYER_COUNT_LIMIT {
                return Err(String::from(
                    "Player limit reached, cannot create new players",
                ));
            }
        } else {
            return Err(String::from("Failed to get player count in db"));
        }

        let mut player_id: u32 = thread_rng().gen();
        loop {
            let exists_result = self.check_if_player_exists(Some(conn), player_id);
            if let Ok(exists) = exists_result {
                if exists {
                    player_id = thread_rng().gen();
                } else {
                    break;
                }
            } else {
                let error = exists_result.unwrap_err();
                return Err(format!(
                    "Failed to check if player exists in db: {:?}",
                    error
                ));
            }
        }

        let insert_result = conn.execute(
            "INSERT INTO players (id, phrase) VALUES (?, ?);",
            params![player_id, phrase],
        );
        if let Err(e) = insert_result {
            return Err(format!("Failed to insert player into db: {:?}", e));
        }

        Ok(player_id)
    }

    fn pair_up_players(&self, conn: Option<&Connection>) -> Result<(), String> {
        let mut _conn_result = Err(String::new());
        let conn = if let Some(c) = conn {
            c
        } else {
            _conn_result = self.get_conn(DBFirstRun::NotFirstRun);
            _conn_result.as_ref().unwrap()
        };

        let mut to_pair: Option<u32> = None;
        let mut unpaired_players_stmt = conn
            .prepare("SELECT id, phrase FROM players WHERE game_id ISNULL ORDER BY date_added;")
            .map_err(|e| format!("{:?}", e))?;
        let mut unpaired_players_rows = unpaired_players_stmt
            .query([])
            .map_err(|e| format!("{:?}", e))?;
        let mut phrase_map: HashMap<String, u32> = HashMap::new();
        while let Some(row) = unpaired_players_rows
            .next()
            .map_err(|e| format!("{:?}", e))?
        {
            if let Ok(phrase_text) = row.get::<usize, String>(1) {
                // pair players with matching phrases
                if let Some(matching_player_id) = phrase_map.get(&phrase_text) {
                    let players: [u32; 2] = [
                        *matching_player_id,
                        row.get(0).map_err(|e| format!("{:?}", e))?,
                    ];
                    self.create_game(Some(conn), &players)?;
                    phrase_map.remove(&phrase_text);
                } else {
                    phrase_map.insert(phrase_text, row.get(0).map_err(|e| format!("{:?}", e))?);
                }
            } else {
                // pair players that did not use a phrase
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
        }

        Ok(())
    }

    fn create_game(&self, conn: Option<&Connection>, players: &[u32; 2]) -> Result<u32, String> {
        let mut _conn_result = Err(String::new());
        let conn = if let Some(c) = conn {
            c
        } else {
            _conn_result = self.get_conn(DBFirstRun::NotFirstRun);
            _conn_result.as_ref().unwrap()
        };

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
            "INSERT INTO games (id, cyan_player, magenta_player, board, status) VALUES (?, ?, ?, ?, 0);",
            params![game_id, players[0], players[1], new_string_board()]
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

        let mut _conn_result = Err(String::new());
        let conn = if let Some(c) = conn {
            c
        } else {
            _conn_result = self.get_conn(DBFirstRun::NotFirstRun);
            _conn_result.as_ref().unwrap()
        };

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
        let mut _conn_result = Err(String::new());
        let conn = if let Some(c) = conn {
            c
        } else {
            _conn_result = self.get_conn(DBFirstRun::NotFirstRun);
            _conn_result.as_ref().unwrap()
        };

        let check_player_row: Result<u32, _> =
            conn.query_row("SELECT id FROM players WHERE id = ?;", [player_id], |row| {
                row.get(0)
            });
        if let Ok(_id) = check_player_row {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn check_if_player_in_game(
        &self,
        conn: Option<&Connection>,
        player_id: u32,
    ) -> Result<bool, String> {
        let mut _conn_result = Err(String::new());
        let conn = if let Some(c) = conn {
            c
        } else {
            _conn_result = self.get_conn(DBFirstRun::NotFirstRun);
            _conn_result.as_ref().unwrap()
        };

        let check_player_game_row: Result<u32, _> = conn.query_row(
            "SELECT games.id FROM games JOIN players WHERE players.id = ? AND players.game_id NOTNULL AND players.game_id = games.id;",
            [player_id],
            |row| row.get(0));
        if check_player_game_row.is_ok() {
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
        let mut _conn_result = Err(String::new());
        let conn = if let Some(c) = conn {
            c
        } else {
            _conn_result = self.get_conn(DBFirstRun::NotFirstRun);
            _conn_result.as_ref().unwrap()
        };

        let mut received_emote: Option<EmoteEnum> = None;
        {
            let row_result: Result<(u64, String), RusqliteError> = conn.query_row(
                "SELECT id, type FROM emotes WHERE receiver_id = ? ORDER BY date_added ASC;",
                [player_id],
                |row| {
                    Ok((
                        row.get(0).expect("emotes.id should exist"),
                        row.get(1).expect("emotes.type should exist"),
                    ))
                },
            );
            if let Err(RusqliteError::QueryReturnedNoRows) = row_result {
                // no-op
            } else if let Err(e) = row_result {
                println!("Error while fetching received emotes: {:?}", e);
            } else {
                let (emote_id, emote_type) = row_result.unwrap();
                received_emote = emote_type.as_str().try_into().ok();
                if received_emote.is_none() {
                    println!("WARNING: Invalid emote type \"{}\" in db", emote_type);
                }
                conn.execute("DELETE FROM emotes WHERE id = ?;", [emote_id])
                    .ok();
            }
        }

        // TODO maybe handle "opponent_disconnected" case
        let row_result: Result<(String, i64, Option<u32>, Option<u32>), RusqliteError> = conn.query_row(
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
                Ok((DBGameState::InternalError, None, received_emote))
            } else if cyan_opt.is_none() || magenta_opt.is_none() {
                // One player disconnected
                self.disconnect_player(Some(conn), player_id).ok();
                // Remove the game(s) with disconnected players
                if self.clear_empty_games(Some(conn)).is_err() {
                    Ok((DBGameState::InternalError, None, received_emote))
                } else if status == 2 || status == 3 {
                    Ok((DBGameState::from(status), Some(board), received_emote))
                } else {
                    Ok((
                        DBGameState::OpponentDisconnected,
                        Some(board),
                        received_emote,
                    ))
                }
            } else {
                // Game in progress, or other state depending on "status"
                Ok((DBGameState::from(status), Some(board), received_emote))
            }
        } else if let Err(RusqliteError::QueryReturnedNoRows) = row_result {
            // No rows is either player doesn't exist or not paired
            let (exists, is_paired, _is_cyan) =
                self.check_if_player_is_paired(Some(conn), player_id)?;
            if !exists {
                Ok((DBGameState::UnknownID, None, received_emote))
            } else if !is_paired {
                Ok((DBGameState::NotPaired, None, received_emote))
            } else {
                unreachable!("either exists or is_paired must be false");
            }
        } else {
            // TODO use internal error enum instead of string
            Err(String::from("internal_error"))
        }
    }

    fn disconnect_player(&self, conn: Option<&Connection>, player_id: u32) -> Result<(), String> {
        let mut _conn_result = Err(String::new());
        let conn = if let Some(c) = conn {
            c
        } else {
            _conn_result = self.get_conn(DBFirstRun::NotFirstRun);
            _conn_result.as_ref().unwrap()
        };

        let stmt_result = conn.execute("DELETE FROM players WHERE id = ?;", [player_id]);
        if let Ok(1) = stmt_result {
            Ok(())
        } else {
            Err(String::from("id not found"))
        }
    }

    fn clear_empty_games(&self, conn: Option<&Connection>) -> Result<(), String> {
        let mut _conn_result = Err(String::new());
        let conn = if let Some(c) = conn {
            c
        } else {
            _conn_result = self.get_conn(DBFirstRun::NotFirstRun);
            _conn_result.as_ref().unwrap()
        };

        // Only fails if no rows were removed, and that is not an issue
        conn.execute(
            "DELETE FROM games WHERE cyan_player ISNULL AND magenta_player ISNULL;",
            [],
        )
        .ok();

        Ok(())
    }

    fn place_token(
        &self,
        conn: Option<&Connection>,
        player_id: u32,
        pos: usize,
    ) -> PlaceResultType {
        let mut _conn_result = Err(String::new());
        let conn = if let Some(c) = conn {
            c
        } else {
            _conn_result = self.get_conn(DBFirstRun::NotFirstRun);
            _conn_result.as_ref().unwrap()
        };

        // check if player exists
        let player_exist_check_result = self.check_if_player_exists(Some(conn), player_id);
        if let Ok(exists) = player_exist_check_result {
            if !exists {
                return Err(DBPlaceError::UnknownID);
            }
        } else {
            return Err(DBPlaceError::InternalError);
        }

        // check if player belongs to a game
        let player_game_result = self.check_if_player_in_game(Some(conn), player_id);
        if let Ok(is_in_game) = player_game_result {
            if !is_in_game {
                return Err(DBPlaceError::NotPairedYet);
            }
        } else {
            return Err(DBPlaceError::InternalError);
        }

        // check if player is cyan or magenta
        let query_result_result: Result<Result<(bool, u32, String), DBPlaceError>, _> =
                conn.query_row(
                    "SELECT cyan_player, magenta_player, status, board FROM games JOIN players WHERE players.id = ? AND players.game_id = games.id;",
                    [player_id],
                    |row| {
            let cyan_id_result: Result<Option<u32>, _> = row.get(0);
            let magenta_id_result: Result<Option<u32>, _> = row.get(1);
            let status_result: Result<u32, _> = row.get(2);
            let board_result: Result<String, _> = row.get(3);
            if status_result.is_err() {
                return status_result.map(|_| Ok((false, 0, "".into())));
            }
            let status: u32 = status_result.unwrap();
            if board_result.is_err() {
                return board_result.map(|_| Ok((false, 0, "".into())));
            }
            let board = board_result.unwrap();
            if cyan_id_result.is_ok() && magenta_id_result.is_ok() {
                if let (Ok(cyan_id_opt), Ok(magenta_id_opt)) = (cyan_id_result, magenta_id_result) {
                    if let (Some(cyan_id), Some(_magenta_id)) = (cyan_id_opt, magenta_id_opt) {
                        Ok(Ok((cyan_id == player_id, status, board)))
                    } else if (2..=4).contains(&status) {
                        // game has ended, don't return error
                        // first result will be safely ignored
                        Ok(Ok((false, status, board)))
                    } else {
                        Ok(Err(DBPlaceError::OpponentDisconnected))
                    }
                } else {
                    unreachable!("both row items should be Ok")
                }
            } else if cyan_id_result.is_err() {
                cyan_id_result.map(|_| Err(DBPlaceError::InternalError))
            } else {
                magenta_id_result.map(|_| Err(DBPlaceError::InternalError))
            }
        });

        let query_result = query_result_result.map_err(|_| DBPlaceError::InternalError)?;

        // if opponent has disconnected, disconnect the remaining player as well
        if let Err(DBPlaceError::OpponentDisconnected) = query_result {
            self.disconnect_player(Some(conn), player_id).ok();
            if self.clear_empty_games(Some(conn)).is_err() {
                return Err(DBPlaceError::InternalError);
            }
        }

        let (is_cyan, status, board_string) = query_result?;

        match status {
            0 => {
                // cyan's turn
                if !is_cyan {
                    return Err(DBPlaceError::NotYourTurn);
                }
            }
            1 => {
                // magenta's turn
                if is_cyan {
                    return Err(DBPlaceError::NotYourTurn);
                }
            }
            2 => {
                // game over, cyan won
                return Ok((DBPlaceStatus::GameEndedCyanWon, Some(board_string)));
            }
            3 => {
                // game over, magenta won
                return Ok((DBPlaceStatus::GameEndedMagentaWon, Some(board_string)));
            }
            4 => {
                // game over, draw
                return Ok((DBPlaceStatus::GameEndedDraw, Some(board_string)));
            }
            _ => (),
        }

        // get board state
        let board = board_from_string(board_string);

        // find placement position or return "illegal move" if unable to
        let mut final_pos = pos;
        loop {
            if board[final_pos].get() == BoardState::Empty {
                if final_pos + COLS as usize >= board.len()
                    || board[final_pos + COLS as usize].get() != BoardState::Empty
                {
                    break;
                } else if board[final_pos + COLS as usize].get() == BoardState::Empty {
                    final_pos += COLS as usize;
                }
            } else {
                return Err(DBPlaceError::Illegal);
            }
        }

        // place into board
        if is_cyan {
            board[final_pos].replace(BoardState::Cyan);
        } else {
            board[final_pos].replace(BoardState::Magenta);
        }

        // board back to string
        let (board_string, ended_state_opt) = string_from_board(&board, final_pos);

        // update DB
        let update_result = if ended_state_opt.is_none() {
            conn.execute(
                "UPDATE games SET status = ?, board = ?, turn_time_start = datetime() FROM players WHERE players.game_id = games.id AND players.id = ?;",
                params![if status == 0 { 1u8 }
                            else { 0u8 },
                        board_string,
                        player_id]
            )
        } else {
            conn.execute(
                "UPDATE games SET status = ?, board = ? FROM players WHERE players.game_id = games.id AND players.id = ?;",
                params![if ended_state_opt.unwrap() == BoardState::Empty { 4u8 }
                            else if ended_state_opt.unwrap() == BoardState::CyanWin { 2u8 }
                            else { 3u8 },
                        board_string,
                        player_id]
            )
        };

        if let Err(_e) = update_result {
            return Err(DBPlaceError::InternalError);
        } else if let Ok(count) = update_result {
            if count != 1 {
                return Err(DBPlaceError::InternalError);
            }
        }

        if let Some(ended_state) = ended_state_opt {
            Ok((
                match ended_state {
                    BoardState::Empty => DBPlaceStatus::GameEndedDraw,
                    BoardState::Cyan | BoardState::Magenta => unreachable!(),
                    BoardState::CyanWin => DBPlaceStatus::GameEndedCyanWon,
                    BoardState::MagentaWin => DBPlaceStatus::GameEndedMagentaWon,
                },
                Some(board_string),
            ))
        } else {
            Ok((DBPlaceStatus::Accepted, Some(board_string)))
        }
    }

    fn check_turn_times(&self) -> Result<(), String> {
        let conn = self.get_conn(DBFirstRun::NotFirstRun)?;

        let mut prepared_stmt = conn
            .prepare(
                "SELECT id, status, board FROM games WHERE unixepoch() - unixepoch(turn_time_start) > ? AND cyan_player NOTNULL and magenta_player NOTNULL AND status < 2;",
            )
            .map_err(|_| String::from("Failed to prepare db query based on turn time"))?;

        let rows = prepared_stmt
            .query_map([TURN_SECONDS], |row| {
                let id_result = row.get(0);
                let status_result = row.get(1);
                let board_result = row.get(2);
                if id_result.is_ok() && status_result.is_ok() && board_result.is_ok() {
                    if let (Ok(id), Ok(status), Ok(board)) =
                        (id_result, status_result, board_result)
                    {
                        Ok((id, status, board))
                    } else {
                        unreachable!();
                    }
                } else if id_result.is_err() {
                    id_result.map(|_| (0, 0, String::new()))
                } else if status_result.is_err() {
                    status_result.map(|_| (0, 0, String::new()))
                } else {
                    board_result.map(|_| (0, 0, String::new()))
                }
            })
            .map_err(|_| String::from("Failed to query db based on turn time"))?;

        for row_result in rows {
            if let Ok((id, status, board)) = row_result {
                self.have_ai_take_players_turn(Some(&conn), id, status, board)?;
            } else {
                unreachable!("This part should never execute");
            }
        }

        Ok(())
    }

    fn have_ai_take_players_turn(
        &self,
        conn: Option<&Connection>,
        game_id: u32,
        status: u32,
        board_string: String,
    ) -> Result<(), String> {
        if status > 1 {
            return Err(String::from(
                "have_ai_take_players_turn: got invalid status",
            ));
        }

        let mut _conn_result = Err(String::new());
        let conn = if let Some(c) = conn {
            c
        } else {
            _conn_result = self.get_conn(DBFirstRun::NotFirstRun);
            _conn_result.as_ref().unwrap()
        };

        let is_cyan = status == 0;
        let board = board_from_string(board_string);
        let mut ai_choice_pos: usize = get_ai_choice(
            AIDifficulty::Hard,
            if is_cyan {
                Turn::CyanPlayer
            } else {
                Turn::MagentaPlayer
            },
            &board,
        )
        .map_err(|_| String::from("Failed to get ai choice on turn timeout"))?
        .into();

        if board[ai_choice_pos].get() != BoardState::Empty {
            return Err(String::from("ai returned illegal move on turn timeout"));
        }

        // get final position of token
        loop {
            if board.len() <= ai_choice_pos + COLS as usize {
                break;
            } else if board[ai_choice_pos + COLS as usize].get() == BoardState::Empty {
                ai_choice_pos += COLS as usize;
            } else {
                break;
            }
        }

        // place token
        board[ai_choice_pos].replace(if is_cyan {
            BoardState::Cyan
        } else {
            BoardState::Magenta
        });

        // get board string from board while checking if game has ended
        let (board_string, end_state_opt) = string_from_board(&board, ai_choice_pos);

        let state;
        if let Some(board_state) = end_state_opt {
            if board_state == BoardState::Empty {
                state = 4;
            } else if board_state.from_win() == BoardState::Cyan {
                state = 2;
            } else if board_state.from_win() == BoardState::Magenta {
                state = 3;
            } else {
                unreachable!();
            }
        } else {
            state = if is_cyan { 1 } else { 0 };
        }

        conn.execute(
            "UPDATE games SET board = ?, status = ?, turn_time_start = datetime() WHERE id = ?;",
            params![board_string, state, game_id],
        )
        .map_err(|_| String::from("Failed to update game with ai choice on turn timeout"))?;

        Ok(())
    }

    fn cleanup_stale_games(&self, conn: Option<&Connection>) -> Result<(), String> {
        let mut _conn_result = Err(String::new());
        let conn = if let Some(c) = conn {
            c
        } else {
            _conn_result = self.get_conn(DBFirstRun::NotFirstRun);
            _conn_result.as_ref().unwrap()
        };

        conn.execute(
            "DELETE FROM games WHERE unixepoch() - unixepoch(date_added) > ?;",
            [GAME_CLEANUP_TIMEOUT],
        )
        .ok();

        Ok(())
    }

    fn cleanup_stale_players(&self, conn: Option<&Connection>) -> Result<(), String> {
        let mut _conn_result = Err(String::new());
        let conn = if let Some(c) = conn {
            c
        } else {
            _conn_result = self.get_conn(DBFirstRun::NotFirstRun);
            _conn_result.as_ref().unwrap()
        };

        conn.execute(
            "DELETE FROM players WHERE unixepoch() - unixepoch(date_added) > ? AND game_id ISNULL;",
            [PLAYER_CLEANUP_TIMEOUT],
        )
        .ok();

        Ok(())
    }

    fn cleanup_stale_emotes(&self, conn: Option<&Connection>) -> Result<(), String> {
        let mut _conn_result = Err(String::new());
        let conn = if let Some(c) = conn {
            c
        } else {
            _conn_result = self.get_conn(DBFirstRun::NotFirstRun);
            _conn_result.as_ref().unwrap()
        };

        conn.execute(
            "DELETE FROM emotes WHERE unixepoch() - unixepoch(date_added) > ?;",
            [GAME_CLEANUP_TIMEOUT],
        )
        .ok();
        Ok(())
    }

    fn create_new_sent_emote(
        &self,
        conn: Option<&Connection>,
        sender_id: u32,
        emote: EmoteEnum,
    ) -> Result<(), String> {
        let mut _conn_result = Err(String::new());
        let conn = if let Some(c) = conn {
            c
        } else {
            _conn_result = self.get_conn(DBFirstRun::NotFirstRun);
            _conn_result.as_ref().unwrap()
        };

        let mut prepared_stmt = conn.prepare("SELECT games.cyan_player, games.magenta_player FROM games JOIN players WHERE players.id = ?, games.id = players.game_id;")
            .map_err(|_| String::from("Failed to prepare db query for getting opponent id for sending emote"))?;
        let row_result: Result<(Option<u32>, Option<u32>), RusqliteError> =
            prepared_stmt.query_row([sender_id], |row| Ok((row.get(0).ok(), row.get(1).ok())));
        if let Err(RusqliteError::QueryReturnedNoRows) = row_result {
            return Err(String::from("Failed to send emote, game doesn't exist"));
        } else if let Err(e) = row_result {
            return Err(format!("Failed to send emote: {:?}", e));
        }
        let (cyan_player_opt, magenta_player_opt) = row_result.unwrap();
        if cyan_player_opt.is_none() {
            return Err(String::from(
                "Failed to send emote, cyan player disconnected",
            ));
        } else if magenta_player_opt.is_none() {
            return Err(String::from(
                "Failed to send emote, magenta player disconnected",
            ));
        }
        let cyan_player_id = cyan_player_opt.unwrap();
        let magenta_player_id = magenta_player_opt.unwrap();

        let receiver_id = if cyan_player_id == sender_id {
            magenta_player_id
        } else {
            cyan_player_id
        };

        conn.execute(
            "INSERT INTO emotes (type, receiver_id) VALUES (?, ?);",
            params![String::from(emote), receiver_id],
        )
        .map_err(|_| {
            format!(
                "Failed to store emote from player {} to player {}",
                sender_id, receiver_id
            )
        })?;

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

        let mut cleanup_instant = Instant::now();
        let cleanup_duration = Duration::from_secs(BACKEND_CLEANUP_INTERVAL_SECONDS);
        'outer: loop {
            if handler.handle_request() {
                handler.shutdown_tx.send(()).ok();
                break 'outer;
            }

            if let Err(e) = handler.check_turn_times() {
                println!("{}", e);
            }

            if cleanup_instant.elapsed() > cleanup_duration {
                cleanup_instant = Instant::now();
                if let Err(e) = handler.cleanup_stale_games(None) {
                    println!("{}", e);
                }
                if let Err(e) = handler.cleanup_stale_players(None) {
                    println!("{}", e);
                }
                if let Err(e) = handler.cleanup_stale_emotes(None) {
                    println!("{}", e);
                }
                if let Err(e) = handler.clear_empty_games(None) {
                    println!("{}", e);
                }
            }
        }
    });
}
