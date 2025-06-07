mod languages;
use regex::Regex;

/// Represents a colored token.
#[derive(Debug)]
pub struct Token {
    pub text: String,
    pub color: String,
}

pub fn highlight(code: &str, language: &str) -> Vec<(String, Vec<Token>)> {
    let (keywords, _function_definers) = languages::get_language_config(language);

    // Regex patterns
    let re_paren = Regex::new(r"[\(\)]").unwrap();
    let re_func_name = Regex::new(r"\b([a-zA-Z_][a-zA-Z0-9_]*)\s*\(").unwrap();
    let re_method = Regex::new(r"\b([a-zA-Z_][a-zA-Z0-9_]*)\.([a-zA-Z_][a-zA-Z0-9_]*)\b").unwrap();
    let re_keyword = Regex::new(&format!(r"\b({})", keywords.keys().cloned().collect::<Vec<_>>().join("|"))).unwrap();
    let re_number = Regex::new(r"\b\d+(\.\d+)?\b").unwrap();

    code.lines()
        .map(|line| {
            let indent = line.chars().take_while(|c| c.is_whitespace()).collect::<String>();
            let mut tokens = Vec::new();
            let rest = line.trim_start().to_string();

            // Skip comments for highlighting (simple version)
            let comment_start = rest.find("//").unwrap_or(rest.len());
            let (code_part, comment_part) = rest.split_at(comment_start);

            // Highlight function names before '(' (yellow)
            let code_part = re_func_name.replace_all(&code_part, |caps: &regex::Captures| {
                format!("\x1b[33m{}\x1b[0m(", &caps[1])
            }).to_string();

            // Highlight numbers (green)
            let code_part = re_number.replace_all(&code_part, |caps: &regex::Captures| {
                format!("\x1b[32m{}\x1b[0m", &caps[0])
            }).to_string();

            // Highlight all keywords (including function definers) as blue
            let code_part = re_keyword.replace_all(&code_part, |caps: &regex::Captures| {
                format!("\x1b[34m{}\x1b[0m", &caps[1])
            }).to_string();

            // Highlight parentheses (blue)
            let code_part = re_paren.replace_all(&code_part, |caps: &regex::Captures| {
                format!("\x1b[34m{}\x1b[0m", &caps[0])
            }).to_string();

            // Re-attach comment part (no highlighting)
            let rest = if comment_start < line.len() {
                format!("{}{}", code_part, comment_part)
            } else {
                code_part
            };

            tokens.push(Token { text: rest, color: "default".to_string() });
            (indent, tokens)
        })
        .collect()
}

pub fn colorize(highlighted: Vec<(String, Vec<Token>)>) {
    for (indent, words) in highlighted {
        print!("{}", indent);
        for token in words {
            print!("{}", token.text);
        }
        println!();
    }
}
