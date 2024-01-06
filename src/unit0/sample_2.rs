use wasm_bindgen::JsValue;
use web_sys::Element;
use crate::common::unit_trait::{UnitTrait, UnitIdentifierTrait, UnitRenderTrait};

pub struct Sample2;

impl UnitIdentifierTrait for Sample2 {
    fn new() -> Self {
        Self {}
    }
    fn identifier(&self) -> String {
        String::from("sample_2")
    }

    fn label(&self) -> String {
        String::from("Unit 0: Second sample")
    }
}

impl UnitRenderTrait for Sample2 {
    fn render(&self, base: &Element) -> Result<(), JsValue> {
        let window = web_sys::window().expect("Window does not exist");
        let document = window.document().expect("No document");

        let card = document.create_element("div").unwrap();

        let title = document.create_element("h1").unwrap();
        title.set_text_content(Some("Another lesson"));
        card.append_child(&title).unwrap();

        let sub_title = document.create_element("h2").unwrap();
        sub_title.set_text_content(Some("With custom markup"));
        card.append_child(&sub_title).unwrap();

        base.append_child(&card).unwrap();
        Ok(())
    }
}

impl UnitTrait for Sample2 {}