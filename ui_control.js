
export function append_source(new_source_name, new_source_description){
    let source_button = document.createElement("input");

    source_button.type = "button";
    source_button.className = "pactl_webapp_main_source_button";
    source_button.value = new_source_description;
    source_button.id = new_source_name;

    document.getElementById('sources_display').appendChild(source_button);
}