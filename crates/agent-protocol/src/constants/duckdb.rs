pub const CREATE_ACTIVITY_EVENTS_TABLE: &str = "
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
);";

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
) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);";

pub const COUNT_ACTIVITY_EVENT_ID: &str =
    "SELECT COUNT(*) FROM activity_events WHERE event_id = ?;";
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
LIMIT ?;";
