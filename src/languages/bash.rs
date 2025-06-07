use std::collections::HashMap;

pub fn keywords() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    map.insert("if", "keyword");
    map.insert("then", "keyword");
    map.insert("else", "keyword");
    map.insert("elif", "keyword");
    map.insert("fi", "keyword");
    map.insert("case", "keyword");
    map.insert("esac", "keyword");
    map.insert("for", "keyword");
    map.insert("select", "keyword");
    map.insert("while", "keyword");
    map.insert("until", "keyword");
    map.insert("do", "keyword");
    map.insert("done", "keyword");
    map.insert("in", "keyword");
    map.insert("function", "keyword");
    map.insert("time", "keyword");
    map.insert("coproc", "keyword");
    map.insert("break", "keyword");
    map.insert("continue", "keyword");
    map.insert("return", "keyword");
    map.insert("exit", "keyword");
    map.insert("export", "keyword");
    map.insert("readonly", "keyword");
    map.insert("declare", "keyword");
    map.insert("typeset", "keyword");
    map.insert("local", "keyword");
    map.insert("let", "keyword");
    map.insert("eval", "keyword");
    map.insert("test", "keyword");
    map.insert("true", "keyword");
    map.insert("false", "keyword");
    map
}

pub fn function_definers() -> Vec<&'static str> {
    vec!["function"]
}