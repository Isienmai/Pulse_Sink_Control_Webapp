use super::pactl_list_format;

/// Struct to represent a pulse audio Source object
/// For now we only care about the Source's Name and Description
pub struct Source{
    pub name: String,
    pub description: String
}

impl Source{
    /// A function that parses a single Source object from the output of "pactl list sources"
    /// Expects the provided string to be the full info dump for a SINGLE Source object.
    pub fn parse_from_pactl_list_output(object_info: &str) -> Option<Source> {
        let object_info = pactl_list_format::parse_single_object(object_info);

        let name = object_info.get("Name");
        let description = object_info.get("Description");

        let mut result = None;
        if name.is_some() && description.is_some() {
            let name = name.unwrap();
            let description = description.unwrap();
    
            result = Some(Source { name, description });
        }

        return result;
    }
}

pub fn parse_sources_list(list: &str) -> Vec<Source>{
    return Vec::new();
}