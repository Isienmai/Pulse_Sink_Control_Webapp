use wasm_bindgen::prelude::*;

mod pulse;
use pulse::pulse_object::source::Source;
use pulse::pulse_object::sink::Sink;

/// Struct to hold all sources/sinks/connections/module changes
#[wasm_bindgen]
pub struct PulseData{
    sources: Vec<Source>,
    sinks: Vec<Sink>
}

#[wasm_bindgen]
impl PulseData{
    /// Set this object's Sources to those contained in a pactl list output string
    pub fn update_sources(&mut self, sources_list: &str){
        self.sources = pulse::parse_sources(sources_list);
    }

    /// Set this object's Sinks to those contained in a pactl list output string
    pub fn update_sinks(&mut self, sinks_list: &str){
        self.sinks = pulse::parse_sinks(sinks_list);
    }

    /// Add a new Sink this object's list of sinks
    /// This will also automatically add a monitor of the new sink to the sources list
    pub fn add_new_sink(&mut self, new_sink_name: &str){
        self.sinks.push(Sink{
            name: String::from(new_sink_name), 
            description: String::from(new_sink_name)});
        self.sources.push(Source{
            name: String::from(new_sink_name) + ".monitor", 
            description: String::from(new_sink_name) + ".monitor"});
    }

    /// Get the number of Sources
    pub fn get_sources_count(&self)->usize{
        return self.sources.len();
    }

    /// Get the number of Sinks
    pub fn get_sinks_count(&self)->usize{
        return self.sources.len();
    }

    /// Get the description of a specified Source object (Specified by index)
    /// Returns "NULL" if the specified index is invalid
    pub fn get_source_description(&self, source_index: usize)->String{
        return self.sources.get(source_index)
            .and_then(|source| Some(source.description.clone()))
            .or(Some(String::from("NULL")))
            .unwrap();
    }

    /// Get the description of a specified Sink object (Specified by index)
    /// Returns "NULL" if the specified index is invalid
    pub fn get_sink_description(&self, sink_index: usize)->String{
        return self.sinks.get(sink_index)
            .and_then(|sink| Some(sink.description.clone()))
            .or(Some(String::from("NULL")))
            .unwrap();
    }
}
