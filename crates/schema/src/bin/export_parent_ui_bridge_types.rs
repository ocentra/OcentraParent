use std::io::{self, Write};

fn main() -> io::Result<()> {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    let output = if arguments
        .iter()
        .any(|argument| argument == "--validation-primitives")
    {
        ocentra_schema::parent_ui_bridge_ts::parent_ui_bridge_validation_primitives_typescript()
    } else if arguments.iter().any(|argument| argument == "--validation") {
        ocentra_schema::parent_ui_bridge_ts::parent_ui_bridge_validation_typescript()
    } else {
        ocentra_schema::parent_ui_bridge_ts::parent_ui_bridge_typescript()
    };
    io::stdout().write_all(output.as_bytes())
}
