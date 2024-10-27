use std::{error::Error, fmt::Display};

/// Error thrown if Mermaid is unable to compile the diagram
#[derive(Debug)]
pub struct CompileError;

impl Error for CompileError {
    fn description(&self) -> &str {
        "Error occured while compiling the diagram!"
    }
}

impl Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CompileError")
    }
}
