use std::collections::HashMap;

pub fn keywords() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    map.insert("def", "keyword");
    map.insert("class", "keyword");
    map.insert("if", "keyword");
    map.insert("elif", "keyword");
    map.insert("else", "keyword");
    map.insert("for", "keyword");
    map.insert("while", "keyword");
    map.insert("break", "keyword");
    map.insert("continue", "keyword");
    map.insert("return", "keyword");
    map.insert("import", "keyword");
    map.insert("from", "keyword");
    map.insert("as", "keyword");
    map.insert("pass", "keyword");
    map.insert("raise", "keyword");
    map.insert("try", "keyword");
    map.insert("except", "keyword");
    map.insert("finally", "keyword");
    map.insert("with", "keyword");
    map.insert("lambda", "keyword");
    map.insert("yield", "keyword");
    map.insert("global", "keyword");
    map.insert("nonlocal", "keyword");
    map.insert("assert", "keyword");
    map.insert("del", "keyword");
    map.insert("not", "keyword");
    map.insert("and", "keyword");
    map.insert("or", "keyword");
    map.insert("is", "keyword");
    map.insert("in", "keyword");
    map.insert("True", "keyword");
    map.insert("False", "keyword");
    map.insert("None", "keyword");
    map
}

pub fn function_definers() -> Vec<&'static str> {
    vec!["def", "lambda"]
}