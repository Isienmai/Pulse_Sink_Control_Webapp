use std::collections::HashMap;
use regex::Regex;

pub struct PulseObject{
    pub name: String, // TODO: use an enum not a string
    pub index: usize,
    properties: HashMap<String, String>
}

impl PulseObject{
    pub fn get(&self, key: &str) -> Option<String>{
        let property = self.properties.get(key);
        if property.is_some(){
            return Some(property.unwrap().clone());
        }
        else
        {
            return None;
        }
    }

    pub fn property_count(&self) -> usize{
        return self.properties.len();
    }
}

use std::ops::Index;
impl Index<&str> for PulseObject {
    type Output = String;

    fn index(&self, key: &str) -> &String{
        return &self.properties[key];
    }
}

// TODO: How to handle invalid input?

pub fn parse_pactl_list_output(list_output: &str) -> Vec<PulseObject>
{
    let mut objects: Vec<PulseObject> = Vec::new();

    let object_regex = Regex::new(r"(?m)(^\S+ #\d+$(\t.*$)*)").unwrap();
    for captures in object_regex.captures_iter(list_output){
        objects.push(parse_single_object(&captures[1]));
    }

    return objects;
}

/// Given a string formatted to match the output of "pactl list" containing the definition of a single object
/// Return a HashMap containing the individual properties of the object
pub fn parse_single_object(object: &str) -> PulseObject{
    let mut properties = HashMap::new();

    let mut name = String::from("DUMMY");
    let mut index = 0;

    let name_and_index_regex = Regex::new(r"(?m)^(\S+) #(\d)$").unwrap();
    for captures in name_and_index_regex.captures_iter(object){
        name = captures[1].to_string();
        index = captures[2].to_string().parse::<usize>().unwrap();
    }

    let property_regex = Regex::new(r"(?m)^\t(\w[\w ]*): ?(.*(\n\t\s.*$)*)").unwrap();
    for captures in property_regex.captures_iter(object){
        properties.insert(captures[1].to_string(), captures[2].to_string());
    }

    return PulseObject{ name, index, properties };
}