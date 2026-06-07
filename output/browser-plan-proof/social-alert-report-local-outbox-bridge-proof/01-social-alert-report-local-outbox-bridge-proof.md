# Social Alert Report Local Outbox Bridge Proof

Generated: 2026-06-07T07:04:00Z

Linked local outbox rows: 2
Manual-required rows: 1
Unavailable rows: 1
JSONL records reread: 2
Provider delivery runtime claimed: false
Final policy execution claimed: false
Enforcement claimed: false

This proof consumes parsed social alert/report intents and links only
local-outbox-eligible rows to the existing parent-owned
`NotificationLocalOutboxRecord` schema. The generated JSONL is reread
through the same parser. Manual-required and unavailable rows remain in
the bridge read model but do not produce queued local outbox records.

It does not claim provider delivery execution, receipt ingestion, provider
credentials, scheduler runtime, cloud routing, parent notification UI
delivery, report delivery execution, final policy execution, connector or
native runtime, or enforcement.
