# WP30 - Rust Protocol Authority Classifier Parity

## Scope

Mirror the existing `packages/parent-domain` app/game control authority,
platform authority, and AI classifier boundary contracts into
`crates/agent-protocol` so Rust service/runtime code can depend on explicit
protocol shapes before policy, service, portal, or adapter consumers store or
render them.

Covered shapes:

- `AppGameControlApprovalAuthority`
- `AppGameControlApprovalRequest`
- `AppGameControlApprovalDecision`
- `AppGameControlActionResult`
- `AppGamePlatformAuthorityRow`
- `AppGamePlatformAuthorityMatrix`
- `AppGameAiClassifierResult`

## Implementation Boundary

This workpack is Rust protocol serialization proof only.

It does not add:

- service or journal storage for authority or classifier rows;
- live local AI provider execution or classifier quality proof;
- policy evaluator consumption;
- portal authority, classifier, approval, or platform rows;
- platform adapter execution;
- AppLocker, App Control, MDM, Endpoint Security, Device Owner/Profile Owner,
  FamilyControls/ManagedSettings, cgroup/systemd, kiosk, or single-app proof;
- product status movement.

## Required Proof

- Rust structs and constants in
  `crates/agent-protocol/src/app_game_authority_classifier.rs`.
- Serialization tests in
  `crates/agent-protocol/src/app_game_authority_classifier_tests.rs`.
- Proof output under
  `output/app-game-plan-proof/30-rust-protocol-authority-classifier-parity/`.
- Feature/snapshot/checklist docs that record product status did not move.

## AI Worker Checklist

- [x] Source docs read: app/game feature doc, app-game snapshot, app-plan
      snapshot, parent-domain contracts, agent-protocol README, protocol/test/
      rust rules.
- [x] Hub lock covered exact source, docs, workpack, and proof output paths.
- [x] Existing TypeScript contracts inspected before Rust shapes were added.
- [x] Rust protocol structs mirror existing field names with camelCase serde.
- [x] Rust serialization tests prove approval authority/action result,
      platform authority matrix, and classifier result field names and key
      literals.
- [x] Classifier protocol proof keeps `directActionRequested`,
      `rawScanIncluded`, and `contentClaimIncluded` false.
- [x] Platform authority proof keeps manual-required broad blocking rows from
      claiming adapter execution.
- [x] No service, runtime, journal, portal, policy, classifier-provider, or
      platform adapter claim was added.
- [x] Product checklist was not edited; no product status moved.
