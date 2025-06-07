use std::collections::HashMap;

pub fn keywords() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    map.insert("func", "keyword");
    map.insert("package", "keyword");
    map.insert("import", "keyword");
    map.insert("var", "keyword");
    map.insert("const", "keyword");
    map.insert("type", "keyword");
    map.insert("struct", "keyword");
    map.insert("interface", "keyword");
    map.insert("map", "keyword");
    map.insert("chan", "keyword");
    map.insert("go", "keyword");
    map.insert("defer", "keyword");
    map.insert("select", "keyword");
    map.insert("if", "keyword");
    map.insert("else", "keyword");
    map.insert("switch", "keyword");
    map.insert("case", "keyword");
    map.insert("for", "keyword");
    map.insert("range", "keyword");
    map.insert("break", "keyword");
    map.insert("continue", "keyword");
    map.insert("return", "keyword");
    map.insert("fallthrough", "keyword");
    map.insert("default", "keyword");
    map.insert("goto", "keyword");
    map.insert("true", "keyword");
    map.insert("false", "keyword");
    map.insert("nil", "keyword");
    map
}

pub fn function_definers() -> Vec<&'static str> {
    vec!["func"]
}