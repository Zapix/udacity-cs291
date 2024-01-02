use winit::event_loop::{EventLoop};
use web_sys::{Element, HtmlCanvasElement};
use wasm_bindgen::JsCast;

#[cfg(target_arch = "wasm32")]
use winit::platform::web::WindowBuilderExtWebSys;

const CANVAS_ID: &'static str  = "canvas";

pub fn create_winit_window(event_loop: &EventLoop<()>, base: &Element) -> winit::window::Window {
    let window = web_sys::window().expect("Window does not exist");
    let document = window.document().expect("Can get document");

    let canvas = document
        .create_element("canvas").unwrap()
        .dyn_into::<HtmlCanvasElement>().unwrap();

    canvas.set_attribute("id", CANVAS_ID).unwrap();
    canvas.set_attribute("style", "width: 846px; height: 494px").unwrap();

    base.append_child(&canvas).unwrap();

    let mut builder = winit::window::WindowBuilder::new();
    builder = builder.with_canvas(Some(canvas));

    builder.build(&event_loop).unwrap()
}