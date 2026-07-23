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
CREATE INDEX IF NOT EXISTS activity_events_screen_queue_job_idx
  ON activity_events (
    kind,
    observer,
    json_extract(fields_json, '$.queueJobId'),
    observed_at DESC
  );
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

pub const SELECT_LATEST_ENFORCEMENT_AUDIT_ACTIVITY: &str = "
SELECT fields_json
FROM activity_events
WHERE kind = ?1
ORDER BY observed_at DESC, event_id DESC
LIMIT 1;";

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
WHERE kind <> ?1
ORDER BY observed_at DESC, event_id DESC
;";

pub const SELECT_APP_GAME_JOURNAL_ACTIVITY: &str = "
SELECT
  fields_json
FROM activity_events
ORDER BY observed_at DESC, event_id DESC
LIMIT ?1;";

pub const SELECT_NETWORK_RETENTION_DELETED_ACTIVITY: &str = "
SELECT
  fields_json,
  evidence_json
FROM activity_events
WHERE kind = ?1
ORDER BY observed_at DESC, event_id DESC;";

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

pub const SELECT_LATEST_SCREEN_ANALYSIS_ACTIVITY_FOR_QUEUE_JOB: &str = "
SELECT
  event_id,
  observed_at,
  fields_json,
  evidence_json
FROM activity_events
WHERE kind = ?1
  AND observer = ?2
  AND json_extract(fields_json, '$.queueJobId') = ?3
ORDER BY observed_at DESC,
  CASE json_extract(fields_json, '$.imageDeletionState')
    WHEN 'deleteFailed' THEN 4
    WHEN 'expiredDeleted' THEN 3
    WHEN 'deleted' THEN 2
    WHEN 'deletionRequired' THEN 1
    ELSE 0
  END DESC,
  rowid DESC
LIMIT 1;";

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
WHERE kind IN (?1, ?2, ?3, ?4, ?5, ?6, ?7)
ORDER BY observed_at DESC, event_id DESC
LIMIT ?8;";

pub const SELECT_RECENT_SCREEN_ANALYSIS_ACTIVITY: &str = "
SELECT
  event_id,
  observed_at,
  fields_json,
  evidence_json
FROM activity_events
WHERE kind = ?1
  AND observer = ?2
ORDER BY observed_at DESC,
  CASE json_extract(fields_json, '$.imageDeletionState')
    WHEN 'deleteFailed' THEN 4
    WHEN 'expiredDeleted' THEN 3
    WHEN 'deleted' THEN 2
    WHEN 'deletionRequired' THEN 1
    ELSE 0
  END DESC,
  rowid DESC
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

pub const INITIALIZE_ACTIVITY_MEMORY_GRAPH_INDEX: &str = "
CREATE TABLE IF NOT EXISTS activity_memory_graph_derivation_runs (
  run_id TEXT PRIMARY KEY,
  generated_at TEXT NOT NULL,
  index_version TEXT NOT NULL,
  source_event_count INTEGER NOT NULL,
  indexed_node_count INTEGER NOT NULL,
  indexed_edge_count INTEGER NOT NULL
);
CREATE TABLE IF NOT EXISTS activity_memory_graph_nodes (
  node_id TEXT PRIMARY KEY,
  graph_id TEXT NOT NULL,
  node_kind TEXT NOT NULL,
  label TEXT NOT NULL,
  node_json TEXT NOT NULL,
  trace_json TEXT NOT NULL,
  updated_at TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS activity_memory_graph_nodes_kind_idx
  ON activity_memory_graph_nodes (node_kind, updated_at DESC, node_id DESC);
CREATE TABLE IF NOT EXISTS activity_memory_graph_edges (
  edge_id TEXT PRIMARY KEY,
  graph_id TEXT NOT NULL,
  edge_kind TEXT NOT NULL,
  from_node_id TEXT NOT NULL,
  to_node_id TEXT NOT NULL,
  observed_from TEXT NOT NULL,
  observed_until TEXT,
  duration_ms INTEGER,
  edge_json TEXT NOT NULL,
  trace_json TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  FOREIGN KEY(from_node_id) REFERENCES activity_memory_graph_nodes(node_id),
  FOREIGN KEY(to_node_id) REFERENCES activity_memory_graph_nodes(node_id)
);
CREATE INDEX IF NOT EXISTS activity_memory_graph_edges_recent_idx
  ON activity_memory_graph_edges (observed_from DESC, edge_id DESC);
CREATE TABLE IF NOT EXISTS activity_memory_graph_citations (
  entry_id TEXT NOT NULL,
  entry_kind TEXT NOT NULL,
  evidence_reference_id TEXT NOT NULL,
  evidence_kind TEXT NOT NULL,
  observed_at TEXT NOT NULL,
  PRIMARY KEY(entry_id, entry_kind, evidence_reference_id)
);
CREATE INDEX IF NOT EXISTS activity_memory_graph_citations_evidence_idx
  ON activity_memory_graph_citations (evidence_reference_id, entry_kind, entry_id);";

pub const SELECT_MEMORY_GRAPH_ACTIVITY_FOR_INDEX: &str = "
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
WHERE kind IN (?1, ?2, ?3)
ORDER BY observed_at DESC, event_id DESC
LIMIT ?4;";

pub const UPSERT_ACTIVITY_MEMORY_GRAPH_DERIVATION_RUN: &str = "
INSERT INTO activity_memory_graph_derivation_runs (
  run_id,
  generated_at,
  index_version,
  source_event_count,
  indexed_node_count,
  indexed_edge_count
) VALUES (?1, ?2, ?3, ?4, ?5, ?6)
ON CONFLICT(run_id) DO UPDATE SET
  generated_at = excluded.generated_at,
  index_version = excluded.index_version,
  source_event_count = excluded.source_event_count,
  indexed_node_count = excluded.indexed_node_count,
  indexed_edge_count = excluded.indexed_edge_count;";

pub const UPSERT_ACTIVITY_MEMORY_GRAPH_NODE: &str = "
INSERT INTO activity_memory_graph_nodes (
  node_id,
  graph_id,
  node_kind,
  label,
  node_json,
  trace_json,
  updated_at
) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
ON CONFLICT(node_id) DO UPDATE SET
  graph_id = excluded.graph_id,
  node_kind = excluded.node_kind,
  label = excluded.label,
  node_json = excluded.node_json,
  trace_json = excluded.trace_json,
  updated_at = excluded.updated_at;";

pub const UPSERT_ACTIVITY_MEMORY_GRAPH_EDGE: &str = "
INSERT INTO activity_memory_graph_edges (
  edge_id,
  graph_id,
  edge_kind,
  from_node_id,
  to_node_id,
  observed_from,
  observed_until,
  duration_ms,
  edge_json,
  trace_json,
  updated_at
) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
ON CONFLICT(edge_id) DO UPDATE SET
  graph_id = excluded.graph_id,
  edge_kind = excluded.edge_kind,
  from_node_id = excluded.from_node_id,
  to_node_id = excluded.to_node_id,
  observed_from = excluded.observed_from,
  observed_until = excluded.observed_until,
  duration_ms = excluded.duration_ms,
  edge_json = excluded.edge_json,
  trace_json = excluded.trace_json,
  updated_at = excluded.updated_at;";

pub const DELETE_ACTIVITY_MEMORY_GRAPH_CITATIONS_FOR_ENTRY: &str = "
DELETE FROM activity_memory_graph_citations
WHERE entry_id = ?1
  AND entry_kind = ?2;";

pub const INSERT_ACTIVITY_MEMORY_GRAPH_CITATION: &str = "
INSERT INTO activity_memory_graph_citations (
  entry_id,
  entry_kind,
  evidence_reference_id,
  evidence_kind,
  observed_at
) VALUES (?1, ?2, ?3, ?4, ?5);";

pub const SELECT_INDEXED_ACTIVITY_MEMORY_GRAPH_EDGES: &str = "
SELECT edge_json
FROM activity_memory_graph_edges
ORDER BY observed_from DESC, edge_id DESC
LIMIT ?1;";

pub const SELECT_INDEXED_ACTIVITY_MEMORY_GRAPH_NODE: &str = "
SELECT node_json
FROM activity_memory_graph_nodes
WHERE node_id = ?1;";

pub const COUNT_INDEXED_ACTIVITY_MEMORY_GRAPH_EDGES: &str =
    "SELECT COUNT(*) FROM activity_memory_graph_edges;";

pub const COUNT_INDEXED_ACTIVITY_MEMORY_GRAPH_CITATIONS: &str =
    "SELECT COUNT(*) FROM activity_memory_graph_citations;";

pub const DELETE_ACTIVITY_EVENTS_FOR_MEMORY_GRAPH_TEST: &str = "DELETE FROM activity_events;";
