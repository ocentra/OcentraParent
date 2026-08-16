use std::io::{self, Write};

fn main() -> io::Result<()> {
    io::stdout()
        .write_all(ocentra_schema::logging_contracts_ts::logging_contracts_typescript().as_bytes())
}
