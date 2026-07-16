use std::io::{self, Write};

fn main() -> io::Result<()> {
    io::stdout().write_all(
        ocentra_schema::report_query_custody_ts::report_query_custody_contracts_typescript()
            .as_bytes(),
    )
}
