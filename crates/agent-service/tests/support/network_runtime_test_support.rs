use ocentra_parent_agent_core::network_event_runtime::NetworkRuntimeJournalPath;
use ocentra_parent_agent_protocol::logging::LogFields;
use ocentra_parent_agent_protocol::network_flow::ActivityNetworkFlowReadModel;
use std::{
    fs,
    path::PathBuf,
    string::String as TestString,
    sync::OnceLock,
    time::{SystemTime, UNIX_EPOCH},
};
use tokio::sync::OnceCell;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetworkRuntimeTestError {
    /// BRAND-INVARIANT: this is test-only failure context, not user data.
    context: TestString,
}

impl NetworkRuntimeTestError {
    fn new(context: TestString) -> Self {
        Self { context }
    }

    fn with_cause(context: &str, cause: &impl std::fmt::Debug) -> Self {
        Self::new(format!("{context}: {cause:?}"))
    }
}

impl std::fmt::Display for NetworkRuntimeTestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.context.as_str())
    }
}

impl std::error::Error for NetworkRuntimeTestError {}

static NETWORK_RUNTIME_TEST_SPINE: OnceCell<()> = OnceCell::const_new();
static NETWORK_RUNTIME_TEST_JOURNAL_PATH: OnceLock<
    Result<NetworkRuntimeJournalPath, NetworkRuntimeTestError>,
> = OnceLock::new();

async fn ensure_network_runtime_spine_for_test() -> Result<(), NetworkRuntimeTestError> {
    NETWORK_RUNTIME_TEST_SPINE
        .get_or_try_init(|| async {
            let path = network_runtime_journal_path_for_test()?;
            crate::network_runtime_delivery::initialize_network_runtime_spine(&path)
                .await
                .map_err(|error| {
                    NetworkRuntimeTestError::with_cause(
                        "durable network runtime test spine must initialize",
                        &error,
                    )
                })
        })
        .await
        .map(|_| ())
}

pub fn network_runtime_journal_path_for_test(
) -> Result<NetworkRuntimeJournalPath, NetworkRuntimeTestError> {
    NETWORK_RUNTIME_TEST_JOURNAL_PATH
        .get_or_init(|| {
            let artifact_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("..")
                .join("..")
                .join("target")
                .join("test-artifacts")
                .join("network-runtime");
            fs::create_dir_all(&artifact_dir).map_err(|error| {
                NetworkRuntimeTestError::with_cause(
                    "network runtime test artifact directory must exist",
                    &error,
                )
            })?;
            let artifact_dir = fs::canonicalize(&artifact_dir).map_err(|error| {
                NetworkRuntimeTestError::with_cause(
                    "network runtime test artifact directory must resolve",
                    &error,
                )
            })?;
            let nonce = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map_err(|error| {
                    NetworkRuntimeTestError::with_cause(
                        "test clock must be after the Unix epoch",
                        &error,
                    )
                })?
                .as_nanos();
            let path = artifact_dir.join(format!(
                "{}-{}-network-runtime-journal.ndjson",
                std::process::id(),
                nonce
            ));
            Ok(NetworkRuntimeJournalPath::new(path))
        })
        .clone()
}

pub async fn lock_activity_report_env_for_test() -> tokio::sync::MutexGuard<'static, ()> {
    crate::activity_report_env_lock::REPORT_ENV_LOCK
        .lock()
        .await
}

pub async fn deliver_network_runtime_for_read_model_for_test(
    read_model: &ActivityNetworkFlowReadModel,
) -> Result<
    crate::network_runtime_delivery::NetworkRuntimeServiceDeliveryReport,
    NetworkRuntimeTestError,
> {
    ensure_network_runtime_spine_for_test().await?;
    let report =
        crate::network_runtime_delivery::read_network_runtime_delivery_for_read_model(read_model)
            .await;
    Ok(report)
}

pub async fn seed_network_runtime_for_test(
    read_model: &ActivityNetworkFlowReadModel,
) -> Result<(), NetworkRuntimeTestError> {
    ensure_network_runtime_spine_for_test().await?;
    let observations = read_model
        .rows
        .iter()
        .map(|row| {
            let observation =
                crate::network_runtime_delivery::network_runtime_observation_from_row(row)
                    .map_err(|error| {
                        NetworkRuntimeTestError::with_cause(
                            "canonical associated PID count must be present in test read model",
                            &error,
                        )
                    })?;
            Ok(
                crate::activity_capture_network_observation::NetworkCaptureObservation {
                    source_event_id: row.event_id.clone(),
                    observed_at: row.observed_at.clone(),
                    observation,
                },
            )
        })
        .collect::<Result<Vec<_>, NetworkRuntimeTestError>>()?;
    crate::network_runtime_delivery::publish_captured_network_observations(&observations)
        .await
        .map_err(|error| {
            NetworkRuntimeTestError::with_cause(
                "durable network runtime test publication must succeed",
                &error,
            )
        })
}

pub fn network_flow_read_model_payload_with_runtime_delivery_for_test(
    read_model: &ActivityNetworkFlowReadModel,
    delivery: Option<&crate::network_runtime_delivery::NetworkRuntimeServiceDeliveryReport>,
) -> LogFields {
    crate::activity_network_flow_payload::network_flow_read_model_payload_with_runtime_delivery(
        read_model, delivery,
    )
}

pub async fn stream_network_runtime_event_chain_for_read_model_for_test(
    read_model: &ActivityNetworkFlowReadModel,
) -> Result<
    crate::network_runtime_stream_payload::NetworkRuntimeServiceStreamReport,
    NetworkRuntimeTestError,
> {
    ensure_network_runtime_spine_for_test().await?;
    Ok(
        crate::network_runtime_stream_payload::stream_network_runtime_event_chain_for_read_model(
            read_model,
        )
        .await,
    )
}

pub fn network_runtime_event_chain_stream_payload_for_test(
    report: &crate::network_runtime_stream_payload::NetworkRuntimeServiceStreamReport,
) -> LogFields {
    crate::network_runtime_stream_payload::network_runtime_event_chain_stream_payload(report)
}
