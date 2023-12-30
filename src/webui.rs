use wasm_bindgen::prelude::*;
use web_sys::{console, Document, HtmlElement, Element, Event, HtmlSelectElement};

const SELECT_EXERCISE_LABEL: &str = "Select exercise";

fn create_menu(document: &Document) -> Result<Element, JsValue> {
    let menu_block = document.create_element("div").unwrap();
    menu_block.set_class_name("menu");
    menu_block.set_attribute("style", "display: flex; flex-direction: row;");

    let label = document.create_element("label").unwrap();
    label.set_text_content(Some(SELECT_EXERCISE_LABEL));

    let select = document.create_element("select").unwrap();
    for i in 0..10 {
        let option = document.create_element("option").unwrap();
        option.set_text_content(
            Some(
                format!("Option {}", (i + 1)).as_str()
            )
        );
        option.set_node_value(Some(format!("{}", i).as_str()));
        select.append_child(&option);
    }

    let select_handler = Closure::wrap(Box::new(|event: Event| {
        let select = event.current_target().unwrap().dyn_into::<HtmlSelectElement>().unwrap();
        console::log_1(&format!("Selected value: {}", select.value()).as_str().into());
    }) as Box<dyn Fn(_)>);

    select.add_event_listener_with_callback(
        "change",
        &select_handler.as_ref().unchecked_ref()
    );
    select_handler.forget();

    menu_block.append_child(&label);
    menu_block.append_child(&select);

    Ok(menu_block)
}

pub fn run_app(document: &Document, base: &HtmlElement) -> Result<(), JsValue> {
    let app = document.create_element("div").unwrap();
    app.set_class_name("main");
    app.set_attribute("style", "display: flex; flex-direction: column;");

    let menu = create_menu(&document).unwrap();
    app.append_child(&menu);

    base.append_child(&app);

    Ok(())
}