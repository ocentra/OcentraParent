pub const COMMAND_ID: &str = "tracking-retention-settings-write-command";
pub const EVENT_ID: &str = "tracking-retention-settings-write-result";
pub const ACCEPTED_AT: &str = "2026-06-06T19:50:00Z";
pub const SETTINGS_KIND_RETENTION_WINDOW: &str = "retention-window-setting";
pub const WRITE_STATE_ACCEPTED: &str = "service-write-command-accepted";
pub const WRITE_STATE_REJECTED: &str = "service-write-command-rejected";
pub const MUTATION_PROOF_REF: &str = "output/tracking-plan-proof/07-retention-and-custody-model/20-retention-settings-mutation-proof.json";
pub const WRITER_INTENT_REF: &str = "tracking-retention-settings-write-retention-window";
pub const READ_MODEL_PROOF_REF: &str = "output/tracking-plan-proof/07-retention-and-custody-model/18-retention-settings-read-model-proof.json";
pub const JOURNAL_READ_MODEL_PROOF_REF: &str = "output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/24-retention-settings-read-model-proof.json";
pub const LOCAL_SERVICE_STATE_SNAPSHOT_REF: &str = "agent-service-local-retention-settings-state";
pub const DURABLE_SETTINGS_STORE_REF: &str = "agent-service-local-retention-settings-durable-json";
pub const DURABLE_SETTINGS_STORE_FILE_NAME: &str =
    "ocentra-parent-tracking-retention-settings-durable-proof.json";
