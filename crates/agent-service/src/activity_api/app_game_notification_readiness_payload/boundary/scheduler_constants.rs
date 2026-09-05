pub const DEFAULT_ACTIVITY_DB: &str = "ocentra-activity.sqlite";
pub const DEFAULT_PATH: &str = ".";
pub const PROOF_UNAVAILABLE_PREFIX: &str = "notification scheduler proof store unavailable: ";

const SCHEDULER_DIRECTORY_PREFIX: &str = "notification scheduler directory is not a";
const SCHEDULER_DIRECTORY_SUFFIX: &str = " private directory";
const SCHEDULER_BRIDGE_PREFIX: &str = "notification scheduler bridge is not a";
const SCHEDULER_BRIDGE_SUFFIX: &str = " private file";
const SCHEDULER_SIZE_PREFIX: &str = "notification scheduler bridge exceeds the";
const SCHEDULER_SIZE_SUFFIX: &str = " bounded read size";
const SCHEDULER_PROOF_PREFIX: &str = "notification scheduler proof store is not a";
const SCHEDULER_PROOF_SUFFIX: &str = " private directory";

pub(super) fn scheduler_directory_not_private() -> String {
    format!("{SCHEDULER_DIRECTORY_PREFIX}{SCHEDULER_DIRECTORY_SUFFIX}")
}

pub(super) fn scheduler_bridge_not_private() -> String {
    format!("{SCHEDULER_BRIDGE_PREFIX}{SCHEDULER_BRIDGE_SUFFIX}")
}

pub(super) fn scheduler_bridge_too_large() -> String {
    format!("{SCHEDULER_SIZE_PREFIX}{SCHEDULER_SIZE_SUFFIX}")
}

pub(super) fn scheduler_proof_not_private() -> String {
    format!("{SCHEDULER_PROOF_PREFIX}{SCHEDULER_PROOF_SUFFIX}")
}
