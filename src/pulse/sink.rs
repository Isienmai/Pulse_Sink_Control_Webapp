use super::pactl_list_format;

/// Struct to represent a pulse audio Sink object
/// For now we only care about the Sink's Name and Description
pub struct Sink{
    pub name: String,
    pub description: String
}

impl Sink{
    /// A function that parses a single Sink object from the output of "pactl list sinks"
    /// Expects the provided string to be the full info dump for a SINGLE Sink object.
    pub fn parse_from_pactl_list_output(object_info: &str) -> Option<Sink> {
        let object_info = pactl_list_format::parse_single_object(object_info);

        let name = object_info.get("Name");
        let description = object_info.get("Description");

        let mut result = None;
        if name.is_some() && description.is_some() {
            let name = name.unwrap();
            let description = description.unwrap();
    
            result = Some(Sink { name, description });
        }

        return result;
    }
}