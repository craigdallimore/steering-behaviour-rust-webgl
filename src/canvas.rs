use wasm_bindgen::{JsCast, JsValue};
use web_sys::CanvasRenderingContext2d;

pub fn get_context() -> Result<CanvasRenderingContext2d, JsValue> {

  let document = web_sys::window().unwrap().document().unwrap();

  let canvas = document.get_element_by_id("canvas-main").unwrap();

  let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

  let ctx = canvas
    .get_context("2d")?
    .unwrap()
    .dyn_into::<CanvasRenderingContext2d>()?;

  Ok(ctx)
}
