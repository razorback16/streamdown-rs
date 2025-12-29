//! Basic example: Render a markdown string to the terminal.
//!
//! Run with: `cargo run --example basic`

use streamdown_parser::Parser;
use streamdown_render::Renderer;

fn main() {
    let markdown = r#"# Welcome to Streamdown!

This is a **Rust** port of the original Python [Streamdown](https://github.com/kristopolous/Streamdown).

## Features

- Streaming markdown rendering
- Syntax highlighting
- Beautiful terminal output

## Code Example

```rust
fn main() {
    println!("Hello, World!");
}
```

## Table

| Feature | Status |
|---------|--------|
| Parser  | ✅ Done |
| Render  | ✅ Done |

> "The best way to predict the future is to invent it."
> — Alan Kay
"#;

    // Create output buffer
    let mut output = Vec::new();

    // Create parser and renderer
    let mut parser = Parser::new();

    // Get terminal width (default to 80 if not available)
    let width = terminal_size::terminal_size()
        .map(|(w, _)| w.0 as usize)
        .unwrap_or(80);

    {
        let mut renderer = Renderer::new(&mut output, width);

        // Parse and render each line
        for line in markdown.lines() {
            let events = parser.parse_line(line);
            for event in events {
                renderer.render_event(&event).unwrap();
            }
        }
    }

    // Print the rendered output
    print!("{}", String::from_utf8(output).unwrap());
}
