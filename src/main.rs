use clap::Parser;
use anyhow::{Context, Result};
/// Search for a pattern in a given file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The Pattern to look for.
    pattern: String,
    /// The path to the file to read.
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}
#[derive(Debug)]
struct CustomError(String);

fn main() -> Result<()> {
    let path = "test.txt";
    let content = std::fs::read_to_string(path)
        .with_context(||format!("Could not read file '{}'", path))?;
    println!("File content: {}", content);
    Ok(())
}