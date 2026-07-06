use std::io::{self, Write};

fn main() -> io::Result<()> {
    io::stdout().write_all(
        ocentra_schema::notification_v3_provider_retry_ts::notification_v3_provider_retry_typescript()
            .as_bytes(),
    )
}
