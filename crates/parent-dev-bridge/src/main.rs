use ocentra_parent_agent_protocol::constants;
use ocentra_parent_dev_bridge::{
    configured_parent_dev_bridge_address, log_parent_dev_bridge_error, serve_parent_dev_bridge,
    ParentDevBridgeErrorMessage,
};
use ocentra_parent_logging_core::field::{LogFieldValue, LogFields};
use ocentra_parent_logging_core::{dev_log::DevLogger, source::LogSource};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let Some(address) = configured_parent_dev_bridge_address() else {
        let _ = DevLogger::from_env(LogSource::LocalApi).and_then(|logger| {
            let mut fields = LogFields::new();
            fields.insert(
                constants::field::REASON.to_string(),
                LogFieldValue::String(constants::error::PARENT_DEV_BRIDGE_RUNS.to_string()),
            );
            logger.error(constants::error::PARENT_DEV_BRIDGE_RUNS, fields)
        });
        std::process::exit(1);
    };

    if let Err(error) = serve_parent_dev_bridge(address).await {
        log_parent_dev_bridge_error(ParentDevBridgeErrorMessage::Run, Some(address), &error);
        std::process::exit(1);
    }
}
