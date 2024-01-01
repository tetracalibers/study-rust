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
    .unwrap()
    .unwrap()
    .dyn_into::<web_sys::CanvasRenderingContext2d>()
    .unwrap();

  sierpinski(&context, [(300.0, 0.0), (0.0, 600.0), (600.0, 600.0)], 2);

  Ok(())
}

fn sierpinski(
  context: &web_sys::CanvasRenderingContext2d,
  points: [(f64, f64); 3],
  depth: u8,
) {
  // container triangle
  draw_triangle(&context, points);

  let depth = depth - 1;

  if depth > 0 {
    // inner top triangle
    sierpinski(
      &context,
      [(300.0, 0.0), (150.0, 300.0), (450.0, 300.0)],
      depth,
    );
    // inner left bottom triangle
    sierpinski(
      &context,
      [(150.0, 300.0), (0.0, 600.0), (300.0, 600.0)],
      depth,
    );
    // inner right bottom triangle
    sierpinski(
      &context,
      [(450.0, 300.0), (300.0, 600.0), (600.0, 600.0)],
      depth,
    );
  }
}

fn draw_triangle(
  context: &web_sys::CanvasRenderingContext2d,
  points: [(f64, f64); 3],
) {
  let [top, left, right] = points;
  context.move_to(top.0, top.1);
  context.begin_path();
  context.line_to(left.0, left.1);
  context.line_to(right.0, right.1);
  context.line_to(top.0, top.1);
  context.close_path();
  context.stroke();
}
