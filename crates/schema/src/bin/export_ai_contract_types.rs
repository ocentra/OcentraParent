use std::io::{self, Write};

fn main() -> io::Result<()> {
    io::stdout()
        .write_all(ocentra_ai_contracts::ai_contracts_ts::ai_contracts_typescript().as_bytes())
}
