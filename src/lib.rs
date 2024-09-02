mod webgl;
mod complex_math;

use wasm_bindgen::prelude::*;
use crate::webgl::init_webgl;
use crate::complex_math::apply_function;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    init_webgl()?;
    Ok(())
}
