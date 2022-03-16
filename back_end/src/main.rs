mod json_handlers;

use serde_json::Value;
use warp::Filter;

#[tokio::main]
async fn main() {
    let route = warp::body::content_length_limit(1024 * 32)
        .and(warp::body::json())
        .map(|json_value: Value| {
            let result = json_handlers::handle_json(json_value);
            if let Ok(result_str) = result {
                result_str
            } else if let Err(error_str) = result {
                error_str
            } else {
                unreachable!()
            }
        });

    warp::serve(route)
        .run(([0,0,0,0], 1237))
        .await;
}
