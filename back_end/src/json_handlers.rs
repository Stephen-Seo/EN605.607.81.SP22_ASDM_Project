use std::{
    sync::mpsc::{sync_channel, SyncSender},
    time::Duration,
};

use serde_json::Value;

pub fn handle_json(
    root: Value,
    tx: SyncSender<SyncSender<u32>>,
    _shutdown_tx: SyncSender<()>, // maybe used here, not sure if it will be
) -> Result<String, String> {
    if let Some(Value::String(type_str)) = root.get("type") {
        match type_str.as_str() {
            "pairing_request" => handle_pairing_request(tx),
            "check_pairing" => handle_check_pairing(root),
            "place_token" => handle_place_token(root),
            "disconnect" => handle_disconnect(root),
            "game_state" => handle_game_state(root),
            _ => Err("{\"type\":\"invalid_type\"}".into()),
        }
    } else {
        Err("{\"type\":\"invalid_json\"}".into())
    }
}

fn handle_pairing_request(tx: SyncSender<SyncSender<u32>>) -> Result<String, String> {
    let (player_tx, player_rx) = sync_channel::<u32>(1);
    if tx.send(player_tx).is_err() {
        return Err("{\"type\":\"pairing_response\", \"status\":\"internal_error\"}".into());
    }
    if let Ok(pid) = player_rx.recv_timeout(Duration::from_secs(5)) {
        Ok(format!(
            "{{\"type\":\"pairing_response\", \"id\": \"{}\", \"status\": \"waiting\"}}",
            pid
        ))
    } else {
        Err("{\"type\":\"pairing_response\", \"status\":\"internal_error_timeout\"}".into())
    }
}

fn handle_check_pairing(root: Value) -> Result<String, String> {
    Err("{\"type\":\"unimplemented\"}".into())
}

fn handle_place_token(root: Value) -> Result<String, String> {
    Err("{\"type\":\"unimplemented\"}".into())
}

fn handle_disconnect(root: Value) -> Result<String, String> {
    Err("{\"type\":\"unimplemented\"}".into())
}

fn handle_game_state(root: Value) -> Result<String, String> {
    Err("{\"type\":\"unimplemented\"}".into())
}
