<!-- agent-capsule -->

> Agent Capsule
> Plan: `account-identity-family-plan`
> Doc: `WP03 Current Boundary Addendum`
> Kind: mandatory addendum for WP03 lifecycle work.
> Read when: before editing WP03 or related lifecycle source files.
> Stop rule: use this addendum to constrain WP03; do not broaden scope into provider, UI, payment, policy, remote, LAN, data custody, or device-trust runtime.
> Proves: current owner/import/proof discipline for WP03 only.
> Does not prove: runtime login, browser request enforcement, provider implementation, trusted-device bootstrap, or PR readiness.
> Proof rule: if WP03 conflicts with this addendum, treat this addendum plus `00-owner-boundary-proof-gate.md` as the current route and record the conflict before source changes.

<!-- /agent-capsule -->

# WP03 Current Boundary Addendum

This addendum exists because direct editing of the WP03 lifecycle file was blocked by the tool safety layer. It is the current branch overlay for WP03 and must be read with the WP03 file.

## Owner path

```text
schema-domain:
  canonical shared lifecycle, freshness, and safety shapes when those shapes cross package, crate, app, or plan boundaries.
family-domain:
  TypeScript helper/projection and local lifecycle tests.
family-identity-core:
  Rust parity only when selected.
browser/runtime surface:
  external owner unless a selected runtime surface is assigned.
device-trust/payment/policy/remote/LAN/data-custody:
  adjacent consumers or owners only.
```

## Import boundary

Allowed direct imports:

```text
schema-domain account/lifecycle/freshness shapes
neutral protocol/event/evidence/logging/capability primitives
approved public family-domain helpers
selected Rust parity crate APIs when selected
pure common helpers without feature behavior
```

Forbidden direct imports:

```text
provider runtime internals
browser runtime internals
policy/payment/remote/LAN/device-trust/data-custody runtime internals
private source files from sibling plan owners
peer feature contracts that should live in schema-domain or another neutral boundary
```

## Proof requirements

Every WP03 update must include:

```text
owner module/path
focused command or explicit blocker
negative cases for expired, revoked, stale, wrong-class, missing freshness, and browser-surface boundaries
artifact path
redaction/custody note
no-claim boundary
```

If browser-surface safety remains a blocker, state it as a blocker. Do not claim real browser runtime enforcement from local lifecycle proof alone.

## No-claim boundary

WP03 can prove local lifecycle semantics and parity for the selected tier. It cannot prove provider runtime completion, browser runtime enforcement, invite/recovery completion, trusted-device bootstrap, UI readiness, payment readiness, remote transport, LAN transport, data-custody execution, or whole-plan PR readiness.
