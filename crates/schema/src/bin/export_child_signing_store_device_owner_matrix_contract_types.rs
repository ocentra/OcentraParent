use std::io::{self, Write};

fn main() -> io::Result<()> {
    io::stdout().write_all(
        ocentra_schema::child_signing_store_device_owner_matrix_ts::child_signing_store_device_owner_matrix_contracts_typescript()
            .as_bytes(),
    )
}
