use std::collections::HashMap;

pub fn keywords() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    map.insert("fn", "keyword");
    map.insert("let", "keyword");
    map.insert("mut", "keyword");
    map.insert("pub", "keyword");
    map.insert("struct", "keyword");
    map.insert("enum", "keyword");
    map.insert("impl", "keyword");
    map.insert("trait", "keyword");
    map.insert("use", "keyword");
    map.insert("mod", "keyword");
    map.insert("const", "keyword");
    map.insert("static", "keyword");
    map.insert("match", "keyword");
    map.insert("if", "keyword");
    map.insert("else", "keyword");
    map.insert("while", "keyword");
    map.insert("loop", "keyword");
    map.insert("for", "keyword");
    map.insert("in", "keyword");
    map.insert("return", "keyword");
    map.insert("break", "keyword");
    map.insert("continue", "keyword");
    map.insert("as", "keyword");
    map.insert("crate", "keyword");
    map.insert("super", "keyword");
    map.insert("self", "keyword");
    map.insert("Self", "keyword");
    map.insert("ref", "keyword");
    map.insert("move", "keyword");
    map.insert("type", "keyword");
    map.insert("where", "keyword");
    map.insert("unsafe", "keyword");
    map.insert("extern", "keyword");
    map.insert("dyn", "keyword");
    map.insert("async", "keyword");
    map.insert("await", "keyword");
    map.insert("println!", "keyword");
    map
}

pub fn function_definers() -> Vec<&'static str> {
    vec!["fn"]
}