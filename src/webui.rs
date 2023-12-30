use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{console, Document, HtmlElement, Element, Event, HtmlSelectElement};
use crate::state::{Observer, State};

const SELECT_EXERCISE_LABEL: &str = "Select exercise";
const NO_SELECTED_EXERCISE_LABEL: &str = "Not select";

fn create_menu(document: &Document, state: Rc<RefCell<State>>) -> Result<Element, JsValue> {
    let menu_block = document.create_element("div").unwrap();
    menu_block.set_class_name("menu");
    menu_block.set_attribute("style", "display: flex; flex-direction: row;");

    let label = document.create_element("label").unwrap();
    label.set_text_content(Some(SELECT_EXERCISE_LABEL));

    let select = document.create_element("select").unwrap();
    let option = document.create_element("option").unwrap();
    select.append_child(&option).unwrap();
    option.set_text_content(Some(NO_SELECTED_EXERCISE_LABEL));
    option.set_attribute("value", "");
    for unit in state.borrow().units().iter() {
        let option = document.create_element("option").unwrap();
        option.set_attribute("value", format!("{}", unit.identifier()).as_str()).unwrap();
        option.set_text_content(Some(unit.label().as_str()));
        select.append_child(&option).unwrap();
    }

    let select_handler = Closure::wrap(Box::new(move |event: Event| {
        let select = event.current_target().unwrap().dyn_into::<HtmlSelectElement>().unwrap();
        let mut mutable_state = state.borrow_mut();
        mutable_state.set_value(select.value().as_str().into());
    }) as Box<dyn Fn(_)>);

    select.add_event_listener_with_callback(
        "change",
        &select_handler.as_ref().unchecked_ref()
    ).unwrap();
    select_handler.forget();

    menu_block.append_child(&label).unwrap();
    menu_block.append_child(&select).unwrap();

    Ok(menu_block)
}

pub fn create_plot(document: &Document, state: Rc<RefCell<State>>) -> Result<Rc<Element>, JsValue> {
    let plot = Rc::new(document.create_element("div").unwrap());
    plot.set_class_name("plot");
    plot.set_attribute(
        "style",
        "display: flex; flex-grow: 1; align-items: center; justify-content: center;"
    ).unwrap();

    let movable_plot = plot.clone();
    (*state.borrow_mut()).subscribe(
        "plot",
        Box::new(move |value| {
            console::log_1(&format!("Plot received value {}", value.identifier()).as_str().into());
            let inner_plot = movable_plot.clone();
            inner_plot.set_inner_html("");

            let window = web_sys::window().expect("No global `window` exists");
            let document = window.document().expect("Should have a document on window");
            let header = document.create_element("h1").unwrap();
            value.render(inner_plot.as_ref()).unwrap();
        })
    ).unwrap();

    Ok(plot.clone().into())
}

pub fn run_app(document: &Document, base: &HtmlElement, state: Rc<RefCell<State>>) -> Result<(), JsValue> {
    let app = document.create_element("div").unwrap();
    app.set_class_name("main");
    app.set_attribute("style", "display: flex; flex-direction: column; height: 100vh;");

    let menu = create_menu(&document, state.clone()).unwrap();
    app.append_child(&menu).unwrap();

    let plot = create_plot(&document, state.clone()).unwrap();
    app.append_child(&plot).unwrap();

    (*state.clone().borrow_mut()).subscribe(
        "app",
        Box::new(|value| {
            console::log_1(&format!(
                "Application received value {}",
                value.identifier()
            ).as_str().into());
        }),
    ).unwrap();

    base.append_child(&app).unwrap();

    Ok(())
}