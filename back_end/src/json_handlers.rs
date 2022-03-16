use serde_json::Value;

pub fn handle_json(root: Value) -> Result<String, String> {
    if let Some(Value::String(type_str)) = root.get("type") {
        match type_str.as_str() {
            "pairing_request" => handle_pairing_request(root),
            "check_pairing" => handle_check_pairing(root),
            "place_token" => handle_place_token(root),
            "whose_turn" => handle_whose_turn(root),
            "disconnect" => handle_disconnect(root),
            "request_board_state" => handle_request_board_state(root),
            "game_state" => handle_game_state(root),
            _ => {
                Err("{\"type\":\"invalid_type\"}".into())
            }
        }
    } else {
        Err("{\"type\":\"invalid_json\"}".into())
    }
}

fn handle_pairing_request(root: Value) -> Result<String, String> {
    Err("{\"type\":\"unimplemented\"}".into())
}

fn handle_check_pairing(root: Value) -> Result<String, String> {
    Err("{\"type\":\"unimplemented\"}".into())
}

fn handle_place_token(root: Value) -> Result<String, String> {
    Err("{\"type\":\"unimplemented\"}".into())
}

fn handle_whose_turn(root: Value) -> Result<String, String> {
    Err("{\"type\":\"unimplemented\"}".into())
}

fn handle_disconnect(root: Value) -> Result<String, String> {
    Err("{\"type\":\"unimplemented\"}".into())
}

fn handle_request_board_state(root: Value) -> Result<String, String> {
    Err("{\"type\":\"unimplemented\"}".into())
}

fn handle_game_state(root: Value) -> Result<String, String> {
    Err("{\"type\":\"unimplemented\"}".into())
}
