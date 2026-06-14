# Tracking Agent Driver — Validation and Proof Enforcement

## Purpose

This document defines validation and proof rules for tracking implementation slices.

The goal is to make agents produce real source behavior, real tests, and accurate proof-tier claims.

## Required proof model

Every checked item must cite proof artifacts under the assigned workpack proof root.

Required proof metadata:

```text
workpack id
proof tier required
current proof tier
status
artifact path
commands run
what this proves
what this does not prove
manual-required gaps
```

## Proof tier discipline

Keep proof tiers explicit.

Examples:

```text
P0_CONTRACT       schema/contract only
P1_FIXTURE        fixture/simulation behavior
P2_HOSTED_CI      hosted/local service or UI CI proof
P3_LOCAL_MACHINE  local emulator/simulator/host proof
P4_PHYSICAL       real physical-device proof
P5_AUTHORITY      authority-enrolled/admin-managed proof
P6_PRODUCTION     production pilot/runtime proof
```

Do not collapse these tiers.

A P1/P2 proof can make a code boundary ready. It cannot prove physical-device, authority, or production behavior.

## Source-change gate

For implementation work, proof is invalid unless one of these changed first:

```text
packages/tracking-domain/src/**
packages/agent-protocol-domain/src/**
crates/tracking-core/src/**
crates/agent-protocol/src/**
crates/agent-service/src/**
apps/portal/src/**
packages/portal-domain/src/**
relevant tests
```

Exception: workpack is explicitly proof-routing-only.

## Command log requirement

Every slice must record focused commands.

Minimum command log shape:

```text
command: <exact command>
exit: <code>
result: pass | fail | blocked
notes: <why blocked if blocked>
```

Store in the assigned proof root as:

```text
16-validation-commands.log
```

or another path named by the workpack.

## Validation command selection

Use focused commands first.

### TypeScript tracking-domain

```bash
npm run build --workspace @ocentra-parent/tracking-domain
npm run test --workspace @ocentra-parent/tracking-domain
```

### Rust tracking-core

```bash
cargo test -p ocentra-tracking-core
cargo clippy -p ocentra-tracking-core --all-targets -- -D warnings
```

### Rust protocol

```bash
cargo test -p ocentra-parent-agent-protocol
```

### Rust service

```bash
cargo test -p ocentra-parent-agent-service
```

### Portal

```bash
npm run test --workspace @ocentra-parent/portal
npm run test:e2e --workspace @ocentra-parent/portal
```

Run broader validation only after focused commands pass or a precise blocker is recorded.

## Required negative cases

Tracking tests must cover degraded and invalid states, not only happy paths.

Use applicable cases:

```text
stale sample
last-known only
offline device
permission denied
permission required
background permission missing
approximate-only location
unknown provider
provider unavailable
manual-required state
duplicate event/idempotency
replay must not resend side effects
AI result is evidence only
portal cannot publish business event directly
LAN/IP/pairing cannot become precise location
```

## No-claim enforcement

Reject proof or docs that imply these without required artifacts:

```text
Android system geofence delivery
iOS background/region monitoring behavior
physical-device behavior
authority-enrolled device behavior
provider dispatch/delivery behavior
production worker behavior
product-ready tracking
```

Use exact weaker language when appropriate:

```text
fixture simulated
hosted UI rendered
local emulator observed
local service read-model proved
manual-required
authority-required
not claimed
```

## Workpack completion gate

Before reporting `DONE` or `PR_READY`, fill the workpack section:

```text
Workpack id and branch:
Touched files:
Validation commands and results:
Proof artifacts:
Product doc/checklist updates:
Known gaps/manual-required states:
```

Do not report done if this section is still placeholder text.

## Checklist update gate

A checklist row can be checked only if:

```text
1. assigned workpack row is filled
2. exact proof artifact exists
3. commands are listed
4. no-claim boundaries are stated
5. source behavior or proof-routing exception is clear
```

Do not check rows from sibling workpacks.

## Suggested enforcement scripts

Add later if not already present:

```text
scripts/check-tracking-workpack-claims.mjs
scripts/check-tracking-proof-tier-claims.mjs
scripts/check-tracking-no-proof-only-implementation.mjs
scripts/check-tracking-no-new-barrels.mjs
```

### `check-tracking-workpack-claims.mjs`

Checks:

```text
- workpack completion section is not placeholder when checkbox status changes
- proof root exists for checked rows
- validation command log exists
- no sibling workpack checkbox changed without assignment note
```

### `check-tracking-proof-tier-claims.mjs`

Checks:

```text
- product-ready claims require P4/P5/P6 proof depending on claim
- hosted/emulator/fixture proofs use weaker status labels
- manual-required gaps are preserved
```

### `check-tracking-no-proof-only-implementation.mjs`

Checks PR diff:

```text
- if proof/checklist files changed but no source/test files changed, require proof-routing-only marker
- if marker absent, fail
```

### `check-tracking-no-new-barrels.mjs`

Checks:

```text
- no new aggregate export files
- no new export * from statements
- existing tracking.ts aggregate is not expanded
```

## Integration with local evidence wrapper

When the local evidence wrapper exists, tracking validation should use:

```bash
npm run agent:run -- cargo test -p ocentra-tracking-core
npm run agent:run -- npm run test --workspace @ocentra-parent/tracking-domain
npm run codex:evidence -- latest-failures
```

Before that wrapper exists, use normal focused commands and record exact output path in proof root.

## Completion report template

```text
Assigned workpack:
Source behavior added:
Tests added/changed:
Commands run:
Proof artifacts:
Checklist/docs updated:
No-claim boundaries preserved:
Remaining gaps:
```

If `Source behavior added` is empty, say `proof-routing-only` and justify why.

## Rejection criteria

Reject the slice if any are true:

```text
- proof JSON changed before source/tests without proof-routing-only reason
- full checklist was scanned/edited instead of exact rows
- sibling workpacks were modified without assignment
- product-ready language appears without physical/authority/production proof
- runtime decision logic was added to portal or WebSocket transport instead of tracking-core
- portal sends direct business events instead of typed intents
- TS/Rust protocol parity changed without parity tests
- new aggregate re-export/barrel file was added
```
