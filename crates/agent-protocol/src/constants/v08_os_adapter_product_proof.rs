pub const READ_MODEL_ID: &str = "v0-8-os-adapter-product-proof";

pub const ENTRY_ID_OWNED_PROCESS_TERMINATE: &str = "v0-8-proof-owned-process-terminate";
pub const ENTRY_ID_APP_TIME_LIMIT_LIFECYCLE: &str = "v0-8-proof-app-time-limit-lifecycle";
pub const ENTRY_ID_BROAD_APP_BLOCKING: &str = "v0-8-proof-broad-app-blocking";
pub const ENTRY_ID_NETWORK_DOMAIN_BLOCKING: &str = "v0-8-proof-network-domain-blocking";
pub const ENTRY_ID_MANAGED_BROWSER_SERVICE_COMMAND: &str =
    "v0-8-proof-managed-browser-service-command";
pub const ENTRY_ID_MANAGED_BROWSER_EXACT_URL: &str = "v0-8-proof-managed-browser-exact-url";
pub const ENTRY_ID_UNMANAGED_BROWSER_PROCESS_ONLY: &str =
    "v0-8-proof-unmanaged-browser-process-only";
pub const ENTRY_ID_UNMANAGED_BROWSER_EXACT_EVIDENCE: &str =
    "v0-8-proof-unmanaged-browser-exact-evidence";
pub const ENTRY_ID_RESTART_RECOVERY: &str = "v0-8-proof-restart-recovery";
pub const ENTRY_ID_PARENT_CANCEL_OVERRIDE: &str = "v0-8-proof-parent-cancel-override";
pub const ENTRY_ID_AUDIT_CUSTODY: &str = "v0-8-proof-audit-custody";
pub const ENTRY_ID_ROLLBACK_ARTIFACT_GATE: &str = "v0-8-proof-rollback-artifact-gate";

pub const SURFACE_OWNED_PROCESS_TERMINATE: &str = "owned-process-terminate";
pub const SURFACE_APP_TIME_LIMIT_LIFECYCLE: &str = "app-time-limit-lifecycle";
pub const SURFACE_BROAD_APP_BLOCKING: &str = "broad-app-blocking";
pub const SURFACE_NETWORK_DOMAIN_BLOCKING: &str = "network-domain-blocking";
pub const SURFACE_MANAGED_BROWSER_SERVICE_COMMAND: &str = "managed-browser-service-command";
pub const SURFACE_MANAGED_BROWSER_EXACT_URL: &str = "managed-browser-exact-url";
pub const SURFACE_UNMANAGED_BROWSER_PROCESS_ONLY: &str = "unmanaged-browser-process-only";
pub const SURFACE_UNMANAGED_BROWSER_EXACT_EVIDENCE: &str = "unmanaged-browser-exact-evidence";
pub const SURFACE_RESTART_RECOVERY: &str = "restart-recovery";
pub const SURFACE_PARENT_CANCEL_OVERRIDE: &str = "parent-cancel-override";
pub const SURFACE_AUDIT_CUSTODY: &str = "audit-custody";
pub const SURFACE_ROLLBACK_ARTIFACT_GATE: &str = "rollback-artifact-gate";

pub const TIMER_STATE_NOT_REQUIRED: &str = "not-required";
pub const TIMER_STATE_RESTART_RECOVERED: &str = "restart-recovered";
pub const TIMER_STATE_CANCELLED: &str = "cancelled";
pub const TIMER_STATE_EXPIRED: &str = "expired";
pub const TIMER_STATE_MANUAL_REQUIRED: &str = "manual-required";
pub const TIMER_STATE_UNAVAILABLE: &str = "unavailable";

pub const AUDIT_STATE_JOURNALED: &str = "journaled";
pub const AUDIT_STATE_MANUAL_REQUIRED: &str = "manual-required";
pub const AUDIT_STATE_UNAVAILABLE: &str = "unavailable";

pub const PARENT_OVERRIDE_NOT_REQUIRED: &str = "not-required";
pub const PARENT_OVERRIDE_CANCEL_SUPPORTED: &str = "cancel-supported";
pub const PARENT_OVERRIDE_MANUAL_REQUIRED: &str = "manual-required";
pub const PARENT_OVERRIDE_UNAVAILABLE: &str = "unavailable";

pub const CAPABILITY_OWNED_PROCESS: &str = "Owned process pid plus expected process name.";
pub const CAPABILITY_APP_TIME_LIMIT: &str =
    "Persisted app time-limit state and owned-process expiry path.";
pub const CAPABILITY_BROAD_APP: &str =
    "OS-approved installed app identity and apply/rollback adapter.";
pub const CAPABILITY_NETWORK_DOMAIN: &str =
    "Host network filter adapter with apply and rollback proof.";
pub const CAPABILITY_MANAGED_BROWSER_COMMAND: &str =
    "Managed browser command channel with audited apply behavior.";
pub const CAPABILITY_MANAGED_EXACT_URL: &str =
    "Managed browser active-tab and exact URL integration.";
pub const CAPABILITY_UNMANAGED_PROCESS: &str = "Unmanaged browser process pid/name evidence only.";
pub const CAPABILITY_BROWSER_INTEGRATION: &str =
    "Managed browser or another explicit browser integration.";
pub const CAPABILITY_RESTART_RECOVERY: &str = "Persisted timer state after service restart.";
pub const CAPABILITY_PARENT_CANCEL: &str =
    "Parent cancel/override reference tied to active timer state.";
pub const CAPABILITY_AUDIT_CUSTODY: &str =
    "Local audit journal/store custody for enforcement outcomes.";
pub const CAPABILITY_ROLLBACK_ARTIFACT_GATE: &str =
    "Same-identity apply, rollback, and custody artifacts.";

pub const PROOF_OWNED_PROCESS: &str = "Real service termination result and audit journal event.";
pub const PROOF_APP_TIME_LIMIT: &str =
    "Timer create, expiry, cancel, restart recovery, and audit tests.";
pub const PROOF_BROAD_APP: &str =
    "Same-identity app package evidence, apply result, rollback result, and custody event.";
pub const PROOF_NETWORK_DOMAIN: &str =
    "Network/domain filter apply result, rollback result, and audit custody event.";
pub const PROOF_MANAGED_BROWSER_COMMAND: &str =
    "Managed-browser command enforcement proof and exact URL apply/audit proof.";
pub const PROOF_MANAGED_EXACT_URL: &str =
    "Managed exact URL evidence, apply result, and custody audit.";
pub const PROOF_UNMANAGED_PROCESS: &str =
    "Process-only warn/terminate proof without exact URL or active tab evidence.";
pub const PROOF_UNMANAGED_EXACT: &str =
    "Exact URL, active tab, title, download, page, HTTPS content, or intent proof.";
pub const PROOF_RESTART_RECOVERY: &str =
    "Restart recovery test preserving action/result/audit/timer identity.";
pub const PROOF_PARENT_CANCEL: &str = "Parent cancel path that records rollback and audit state.";
pub const PROOF_AUDIT_CUSTODY: &str = "Audit event and journal sequence for attempted, succeeded, unavailable, expired, and cancelled paths.";
pub const PROOF_ROLLBACK_ARTIFACT_GATE: &str =
    "Artifact gate proof before any broad rollback or anti-tamper product claim.";

pub const CLAIM_OWNED_PROCESS: &str =
    "Only owned-process pid/name termination is proved; this is not global app blocking.";
pub const CLAIM_APP_TIME_LIMIT: &str =
    "App time-limit proof is lifecycle proof, not broad installed-app blocking.";
pub const CLAIM_BROAD_APP: &str =
    "Broad installed-app blocking is not proved by owned-process termination.";
pub const CLAIM_NETWORK_DOMAIN: &str =
    "Network flow metadata is not decrypted content and does not prove domain blocking.";
pub const CLAIM_MANAGED_BROWSER_COMMAND: &str =
    "A managed-browser service-command target string is not exact URL enforcement proof.";
pub const CLAIM_MANAGED_EXACT_URL: &str =
    "Exact URL, active tab, and page-title control require the managed browser boundary.";
pub const CLAIM_UNMANAGED_PROCESS: &str = "Unmanaged browser proof is process-only and cannot become URL/tab/title/download/page evidence.";
pub const CLAIM_UNMANAGED_EXACT: &str =
    "Process/window/network evidence does not prove exact unmanaged browser activity.";
pub const CLAIM_RESTART_RECOVERY: &str =
    "Restart recovery proves local timer custody, not anti-tamper or bypass resistance.";
pub const CLAIM_PARENT_CANCEL: &str =
    "Parent cancel is timer-scoped and does not prove broad unblock rollback.";
pub const CLAIM_AUDIT_CUSTODY: &str =
    "Audit custody is local evidence recording, not production anti-tamper hardening.";
pub const CLAIM_ROLLBACK_ARTIFACT_GATE: &str =
    "Admin hardening, anti-tamper, bypass resistance, and broad rollback are not proved.";

pub const FALLBACK_OWNED_PROCESS: &str =
    "Reject missing pid/name mismatch and return unavailable on unsupported hosts.";
pub const FALLBACK_APP_TIME_LIMIT: &str =
    "Return unavailable when active timer state or platform adapter cannot support the request.";
pub const FALLBACK_BROAD_APP: &str = "Return manual-required or unavailable and avoid adapter requests until OS-approved proof exists.";
pub const FALLBACK_NETWORK_DOMAIN: &str =
    "Return manual-required or unavailable until a host network control adapter has proof.";
pub const FALLBACK_MANAGED_BROWSER_COMMAND: &str =
    "Return manual-required or unavailable until managed browser command proof exists.";
pub const FALLBACK_MANAGED_EXACT_URL: &str =
    "Keep exact URL control manual-required unless managed browser proof is present.";
pub const FALLBACK_UNMANAGED_PROCESS: &str =
    "Restrict control to pid/name guardrails and preserve exact browser evidence as not-claimed.";
pub const FALLBACK_UNMANAGED_EXACT: &str = "Use managed browser or another explicit browser integration before representing exact evidence.";
pub const FALLBACK_RESTART_RECOVERY: &str =
    "Return unavailable when persisted timer state is missing or inconsistent.";
pub const FALLBACK_PARENT_CANCEL: &str =
    "Reject parent action when active timer state is missing or mismatched.";
pub const FALLBACK_AUDIT_CUSTODY: &str =
    "Keep broad rollback and bypass-resistance claims manual-required until artifact gate passes.";
pub const FALLBACK_ROLLBACK_ARTIFACT_GATE: &str =
    "Keep claims manual-required until real host hardening and rollback evidence exists.";
