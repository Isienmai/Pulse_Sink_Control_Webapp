use wasm_bindgen::prelude::*;

mod pulse;

#[wasm_bindgen(module = "/testing.js")]
extern{
    pub fn custom_alert(s: &str);
}

#[wasm_bindgen]
pub fn parse_sources(sources_list: &str) {
    let sources = pulse::parse_sources(sources_list);
    custom_alert(&format!("Sources: {}!", sources.len()));
}

#[wasm_bindgen]
pub fn parse_sinks(sources_list: &str) {
    let sinks = pulse::parse_sinks(sources_list);
    custom_alert(&format!("Sinks: {}!", sinks.len()));
}

