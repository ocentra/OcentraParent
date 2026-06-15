use std::io::{self, Write};

use ocentra_eventing::compatibility::EventCompatibilityMatrix;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matrix = EventCompatibilityMatrix::ocentra_games_lineage();
    let mut stdout = io::stdout().lock();
    stdout.write_all(matrix.render_markdown().as_bytes())?;
    Ok(())
}
