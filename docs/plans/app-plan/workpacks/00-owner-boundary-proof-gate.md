<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-plan`
> Doc: `WP00 Owner Boundary Proof Gate`
> Kind: mandatory local workpack overlay.
> Read when: before editing any app-plan workpack or source file.
> Stop rule: use this gate to constrain the selected workpack; do not broaden scope.
> Proves: ownership/proof discipline only.
> Does not prove: source readiness, product readiness, platform parity, or PR readiness.
> Proof rule: if a selected workpack conflicts with this gate, update the selected workpack or record a blocker before code changes.

<!-- /agent-capsule -->

# WP00 Owner Boundary Proof Gate

This file applies to every native-app workpack. It keeps this plan as an app-only narrowing/reconciliation plan and prevents it from accidentally taking over the shared app/game evidence spine.

## Owner path

```text
schema-domain:
  canonical shared native-app/app-game contracts that cross package, crate, app, or plan boundaries.
app-core:
  child-local native-app observation, evidence event, AI-request event, policy-request event, and source-readiness boundary.
app-plan:
  app-only route, app-only meaning, app-specific reconciliation, and proof expectations.
app-game-plan:
  shared native app/game evidence spine and combined runtime/read-model/proof chains.
agent-protocol/agent-service:
  wire/service/read-model only when selected.
portal/policy/enforcement/notification/child-runtime/setup/install:
  sibling owners or handoff consumers only.
```

## Import boundary

Allowed direct imports:

```text
schema-domain app/app-game/evidence/policy-reference/protocol/capability/logging shapes
neutral event/evidence/logging/protocol primitives
approved public helper exports named by selected workpack
app-core when Rust observation/event proof is selected
pure common helpers without feature behavior
```

Forbidden direct imports:

```text
app-game/policy/enforcement/notification/portal/child-runtime runtime behavior
private files from sibling plan owners
peer contracts that should live in schema-domain or another neutral boundary
portal/policy/AI/enforcement/notification code that scans native-app source state
adapter execution without native-app source readiness, authority, and adapter-readiness proof
```

## Proof discipline

Every selected workpack proof must include:

```text
owner module/path
workpack family from WORKPACK_FAMILIES.md when owner path is unclear
E2E tier from TEST_PROOF_EXPECTATIONS.md
focused command or explicit blocker
negative case for stale source, unsupported platform, permission-limited, manual-required, or handoff boundary as applicable
artifact path
source/custody/platform note
no-claim boundary
```

## Stop conditions

Stop and write a blocker instead of coding when:

```text
selected workpack would invent packages/app-domain as an owner without implementation proof
app-game-plan proof is used to close app-plan without named app-only handoff
route normalization or package preview is used to claim runtime support
portal/policy/notification row is used to claim native-app source readiness
policy dry-run is used to claim enforcement
platform preflight is used to claim platform parity
```
