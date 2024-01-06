use wasm_bindgen::JsValue;
use web_sys::Element;

pub trait UnitIdentifierTrait {
    fn new() -> Self where Self: Sized;
    fn identifier(&self) -> String;
    fn label(&self) -> String;
}

pub trait UnitRenderTrait {
    fn render(&self, base: &Element) -> Result<(), JsValue>;
}

pub trait UnitTrait: UnitIdentifierTrait + UnitRenderTrait {
    fn new()  -> Self where Self: Sized{
        UnitIdentifierTrait::new()
    }
}