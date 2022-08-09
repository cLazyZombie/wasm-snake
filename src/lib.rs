use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub async fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).expect("could not initialize logger");
    log::debug!("start");

    Ok(())
}
