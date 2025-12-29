//! LaTeX plugin example: Convert LaTeX to Unicode.
//!
//! Run with: `cargo run --example latex`

use streamdown_plugin::latex::latex_to_unicode;

fn main() {
    println!("LaTeX to Unicode Conversion Examples\n");
    println!("=====================================\n");

    // Greek letters
    let examples = [
        (r"\alpha", "Alpha"),
        (r"\beta", "Beta"),
        (r"\gamma", "Gamma"),
        (r"\delta", "Delta"),
        (r"\pi", "Pi"),
        (r"\sigma", "Sigma"),
        (r"\omega", "Omega"),
    ];

    println!("Greek Letters:");
    for (latex, name) in &examples {
        let unicode = latex_to_unicode(latex);
        println!("  {} ({}) → {}", latex, name, unicode);
    }
    println!();

    // Math operators
    let operators = [
        (r"\times", "Multiplication"),
        (r"\div", "Division"),
        (r"\pm", "Plus-minus"),
        (r"\neq", "Not equal"),
        (r"\leq", "Less than or equal"),
        (r"\geq", "Greater than or equal"),
        (r"\infty", "Infinity"),
    ];

    println!("Math Operators:");
    for (latex, name) in &operators {
        let unicode = latex_to_unicode(latex);
        println!("  {} ({}) → {}", latex, name, unicode);
    }
    println!();

    // Superscripts and subscripts
    let scripts = [
        ("x^2", "x squared"),
        ("x^3", "x cubed"),
        ("x_1", "x subscript 1"),
        ("x_n", "x subscript n"),
        ("a^2 + b^2 = c^2", "Pythagorean theorem"),
    ];

    println!("Superscripts and Subscripts:");
    for (latex, name) in &scripts {
        let unicode = latex_to_unicode(latex);
        println!("  {} ({}) → {}", latex, name, unicode);
    }
    println!();

    // Full equations
    let equations = [
        r"E = mc^2",
        r"\alpha + \beta = \gamma",
        r"\sum_{i=1}^{n} x_i",
        r"\int_0^\infty e^{-x} dx = 1",
    ];

    println!("Full Equations:");
    for latex in &equations {
        let unicode = latex_to_unicode(latex);
        println!("  {} \n    → {}\n", latex, unicode);
    }
}
