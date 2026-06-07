# Social Alert Report Scheduler Bridge Proof

Generated: 2026-06-07T08:12:00Z

Source bridge rows: 4
Scheduled local rows: 2
Manual-required unscheduled rows: 1
Unavailable unscheduled rows: 1
Scheduler JSONL records reread: 2
Quiet-hours timer runtime claimed: false
Retry execution runtime claimed: false
Provider delivery runtime claimed: false
Final policy execution claimed: false
Enforcement claimed: false

This proof consumes the parent-owned social alert/report local outbox bridge
and writes only linked local outbox rows into the existing notification
local outbox scheduler record schema. The generated scheduler JSONL is
reread through that scheduler parser. Manual-required and unavailable rows
remain visible in the scheduler bridge read model but do not produce
scheduler JSONL records.

It proves a deterministic handoff into local scheduler records only. It
does not claim provider dispatch, provider receipt ingestion, quiet-hours
timer execution, retry worker execution, parent or child notification UI
delivery, report delivery execution, final policy execution, connector or
native runtime, or enforcement.
