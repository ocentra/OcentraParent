<!-- agent-capsule -->

> Agent Capsule
> Doc: Evidence Storage Expectations
> Kind: expectation/acceptance documentation; read only when selected by feature doc, plan route, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# Evidence Storage Expectations

Evidence storage features must protect facts before analysis. V0.2 is the first
trusted evidence milestone, and V0.3 through V0.5 depend on it instead of
inventing feature-specific storage paths.

## Outcome Bar

Parent outcome:

- A parent can trust that child-device activity facts are recorded before any
  summary, portal view, AI preview, policy decision, sync, or report uses them.
- A parent can inspect high-level status, recent activity, and failure state
  through the agent service without opening private storage files directly.

Child-device outcome:

- The child-device agent writes schema-versioned events to an encrypted
  append-only journal, then derives queryable views from that journal.
- Restart, rotation, or query-store loss does not discard the evidence source
  of truth.

Platform scope:

- Windows is the first required runtime target.
- Contracts and journal/query semantics must stay platform-neutral so macOS,
  Linux, Android, and iOS agents can reuse the evidence model later.
- Portal code may request evidence status and read models, but it must not read
  journal files or SQLite files directly.

Data scope:

- Store typed activity facts, source metadata, timestamps, schema versions,
  ingest state, replay cursor state, and validation failures.
- Store enough source references for later policy or AI explanations to cite
  evidence without reinterpreting raw files.
- Do not store decrypted content payloads, screenshots, keystrokes, chat text,
  browser page bodies, or hidden surveillance data in the V0.2 through V0.5
  storage path.

Trust boundary:

- The encrypted journal is the evidence source of truth.
- SQLite is a local query/index store rebuilt from journal replay.
- Agent service APIs expose validated read models and status. They do not hand
  the portal raw secrets, encryption keys, or direct filesystem authority.

Contract boundary:

- Event envelopes, journal record metadata, ingest status, query status, recent
  activity summaries, and failure reasons belong in TypeScript domain packages
  before Rust or portal runtime code consumes them.
- Rust protocol structs mirror shared contracts when the Rust service sends or
  receives those shapes.
- Storage-specific implementation details should not leak into general activity
  contracts unless the feature is explicitly a storage status or replay feature.

## Expected Deliverables

- Encrypted append-only journal write path.
- Replayable event format with schema version, event id, timestamp, source id,
  and typed payload boundary.
- Rotation policy that preserves replay order and reports segment state.
- Tamper rejection for malformed, modified, truncated, or wrong-key records.
- SQLite ingest path for journaled events.
- Duplicate event protection across replay and live ingest.
- Query-store status, including last replayed event, segment cursor, row counts,
  and degraded/failure state.
- Rebuild path from journal to SQLite without requiring capture to run again.
- Health/status payloads exposed through the agent service.
- Recent activity summary API backed by real stored rows.

## Acceptance

- Plain activity payloads do not appear in journal files.
- Tampered journal lines fail to decrypt or parse and produce a typed failure
  state rather than silent partial success.
- Rotated segments replay in write order.
- Duplicate events are not double-counted in SQLite.
- SQLite queries return exact expected summaries for known journal fixtures and
  for live-written events.
- Query-store loss is recoverable by replaying the journal.
- The service can report journal health, ingest health, replay progress, and
  query-store degradation through typed service payloads.
- The portal can show storage and ingest status only through the real service
  path.

## V0.2 Through V0.5 Expectations

V0.2 Trusted Local Evidence Store:

- Proves encrypted journal write, rotation, replay, tamper rejection, SQLite
  ingest, rebuild, recent summary, and status APIs.
- Focused validation must include real journal and real SQLite paths.

V0.3 Windows process/window capture:

- Real Windows observations must be journaled before or as part of becoming
  queryable.
- Capture adapters must not bypass the journal by writing only to SQLite or
  portal state.

V0.4 network/domain observation:

- Network/domain facts, when implemented, follow the same journal first and
  rebuildable SQLite path.
- Raw packet dumps and decrypted HTTPS payloads are not part of this evidence
  store.

V0.5 live activity portal:

- Portal activity views use typed read models from the service.
- Copy/debug output may include status, event ids, timestamps, source ids, and
  high-level activity summaries, but not decrypted private payloads or secrets.

## Failure Behavior

- Encryption, parse, schema, replay, and SQLite errors must become typed status
  or failure events that the service can report.
- A failed replay must stop at the failed record or segment with enough context
  for debugging; it must not pretend the query store is complete.
- If SQLite is missing, locked, corrupted, or schema-incompatible, the agent
  should report degraded query capability and keep the journal as the source of
  truth.
- If storage is unavailable, capture must degrade safely and report failure
  instead of blocking the WebSocket service indefinitely.

## Non-Goals

- SQLite is not the evidence source of truth.
- Do not make policy or AI decisions directly from unreplayed raw files.
- Do not silently upload raw evidence before sync/privacy decisions exist.
- Do not add blocking, enforcement, stealth, anti-tamper, or local AI decision
  behavior as part of V0.2 storage.
- Do not store inspected page content, chat content, screenshots, or decrypted
  network payloads under a generic "evidence" claim.

## Validation Gates

- Unit and contract tests for valid/invalid event envelopes, journal metadata,
  ingest status, and query status.
- Rust tests for journal write/read, tamper rejection, rotation ordering, replay,
  duplicate protection, and SQLite rebuild.
- Integration smoke that writes real events through the service path and queries
  recent summaries from SQLite.
- Portal E2E only when portal behavior changes, and it must use the real Rust
  service path.
- `npm run validate` before merge unless the change is explicitly docs-only and
  the agreed branch gate is narrower.

## Done Signal

The service can write real typed events to the encrypted journal, replay them
into SQLite, report journal and ingest status, rebuild the query store, and
answer recent summary queries. Tests must use the real journal and real query
store path, not fake in-memory behavior unless the product path itself supports
in-memory mode.
