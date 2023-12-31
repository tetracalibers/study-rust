use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
  console_error_panic_hook::set_once();

  let window = web_sys::window().unwrap();
  let document = window.document().unwrap();

  let canvas = document
    .get_element_by_id("canvas")
    .unwrap()
    .dyn_into::<web_sys::HtmlCanvasElement>()
    .unwrap();

  let context = canvas
    .get_context("2d")
    // Result<Option<Object>>を返すので、2回unwrap
    .unwrap()
    .unwrap()
    // dyn_intoで型をキャスト
    // dyn_into from wasm_bindgen::JsCast;
    .dyn_into::<web_sys::CanvasRenderingContext2d>()
    .unwrap();

  context.move_to(300.0, 0.0); // top of triangle
  context.begin_path();
  context.line_to(0.0, 600.0); // bottom left of triangle
  context.line_to(600.0, 600.0); // bottom right of triangle
  context.line_to(300.0, 0.0); // back to top of triangle
  context.close_path();
  context.stroke();
  context.fill();

  Ok(())
}
