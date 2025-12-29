//! Streaming example: Render markdown from stdin in real-time.
//!
//! Run with: `echo "# Hello" | cargo run --example streaming`
//! Or: `cat README.md | cargo run --example streaming`

use std::io::{self, BufRead, Write};

use streamdown_parser::Parser;
use streamdown_render::Renderer;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    // Create parser
    let mut parser = Parser::new();

    // Get terminal width
    let width = terminal_size::terminal_size()
        .map(|(w, _)| w.0 as usize)
        .unwrap_or(80);

    // Collect lines first, then render
    let lines: Vec<String> = stdin.lock().lines().collect::<Result<_, _>>()?;

    {
        // Create renderer writing directly to stdout
        let mut renderer = Renderer::new(&mut stdout, width);

        // Process each line
        for line in &lines {
            // Parse the line into events
            let events = parser.parse_line(line);

            // Render each event
            for event in events {
                renderer.render_event(&event)?;
            }
        }
    }

    // Flush at the end
    stdout.flush()?;

    Ok(())
}
