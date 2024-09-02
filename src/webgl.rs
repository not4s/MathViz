use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGlRenderingContext};

pub fn init_webgl() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()?;
    let gl: WebGlRenderingContext = canvas
        .get_context("webgl")?
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()?;

    gl.clear_color(0.0, 0.0, 0.0, 1.0);
    gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

    Ok(())
}