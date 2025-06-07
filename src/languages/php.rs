use std::collections::HashMap;

pub fn keywords() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    map.insert("function", "keyword");
    map.insert("class", "keyword");
    map.insert("interface", "keyword");
    map.insert("trait", "keyword");
    map.insert("public", "keyword");
    map.insert("private", "keyword");
    map.insert("protected", "keyword");
    map.insert("static", "keyword");
    map.insert("abstract", "keyword");
    map.insert("final", "keyword");
    map.insert("const", "keyword");
    map.insert("var", "keyword");
    map.insert("if", "keyword");
    map.insert("else", "keyword");
    map.insert("elseif", "keyword");
    map.insert("for", "keyword");
    map.insert("foreach", "keyword");
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
    map.insert("use", "keyword");
    map.insert("namespace", "keyword");
    map.insert("new", "keyword");
    map.insert("clone", "keyword");
    map.insert("instanceof", "keyword");
    map.insert("extends", "keyword");
    map.insert("implements", "keyword");
    map.insert("echo", "keyword");
    map.insert("print", "keyword");
    map.insert("require", "keyword");
    map.insert("include", "keyword");
    map.insert("require_once", "keyword");
    map.insert("include_once", "keyword");
    map.insert("global", "keyword");
    map.insert("isset", "keyword");
    map.insert("unset", "keyword");
    map.insert("empty", "keyword");
    map.insert("array", "keyword");
    map.insert("list", "keyword");
    map.insert("true", "keyword");
    map.insert("false", "keyword");
    map.insert("null", "keyword");
    map
}

pub fn function_definers() -> Vec<&'static str> {
    vec!["function"]
}