import { DatabaseSync } from 'node:sqlite';

export function seedPortalNetworkActivityStore(activityDbPath) {
  const database = new DatabaseSync(activityDbPath);
  try {
    database.exec(`
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
`);

    database
      .prepare(
        `
INSERT OR REPLACE INTO activity_events (
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
) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);
`
      )
      .run(
        'network-ui-flow-1',
        networkActivityObservedAt(),
        'child-device-network-ui',
        'windows',
        'windows-network',
        'activity.domain.observed',
        'domain',
        'example-network.test',
        'example-network.test',
        JSON.stringify(networkActivityFields()),
        JSON.stringify(networkActivityEvidence())
      );
    database.exec('PRAGMA wal_checkpoint(TRUNCATE);');
    assertSeededEvidence(database);
  } finally {
    database.close();
  }
}

function assertSeededEvidence(database) {
  const row = database
    .prepare(
      `
SELECT evidence_json
FROM activity_events
WHERE event_id = ?;
`
    )
    .get('network-ui-flow-1');
  if (row === undefined || typeof row.evidence_json !== 'string') {
    throw new Error('Network drawer E2E ActivityStore seed did not persist.');
  }
  if (!row.evidence_json.includes('network-ui-evidence-1')) {
    throw new Error('Network drawer E2E ActivityStore seed missed the expected evidence ref.');
  }
}

function networkActivityObservedAt() {
  return new Date(Date.now() + 5 * 60 * 1000).toISOString();
}

function networkActivityFields() {
  return {
    capabilityStatus: 'available',
    adapterId: 'windows-network-snapshot',
    networkProtocol: 'tcp',
    tcpState: 'established',
    localIp: '127.0.0.1',
    localPort: 4242,
    destinationIp: '203.0.113.10',
    destinationPort: 443,
    destinationDomain: 'example-network.test',
    domainAttributionStatus: 'domain-observed',
    processAttributionStatus: 'process-attributed',
    pid: 4242,
    processName: 'notepad.exe',
  };
}

function networkActivityEvidence() {
  return [
    {
      evidenceId: 'network-ui-evidence-1',
      kind: 'local-db-row',
      digest: 'sha256:network-ui-evidence-1',
      uri: null,
    },
    {
      evidenceId: 'network-ui-journal-1',
      kind: 'journal-entry',
      digest: 'sha256:network-ui-journal-1',
      uri: null,
    },
  ];
}
