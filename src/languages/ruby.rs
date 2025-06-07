use std::collections::HashMap;

pub fn keywords() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    map.insert("def", "keyword");
    map.insert("class", "keyword");
    map.insert("module", "keyword");
    map.insert("if", "keyword");
    map.insert("elsif", "keyword");
    map.insert("else", "keyword");
    map.insert("unless", "keyword");
    map.insert("case", "keyword");
    map.insert("when", "keyword");
    map.insert("while", "keyword");
    map.insert("until", "keyword");
    map.insert("for", "keyword");
    map.insert("break", "keyword");
    map.insert("next", "keyword");
    map.insert("redo", "keyword");
    map.insert("retry", "keyword");
    map.insert("return", "keyword");
    map.insert("yield", "keyword");
    map.insert("super", "keyword");
    map.insert("self", "keyword");
    map.insert("nil", "keyword");
    map.insert("true", "keyword");
    map.insert("false", "keyword");
    map.insert("and", "keyword");
    map.insert("or", "keyword");
    map.insert("not", "keyword");
    map.insert("in", "keyword");
    map.insert("do", "keyword");
    map.insert("end", "keyword");
    map
}

pub fn function_definers() -> Vec<&'static str> {
    vec!["def"]
}