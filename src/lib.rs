use wasm_bindgen::prelude::*;

mod pulse;

#[wasm_bindgen]
extern{
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("why HELLO THERE {}!", name));
}

#[wasm_bindgen]
pub fn parse_sources(sources_list: &str) {

    // TODO: Parse the sources list using the pulse module
}
