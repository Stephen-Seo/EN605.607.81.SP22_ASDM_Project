mod json_handlers;

use warp::{Filter, Rejection};

#[tokio::main]
async fn main() {
    let route = warp::body::content_length_limit(1024 * 32)
        .and(warp::body::bytes())
        .and_then(|bytes: bytes::Bytes| async move {
            let body_str_result = std::str::from_utf8(bytes.as_ref());
            if let Ok(body_str) = body_str_result {
                let json_result = serde_json::from_str(body_str);
                if let Ok(json_value) = json_result {
                    Ok(json_handlers::handle_json(json_value).unwrap_or_else(|e| e))
                } else {
                    Ok(String::from("{\"type\": \"invalid_syntax\"}"))
                }
            } else {
                Ok::<String, Rejection>(String::from("{\"type\": \"invalid_syntax\"}"))
            }
        });

    warp::serve(route).run(([0, 0, 0, 0], 1237)).await;
}
