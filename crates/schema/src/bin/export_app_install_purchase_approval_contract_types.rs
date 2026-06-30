use std::io::{self, Write};

fn main() -> io::Result<()> {
    io::stdout().write_all(
        ocentra_schema::app_install_purchase_approval_ts::app_install_purchase_approval_contracts_typescript()
            .as_bytes(),
    )
}
