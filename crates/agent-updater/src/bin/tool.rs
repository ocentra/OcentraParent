use std::io::{stderr, Write};

#[tokio::main]
async fn main() {
    if let Err(error) = ocentra_parent_agent_maintenance::cli::run_cli().await {
        let mut output = stderr().lock();
        drop(writeln!(output, "{error}"));
        std::process::exit(1);
    }
}
