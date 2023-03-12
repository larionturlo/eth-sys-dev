
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use eth_sys::bindings::is_connected;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

fn main() {
    console_log!("Wasm loaded corectly");
    spawn_local(async move{
        console_log!("Wasm loaded corectly");
        match is_connected().await {
            Ok(con) => console_log!("Eth conected state: {}", con),
            Err(err) => console_log!("Eth conected state: {}", err),
        }
    })
}