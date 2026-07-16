use std::io::{self, Write};

fn main() -> io::Result<()> {
    io::stdout().write_all(
        ocentra_schema::parent_storage_settings_apply_flow_ts::parent_storage_settings_apply_flow_contracts_typescript()
            .as_bytes(),
    )
}
