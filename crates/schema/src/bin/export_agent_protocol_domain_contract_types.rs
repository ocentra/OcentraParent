use std::io::{self, Write};

fn main() -> io::Result<()> {
    io::stdout().write_all(
        ocentra_schema::parent_ui_bridge_ts::agent_protocol_domain_contracts_typescript()
            .as_bytes(),
    )
}
