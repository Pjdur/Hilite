use std::collections::HashMap;

pub fn keywords() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    map.insert("let", "keyword");
    map.insert("in", "keyword");
    map.insert("where", "keyword");
    map.insert("module", "keyword");
    map.insert("import", "keyword");
    map.insert("data", "keyword");
    map.insert("type", "keyword");
    map.insert("newtype", "keyword");
    map.insert("class", "keyword");
    map.insert("instance", "keyword");
    map.insert("deriving", "keyword");
    map.insert("if", "keyword");
    map.insert("then", "keyword");
    map.insert("else", "keyword");
    map.insert("case", "keyword");
    map.insert("of", "keyword");
    map.insert("do", "keyword");
    map.insert("default", "keyword");
    map.insert("infix", "keyword");
    map.insert("infixl", "keyword");
    map.insert("infixr", "keyword");
    map.insert("foreign", "keyword");
    map.insert("forall", "keyword");
    map.insert("mdo", "keyword");
    map.insert("rec", "keyword");
    map.insert("proc", "keyword");
    map.insert("family", "keyword");
    map.insert("role", "keyword");
    map.insert("pattern", "keyword");
    map.insert("static", "keyword");
    map.insert("group", "keyword");
    map.insert("by", "keyword");
    map.insert("using", "keyword");
    map.insert("qualified", "keyword");
    map.insert("as", "keyword");
    map.insert("hiding", "keyword");
    map
}

pub fn function_definers() -> Vec<&'static str> {
    vec!["let", "where"]
}