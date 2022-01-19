use super::*;

/// Struct to represent a pulse audio Sink object
/// For now we only care about the Sink's Name and Description
pub struct Sink{
    pub name: String,
    pub description: String
}

impl Sink{
    /// A function that converts a single Pulse Object into a Sink
    /// Returns None if the provided object does not define a Sink or is missing the Name and Description properties
    pub fn from(object: &PulseObject) -> Option<Sink> {
        if object.object_type != "Sink"{
            return None;
        }

        let name = object.get("Name");
        let description = object.get("Description");

        let mut result = None;
        if name.is_some() && description.is_some() {
            let name = name.unwrap();
            let description = description.unwrap();
    
            result = Some(Sink { name, description });
        }

        return result;
    }
}
