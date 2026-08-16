# WP29 - Rust Protocol Evidence Identity Parity

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP29 - Rust Protocol Evidence Identity Parity`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Scope

Mirror the existing `packages/activity-domain` app/game evidence and identity
contracts into `crates/agent-protocol` so Rust service/runtime code can depend
on explicit protocol shapes before journal, SQLite, service, or portal work
stores or renders them.

Covered shapes:

- `AppGameEvidenceClaim`
- `AppGameAiDigestReference`
- `AppGameAiClassificationDigest`
- `AppGameIdentity`
- `AppGameIdentityMergeProof`

## Implementation Boundary

This workpack is Rust protocol serialization proof only.

It does not add:

- live Windows inventory, process, foreground, or launcher source readers;
- journal or SQLite storage for evidence claim, digest, identity, or merge rows;
- app/game control authority-schema parity;
- parent-domain classifier-boundary parity;
- policy runtime evaluation;
- portal identity rows or approval UI;
- platform adapter execution or broad app/game blocking.

## Required Proof

- Rust structs and constants in `crates/agent-protocol/src/app_game.rs`.
- Serialization tests in `crates/agent-protocol/src/app_game_tests.rs`.
- Proof output under
  `output/app-game-plan-proof/29-rust-protocol-evidence-identity-parity/`.
- Feature/snapshot/checklist docs that record product status did not move.

## AI Worker Checklist

- [ ] Source docs read: app/game feature doc, app-game snapshot, app-plan
      snapshot, agent-protocol README, protocol/test/rust rules.
- [ ] Hub lock covered exact source, docs, workpack, and proof output paths.
- [ ] Existing TypeScript contracts inspected before Rust shapes were added.
- [ ] Rust protocol structs mirror existing field names with camelCase serde.
- [ ] Rust serialization tests prove evidence claim, digest, identity, and
      identity-merge field names and key literals.
- [ ] No service, runtime, journal, portal, policy, or platform adapter claim
      was added.
- [ ] Product checklist was not edited; no product status moved.
