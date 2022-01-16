use std::collections::HashMap;
use regex::Regex;

/// Given a string formatted to match the output of "pactl list" containing the definition of a single object
/// Return a HashMap containing the individual properties of the object
pub fn parse_single_object(object: &str) -> HashMap<String, String>{
    let mut properties = HashMap::new();

    let property_regex = Regex::new(r"(?m)^\t(\w[\w ]*): ?(.*(\n\t\s.*$)*)").unwrap();
    for captures in property_regex.captures_iter(object){
        properties.insert(captures[1].to_string(), captures[2].to_string());
    }

    return properties;
}