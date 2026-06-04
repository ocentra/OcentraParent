# WP29 - Rust Protocol Evidence Identity Parity

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

- [x] Source docs read: app/game feature doc, app-game snapshot, app-plan
      snapshot, agent-protocol README, protocol/test/rust rules.
- [x] Hub lock covered exact source, docs, workpack, and proof output paths.
- [x] Existing TypeScript contracts inspected before Rust shapes were added.
- [x] Rust protocol structs mirror existing field names with camelCase serde.
- [x] Rust serialization tests prove evidence claim, digest, identity, and
      identity-merge field names and key literals.
- [x] No service, runtime, journal, portal, policy, or platform adapter claim
      was added.
- [x] Product checklist was not edited; no product status moved.
