# WP06 Journal And Replay Proof

plan: eventing-plan
workpack: WP06 Journal Replay And Lineage
owner: ocentra-eventing
event_namespace: eventing.test
event_type: eventing.test.observed
schema_version: 1
event_id_state: validated
idempotency_state: duplicate-rejected
correlation_id_state: present
causation_id_state: not-tested
journal_replay_state: appended, replayed, hash-checked, version-skew-checked, corrupted-rejected
delivery_route_state: local-only
consumer_handoff_state: manual-required
transport_boundary: local-bus-only
redaction_state: redacted

## Fixture and transcript

The focused suite uses a temporary filesystem journal path through
`crates/ocentra-eventing/tests/journal_replay/`. It appends three stored
envelopes, filters by event type and correlation id after cursor `1`, and
returns only sequence `3` with the next cursor at `4`; two records are skipped.
The fixture is removed after each test.

## Focused results

- `cargo test -p ocentra-eventing --test journal_replay`: 22 passed. Covers
  NDJSON/hash-chain append and recovery, corruption/tamper rejection,
  cursor/filter ordering, projection-only safety, and journal policy.
- `cargo test -p ocentra-eventing --test version_skew`: 2 passed. Stored
  envelopes reject both newer and older schema versions without silent decode.

## Replay safety and no claim

Projection replay is proved not to execute handlers; no enforcement action was
replayed. This is reusable local journal/replay evidence, not production
retention, deletion, export, remote replication, enforcement authorization, or
adapter rollback proof.
