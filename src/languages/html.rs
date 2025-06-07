use std::collections::HashMap;

pub fn keywords() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    map.insert("html", "keyword");
    map.insert("head", "keyword");
    map.insert("body", "keyword");
    map.insert("div", "keyword");
    map.insert("span", "keyword");
    map.insert("h1", "keyword");
    map.insert("h2", "keyword");
    map.insert("h3", "keyword");
    map.insert("h4", "keyword");
    map.insert("h5", "keyword");
    map.insert("h6", "keyword");
    map.insert("p", "keyword");
    map.insert("a", "keyword");
    map.insert("img", "keyword");
    map.insert("ul", "keyword");
    map.insert("ol", "keyword");
    map.insert("li", "keyword");
    map.insert("table", "keyword");
    map.insert("tr", "keyword");
    map.insert("td", "keyword");
    map.insert("th", "keyword");
    map.insert("form", "keyword");
    map.insert("input", "keyword");
    map.insert("button", "keyword");
    map.insert("script", "keyword");
    map.insert("link", "keyword");
    map.insert("meta", "keyword");
    map.insert("title", "keyword");
    map.insert("style", "keyword");
    map.insert("br", "keyword");
    map.insert("hr", "keyword");
    map
}

pub fn function_definers() -> Vec<&'static str> {
    vec![]
}