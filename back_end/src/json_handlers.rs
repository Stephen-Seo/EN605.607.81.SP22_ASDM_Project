//Four Line Dropper Backend - A server enabling networked multiplayer for Four Line Dropper
//Copyright (C) 2022 Stephen Seo
//
//This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
//
//This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
//
//You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
use crate::db_handler::{CheckPairingType, DBHandlerRequest, GetIDSenderType};

use std::{
    sync::mpsc::{sync_channel, SyncSender},
    time::Duration,
};

use serde_json::Value;

const DB_REQUEST_TIMEOUT: Duration = Duration::from_secs(5);

pub fn handle_json(
    root: Value,
    tx: SyncSender<DBHandlerRequest>,
    _shutdown_tx: SyncSender<()>, // maybe used here, not sure if it will be
) -> Result<String, String> {
    if let Some(Value::String(type_str)) = root.get("type") {
        match type_str.as_str() {
            "pairing_request" => handle_pairing_request(root, tx),
            "check_pairing" => handle_check_pairing(root, tx),
            "place_token" => handle_place_token(root, tx),
            "disconnect" => handle_disconnect(root, tx),
            "game_state" => handle_game_state(root, tx),
            _ => Err("{\"type\":\"invalid_type\"}".into()),
        }
    } else {
        Err("{\"type\":\"invalid_json\"}".into())
    }
}

fn handle_pairing_request(root: Value, tx: SyncSender<DBHandlerRequest>) -> Result<String, String> {
    let (player_tx, player_rx) = sync_channel::<GetIDSenderType>(1);
    let mut phrase: Option<String> = None;
    if let Some(phrase_text) = root.get("phrase") {
        if let Some(phrase_str) = phrase_text.as_str() {
            if !phrase_str.is_empty() {
                phrase = Some(phrase_str.to_owned());
            }
        }
    }
    if tx
        .send(DBHandlerRequest::GetID {
            response_sender: player_tx,
            phrase,
        })
        .is_err()
    {
        return Err("{\"type\":\"pairing_response\", \"status\":\"internal_error\"}".into());
    }
    if let Ok((pid_opt, is_cyan_opt)) = player_rx.recv_timeout(DB_REQUEST_TIMEOUT) {
        if pid_opt.is_none() {
            return Ok("{\"type\":\"pairing_response\", \"status\":\"too_many_players\"}".into());
        }
        let pid = pid_opt.unwrap();
        if let Some(is_cyan) = is_cyan_opt {
            Ok(format!(
                "{{\"type\":\"pairing_response\", \"id\": {}, \"status\": \"paired\", \"color\": \"{}\"}}",
                pid,
                if is_cyan { "cyan" } else { "magenta" }
            ))
        } else {
            Ok(format!(
                "{{\"type\":\"pairing_response\", \"id\": {}, \"status\": \"waiting\"}}",
                pid
            ))
        }
    } else {
        Err("{\"type\":\"pairing_response\", \"status\":\"internal_error_timeout\"}".into())
    }
}

fn handle_check_pairing(root: Value, tx: SyncSender<DBHandlerRequest>) -> Result<String, String> {
    let id_option = root.get("id");
    if id_option.is_none() {
        return Err("{\"type\":\"invalid_syntax\"}".into());
    }
    let player_id = id_option
        .unwrap()
        .as_u64()
        .ok_or_else(|| String::from("{\"type\":\"invalid_syntax\"}"))?;
    let player_id: u32 = player_id
        .try_into()
        .map_err(|_| String::from("{\"type\":\"invalid_syntax\"}"))?;
    let (request_tx, request_rx) = sync_channel::<CheckPairingType>(1);
    if tx
        .send(DBHandlerRequest::CheckPairing {
            id: player_id,
            response_sender: request_tx,
        })
        .is_err()
    {
        return Err("{\"type\":\"pairing_status\", \"status\":\"internal_error\"}".into());
    }
    if let Ok((exists, is_paired, is_cyan)) = request_rx.recv_timeout(DB_REQUEST_TIMEOUT) {
        if !exists {
            Err("{\"type\":\"pairing_status\", \"status\":\"unknown_id\"}".into())
        } else if is_paired {
            Ok(format!(
                "{{\"type\":\"pairing_status\", \"status\":\"paired\", \"color\":\"{}\"}}",
                if is_cyan { "cyan" } else { "magenta" }
            ))
        } else {
            Ok("{\"type\":\"pairing_status\", \"status\":\"waiting\"}".into())
        }
    } else {
        Err("{\"type\":\"pairing_status\", \"status\":\"internal_error_timeout\"}".into())
    }
}

fn handle_place_token(root: Value, tx: SyncSender<DBHandlerRequest>) -> Result<String, String> {
    let id_option = root.get("id");
    if id_option.is_none() {
        return Err("{\"type\":\"invalid_syntax\"}".into());
    }
    let player_id = id_option
        .unwrap()
        .as_u64()
        .ok_or_else(|| String::from("{\"type\":\"invalid_syntax\"}"))?;
    let player_id: u32 = player_id
        .try_into()
        .map_err(|_| String::from("{\"type\":\"invalid_syntax\"}"))?;

    let position_option = root.get("position");
    if position_option.is_none() {
        return Err("{\"type\":\"invalid_syntax\"}".into());
    }
    let position = position_option
        .unwrap()
        .as_u64()
        .ok_or_else(|| String::from("{\"type\":\"invalid_syntax\"}"))?;
    let position: usize = position
        .try_into()
        .map_err(|_| String::from("{\"type\":\"invalid_syntax\"}"))?;

    let (resp_tx, resp_rx) = sync_channel(1);

    if tx
        .send(DBHandlerRequest::PlaceToken {
            id: player_id,
            pos: position,
            response_sender: resp_tx,
        })
        .is_err()
    {
        return Err(String::from(
            "{\"type\":\"place_token\", \"status\":\"internal_error\"}",
        ));
    }

    let place_result = resp_rx.recv_timeout(DB_REQUEST_TIMEOUT);
    if let Ok(Ok((place_status, board_opt))) = place_result {
        if let Some(board_string) = board_opt {
            Ok(format!(
                "{{\"type\":\"place_token\", \"status\":\"{}\", \"board\":\"{}\"}}",
                place_status, board_string
            ))
        } else {
            Ok(format!(
                "{{\"type\":\"place_token\", \"status\":\"{}\"}}",
                place_status
            ))
        }
    } else if let Ok(Err(place_error)) = place_result {
        Err(format!(
            "{{\"type\":\"place_token\", \"status\":\"{}\"}}",
            place_error
        ))
    } else {
        Err(String::from(
            "{\"type\":\"place_token\", \"status\":\"internal_error\"}",
        ))
    }
}

fn handle_disconnect(root: Value, tx: SyncSender<DBHandlerRequest>) -> Result<String, String> {
    let id_option = root.get("id");
    if id_option.is_none() {
        return Err("{\"type\":\"invalid_syntax\"}".into());
    }
    let player_id = id_option
        .unwrap()
        .as_u64()
        .ok_or_else(|| String::from("{\"type\":\"invalid_syntax\"}"))?;
    let player_id: u32 = player_id
        .try_into()
        .map_err(|_| String::from("{\"type\":\"invalid_syntax\"}"))?;

    let (resp_tx, resp_rx) = sync_channel(1);

    if tx
        .send(DBHandlerRequest::DisconnectID {
            id: player_id,
            response_sender: resp_tx,
        })
        .is_err()
    {
        return Err(String::from(
            "{\"type\":\"disconnect\", \"status\":\"internal_error\"}",
        ));
    }

    if let Ok(was_removed) = resp_rx.recv_timeout(DB_REQUEST_TIMEOUT) {
        if was_removed {
            Ok(String::from("{\"type\":\"disconnect\", \"status\":\"ok\"}"))
        } else {
            Ok(String::from(
                "{\"type\":\"disconnect\", \"status\":\"unknown_id\"}",
            ))
        }
    } else {
        Err(String::from(
            "{\"type\":\"disconnect\", \"status\":\"internal_error\"}",
        ))
    }
}

fn handle_game_state(root: Value, tx: SyncSender<DBHandlerRequest>) -> Result<String, String> {
    let id_option = root.get("id");
    if id_option.is_none() {
        return Err("{\"type\":\"invalid_syntax\"}".into());
    }
    let player_id = id_option
        .unwrap()
        .as_u64()
        .ok_or_else(|| String::from("{\"type\":\"invalid_syntax\"}"))?;
    let player_id: u32 = player_id
        .try_into()
        .map_err(|_| String::from("{\"type\":\"invalid_syntax\"}"))?;

    let (resp_tx, resp_rx) = sync_channel(1);

    if tx
        .send(DBHandlerRequest::GetGameState {
            id: player_id,
            response_sender: resp_tx,
        })
        .is_err()
    {
        return Err("{\"type\":\"game_state\", \"status\":\"internal_error\"}".into());
    }

    if let Ok((db_game_state, board_string_opt)) = resp_rx.recv_timeout(DB_REQUEST_TIMEOUT) {
        if let Some(board_string) = board_string_opt {
            Ok(format!(
                "{{\"type\":\"game_state\", \"status\":\"{}\", \"board\":\"{}\"}}",
                db_game_state, board_string
            ))
        } else {
            Ok(format!(
                "{{\"type\":\"game_state\", \"status\":\"{}\"}}",
                db_game_state
            ))
        }
    } else {
        Err("{\"type\":\"game_state\", \"status\":\"internal_error_timeout\"}".into())
    }
}
