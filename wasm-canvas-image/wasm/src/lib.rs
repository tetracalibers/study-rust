use std::{rc::Rc, sync::Mutex};
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

  wasm_bindgen_futures::spawn_local(async move {
    // チャネルを作る
    // チャネルにResultを送るようにしたことで、後から成功と失敗を見分けることができる
    // 成功時にはunitを、失敗時にはJSからのエラーコード（JsValue）を受け取るようにする
    // JsValueはJSから直接渡される値すべてを表す型
    let (success_tx, success_rx) = futures::channel::oneshot::channel::<Result<(), JsValue>>();

    let success_tx = Rc::new(Mutex::new(Some(success_tx)));
    let error_tx = Rc::clone(&success_tx);

    let image = web_sys::HtmlImageElement::new().unwrap();

    // Closure は wasm-bindgen が提供する構造体で、RustのクロージャをJavaScriptに渡すためのもの
    // onloadハンドラは一度しか呼ばれないので、Closure::onceを使う
    let callback = Closure::once(move || {
      if let Some(success_tx) = success_tx.lock().ok().and_then(|mut opt| opt.take()) {
        success_tx.send(Ok(()));
      }
    });

    let error_callback = Closure::once(move |err| {
      if let Some(error_tx) = error_tx.lock().ok().and_then(|mut opt| opt.take()) {
        error_tx.send(Err(err));
      }
    });

    // as_ref() は 生のJsValue を返す
    // unchecked_ref() で &Functionオブジェクト に変換する
    // 引数はJSではnullの可能性があるため、Someでラップする
    image.set_onload(Some(callback.as_ref().unchecked_ref()));
    image.set_onerror(Some(error_callback.as_ref().unchecked_ref()));

    image.set_src("/Idle/image_3.png");

    success_rx.await;
    context.draw_image_with_html_image_element(&image, 0.0, 0.0);
  });

  Ok(())
}
