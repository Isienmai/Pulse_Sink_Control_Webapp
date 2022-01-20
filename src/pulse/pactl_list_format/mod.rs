use regex::Regex;
use super::pulse_object::PulseObject;
use std::collections::HashMap;

// TODO: How to handle invalid input?

/// Given a string formatted to match the output of a "pactl list" command
/// Return all the pulse objects in that string
pub fn parse_pactl_list_output(list_output: &str) -> Vec<PulseObject>{
    let mut objects: Vec<PulseObject> = Vec::new();

    let object_regex = Regex::new(r"(?ms)(^\S+ #\d+$(\t.*$)*)").unwrap();
    for captures in object_regex.captures_iter(&list_output){
        objects.push(parse_single_object(&captures[1]));
    }

    return objects;
}

/// Given a string formatted to match the output of "pactl list" containing the definition of a single object
/// Return a HashMap containing the individual properties of the object
pub fn parse_single_object(object: &str) -> PulseObject{
    let mut properties = HashMap::new();

    let mut object_type = String::from("NULL");
    let mut index = 0;

    let name_and_index_regex = Regex::new(r"(?m)^(\S+) #(\d)$").unwrap();
    for captures in name_and_index_regex.captures_iter(object){
        object_type = captures[1].to_string();
        index = captures[2].to_string().parse::<usize>().unwrap();
    }

    let property_regex = Regex::new(r"(?m)^\t(\w[\w ]*): ?(.*(\n\t\s.*$)*)").unwrap();
    for captures in property_regex.captures_iter(object){
        properties.insert(captures[1].to_string(), captures[2].to_string());
    }

    return PulseObject{ object_type, index, properties };
}

#[cfg(test)]
mod parse_pactl_list_output_tests;
#[cfg(test)]
mod parse_single_object_tests;
