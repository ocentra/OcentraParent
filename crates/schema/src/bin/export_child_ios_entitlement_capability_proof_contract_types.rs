use std::io::{self, Write};

fn main() -> io::Result<()> {
    io::stdout().write_all(
        ocentra_schema::child_ios_entitlement_capability_proof_ts::child_ios_entitlement_capability_proof_contracts_typescript()
            .as_bytes(),
    )
}
