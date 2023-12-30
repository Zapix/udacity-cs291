use wasm_bindgen::JsValue;
use web_sys::Element;

pub trait UnitTrait {
    fn new() -> Self where Self: Sized;
    fn identifier(&self) -> String;
    fn label(&self) -> String;

    fn render(&self, base: &Element) -> Result<(), JsValue>;
}