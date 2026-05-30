pub const READ_MODEL_ID: &str = "local-ai-runtime-provider-proof";
pub const SOURCE_LOCAL_AI_PROVIDER_SCHEDULER: &str = "local-ai-provider-scheduler";
pub const SOURCE_DEVICE_ROLE_RUNTIME_READ_MODEL: &str = "device-role-runtime-read-model";
pub const SOURCE_PARENT_ASSISTANT_RUNTIME: &str = "parent-assistant-runtime";
pub const ENTRY_ID_SINGLE_PROVIDER_ROLE: &str = "local-ai-proof-single-provider-role";
pub const ENTRY_ID_SHARED_PARENT_CHILD_PROVIDER: &str =
    "local-ai-proof-shared-parent-child-provider";
pub const ENTRY_ID_SINGLE_RUNTIME_LANE: &str = "local-ai-proof-single-runtime-lane";
pub const ENTRY_ID_CHILD_SAFETY_PRIORITY: &str = "local-ai-proof-child-safety-priority";
pub const ENTRY_ID_QUEUED_DEGRADED_LIFECYCLE: &str = "local-ai-proof-queued-degraded-lifecycle";
pub const ENTRY_ID_PARENT_ASSISTANT_SUBMIT: &str = "local-ai-proof-parent-assistant-submit";
pub const ENTRY_ID_NO_DUPLICATE_MODEL_LOAD: &str = "local-ai-proof-no-duplicate-model-load";
pub const ENTRY_ID_STATUS_CONTRACT_HARDENING: &str = "local-ai-proof-status-contract-hardening";

pub const REQUIREMENT_ONE_PROVIDER_ROLE: &str = "one-ai-provider-role-per-physical-device";
pub const REQUIREMENT_SHARED_PARENT_CHILD_PROVIDER: &str = "shared-parent-child-provider";
pub const REQUIREMENT_SINGLE_RUNTIME_LANE: &str = "single-local-runtime-lane";
pub const REQUIREMENT_CHILD_SAFETY_PRIORITY: &str = "child-safety-priority";
pub const REQUIREMENT_LIFECYCLE: &str = "queued-degraded-unavailable-lifecycle";
pub const REQUIREMENT_PARENT_ASSISTANT_SUBMIT: &str = "parent-assistant-submits-when-allowed";
pub const REQUIREMENT_NO_DUPLICATE_MODEL_LOAD: &str = "no-duplicate-local-model-load";
pub const REQUIREMENT_STATUS_CONTRACT_HARDENING: &str = "provider-status-contract-hardening";

pub const PROOF_STATUS_PROVED: &str = "proved";
pub const PROOF_STATUS_DEGRADED: &str = "degraded";
pub const PROOF_STATUS_UNAVAILABLE: &str = "unavailable";
pub const PROOF_STATUS_NOT_CLAIMED: &str = "not-claimed";

pub const CAPABILITY_ONE_PROVIDER_ROLE: &str =
    "Physical-device id and singleton scope must identify the provider lane.";
pub const CAPABILITY_SHARED_PARENT_CHILD: &str =
    "Parent and child roles must share one local provider on the physical device.";
pub const CAPABILITY_SINGLE_RUNTIME_LANE: &str =
    "One local model runtime lane per physical device.";
pub const CAPABILITY_CHILD_PRIORITY: &str =
    "Child-safety jobs must outrank parent assistant jobs on the shared lane.";
pub const CAPABILITY_LIFECYCLE: &str =
    "Queued and degraded state must be explicit for Portal and runtime clients.";
pub const CAPABILITY_PARENT_ASSISTANT: &str =
    "Configured local runtime, local-only privacy mode, and allowed parent-assistant job class.";
pub const CAPABILITY_NO_DUPLICATE_LOAD: &str =
    "No duplicate local model load for the same physical device.";
pub const CAPABILITY_STATUS_HARDENING: &str =
    "Unavailable/degraded provider status must be schema-valid and reasoned.";

pub const PROOF_ONE_PROVIDER_ROLE: &str =
    "Typed contract rejects duplicate roles and duplicate runtime loads.";
pub const PROOF_SHARED_PARENT_CHILD: &str =
    "Scheduler state preserves identical physicalDeviceId, providerId, and runtimeReferenceId.";
pub const PROOF_SINGLE_RUNTIME_LANE: &str =
    "Service scheduler queue tests assert max active local generation jobs stays one.";
pub const PROOF_CHILD_PRIORITY: &str =
    "Rust service scheduler test observes parent-report, child-safety, parent-assistant order.";
pub const PROOF_LIFECYCLE: &str =
    "Typed scheduler status and parent assistant runtime tests preserve queued degraded states.";
pub const PROOF_PARENT_ASSISTANT: &str =
    "Parent assistant runtime submits through the same scheduler instead of bypassing the lane.";
pub const PROOF_NO_DUPLICATE_LOAD: &str =
    "Contract and service tests fail when runtimeLoadCount exceeds one.";
pub const PROOF_STATUS_HARDENING: &str =
    "Typed contracts reject unavailable provider status without an unavailable reason.";

pub const CLAIM_LOCAL_ONLY: &str =
    "This proves a local physical-device provider role, not LAN provider pooling.";
pub const CLAIM_SHARED_PROVIDER: &str =
    "This is same-device local sharing, not cross-device LAN AI routing.";
pub const CLAIM_NO_MODEL_QUALITY: &str =
    "This does not claim model quality, classifier quality, or remote provider access.";
pub const CLAIM_PRIORITY_ONLY: &str =
    "This is scheduler priority proof, not a child-safety model accuracy claim.";
pub const CLAIM_DEGRADED: &str =
    "Degraded state is a runtime availability claim, not enforcement or safety approval.";
pub const CLAIM_NO_API_PROVIDER: &str =
    "Parent assistant local submission does not authorize API/remote providers by default.";
pub const CLAIM_NO_CROSS_DEVICE_SHARING: &str =
    "This does not claim memory sharing across different physical devices.";
pub const CLAIM_UNAVAILABLE_HONEST: &str =
    "Unavailable status is honest and must not be promoted to a working provider.";

pub const FALLBACK_UNAVAILABLE: &str =
    "Return unavailable when no local provider runtime can be configured.";
pub const FALLBACK_QUEUE_OR_DEGRADE: &str =
    "Degrade or queue jobs instead of starting a second model runtime.";
pub const FALLBACK_BUSY: &str =
    "Queue lower-priority jobs or return degraded when the lane is busy.";
pub const FALLBACK_PARENT_ASSISTANT_PRIORITY: &str =
    "Keep parent-assistant work queued or degraded until child-safety work can run first.";
pub const FALLBACK_DEGRADED_ANSWER: &str =
    "Return queued/degraded answers with no local AI result id when the provider is busy.";
pub const FALLBACK_LOCAL_RUNTIME_MISSING: &str =
    "Return unavailable or degraded when local runtime config is missing or the lane is busy.";
pub const FALLBACK_BLOCK_DUPLICATE: &str =
    "Block duplicate runtime admission and queue/degrade additional work.";
pub const FALLBACK_UNCONFIGURED: &str =
    "Return unavailable with local-ai-provider-unconfigured and no selected runtime.";
