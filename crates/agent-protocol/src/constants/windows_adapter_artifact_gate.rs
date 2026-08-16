pub const READ_MODEL_ID_V0_8: &str = "v0-8-windows-adapter-artifact-gate";

pub const ENTRY_ID_APP_TARGET: &str = "windows-adapter-artifact-gate-app-target";
pub const ENTRY_ID_DOMAIN_NETWORK_TARGET: &str =
    "windows-adapter-artifact-gate-domain-network-target";
pub const ENTRY_ID_MANAGED_BROWSER_TARGET: &str =
    "windows-adapter-artifact-gate-managed-browser-target";
pub const ENTRY_ID_UNMANAGED_BROWSER_TARGET: &str =
    "windows-adapter-artifact-gate-unmanaged-browser-target";
pub const ENTRY_ID_UNSUPPORTED_OS_TARGET: &str =
    "windows-adapter-artifact-gate-unsupported-os-target";
pub const ENTRY_ID_ROLLBACK_AUDIT_TARGET: &str =
    "windows-adapter-artifact-gate-rollback-audit-target";

pub const ARTIFACT_KIND_SAME_IDENTITY_APP: &str = "same-identity-app-package-evidence";
pub const ARTIFACT_KIND_APPLY_RESULT: &str = "adapter-apply-result";
pub const ARTIFACT_KIND_ROLLBACK_RESULT: &str = "adapter-rollback-result";
pub const ARTIFACT_KIND_AUDIT_CUSTODY_EVENT: &str = "audit-custody-event";
pub const ARTIFACT_KIND_MANAGED_BROWSER_EXACT_URL: &str = "managed-browser-exact-url-evidence";
pub const ARTIFACT_KIND_NETWORK_FILTER_APPLY: &str = "network-domain-filter-apply";
pub const ARTIFACT_KIND_NETWORK_FILTER_ROLLBACK: &str = "network-domain-filter-rollback";

pub const DECISION_REFUSED_MISSING_ARTIFACTS: &str = "refused-missing-artifacts";
pub const DECISION_REFUSED_UNSUPPORTED_SURFACE: &str = "refused-unsupported-surface";
pub const DECISION_READY_FOR_MANUAL_REVIEW: &str = "ready-for-manual-review";

pub const CLAIM_BOUNDARY_APP_TARGET: &str = "App target claims cannot upgrade until same-identity app evidence, apply, rollback, and audit custody artifacts are present.";
pub const CLAIM_BOUNDARY_DOMAIN_NETWORK_TARGET: &str = "Domain and network claims cannot upgrade until filter apply, filter rollback, and audit custody artifacts are present.";
pub const CLAIM_BOUNDARY_MANAGED_BROWSER_TARGET: &str = "Managed browser exact URL claims cannot upgrade until exact URL evidence and audit custody artifacts are present.";
pub const CLAIM_BOUNDARY_UNMANAGED_BROWSER_TARGET: &str = "Unmanaged browser capability remains process-only and cannot upgrade into exact URL control from capability rows.";
pub const CLAIM_BOUNDARY_UNSUPPORTED_OS_TARGET: &str =
    "Unsupported OS targets cannot borrow Windows adapter artifacts or upgrade claims.";
pub const CLAIM_BOUNDARY_ROLLBACK_AUDIT_TARGET: &str = "Rollback and audit claims require same-identity apply, rollback, and custody artifacts before manual review.";

pub const REFUSAL_MISSING_APP_ARTIFACTS: &str =
    "missing same-identity app evidence, apply result, rollback result, or audit custody event";
pub const REFUSAL_MISSING_DOMAIN_ARTIFACTS: &str =
    "missing network/domain filter apply result, rollback result, or audit custody event";
pub const REFUSAL_MISSING_MANAGED_BROWSER_ARTIFACTS: &str =
    "missing managed-browser exact URL evidence or audit custody event";
pub const REFUSAL_UNMANAGED_BROWSER_PROCESS_ONLY: &str =
    "unmanaged browser support is process-only and cannot upgrade exact URL claims";
pub const REFUSAL_UNSUPPORTED_OS: &str =
    "unsupported OS target has no Windows adapter artifact path";
pub const REFUSAL_MISSING_ROLLBACK_AUDIT_ARTIFACTS: &str =
    "missing same-identity apply, rollback, or audit custody artifacts";

pub const TEST_ARTIFACT_APP_IDENTITY: &str = "test-windows-app-identity-artifact";
pub const TEST_ARTIFACT_APP_APPLY: &str = "test-windows-app-apply-artifact";
pub const TEST_ARTIFACT_APP_ROLLBACK: &str = "test-windows-app-rollback-artifact";
pub const TEST_ARTIFACT_AUDIT: &str = "test-windows-audit-custody-artifact";
pub const TEST_ARTIFACT_MANAGED_EXACT_URL: &str = "test-windows-managed-browser-exact-url-artifact";
pub const TEST_ARTIFACT_MANAGED_AUDIT: &str = "test-windows-managed-browser-audit-artifact";
pub const TEST_ARTIFACT_DOMAIN_APPLY: &str = "test-windows-domain-apply-artifact";
pub const TEST_ARTIFACT_DOMAIN_ROLLBACK: &str = "test-windows-domain-rollback-artifact";
pub const TEST_ARTIFACT_DOMAIN_AUDIT: &str = "test-windows-domain-audit-artifact";
pub const TEST_SUBJECT_REF: &str = "test-windows-target-subject";
pub const TEST_CUSTODY_EVENT_ID: &str = "test-windows-audit-event";
