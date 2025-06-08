use synk::{colorize, highlight};

fn main() {
    let code = r#"
fn main() {
    let x = 42; // This is a number
    let y = 3.14; // Another number
    println!("Hello, world!"); // Print statement
    if x > 10 {
        println!("x is greater than 10");
    } else {
        println!("x is not greater than 10");
    }
}
// This is a comment
fn add(a: i32, b: i32) -> i32 {
    a + b // Add two numbers
}
"#;

    let highlighted = highlight(code, "rust");
    colorize(highlighted);
}
