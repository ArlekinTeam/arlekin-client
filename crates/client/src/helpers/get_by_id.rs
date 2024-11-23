use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

pub fn get_by_id(id: &str) -> String {
    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id(id)
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap()
        .value()
}
