pub mod rust;
pub mod python;
pub mod javascript;
pub mod c;
pub mod cpp;
pub mod java;
pub mod go;
pub mod swift;
pub mod kotlin;
pub mod php;
pub mod ruby;
pub mod typescript;
pub mod csharp;
pub mod haskell;
pub mod bash;
pub mod html;
pub mod css;
pub mod sql;

use std::collections::HashMap;

/// Returns (keywords, function_definers) for a language.
pub fn get_language_config(language: &str) -> (HashMap<&'static str, &'static str>, Vec<&'static str>) {
    match language {
        "rust" => (rust::keywords(), rust::function_definers()),
        "python" => (python::keywords(), python::function_definers()),
        "javascript" => (javascript::keywords(), javascript::function_definers()),
        "c" => (c::keywords(), c::function_definers()),
        "cpp" => (cpp::keywords(), cpp::function_definers()),
        "java" => (java::keywords(), java::function_definers()),
        "go" => (go::keywords(), go::function_definers()),
        "swift" => (swift::keywords(), swift::function_definers()),
        "kotlin" => (kotlin::keywords(), kotlin::function_definers()),
        "php" => (php::keywords(), php::function_definers()),
        "ruby" => (ruby::keywords(), ruby::function_definers()),
        "typescript" => (typescript::keywords(), typescript::function_definers()),
        "csharp" => (csharp::keywords(), csharp::function_definers()),
        "haskell" => (haskell::keywords(), haskell::function_definers()),
        "bash" => (bash::keywords(), bash::function_definers()),
        "html" => (html::keywords(), html::function_definers()),
        "css" => (css::keywords(), css::function_definers()),
        "sql" => (sql::keywords(), sql::function_definers()),
        _ => (rust::keywords(), rust::function_definers()),
    }
}