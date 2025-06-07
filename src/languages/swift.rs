use std::collections::HashMap;

pub fn keywords() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    map.insert("func", "keyword");
    map.insert("class", "keyword");
    map.insert("struct", "keyword");
    map.insert("enum", "keyword");
    map.insert("protocol", "keyword");
    map.insert("extension", "keyword");
    map.insert("import", "keyword");
    map.insert("let", "keyword");
    map.insert("var", "keyword");
    map.insert("if", "keyword");
    map.insert("else", "keyword");
    map.insert("for", "keyword");
    map.insert("while", "keyword");
    map.insert("repeat", "keyword");
    map.insert("switch", "keyword");
    map.insert("case", "keyword");
    map.insert("break", "keyword");
    map.insert("continue", "keyword");
    map.insert("return", "keyword");
    map.insert("throw", "keyword");
    map.insert("throws", "keyword");
    map.insert("try", "keyword");
    map.insert("in", "keyword");
    map.insert("where", "keyword");
    map.insert("as", "keyword");
    map.insert("is", "keyword");
    map.insert("super", "keyword");
    map.insert("self", "keyword");
    map.insert("true", "keyword");
    map.insert("false", "keyword");
    map.insert("nil", "keyword");
    map
}

pub fn function_definers() -> Vec<&'static str> {
    vec!["func"]
}