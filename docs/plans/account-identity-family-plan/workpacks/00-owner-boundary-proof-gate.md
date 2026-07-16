<!-- agent-capsule -->

> Agent Capsule
> Plan: `account-identity-family-plan`
> Doc: `WP00 Owner Boundary Proof Gate`
> Kind: mandatory local workpack overlay.
> Read when: before editing any account-identity workpack or source file.
> Stop rule: use this gate to constrain the selected workpack; do not broaden scope.
> Proves: ownership/proof discipline only.
> Does not prove: implementation completion, security readiness, or PR readiness.
> Proof rule: if a selected workpack conflicts with this gate, update the selected workpack or record a blocker before code changes.

<!-- /agent-capsule -->

# WP00 Owner Boundary Proof Gate

This file applies to every account-identity-family workpack. It exists because older workpacks may contain historical branch names, pre-central-schema package assumptions, or proof language that predates the current route contract.

## Owner path

```text
crates/schema or the owning Rust crate:
  canonical shared account/family/session/device-authority shapes that cross package, crate, app, or plan boundaries.
schema-domain:
  temporary generated-validation or edge-decoder surface only where TypeScript still needs one during migration.
family-domain:
  TypeScript helper/projection consumer over Rust-owned/generated contracts.
family-identity-core:
  Rust parity/runtime authority semantics when the selected workpack names Rust proof.
setup-domain/provisioning-core:
  setup/provisioning consumers only.
portal-domain/apps/portal:
  parent-visible projection/rendering consumers only.
Cloudflare runtime/schema:
  runtime/persistence implementation target only when provider/schema/runtime proof is selected.
```

## Import boundary

Allowed direct imports:

```text
Rust-owned canonical shapes, generated DTOs, temporary edge decoders, brands, parsers, and literals
neutral protocol/event/evidence/logging/capability primitives
approved public family-domain helper exports
approved Rust parity crates when selected
pure common helpers without feature behavior
```

Forbidden direct imports:

```text
sibling feature runtime behavior
private files from setup/payment/policy/data-custody/device-trust/LAN/remote/portal plans
peer feature contracts that should live in crates/schema or another neutral Rust-owned boundary
account/family authority moved into IdP, setup, payment, policy, remote, LAN, device-trust, or data-custody code
```

## Proof discipline

Every selected workpack proof must include:

```text
owner module/path
focused command or explicit blocker
negative case
artifact path
redaction/custody note when auth/session/recovery/device data is involved
no-claim boundary
updated checklist/state rows only after proof exists
```

Use `TEST_PROOF_EXPECTATIONS.md` for exact commands and E2E tier meaning. Use `PROOF_INDEX.md` for structured metadata.

## Current-state override

Historical `Fill before DONE` entries are evidence records, not current branch truth. Current branch is `codex/plan-harness-update`. Historical command logs remain valid only as the artifacts they name; do not use old branch labels to claim current validation.

## Stop conditions

Stop and write a blocker instead of coding when:

```text
selected workpack would move canonical shared shape out of crates/schema or the owning Rust crate
runtime auth/session implementation starts before provider/custody boundary is accepted
UI claims trusted household/device readiness from login alone
session/auth proof omits replay/revocation/freshness negatives
child activity evidence is routed into account/identity state
```
