<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `WP00 Owner Boundary Proof Gate`
> Kind: mandatory local workpack overlay.
> Read when: before editing any AI workpack or source file.
> Stop rule: use this gate to constrain the selected workpack; do not broaden scope.
> Proves: ownership/proof discipline only.
> Does not prove: model readiness, provider readiness, policy readiness, or PR readiness.
> Proof rule: if a selected workpack conflicts with this gate, update the selected workpack or record a blocker before code changes.

<!-- /agent-capsule -->

# WP00 Owner Boundary Proof Gate

This file applies to every AI workpack. It prevents AI code and docs from becoming a hidden coupling layer between browser, screen, app/game, tracking, network, policy, enforcement, LAN, remote, portal, and account authority.

## Owner path

```text
schema-domain:
  canonical AI context/runtime/reference/model/provider/memory/graph/prompt/result shapes shared across packages/crates/apps/plans.
ai-domain:
  helper/projection/focused validation only; not the canonical cross-plan contract owner.
child-ai-core:
  child-local AI runtime/evaluator boundary when selected.
screen-ai-core:
  screen AI worker/router boundary when selected.
agent-protocol/agent-service:
  wire/service routes only when selected.
portal-domain/apps/portal:
  parent-visible projection only.
```

## Import boundary

Allowed direct imports:

```text
schema-domain AI/evidence/policy-reference/family-reference/protocol/capability/logging shapes
neutral event/evidence/logging/protocol primitives
approved public ai-domain helper exports when selected
selected Rust AI runtime/parity crates when selected
pure common helpers without feature behavior
```

Forbidden direct imports:

```text
browser/screen/tracking/network/app-game runtime internals
policy/enforcement internals that turn model output into action
portal internals as source truth
LAN/remote runtime behavior except typed provider-job handoff when selected
private source files from sibling plan owners
peer feature contracts that should live in schema-domain or neutral shared boundary
```

## Proof discipline

Every selected workpack proof must include:

```text
owner module/path
AI E2E tier from TEST_PROOF_EXPECTATIONS.md
focused command or explicit blocker
negative case: invalid output, missing evidence, timeout/degraded, privacy/custody, or authority-boundary case as applicable
artifact path
redaction/custody note
no-claim boundary
```

## Default-sensitive stance

```text
local-only child safety path by default
remote/API assistant outside normal blocking path
no default remote transfer of child evidence
mock/dry-run provider cannot prove product readiness
AI classification is evidence, not deterministic policy/enforcement authority
```

## Stop conditions

Stop and write a blocker instead of coding when:

```text
AI would import a source owner runtime directly
AI output would replace deterministic policy authority
remote assistant would use child evidence without parent authorization/custody proof
memory/graph claim lacks source evidence refs
provider/runtime proof lacks invalid-output, timeout, or degraded-state negatives
```
