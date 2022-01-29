use wasm_bindgen::prelude::*;

mod pulse;

#[wasm_bindgen(module = "/ui_control.js")]
extern{
    pub fn append_source(name: &str, description: &str);
}

#[wasm_bindgen]
pub fn parse_sources(sources_list: &str) {
    let sources = pulse::parse_sources(sources_list);
    for source in sources{
        append_source(&source.name, &source.description);
    }
}

#[wasm_bindgen]
pub fn parse_sinks(sources_list: &str) {
    let sinks = pulse::parse_sinks(sources_list);
    //custom_alert(&format!("Sinks: {}!", sinks.len()));
}

