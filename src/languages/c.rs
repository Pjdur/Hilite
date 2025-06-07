use std::collections::HashMap;

pub fn keywords() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    map.insert("auto", "keyword");
    map.insert("break", "keyword");
    map.insert("case", "keyword");
    map.insert("char", "keyword");
    map.insert("const", "keyword");
    map.insert("continue", "keyword");
    map.insert("default", "keyword");
    map.insert("do", "keyword");
    map.insert("double", "keyword");
    map.insert("else", "keyword");
    map.insert("enum", "keyword");
    map.insert("extern", "keyword");
    map.insert("float", "keyword");
    map.insert("for", "keyword");
    map.insert("goto", "keyword");
    map.insert("if", "keyword");
    map.insert("int", "keyword");
    map.insert("long", "keyword");
    map.insert("register", "keyword");
    map.insert("return", "keyword");
    map.insert("short", "keyword");
    map.insert("signed", "keyword");
    map.insert("sizeof", "keyword");
    map.insert("static", "keyword");
    map.insert("struct", "keyword");
    map.insert("switch", "keyword");
    map.insert("typedef", "keyword");
    map.insert("union", "keyword");
    map.insert("unsigned", "keyword");
    map.insert("void", "keyword");
    map.insert("volatile", "keyword");
    map
}

pub fn function_definers() -> Vec<&'static str> {
    vec!["void", "int", "float", "double", "char"]
}