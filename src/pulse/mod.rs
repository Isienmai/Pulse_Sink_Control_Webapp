mod pactl_list_format;
pub mod pulse_object;

use pulse_object::source::Source;
use pulse_object::sink::Sink;

pub fn parse_sources(pactl_list_sources: &str) -> Vec<Source>{
    let objects = pactl_list_format::parse_pactl_list_output(pactl_list_sources);

    let mut sources: Vec<Source> = Vec::new();
    for object in objects{
        let source = Source::from(&object);
        if source.is_some(){
            sources.push(source.unwrap());
        }
    }
    
    return sources;
}

pub fn parse_sinks(pactl_list_sinks: &str) -> Vec<Sink>{
    let objects = pactl_list_format::parse_pactl_list_output(pactl_list_sinks);

    let mut sinks: Vec<Sink> = Vec::new();
    for object in objects{
        let sink = Sink::from(&object);
        if sink.is_some(){
            sinks.push(sink.unwrap());
        }
    }
    
    return sinks;
}

#[cfg(test)]
mod tests;