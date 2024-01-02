extern crate console_error_panic_hook;
use std::rc::Rc;
use std::cell::RefCell;
mod utils;
mod webui;

mod state;
mod common;
mod unit0;
mod unit2;

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
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    let window = web_sys::window().expect("No global `window` exists");
    let document = window.document().expect("Should have a document on window");
    let body = document.body().expect("Should have a body");

    let state = Rc::new(RefCell::new(state::State::new()));
    webui::run_app(&document, &body, state.clone());
    Ok(())
}
