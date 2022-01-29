
export function append_source(new_source_name, new_source_description){
    let source_list_item = document.createElement("li");
    source_list_item.appendChild(document.createTextNode(new_source_description));

    document.getElementById('sources_display').appendChild(source_list_item);
}

export function append_sink(new_sink_name, new_sink_description){
    let sink_list_item = document.createElement("li");
    sink_list_item.appendChild(document.createTextNode(new_sink_description));

    document.getElementById('sinks_display').appendChild(sink_list_item);
}