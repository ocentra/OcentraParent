use std::io::{self, Write};

fn main() -> io::Result<()> {
    io::stdout().write_all(
        ocentra_schema::data_custody_source_of_truth_ts::data_custody_source_of_truth_contracts_typescript()
            .as_bytes(),
    )
}
