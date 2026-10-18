//! `cargo run --release` prints the model tables.
//! `cargo run --release -- --write` regenerates `output/revenue_tables.md`, the paper,
//! the README and `CLAIMS.md` from the templates.

use std::path::Path;
use vat_revenue_model::{docs, report, Result};

fn main() -> Result<()> {
    let tables = report::render()?;
    if std::env::args().any(|a| a == "--write") {
        let crate_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        std::fs::write(crate_dir.join("output/revenue_tables.md"), &tables)?;
        let repo = crate_dir.parent().unwrap_or(crate_dir);
        for (path, text) in docs::render_documents()? {
            std::fs::write(repo.join(path), text)?;
            println!("wrote {path}");
        }
        println!("wrote analysis/output/revenue_tables.md");
    } else {
        print!("{tables}");
    }
    Ok(())
}
