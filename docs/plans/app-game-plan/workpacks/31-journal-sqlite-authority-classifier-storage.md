# WP31 - Journal/SQLite Authority Classifier Storage

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP31 - Journal/SQLite Authority Classifier Storage`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Scope

Extend the existing `agent-core` app/game journal and SQLite replay path so the
new Rust protocol rows from WP29 and WP30 can be stored and projected before
portal, policy, classifier-provider, or adapter consumers depend on them.

Covered rows:

- `AppGameEvidenceClaim`
- `AppGameIdentity`
- `AppGameControlApprovalAuthority`
- `AppGameControlActionResult`
- `AppGamePlatformAuthorityMatrix`
- `AppGameAiClassifierResult`

## Implementation Boundary

This workpack is staged storage/read-model proof only.

It does not add:

- live Windows inventory, runtime, foreground, launcher, or classifier source
  adapters;
- live classifier/provider execution or model-quality proof;
- new service event endpoints for the newly stored rows;
- portal identity, authority, platform, or classifier dashboard rows;
- policy evaluator consumption;
- platform adapter execution;
- AppLocker, App Control, MDM, Endpoint Security, Device Owner/Profile Owner,
  FamilyControls/ManagedSettings, cgroup/systemd, kiosk, or single-app proof;
- product status movement.

## Required Proof

- Journal event builders in
  `crates/agent-core/src/activity_store_app_game/app_game_journal_sqlite_ingest/protocol_rows.rs`.
- Read-model projection in
  `crates/agent-core/src/activity_store_app_game/app_game_journal_sqlite_ingest/read_model.rs`.
- Integration tests in
  `crates/agent-core/src/activity_store_app_game/app_game_journal_sqlite_protocol_rows_tests.rs`.
- Proof output under
  `output/app-game-plan-proof/31-journal-sqlite-authority-classifier-storage/`.
- Feature/snapshot/checklist docs that record product status did not move.

## AI Worker Checklist

- [ ] Source docs read: app/game feature doc, app-game snapshot, app-plan
      snapshot, agent-core README, protocol/test/rust/source-shape/validation
      rules.
- [ ] Hub lock covered exact source, docs, workpack, and proof output paths.
- [ ] Existing app-game journal/SQLite replay path was extended instead of
      creating a second storage path.
- [ ] Journal event builders serialize row JSON with protocol row-kind fields.
- [ ] SQLite read model projects evidence claim, identity, approval authority,
      approval action-result, platform authority matrix, and classifier result
      rows with returned counts.
- [ ] Positive test appends rows through the real encrypted journal and replays
      them through the real SQLite `ActivityStore`.
- [ ] Negative tests reject inventory-use upgrades, inactive authority grants,
      manual-required action execution, manual platform adapter execution, and
      classifier direct action/raw-content claims before SQLite ingest.
- [ ] No live source adapter, classifier provider, service event, portal,
      policy evaluator, or platform adapter claim was added.
- [ ] Product checklist was not edited; no product status moved.
