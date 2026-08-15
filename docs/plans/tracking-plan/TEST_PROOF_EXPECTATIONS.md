<!-- agent-capsule -->

> Agent Capsule
> Plan: `tracking-plan`
> Doc: `Tracking Plan Test Proof Expectations`
> Kind: command/test selector.
> Read when: selected workpack asks which commands or proof artifacts are expected.
> Stop rule: run focused commands first; do not jump to full validation unless required by the workpack or PR_READY.
> Proves: command expectations only.
> Does not prove: implementation completion without matching artifacts.

<!-- /agent-capsule -->

# Tracking Plan Test Proof Expectations

> **2026-08-15 executable-status correction:** the advertised
> `scripts/test/tracking-*.mjs` suite and `packages/tracking-domain` package do
> not exist in the current checkout. The command catalog below remains the
> desired proof contract, not evidence that the commands ran or even exist.
> WP33 owns restoration/replacement after the applicable Phase 1 code/test gaps
> in `CODE_AUDIT.md` are closed.

## General rule

Use the selected workpack's named proof artifacts first. If missing, derive:

```text
output/tracking-plan-proof/<workpack-file-stem>/
```

## Central schema proof rule

Every proof that touches a cross-boundary tracking shape must identify the canonical owner:

```text
schema-domain or neutral protocol/event/evidence boundary: canonical schema
tracking-domain: helper, projection, proof adapter, and focused tests
tracking-core: Rust mirror/parser/runtime helper
```

A proof is incomplete when a public contract, event payload, protocol shape, read-model DTO, policy input, notification input, custody/export shape, or proof metadata shape exists only as a tracking-local schema.

## Common command families

Use the subset relevant to the selected workpack:

```bash
npm run build --workspace @ocentra-parent/tracking-domain
npm run test --workspace @ocentra-parent/tracking-domain
cargo test -p ocentra-tracking-core
cargo test -p ocentra-parent-agent-protocol tracking
npm run test --workspace @ocentra-parent/portal -- tracking
npm run lint:architecture -- --files packages/tracking-domain crates/tracking-core packages/agent-protocol-domain crates/agent-protocol apps/portal docs/plans/tracking-plan
```

Audit note:

- `node scripts/test/tracking-source-reconciliation-gap-map-proof.mjs` is a dependent proof step, not a cheap standalone green check. It requires the product-readiness closure proof artifact first.
- `node scripts/test/tracking-claim-audit-proof.mjs` now reruns from `packages/tracking-domain` source and is the cheap WP33 aggregate proof gate that should stay green before closure/source-reconciliation reruns.
- `node scripts/test/tracking-product-readiness-closure-proof.mjs` must carry blocker rows rather than changing product-ready claims when upstream artifacts are missing.

## Tracking E2E meaning

```text
schema E2E: canonical schema owner -> helper/runtime mirror -> invalid shape rejection.
evidence E2E: sample -> accuracy/source/freshness -> stale/manual-required handling.
status E2E: heartbeat/battery/connectivity -> degraded/offline state.
rule E2E: evidence + parent rule -> evaluated state -> no weak-evidence overclaim.
retention E2E: evidence refs -> retention/delete/export/tombstone -> custody proof.
policy E2E: evidence refs -> policy decision -> no AI/direct notification authority.
event-chain E2E: canonical events -> journal/replay/projection -> no duplicate side effects.
portal E2E: service/event read model -> UI state -> screenshot/accessibility proof only.
rollout E2E: accepted proof roots + blockers -> claim audit -> product-ready remains false unless hard proof exists.
```

## Required negative states

```text
low accuracy produces ambiguous/check-in state
wrong household/device denied
stale evidence visible
offline state visible
manual-required state visible
single-machine proof not physical-device proof
UI read-only proof not delivery/runtime proof
local schema not accepted as public contract unless promoted
```

## Failure conditions

- Do not mark DONE or PR_READY from happy-path-only proof.
- Do not store proof inventories inside this plan folder.
- Do not claim physical platform behavior unless the selected workpack explicitly proves it.
- Do not allow tracking-domain/tracking-core to become canonical schema owners for cross-boundary shapes.
