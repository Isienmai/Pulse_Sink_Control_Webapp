use std::collections::HashMap;

/// Struct to represent one of many kinds of object used by pulseaudo (e.g. sources, sinks, modules)
pub struct PulseObject{
    pub object_type: String, // TODO: use an enum not a string
    pub index: usize,
    pub properties: HashMap<String, String>
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

pub mod sink;
pub mod source;
