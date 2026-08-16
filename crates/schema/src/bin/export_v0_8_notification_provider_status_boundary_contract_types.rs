use std::io::{self, Write};

fn main() -> io::Result<()> {
    io::stdout().write_all(
        ocentra_schema::v0_8_notification_provider_status_boundary_ts::v0_8_notification_provider_status_boundary_typescript()
            .as_bytes(),
    )
}
