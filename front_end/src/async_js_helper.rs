use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen(module = "/src/deferred_helper.js")]
extern "C" {
    fn async_sleep(ms: u32) -> Promise;
}

pub async fn rust_async_sleep(ms: u32) -> Result<(), JsValue> {
    let promise = async_sleep(ms);
    let js_fut = JsFuture::from(promise);
    js_fut.await?;
    Ok(())
}
