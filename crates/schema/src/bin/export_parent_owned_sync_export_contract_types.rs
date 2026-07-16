use std::io::{self, Write};

fn main() -> io::Result<()> {
    io::stdout().write_all(
        ocentra_schema::parent_owned_sync_export_ts::parent_owned_sync_export_contracts_typescript(
        )
        .as_bytes(),
    )
}
