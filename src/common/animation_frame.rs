use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use crate::common::flat_grid_canvas_renderer::FlatGridCanvasRenderer;

pub fn redraw_on_animation_frame(renderer: FlatGridCanvasRenderer, canvas_unmounted: Rc<RefCell<bool>>) {
    let redraw : Rc<RefCell<Option<Closure<dyn FnMut ()>>>> = Rc::new(RefCell::new(None));
    let redraw_closure = redraw.clone();
    *redraw_closure.borrow_mut() = Some(Closure::new(move || {
        if !*canvas_unmounted.borrow() {
            let _ = redraw.borrow_mut().take();
            return;
        }

        // Redraw canvas begins
        renderer.redraw();
        // redraw canvas ends

        let window = web_sys::window().expect("Window does not exist");
        let _ = window.request_animation_frame(
            redraw.borrow().as_ref().unwrap().as_ref().unchecked_ref()
        ).expect("requestAnimationFrame should be available");
    }));

    let window = web_sys::window().expect("Window does not exist");
    let _ = window.request_animation_frame(
        redraw_closure.borrow().as_ref().unwrap().as_ref().unchecked_ref()
    ).expect("requestAnimationFrame should be available");
}
