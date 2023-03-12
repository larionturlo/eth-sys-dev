use wasm_bindgen::prelude::*;
use js_sys::Promise;
use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window"])]
    pub fn ethIsConnected() -> Promise;
}

pub async fn is_connected() -> Result<bool, String> {
    return match JsFuture::from(ethIsConnected()).await {
        Ok(con) => match con.as_bool() {
            Some(con) => Ok(con),
            None => Err("error_as_bool".to_owned())
        },
        Err(err) =>  match err.as_string() {
            Some(err) => Err(err),
            None => Err("error_as_string".to_owned())
        },
    };
}
