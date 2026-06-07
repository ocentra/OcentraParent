#[path = "sqlite_activity_memory_graph.rs"]
mod sqlite_activity_memory_graph;

pub use sqlite_activity_memory_graph::*;

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
  ON activity_events (observed_at DESC, event_id DESC);
CREATE TABLE IF NOT EXISTS parent_rule_contexts (
  parent_rule_ref_id TEXT PRIMARY KEY,
  updated_at TEXT NOT NULL,
  expires_at TEXT,
  context_json TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS parent_rule_contexts_recent_idx
  ON parent_rule_contexts (updated_at DESC, parent_rule_ref_id DESC);";

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

pub const SELECT_POLICY_PREVIEW_ACTIVITY: &str = "
SELECT
  event_id,
  observed_at,
  device_id,
  platform,
  kind,
  subject_kind,
  subject_id,
  subject_display_name,
  fields_json,
  evidence_json
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

pub const SELECT_RECENT_BROWSER_ACTIVITY: &str = "
SELECT
  event_id,
  observed_at,
  device_id,
  fields_json
FROM activity_events
WHERE kind = ?1
  AND observer = ?2
ORDER BY observed_at DESC, event_id DESC
LIMIT ?3;";

pub const SELECT_RECENT_BROWSER_INTERVENTION_ACTIVITY: &str = "
SELECT
  event_id,
  observed_at,
  device_id,
  fields_json
FROM activity_events
WHERE kind = ?1
ORDER BY observed_at DESC, event_id DESC
LIMIT ?2;";

pub const SELECT_RECENT_APP_GAME_ACTIVITY: &str = "
SELECT
  event_id,
  observed_at,
  kind,
  subject_id,
  subject_display_name,
  fields_json,
  evidence_json
FROM activity_events
WHERE kind IN (?1, ?2)
ORDER BY observed_at DESC, event_id DESC
LIMIT ?3;";

pub const SELECT_RECENT_NETWORK_FLOW_ACTIVITY: &str = "
SELECT
  event_id,
  observed_at,
  observer,
  kind,
  fields_json,
  evidence_json
FROM activity_events
WHERE (kind = ?1
  AND observer = ?2)
  OR kind = ?3
ORDER BY observed_at DESC, event_id DESC
LIMIT ?4;";

pub const SELECT_RECENT_TRACKING_ACTIVITY: &str = "
SELECT
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
FROM activity_events
WHERE kind IN (?1, ?2, ?3, ?4, ?5)
ORDER BY observed_at DESC, event_id DESC
LIMIT ?6;";

pub const SELECT_RECENT_SCREEN_ANALYSIS_ACTIVITY: &str = "
SELECT
  event_id,
  observed_at,
  fields_json,
  evidence_json
FROM activity_events
WHERE kind = ?1
  AND observer = ?2
ORDER BY observed_at DESC, event_id DESC
LIMIT ?3;";

pub const DELETE_PARENT_RULE_CONTEXTS: &str = "DELETE FROM parent_rule_contexts;";

pub const UPSERT_PARENT_RULE_CONTEXT: &str = "
INSERT INTO parent_rule_contexts (
  parent_rule_ref_id,
  updated_at,
  expires_at,
  context_json
) VALUES (?1, ?2, ?3, ?4)
ON CONFLICT(parent_rule_ref_id) DO UPDATE SET
  updated_at = excluded.updated_at,
  expires_at = excluded.expires_at,
  context_json = excluded.context_json;";

pub const SELECT_PARENT_RULE_CONTEXTS: &str = "
SELECT context_json
FROM parent_rule_contexts
ORDER BY updated_at DESC, parent_rule_ref_id DESC;";
