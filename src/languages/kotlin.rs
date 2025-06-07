use std::collections::HashMap;

pub fn keywords() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    map.insert("fun", "keyword");
    map.insert("class", "keyword");
    map.insert("object", "keyword");
    map.insert("interface", "keyword");
    map.insert("enum", "keyword");
    map.insert("data", "keyword");
    map.insert("sealed", "keyword");
    map.insert("val", "keyword");
    map.insert("var", "keyword");
    map.insert("if", "keyword");
    map.insert("else", "keyword");
    map.insert("when", "keyword");
    map.insert("for", "keyword");
    map.insert("while", "keyword");
    map.insert("do", "keyword");
    map.insert("break", "keyword");
    map.insert("continue", "keyword");
    map.insert("return", "keyword");
    map.insert("throw", "keyword");
    map.insert("try", "keyword");
    map.insert("catch", "keyword");
    map.insert("finally", "keyword");
    map.insert("import", "keyword");
    map.insert("package", "keyword");
    map.insert("in", "keyword");
    map.insert("is", "keyword");
    map.insert("as", "keyword");
    map.insert("this", "keyword");
    map.insert("super", "keyword");
    map.insert("true", "keyword");
    map.insert("false", "keyword");
    map.insert("null", "keyword");
    map
}

pub fn function_definers() -> Vec<&'static str> {
    vec!["fun"]
}