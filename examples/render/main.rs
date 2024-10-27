use mermaid_rs::Mermaid;

fn main() {
    let mermaid = Mermaid::new().unwrap();
    println!("{}", mermaid.render("graph TB\na-->b").unwrap());
}
