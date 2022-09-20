//Four Line Dropper Backend - A server enabling networked multiplayer for Four Line Dropper
//Copyright (C) 2022 Stephen Seo
//
//This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
//
//This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
//
//You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
mod ai;
mod constants;
mod db_handler;
mod game_logic;
mod json_handlers;
mod random_helper;
mod state;

const SQLITE_DB_PATH: &str = "./fourLineDropper.db";

use db_handler::DBHandlerRequest;

use std::sync::mpsc::sync_channel;

use db_handler::start_db_handler_thread;
use tokio::sync::oneshot;
use warp::{Filter, Rejection};

#[tokio::main]
async fn main() {
    let (db_tx, db_rx) = sync_channel::<DBHandlerRequest>(128);
    let db_tx_clone = db_tx.clone();

    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();

    // Required because shutdown_tx is not cloneable, and its "send" consumes
    // itself.
    let (s_helper_tx, s_helper_rx) = sync_channel::<()>(1);

    std::thread::spawn(move || {
        if let Ok(_unused_value) = s_helper_rx.recv() {
            shutdown_tx
                .send(())
                .expect("Should be able to send shutdown signal");
        }
    });

    start_db_handler_thread(db_rx, SQLITE_DB_PATH.into(), s_helper_tx.clone());

    let route = //warp::body::content_length_limit(1024 * 32)
        //.and(warp::body::bytes())
        warp::body::bytes()
        .and_then(move |bytes: bytes::Bytes| {
            let db_tx_clone = db_tx_clone.clone();
            let s_helper_tx_clone = s_helper_tx.clone();
            async move {
                let body_str_result = std::str::from_utf8(bytes.as_ref());
                if let Ok(body_str) = body_str_result {
                    let json_result = serde_json::from_str(body_str);
                    if let Ok(json_value) = json_result {
                        let reply = warp::reply::with_header(
                            json_handlers::handle_json(json_value, db_tx_clone, s_helper_tx_clone)
                                .unwrap_or_else(|e| e),
                            "Content-Type",
                            "application/json",
                        );
                        let reply = warp::reply::with_header(
                            reply,
                            "Access-Control-Allow-Headers",
                            "*",
                        );
                        Ok::<Box<dyn warp::reply::Reply>, Rejection>(Box::new(
                            warp::reply::with_header(reply, "Access-Control-Allow-Origin", "*"),
                        ))
                    } else {
                        let reply = warp::reply::with_header(
                            String::from("{\"type\": \"invalid_syntax\"}"),
                            "Content-Type",
                            "application/json",
                        );
                        let reply = warp::reply::with_header(
                            reply,
                            "Access-Control-Allow-Headers",
                            "*",
                        );
                        Ok::<Box<dyn warp::reply::Reply>, Rejection>(Box::new(
                            warp::reply::with_header(reply, "Access-Control-Allow-Origin", "*"),
                        ))
                    }
                } else {
                    let reply = warp::reply::with_header(
                        String::from("{\"type\": \"invalid_syntax\"}"),
                        "Content-Type",
                        "application/json",
                    );
                    let reply = warp::reply::with_header(
                        reply,
                        "Access-Control-Allow-Headers",
                        "*",
                    );
                    Ok::<Box<dyn warp::reply::Reply>, Rejection>(Box::new(
                        warp::reply::with_header(reply, "Access-Control-Allow-Origin", "*"),
                    ))
                }
            }
        });

    let (_addr, server) =
        warp::serve(route).bind_with_graceful_shutdown(([0, 0, 0, 0], 1237), async move {
            shutdown_rx.await.ok();
        });

    tokio::task::spawn(server).await.unwrap();
}
