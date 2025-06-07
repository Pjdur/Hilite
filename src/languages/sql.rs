use std::collections::HashMap;

pub fn keywords() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    map.insert("SELECT", "keyword");
    map.insert("FROM", "keyword");
    map.insert("WHERE", "keyword");
    map.insert("INSERT", "keyword");
    map.insert("INTO", "keyword");
    map.insert("VALUES", "keyword");
    map.insert("UPDATE", "keyword");
    map.insert("SET", "keyword");
    map.insert("DELETE", "keyword");
    map.insert("CREATE", "keyword");
    map.insert("TABLE", "keyword");
    map.insert("ALTER", "keyword");
    map.insert("DROP", "keyword");
    map.insert("JOIN", "keyword");
    map.insert("INNER", "keyword");
    map.insert("LEFT", "keyword");
    map.insert("RIGHT", "keyword");
    map.insert("FULL", "keyword");
    map.insert("ON", "keyword");
    map.insert("AS", "keyword");
    map.insert("AND", "keyword");
    map.insert("OR", "keyword");
    map.insert("NOT", "keyword");
    map.insert("NULL", "keyword");
    map.insert("PRIMARY", "keyword");
    map.insert("KEY", "keyword");
    map.insert("FOREIGN", "keyword");
    map.insert("REFERENCES", "keyword");
    map.insert("GROUP", "keyword");
    map.insert("BY", "keyword");
    map.insert("ORDER", "keyword");
    map.insert("HAVING", "keyword");
    map.insert("DISTINCT", "keyword");
    map.insert("UNION", "keyword");
    map.insert("ALL", "keyword");
    map.insert("LIMIT", "keyword");
    map.insert("OFFSET", "keyword");
    map.insert("CASE", "keyword");
    map.insert("WHEN", "keyword");
    map.insert("THEN", "keyword");
    map.insert("ELSE", "keyword");
    map.insert("END", "keyword");
    map
}

pub fn function_definers() -> Vec<&'static str> {
    vec![]
}