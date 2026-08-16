pub const DEFAULT_FAMILY_ID: &str = "family-local";
pub const DEFAULT_DEVICE_ID: &str = "local-dev-agent";
pub const DEFAULT_RANGE_START: &str = "1970-01-01T00:00:00Z";
pub const SCOPE_FAMILY: &str = "family";
pub const SCOPE_DEVICE: &str = "device";
pub const REPORT_ID_DAILY: &str = "activity-report-daily-local";
pub const REPORT_ID_WEEKLY: &str = "activity-report-weekly-local";
pub const REPORT_ID_MONTHLY: &str = "activity-report-monthly-local";
pub const REPORT_FILE_DAILY: &str = "activity-report-daily-local.json";
pub const REPORT_FILE_WEEKLY: &str = "activity-report-weekly-local.json";
pub const REPORT_FILE_MONTHLY: &str = "activity-report-monthly-local.json";
pub const REPORT_FILE_EXTENSION: &str = "json";
pub const REPORT_STORAGE_DIR: &str = "activity-reports";
pub const REPORT_ID_FALLBACK: &str = "activity-report";
pub const SUMMARY_READY: &str = "Activity data is available from the local query store.";
pub const SUMMARY_EMPTY: &str = "No local activity rows are available for this request.";
pub const SUMMARY_STORE_UNAVAILABLE: &str = "Local activity query store is unavailable.";
pub const SUMMARY_STORAGE_UNAVAILABLE: &str = "Local parent report storage is unavailable.";
pub const SUMMARY_STORAGE_SAVED: &str = "Activity report is saved in local parent report storage.";
pub const SUMMARY_STORAGE_DRAFT: &str = "Activity report is generated as an unsaved local draft.";
pub const SUMMARY_STORAGE_DEGRADED: &str =
    "Some saved activity report files could not be read or parsed.";
pub const SUMMARY_HISTORY_EMPTY: &str = "No saved activity reports are available for this request.";
pub const SUMMARY_DEVICE_OFFLINE: &str =
    "Requested device is not available in the local activity store.";
pub const SUMMARY_FAMILY_LOCAL_SOURCE: &str =
    "Family scope is backed by the reachable local child-device query store.";
pub const SUMMARY_FAMILY_FANOUT_UNAVAILABLE: &str =
    "Family fan-out to additional child devices is not wired for this local report.";
pub const SUMMARY_FAMILY_SOURCE_UNREACHABLE: &str =
    "Child-device source is registered but not reachable for this report request.";
pub const SUMMARY_FAMILY_SOURCE_ERROR: &str =
    "Child-device source returned an error before report material could be aggregated.";
pub const SUMMARY_FAMILY_SOURCE_STALE: &str =
    "Child-device source has stale report material and needs a fresh activity sync.";
pub const SUMMARY_SCAFFOLD_ONLY: &str =
    "This Activity read model is typed but not wired to local storage yet.";
pub const FAMILY_FANOUT_SOURCE_ID: &str = "family-fanout-unavailable";
pub const FAMILY_SOURCE_OFFLINE_ID: &str = "family-child-offline";
pub const FAMILY_SOURCE_ERROR_ID: &str = "family-child-error";
pub const FAMILY_SOURCE_STALE_ID: &str = "family-child-stale";
pub const SECTION_SUMMARY: &str = "Summary";
pub const SECTION_SCREEN: &str = "Screen";
pub const SECTION_APP_USE: &str = "App Use";
pub const SECTION_BROWSER: &str = "Browser";
pub const SECTION_GAMES: &str = "Games";
pub const SECTION_NETWORK: &str = "Network";
pub const READ_MODEL_SCREEN: &str = "screen";
pub const READ_MODEL_APP_USE: &str = "app-use";
pub const READ_MODEL_BROWSER: &str = "browser";
pub const READ_MODEL_GAMES: &str = "games";
pub const READ_MODEL_NETWORK: &str = "network";
pub const STATE_READY: &str = "ready";
pub const STATE_EMPTY: &str = "empty";
pub const STATE_UNAVAILABLE: &str = "unavailable";
pub const STATE_OFFLINE: &str = "offline";
pub const STATE_STALE: &str = "stale";
pub const STATE_PERMISSION_REQUIRED: &str = "permission-required";
pub const STATE_SCAFFOLD_ONLY: &str = "scaffold-only";
pub const SAVED_STATE_DRAFT: &str = "draft";
pub const SAVED_STATE_SAVED: &str = "saved";
pub const SAVED_STATE_STORAGE_UNAVAILABLE: &str = "storage-unavailable";
pub const SAVED_STATE_DEGRADED: &str = "degraded";
pub const SAVED_STATE_SCAFFOLD_ONLY: &str = "scaffold-only";
pub const CUSTODY_CHILD_DEVICE_LOCAL_SUMMARY: &str = "child-device-local-summary";
pub const CUSTODY_PARENT_DEVICE_LOCAL_REPORT_JSON: &str = "parent-device-local-report-json";
pub const CUSTODY_PARENT_DEVICE_LOCAL_HISTORY: &str = "parent-device-local-history";
pub const SOURCE_ACTIVITY_QUERY_STORE_SUMMARY: &str = "activity-query-store-summary";
pub const SOURCE_FAMILY_FANOUT_SOURCE_STATE: &str = "family-fanout-source-state";
pub const SOURCE_SAVED_REPORT_JSON: &str = "saved-report-json";
pub const SOURCE_SAVED_REPORT_HISTORY: &str = "saved-report-history";
pub const SOURCE_REACHABLE: &str = "reachable";
pub const SOURCE_UNREACHABLE: &str = "unreachable";
pub const SOURCE_ERROR: &str = "error";
pub const FREQUENCY_DAILY: &str = "daily";
pub const FREQUENCY_WEEKLY: &str = "weekly";
pub const FREQUENCY_MONTHLY: &str = "monthly";
