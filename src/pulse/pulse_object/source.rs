use super::*;

/// Struct to represent a pulse audio Source object
/// For now we only care about the Source's Name and Description
pub struct Source{
    pub name: String,
    pub description: String
}

impl Source{
    /// A function that converts a single Pulse Object into a Source
    /// Returns None if the provided object does not define a Source or is missing the Name and Description properties
    pub fn from(object: &PulseObject) -> Option<Source> {
        if object.object_type != "Source"{
            return None;
        }

        let name = object.get("Name");
        let description = object.get("Description");

        let mut result = None;
        if name.is_some() && description.is_some() {
            let name = name.unwrap();
            let description = description.unwrap();
    
            result = Some(Source { name, description });
        }

        return result;
    }
}
