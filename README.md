# Rust Mermaid

Rust bindings for `mermaid` ([website](https://mermaid.js.org)), a diagram and flowchart tool that converts a text-based definition into a diagram.

This library exposes the `render` function that converts Mermaid definitions to SVG.

## Installation

To install, run the following command
```sh
cargo add mermaid-rs
```

## Usage
```rs
use mermaid_rs::Mermaid;

fn main() {
    let mermaid = Mermaid::new().unwrap(); // An error may occur if the embedded Chromium instance fails to initialize 
    println!("{}", mermaid.render("graph TB\na-->b").unwrap());
}
```

## To-do

* Add support for custom Chromium options
* Add image comparison tests
