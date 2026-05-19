#[tokio::main]
async fn main() {
    if let Err(error) = ocentra_parent_agent_maintenance::cli::run_cli().await {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
