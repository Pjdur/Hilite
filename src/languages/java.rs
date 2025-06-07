use std::collections::HashMap;

pub fn keywords() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    map.insert("class", "keyword");
    map.insert("interface", "keyword");
    map.insert("enum", "keyword");
    map.insert("public", "keyword");
    map.insert("private", "keyword");
    map.insert("protected", "keyword");
    map.insert("static", "keyword");
    map.insert("final", "keyword");
    map.insert("abstract", "keyword");
    map.insert("synchronized", "keyword");
    map.insert("volatile", "keyword");
    map.insert("transient", "keyword");
    map.insert("native", "keyword");
    map.insert("strictfp", "keyword");
    map.insert("void", "keyword");
    map.insert("int", "keyword");
    map.insert("float", "keyword");
    map.insert("double", "keyword");
    map.insert("char", "keyword");
    map.insert("boolean", "keyword");
    map.insert("byte", "keyword");
    map.insert("short", "keyword");
    map.insert("long", "keyword");
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
    map.insert("throws", "keyword");
    map.insert("import", "keyword");
    map.insert("package", "keyword");
    map.insert("super", "keyword");
    map.insert("this", "keyword");
    map.insert("new", "keyword");
    map.insert("instanceof", "keyword");
    map.insert("extends", "keyword");
    map.insert("implements", "keyword");
    map.insert("true", "keyword");
    map.insert("false", "keyword");
    map.insert("null", "keyword");
    map
}

pub fn function_definers() -> Vec<&'static str> {
    vec!["void", "int", "float", "double", "char", "boolean", "byte", "short", "long"]
}