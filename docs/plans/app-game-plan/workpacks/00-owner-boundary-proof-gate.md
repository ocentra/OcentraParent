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
agent-protocol/agent-core:
  canonical contracts, Windows observation, sessionization, storage, evidence, and read models.
app-game-core:
  Rust source-readiness, policy-preview, timer-handoff, notification-intent, and runtime-decision models.
agent-service/parent-runtime-core/apps/portal:
  service composition, parent bridge, and rendered surfaces only when selected.
schema-domain:
  generated validation/decoder edge only.
platforms/android/agent:
  Android runtime source only; executable work requires focused tracked tests.
AI, policy, enforcement, notification, portal, child-runtime:
  sibling owners or handoff consumers only.
platform adapters:
  source observation and capability proof only.
```

## Import boundary

Allowed direct imports:

```text
Rust-owned app/game/evidence/policy-reference/protocol/capability/logging shapes
neutral event/evidence/logging/protocol primitives
app-game-core when Rust observation/event proof is selected
generated schema-domain decoders at TypeScript edges
pure common helpers without feature behavior
```

Forbidden direct imports:

```text
AI/policy/enforcement/notification/portal/child-runtime runtime behavior
private files from sibling plan owners
peer contracts that should live in the owning Rust or neutral boundary
removed app-game-domain/activity-domain/parent-domain/agent-protocol-domain/text-domain owners
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
