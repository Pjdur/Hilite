use std::collections::HashMap;

pub fn keywords() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    map.insert("function", "keyword");
    map.insert("var", "keyword");
    map.insert("let", "keyword");
    map.insert("const", "keyword");
    map.insert("if", "keyword");
    map.insert("else", "keyword");
    map.insert("for", "keyword");
    map.insert("while", "keyword");
    map.insert("do", "keyword");
    map.insert("switch", "keyword");
    map.insert("case", "keyword");
    map.insert("break", "keyword");
    map.insert("continue", "keyword");
    map.insert("return", "keyword");
    map.insert("try", "keyword");
    map.insert("catch", "keyword");
    map.insert("finally", "keyword");
    map.insert("throw", "keyword");
    map.insert("class", "keyword");
    map.insert("extends", "keyword");
    map.insert("super", "keyword");
    map.insert("import", "keyword");
    map.insert("export", "keyword");
    map.insert("default", "keyword");
    map.insert("new", "keyword");
    map.insert("this", "keyword");
    map.insert("delete", "keyword");
    map.insert("typeof", "keyword");
    map.insert("instanceof", "keyword");
    map.insert("in", "keyword");
    map.insert("of", "keyword");
    map.insert("await", "keyword");
    map.insert("async", "keyword");
    map.insert("yield", "keyword");
    map.insert("interface", "keyword");
    map.insert("enum", "keyword");
    map.insert("implements", "keyword");
    map.insert("public", "keyword");
    map.insert("private", "keyword");
    map.insert("protected", "keyword");
    map.insert("readonly", "keyword");
    map.insert("abstract", "keyword");
    map.insert("as", "keyword");
    map.insert("asserts", "keyword");
    map.insert("unknown", "keyword");
    map.insert("never", "keyword");
    map.insert("any", "keyword");
    map.insert("void", "keyword");
    map.insert("undefined", "keyword");
    map.insert("null", "keyword");
    map.insert("true", "keyword");
    map.insert("false", "keyword");
    map
}

pub fn function_definers() -> Vec<&'static str> {
    vec!["function", "=>"]
}