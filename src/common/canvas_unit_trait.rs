use std::rc::Rc;
use std::cell::RefCell;

use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::closure::Closure;
use web_sys::{console, Element, HtmlCanvasElement};
use super::unit_trait::{UnitIdentifierTrait, UnitRenderTrait};

/// Trait that renders canvas as default behavior
pub trait CanvasUnitTrait: UnitIdentifierTrait + UnitRenderTrait {

    async fn start(canvas: HtmlCanvasElement, canvas_unmounted: Rc<RefCell<bool>>);
    fn render_canvas(&self, base: &Element) -> Result<(), JsValue> {
        let window = web_sys::window().expect("Window does not exist");
        let document = window.document().expect("Can not get document");

        let canvas = document
            .create_element("canvas").unwrap()
            .dyn_into::<HtmlCanvasElement>().unwrap();

        canvas.set_attribute("id", self.identifier().as_str()).unwrap();
        canvas.set_attribute("style", "width: 846px; height: 494px").unwrap();

        let unmounted_canvas = Rc::new(RefCell::new(true));
        let movable_unmounted_canvas = unmounted_canvas.clone();
        let canvas_id = Rc::new(self.identifier());
        let mutation_observer_handler = Closure::<dyn FnMut()>::new(move || {
            let window = web_sys::window().expect("window does not exist");
            let document = window.document().expect("Can not get document");
            *movable_unmounted_canvas.borrow_mut() = match document.get_element_by_id(&*canvas_id.as_str()) {
                Some(_) => {
                    console::log_1(&String::from("canvas has been added").as_str().into());
                    true
                },
                None => {
                    console::log_1(&String::from("canvas has been removed").as_str().into());
                    false
                }
            }
        });
        let mutation_observer = web_sys::MutationObserver::new(
            mutation_observer_handler.as_ref().unchecked_ref()
        ).expect("Can't creat observer observer");
        let mut mutation_observer_options = web_sys::MutationObserverInit::new();
        mutation_observer_options.child_list(true);
        mutation_observer
            .observe_with_options(base, &mutation_observer_options)
            .expect("Enable to start observing");
        mutation_observer_handler.forget();

        base.append_child(&canvas);

        // wasm_bindgen_futures::spawn_local(
        //     Self::start(
        //         canvas,
        //         unmounted_canvas.clone(),
        //     )
        // );
        Ok(())
    }
}

impl <T: CanvasUnitTrait> UnitRenderTrait for T {
    fn render(&self, base: &Element) -> Result<(), JsValue> {
        CanvasUnitTrait::render_canvas(self, base)
    }
}

