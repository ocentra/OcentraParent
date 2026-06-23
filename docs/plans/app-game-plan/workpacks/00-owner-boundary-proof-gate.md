<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP00 Owner Boundary Proof Gate`
> Kind: mandatory local workpack overlay.
> Read when: before editing any app-game workpack or source file.
> Stop rule: use this gate to constrain the selected workpack; do not broaden scope.
> Proves: ownership/proof discipline only.
> Does not prove: source readiness, policy readiness, adapter readiness, platform parity, or PR readiness.
> Proof rule: if a selected workpack conflicts with this gate, update the selected workpack or record a blocker before code changes.

<!-- /agent-capsule -->

# WP00 Owner Boundary Proof Gate

This file applies to every app-game workpack. It is the workpack-level overlay for the plan route, workpack families, proof metadata, and E2E tiers.

## Owner path

```text
schema-domain:
  canonical shared app/game contracts that cross package, crate, app, or plan boundaries.
app-game-domain:
  helper/projection/focused validation surface only.
app-game-core:
  child-local app/game observation, sessionization, evidence event, AI-request event, policy-request event, and source-readiness runtime boundary.
agent-protocol/agent-service:
  wire/service/read-model only when selected.
AI, policy, enforcement, notification, portal, child-runtime:
  sibling owners or handoff consumers only.
platform adapters:
  source observation and capability proof only.
```

## Import boundary

Allowed direct imports:

```text
schema-domain app/game/evidence/policy-reference/protocol/capability/logging shapes
neutral event/evidence/logging/protocol primitives
approved public app-game-domain helpers when selected
app-game-core when Rust observation/event proof is selected
pure common helpers without feature behavior
```

Forbidden direct imports:

```text
AI/policy/enforcement/notification/portal/child-runtime runtime behavior
private files from sibling plan owners
peer contracts that should live in schema-domain or another neutral boundary
portal/policy/AI/notification code that scans app/game source state
adapter execution without source readiness, authority, and adapter-readiness proof
```

## Proof discipline

Every selected workpack proof must include:

```text
owner module/path
workpack family from WORKPACK_FAMILIES.md
E2E tier from TEST_PROOF_EXPECTATIONS.md
focused command or explicit blocker
negative case for stale source, unsupported platform, manual-required, adapter-error, or authority boundary as applicable
artifact path
source/custody/platform note
no-claim boundary
```

## Stop conditions

Stop and write a blocker instead of coding when:

```text
selected workpack treats generated handoff rows as implementation scope without owner path
schema or helper proof is used to claim runtime source readiness
policy dry-run is used to claim enforcement
AI digest proof is used to claim AI runtime or OS scanning
portal/notification row is used to claim source readiness, policy readiness, or adapter execution
platform preflight is used to claim platform parity
```
