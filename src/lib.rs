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
    let sources = pulse::parse_sources(sources_list);
    alert(&format!("Sources: {}!", sources.len()));
}

#[wasm_bindgen]
pub fn parse_sinks(sources_list: &str) {
    let sinks = pulse::parse_sinks(sources_list);
    alert(&format!("Sinks: {}!", sinks.len()));
}

