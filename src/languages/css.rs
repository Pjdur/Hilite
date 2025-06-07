use std::collections::HashMap;

pub fn keywords() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    map.insert("color", "keyword");
    map.insert("background", "keyword");
    map.insert("background-color", "keyword");
    map.insert("font-size", "keyword");
    map.insert("font-family", "keyword");
    map.insert("margin", "keyword");
    map.insert("padding", "keyword");
    map.insert("border", "keyword");
    map.insert("width", "keyword");
    map.insert("height", "keyword");
    map.insert("display", "keyword");
    map.insert("position", "keyword");
    map.insert("top", "keyword");
    map.insert("bottom", "keyword");
    map.insert("left", "keyword");
    map.insert("right", "keyword");
    map.insert("float", "keyword");
    map.insert("clear", "keyword");
    map.insert("z-index", "keyword");
    map.insert("overflow", "keyword");
    map.insert("visibility", "keyword");
    map.insert("content", "keyword");
    map.insert("align-items", "keyword");
    map.insert("justify-content", "keyword");
    map.insert("flex", "keyword");
    map.insert("grid", "keyword");
    map.insert("gap", "keyword");
    map.insert("animation", "keyword");
    map.insert("transition", "keyword");
    map.insert("transform", "keyword");
    map.insert("box-shadow", "keyword");
    map.insert("text-align", "keyword");
    map.insert("vertical-align", "keyword");
    map.insert("overflow-x", "keyword");
    map.insert("overflow-y", "keyword");
    map.insert("min-width", "keyword");
    map.insert("max-width", "keyword");
    map.insert("min-height", "keyword");
    map.insert("max-height", "keyword");
    map
}

pub fn function_definers() -> Vec<&'static str> {
    vec![]
}