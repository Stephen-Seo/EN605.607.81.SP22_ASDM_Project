mod db_handler;
mod json_handlers;

const SQLITE_DB_PATH: &str = "./fourLineDropper.db";

use std::sync::mpsc::{sync_channel, SyncSender};

use db_handler::start_db_handler_thread;
use warp::{Filter, Rejection};

#[tokio::main]
async fn main() {
    let (db_tx, db_rx) = sync_channel::<SyncSender<u32>>(32);
    let db_tx_clone = db_tx.clone();

    start_db_handler_thread(db_rx, SQLITE_DB_PATH.into());

    let route = warp::body::content_length_limit(1024 * 32)
        .and(warp::body::bytes())
        .and_then(move |bytes: bytes::Bytes| {
            let db_tx_clone = db_tx_clone.clone();
            async move {
                let body_str_result = std::str::from_utf8(bytes.as_ref());
                if let Ok(body_str) = body_str_result {
                    let json_result = serde_json::from_str(body_str);
                    if let Ok(json_value) = json_result {
                        Ok(json_handlers::handle_json(json_value, db_tx_clone)
                            .unwrap_or_else(|e| e))
                    } else {
                        Ok(String::from("{\"type\": \"invalid_syntax\"}"))
                    }
                } else {
                    Ok::<String, Rejection>(String::from("{\"type\": \"invalid_syntax\"}"))
                }
            }
        });

    warp::serve(route).run(([0, 0, 0, 0], 1237)).await;
}
