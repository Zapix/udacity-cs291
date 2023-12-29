mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, udacity-cs291!");
}

#[wasm_bindgen(start)]
fn run() -> Result<(), JsValue> {
    let window = web_sys::window().expect("No global `window` exists");
    let document = window.document().expect("Should have a document on window");
    let body = document.body().expect("Should have a body");

    let p = document.create_element("div").unwrap();
    p.set_text_content(Some("Hello cs291"));
    body.append_child(&p);

    Ok(())
}
