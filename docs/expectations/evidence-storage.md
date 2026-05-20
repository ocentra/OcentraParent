# Evidence Storage Expectations

Evidence storage features must protect facts before analysis.

## Expected Deliverables

- Encrypted append-only journal write path.
- Replayable event format.
- Rotation policy.
- Tamper rejection.
- Query-store ingest.
- Query-store status.
- Rebuild path from journal to query store.
- Health/status payload exposed through the agent service.

## Acceptance

- Plain activity payloads do not appear in the journal file.
- Tampered journal lines fail to decrypt or parse.
- Rotated segments replay in write order.
- Duplicate events are not double-counted in the query store.
- SQLite queries return exact expected summaries.
- Query-store loss is recoverable by replaying the journal.

## Non-Goals

- SQLite is not the evidence source of truth.
- Do not make policy or AI decisions directly from unreplayed raw files.
- Do not silently upload raw evidence before sync/privacy decisions exist.

## Done Signal

The service can write real typed events to the encrypted journal, replay them into SQLite, report ingest status, and answer recent summary queries. Tests must use the real journal and real query store path, not fake in-memory behavior unless the product path itself supports in-memory mode.
