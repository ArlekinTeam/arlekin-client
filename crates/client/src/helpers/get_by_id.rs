use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

pub fn get_by_id(id: &str) -> String {
    let content = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id(&id)
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap()
        .value();
    return content;
}
