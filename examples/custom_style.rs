//! Custom style example: Render with custom colors.
//!
//! Run with: `cargo run --example custom_style`

use streamdown_parser::Parser;
use streamdown_render::{RenderStyle, Renderer};

fn main() {
    let markdown = r#"# Custom Styled Output

This example shows how to customize the colors and styling.

## Code Block

```python
def greet(name):
    return f"Hello, {name}!"

print(greet("World"))
```

## Features

- **Bold text** stands out
- *Italic text* is emphasized
- `inline code` is highlighted

> A quote with custom colors!
"#;

    // Create a custom style with different colors
    // Colors are in "r;g;bm" format for ANSI sequences
    let custom_style = RenderStyle {
        // Bright cyan for emphasis
        bright: "0;255;255".to_string(),
        // Green for headings
        head: "0;255;128".to_string(),
        // Yellow for symbols/bullets
        symbol: "255;255;0".to_string(),
        // Gray for muted text
        grey: "128;128;128".to_string(),
        // Dark blue for backgrounds
        dark: "20;20;60".to_string(),
        // Medium purple
        mid: "80;60;120".to_string(),
        // Light lavender
        light: "180;160;220".to_string(),
    };

    // Create output buffer
    let mut output = Vec::new();

    // Create parser
    let mut parser = Parser::new();

    // Get terminal width
    let width = terminal_size::terminal_size()
        .map(|(w, _)| w.0 as usize)
        .unwrap_or(80);

    {
        // Create renderer with custom style
        let mut renderer = Renderer::with_style(&mut output, width, custom_style);

        // Parse and render
        for line in markdown.lines() {
            let events = parser.parse_line(line);
            for event in events {
                renderer.render_event(&event).unwrap();
            }
        }
    }

    // Print the styled output
    print!("{}", String::from_utf8(output).unwrap());
}
