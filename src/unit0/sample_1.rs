use wasm_bindgen::JsValue;
use web_sys::Element;
use crate::common::unit_trait::{UnitTrait, UnitIdentifierTrait, UnitRenderTrait};

pub struct Sample1;

impl UnitIdentifierTrait for Sample1 {
    fn new() -> Self {
        Self {}
    }
    fn identifier(&self) -> String {
        String::from("sample_1")
    }

    fn label(&self) -> String {
        String::from("Unit 0: First Sample")
    }
}

impl UnitRenderTrait for Sample1 {

    fn render(&self, base: &Element) -> Result<(), JsValue> {
        let window = web_sys::window().expect("Window expected");
        let document = window.document().expect("Document expected");
        let header = document.create_element("h1").unwrap();

        header.set_text_content(Some("First sample module"));
        base.append_child(&header).unwrap();

        Ok(())
    }
}

impl UnitTrait for Sample1 {}