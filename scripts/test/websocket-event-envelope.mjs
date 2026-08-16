export function parseAgentEventEnvelope(value) {
  if (!isRecord(value)) {
    throw new Error('Expected the Rust WebSocket event envelope to be an object');
  }

  assertNumber(value.schemaVersion, 'schemaVersion');
  assertNonEmptyString(value.eventId, 'eventId');
  assertNonEmptyString(value.correlationId, 'correlationId');
  assertNonEmptyString(value.sentAt, 'sentAt');
  assertAgentPeer(value.source, 'source');
  assertAgentPeer(value.target, 'target');
  assertNonEmptyString(value.event, 'event');
  assertNonEmptyString(value.severity, 'severity');
  if (!isRecord(value.payload)) {
    throw new Error('Expected the Rust WebSocket event envelope payload to be an object');
  }
  if (value.snapshot !== null && !isRecord(value.snapshot)) {
    throw new Error('Expected the Rust WebSocket event envelope snapshot to be an object or null');
  }

  return value;
}

function assertAgentPeer(value, field) {
  if (!isRecord(value)) {
    throw new Error(`Expected ${field} to be an agent peer object`);
  }
  assertNonEmptyString(value.peerId, `${field}.peerId`);
  assertNonEmptyString(value.role, `${field}.role`);
}

function assertNumber(value, field) {
  if (typeof value !== 'number') {
    throw new Error(`Expected ${field} to be a number`);
  }
}

function assertNonEmptyString(value, field) {
  if (typeof value !== 'string' || value.length === 0) {
    throw new Error(`Expected ${field} to be a non-empty string`);
  }
}

function isRecord(value) {
  return typeof value === 'object' && value !== null && !Array.isArray(value);
}
