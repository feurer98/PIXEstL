//! PIXEstL CLI entry point

use pixestl::{cli, Result};

fn main() -> Result<()> {
    let _config = cli::parse_args()?;

    println!("PIXEstL - Color Lithophane Generator");
    println!("Phase 2 (Color Module) implementation in progress...");

    Ok(())
}
