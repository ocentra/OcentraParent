<!-- agent-capsule -->

> Agent Capsule
> Plan: `browser-plan`
> Doc: `WP00 Owner Boundary Proof Gate`
> Kind: mandatory local workpack overlay.
> Read when: before editing any browser workpack or source file.
> Stop rule: use this gate to constrain the selected workpack; do not broaden scope.
> Proves: ownership/proof discipline only.
> Does not prove: exact URL capture, active-tab readiness, intervention readiness, platform parity, or PR readiness.
> Proof rule: if a selected workpack conflicts with this gate, update the selected workpack or record a blocker before code changes.

<!-- /agent-capsule -->

# WP00 Owner Boundary Proof Gate

This file applies to every browser workpack. It prevents browser evidence work from collapsing into AI, policy, enforcement, network, screen, app/game, or portal runtime ownership.

## Owner path

```text
schema-domain:
  canonical shared browser/evidence/read-model/intervention contracts that cross package, crate, app, or plan boundaries.
browser-domain:
  helper/projection/focused validation surface only.
browser-core:
  child-local browser observation, evidence event, AI-request event, policy-request event, and source-readiness boundary.
agent-protocol/agent-service:
  wire/service/read-model only when selected.
AI plan:
  consumes stored browser evidence or digest refs only.
policy/enforcement plans:
  deterministic decision/action owners only.
portal-domain/apps/portal:
  parent-visible projection only.
```

## Import boundary

Allowed direct imports:

```text
schema-domain browser/evidence/policy-reference/protocol/capability/logging shapes
neutral event/evidence/logging/protocol primitives
approved public browser-domain helpers when selected
browser-core when Rust observation/event proof is selected
pure common helpers without feature behavior
```

Forbidden direct imports:

```text
AI/policy/enforcement/network/screen/app-game/tracking/portal/notification runtime behavior
private files from sibling plan owners
peer contracts that should live in schema-domain or another neutral boundary
AI/policy/enforcement/portal code that captures or infers browser source state
process/window/network evidence upgraded into exact URL, active-tab, title, or browser-game proof without browser source proof
```

## Proof discipline

Every selected workpack proof must include:

```text
owner module/path
workpack family from WORKPACK_FAMILIES.md when owner path is unclear
E2E tier from TEST_PROOF_EXPECTATIONS.md
focused command or explicit blocker
negative case for unsupported browser, unmanaged browser, stale evidence, privacy/custody, target-list-vs-active-tab, or authority boundary as applicable
artifact path
source/custody/browser-boundary/platform note
no-claim boundary
```

## Stop conditions

Stop and write a blocker instead of coding when:

```text
CDP target list is used to claim active tab
unmanaged browser detection is used to claim exact URL
target URL proof is used to claim policy or enforcement action
managed intervention harness is used to claim product blocking without policy/action/audit/delivery proof
portal UI is used to claim browser source capture
reference/settings inventory is used to claim runtime support
```
