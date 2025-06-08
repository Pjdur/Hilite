# Hilite

**Hilite** is a fast, efficient, and extensible syntax highlighter written in Rust. It supports multiple programming languages and outputs colorized code for terminal display using ANSI escape codes.

---

## Features

- **Multi-language support**: 15+ languages including Rust, Python, JavaScript, C, C++, Java, Go, Swift, Kotlin, PHP, Ruby, TypeScript, C#, Haskell, Bash, HTML, CSS, SQL, and more.
- **Lightweight**: Minimal dependencies (only `regex`).
- **Flexible API**: Get structured tokens for custom rendering or use built-in terminal colorization.
- **Easy integration**: Use as a library in your Rust projects.
- **Customizable**: Easily add new languages or tweak highlighting rules.

---

## Installation

Add Hilite to your project with [Cargo](https://crates.io):

```sh
cargo add hilite
```

Or add it manually to your `Cargo.toml`:

```toml
[dependencies]
hilite = "0.1"
```

---

## Usage

### 1. Highlighting Code

The main entry point is the `highlight` function, which tokenizes and annotates your code with color information.

```rust
use hilite::highlight;

let code = r#"
fn main() {
    println!("Hello, world!");
}
"#;

let highlighted = highlight(code, "rust");
```

- The first argument is the code as a string.
- The second argument is the language name (case-insensitive, e.g., `"rust"`, `"python"`, `"javascript"`).

#### Output

`highlight` returns a `Vec<(String, Vec<Token>)>`, where each tuple represents a line:
- The first element is the line's indentation.
- The second element is a vector of `Token` structs, each containing:
  - `text`: The code fragment.
  - `color`: The color name (e.g., `"blue"`, `"yellow"`, `"green"`, `"default"`).

Example:

```rust
for (indent, tokens) in highlighted {
    print!("{}", indent);
    for token in tokens {
        print!("[{}:{}]", token.color, token.text);
    }
    println!();
}
```

> **Note**: This is also what the [colorize](#2-colorizing-for-terminal-output) does. You can use this code to manually write out the colored terminal output, but using colorize is general recommended as it makes things easier. But for control over output, this code is better.
---

### 2. Colorizing for Terminal Output

For terminal output with ANSI colors, use the `colorize` function:

```rust
use hilite::{highlight, colorize};

let code = r#"
fn main() {
    println!("Hello, world!");
}
"#;

let highlighted = highlight(code, "rust");
colorize(highlighted);
```

This prints the code to the terminal with syntax highlighting.

---

### 3. Supported Languages

Hilite supports the following languages (and more can be added easily):

- Rust
- Python
- JavaScript
- TypeScript
- C
- C++
- Java
- Go
- Swift
- Kotlin
- PHP
- Ruby
- C#
- Haskell
- Bash
- HTML
- CSS
- SQL

---

### 4. Adding or Customizing Languages

To add or customize language support, edit the corresponding file in `src/languages/` (e.g., [`src/languages/rust.rs`](src/languages/rust.rs)).  
Each language defines:
- A `keywords()` function returning a map of keywords to color names.
- A `function_definers()` function returning a list of function-defining keywords.

---

## API Reference

### Token

```rust
pub struct Token {
    pub text: String,
    pub color: String,
}
```

### Functions

#### `highlight(code: &str, language: &str) -> Vec<(String, Vec<Token>)>`

Tokenizes and annotates code for the given language.

#### `colorize(highlighted: Vec<(String, Vec<Token>)>)`

Prints highlighted code to the terminal using ANSI escape codes.

---

## Example

```rust
use hilite::{highlight, colorize};

fn main() {
    let code = r#"
fn main() {
    let x = 42;
    println!("x = {}", x);
}
"#;
    let highlighted = highlight(code, "rust");
    colorize(highlighted);
}
```

---

## Contribution

Contributions are welcome!  
To add a new language, create a new file in `src/languages/` and implement the required functions.

---

## License

MIT

---

## Acknowledgements

- Inspired by classic syntax highlighters and the Rust ecosystem.
```
