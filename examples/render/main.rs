use mermaid::Mermaid;

fn main() {
    let mermaid = Mermaid::new().unwrap();
    println!("{}", mermaid.render("graph TB\na-->b").unwrap());
}
