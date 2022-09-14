use app::App;
use wasm_bindgen::prelude::wasm_bindgen;

mod app;

#[wasm_bindgen]
pub fn start() {
    yew::start_app::<App>();
}
