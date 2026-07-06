use std::io::{self, Write};

fn main() -> io::Result<()> {
    io::stdout().write_all(
        ocentra_schema::notification_local_outbox_ts::notification_local_outbox_typescript()
            .as_bytes(),
    )
}
