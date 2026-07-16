use std::io::{self, Write};

fn main() -> io::Result<()> {
    io::stdout().write_all(
        ocentra_schema::parent_ui_bridge_ts::parent_ui_screen_bridge_typescript().as_bytes(),
    )
}
