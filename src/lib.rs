mod console;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn hello_wasm() -> Result<(), JsValue> {
  console::log("Hello Wasm");

  Ok(())
}
