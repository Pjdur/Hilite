use std::collections::HashMap;

pub fn keywords() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    map.insert("class", "keyword");
    map.insert("interface", "keyword");
    map.insert("enum", "keyword");
    map.insert("struct", "keyword");
    map.insert("public", "keyword");
    map.insert("private", "keyword");
    map.insert("protected", "keyword");
    map.insert("internal", "keyword");
    map.insert("static", "keyword");
    map.insert("readonly", "keyword");
    map.insert("const", "keyword");
    map.insert("void", "keyword");
    map.insert("int", "keyword");
    map.insert("float", "keyword");
    map.insert("double", "keyword");
    map.insert("char", "keyword");
    map.insert("bool", "keyword");
    map.insert("string", "keyword");
    map.insert("object", "keyword");
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
    map.insert("using", "keyword");
    map.insert("namespace", "keyword");
    map.insert("new", "keyword");
    map.insert("this", "keyword");
    map.insert("base", "keyword");
    map.insert("override", "keyword");
    map.insert("virtual", "keyword");
    map.insert("abstract", "keyword");
    map.insert("sealed", "keyword");
    map.insert("event", "keyword");
    map.insert("delegate", "keyword");
    map.insert("params", "keyword");
    map.insert("out", "keyword");
    map.insert("ref", "keyword");
    map.insert("in", "keyword");
    map.insert("is", "keyword");
    map.insert("as", "keyword");
    map.insert("true", "keyword");
    map.insert("false", "keyword");
    map.insert("null", "keyword");
    map
}

pub fn function_definers() -> Vec<&'static str> {
    vec!["void", "int", "float", "double", "char", "bool", "string", "object"]
}