pub const READ_MODEL_ID_V0_8: &str = "v0-8-windows-adapter-artifact-ingestion-proof";

pub const RECORD_ID_APP_IDENTITY: &str = "windows-adapter-artifact-ingestion-app-identity";
pub const RECORD_ID_APP_APPLY: &str = "windows-adapter-artifact-ingestion-app-apply";
pub const RECORD_ID_APP_ROLLBACK: &str = "windows-adapter-artifact-ingestion-app-rollback";
pub const RECORD_ID_APP_AUDIT: &str = "windows-adapter-artifact-ingestion-app-audit";
pub const RECORD_ID_DOMAIN_APPLY: &str = "windows-adapter-artifact-ingestion-domain-apply";
pub const RECORD_ID_DOMAIN_ROLLBACK: &str = "windows-adapter-artifact-ingestion-domain-rollback";
pub const RECORD_ID_DOMAIN_AUDIT: &str = "windows-adapter-artifact-ingestion-domain-audit";
pub const RECORD_ID_MANAGED_BROWSER_URL: &str =
    "windows-adapter-artifact-ingestion-managed-browser-url";
pub const RECORD_ID_MANAGED_BROWSER_AUDIT: &str =
    "windows-adapter-artifact-ingestion-managed-browser-audit";

pub const REFUSAL_EMPTY_ARTIFACT_ID: &str = "empty artifact id";
pub const REFUSAL_EMPTY_TARGET_SUBJECT: &str = "empty target subject";
pub const REFUSAL_EMPTY_ARTIFACT_SUBJECT: &str = "empty artifact subject";
pub const REFUSAL_SUBJECT_MISMATCH: &str = "artifact subject does not match target subject";
pub const REFUSAL_MISSING_CUSTODY_EVENT: &str = "missing audit custody event id";
pub const REFUSAL_UNSUPPORTED_SURFACE: &str =
    "unsupported surface cannot ingest Windows adapter artifacts";
pub const REFUSAL_KIND_SURFACE_MISMATCH: &str =
    "artifact kind is not valid for the requested surface";

pub const CLAIM_BOUNDARY: &str = "Artifact ingestion constructs custodied evidence for the gate but never upgrades product claims by itself.";

pub const TEST_INGESTED_AT: &str = "2026-05-29T23:59:00Z";
pub const TEST_MISMATCHED_SUBJECT_REF: &str = "test-mismatched-windows-target-subject";
