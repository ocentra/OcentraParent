pub const INITIALIZE_ACTIVITY_STORE: &str = "
PRAGMA foreign_keys = ON;
PRAGMA journal_mode = WAL;
CREATE TABLE IF NOT EXISTS activity_events (
  event_id TEXT PRIMARY KEY,
  observed_at TEXT NOT NULL,
  device_id TEXT NOT NULL,
  platform TEXT NOT NULL,
  observer TEXT NOT NULL,
  kind TEXT NOT NULL,
  subject_kind TEXT NOT NULL,
  subject_id TEXT NOT NULL,
  subject_display_name TEXT,
  fields_json TEXT NOT NULL,
  evidence_json TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS activity_events_recent_idx
  ON activity_events (observed_at DESC, event_id DESC);";

pub const INSERT_ACTIVITY_EVENT: &str = "
INSERT INTO activity_events (
  event_id,
  observed_at,
  device_id,
  platform,
  observer,
  kind,
  subject_kind,
  subject_id,
  subject_display_name,
  fields_json,
  evidence_json
) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11);";

pub const COUNT_ACTIVITY_EVENT_ID: &str =
    "SELECT COUNT(*) FROM activity_events WHERE event_id = ?1;";
pub const COUNT_ACTIVITY_EVENTS: &str = "SELECT COUNT(*) FROM activity_events;";
pub const LAST_ACTIVITY_EVENT_ID: &str =
    "SELECT event_id FROM activity_events ORDER BY observed_at DESC, event_id DESC LIMIT 1;";

pub const SELECT_RECENT_ACTIVITY: &str = "
SELECT
  event_id,
  observed_at,
  device_id,
  platform,
  observer,
  kind,
  subject_kind,
  subject_id,
  subject_display_name
FROM activity_events
ORDER BY observed_at DESC, event_id DESC
LIMIT ?1;";

pub const SELECT_LATEST_BROWSER_ACTIVITY: &str = "
SELECT
  event_id,
  observed_at,
  fields_json
FROM activity_events
WHERE kind = ?1
  AND observer = ?2
ORDER BY observed_at DESC, event_id DESC
LIMIT 1;";
