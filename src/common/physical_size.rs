use wasm_bindgen::JsValue;
use web_sys::HtmlElement;

/// Computes physical size of element as client_width, client_height multiplied
/// by device_pixel_ratio
pub fn get_physical_size(element: &HtmlElement) -> Result<(u32, u32), JsValue> {
    let window = web_sys::window().expect("Can't get window");
    let device_pixel_ratio = window.device_pixel_ratio() as u32;
    let width = element.client_width() as u32;
    let height = element.client_height() as u32;
    return Ok((width * device_pixel_ratio, height * device_pixel_ratio));
}