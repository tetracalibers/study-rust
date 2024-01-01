use wasm_bindgen::prelude::*;
use web_sys::console;

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

  let image = web_sys::HtmlImageElement::new().unwrap();

  // Closure は wasm-bindgen が提供する構造体で、RustのクロージャをJavaScriptに渡すためのもの
  // onloadハンドラは一度しか呼ばれないので、Closure::onceを使う
  let callback = Closure::once(|| {
    web_sys::console::log_1(&JsValue::from_str("Image loaded!"));
  });

  // as_ref() は 生のJsValue を返す
  // unchecked_ref() で &Functionオブジェクト に変換する
  // 引数はJSではnullの可能性があるため、Someでラップする
  image.set_onload(Some(callback.as_ref().unchecked_ref()));
  
  image.set_src("/Idle/image_3.png");

  context.draw_image_with_html_image_element(&image, 0.0, 0.0);

  Ok(())
}
