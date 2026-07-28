# 01 Contract Boundary And Effect Schemas

Sources: [folder README](../README.md), [source index](../source-index.md), [current snapshot](../current-app-snapshot.md), [full scope plan](../v0-5-native-apps-full-scope-plan.md), [platform deep dive](../v0-5-native-apps-platform-deep-dive.md), and [test blueprint](../v0-5-native-apps-test-blueprint.md).

## Where We Are

Historical notes referenced `packages/activity-domain`, `packages/parent-domain`, and `crates/agent-protocol/src/app_game.rs` as app/game contract owners. That is now migration context, not the default owner path for new cross-boundary shapes.

Current owner path (Rust-first):

```text
crates/schema or the owning Rust domain/runtime crate:
  canonical shared native-app/app-game contracts when shapes cross package, crate, app, or plan boundaries.
packages/schema-domain:
  temporary generated edge decoders only; it is not product contract authority.
crates/app-core:
  native-app observation/event consumer when selected.
crates/agent-protocol / crates/agent-service:
  wire/service consumers when selected.
app-game-plan:
  shared native app/game evidence spine and combined app/game runtime/read-model proof chains.
```

This workpack is app-only narrowing/reconciliation. It must not take over the shared app/game spine without an explicit app-only handoff.

## Where We Want To Be

Every native app workpack starts from typed Effect Schema contracts before Rust, service, portal, AI, policy, or enforcement code claims support. Contracts must encode weak evidence, degraded states, authority tiers, and manual-required states so later layers cannot overclaim.

## Scope

- Native app identity, inventory, runtime, foreground, session, category, risk, policy target, approval, authority-tier, capability, action, and enforcement result schemas.
- Evidence refs, confidence, reason codes, stale/degraded states, custody, redaction, and proof refs.
- AI classification input/output refs, with no action authority.
- Platform state language: observe-only, permission-required, managed-device-required, admin/root-required, system-extension-required, supervised-device-required, manual-required, and not-claimed.

## Touched Paths

Prefer current owner paths:

```text
packages/schema-domain/**
crates/app-core/** when Rust app observation/event proof is selected
crates/agent-protocol/** when wire parity is selected
crates/agent-service/** when service projection is selected
docs/plans/app-plan/**
```

Legacy migration context only:

```text
packages/activity-domain/src/app-game*.ts
packages/parent-domain/src/app-control-*
packages/parent-domain/src/enforcement-policy-dispatch.ts
```

Do not create a new `packages/app-domain` owner unless a selected workpack explicitly implements and proves it.

## Tests And Proof

- Effect Schema decode tests accept valid states and reject invalid states.
- No display-name-only deterministic identity.
- Inventory cannot set runtime or foreground fields.
- AI output cannot contain block/terminate/suspend/shield actions.
- Manual-required/unavailable states cannot mark actions as executed.
- Rust protocol parity lands only after canonical TypeScript contracts exist.
- App-game-plan proof cannot close this workpack unless the app-only handoff is named.

Focused proof should use `TEST_PROOF_EXPECTATIONS.md` and include schema-domain tests first when shared contracts change.

## AI Worker Checklist

Fill this before reporting `DONE` or PR-ready:

- [ ] Read `workpacks/00-owner-boundary-proof-gate.md` and confirm this workpack remains app-only.
- [ ] Confirm source docs read: [folder README](../README.md), [source index](../source-index.md), [current snapshot](../current-app-snapshot.md), [full scope plan](../v0-5-native-apps-full-scope-plan.md), [platform deep dive](../v0-5-native-apps-platform-deep-dive.md), [test blueprint](../v0-5-native-apps-test-blueprint.md), [UI/UX guide](../ui-ux-requirements-guide.md), [main checklist](../implementation-checklist.md), and this workpack.
- [ ] Confirm this is native/installed-app scope, not browser pages, browser games, or game-specific product semantics unless the source docs explicitly route that handoff.
- [ ] Hub lock covers this workpack and exact implementation/docs paths.
- [ ] Existing app/game source layout inspected; no parallel app-control truth created.
- [ ] Before-state source snapshot recorded in `output/app-plan-proof/<workpack-id>/00-source-snapshot.md` or explicit docs-only N/A reason.
- [ ] Contracts updated first where this workpack changes behavior.
- [ ] Rust/service/portal parity updated only after canonical contracts exist.
- [ ] Raw evidence artifacts captured where applicable: inventory rows, process/package observations, foreground observations, session summaries, journal entries, SQLite/read-model rows, policy decisions, approval requests, authority-tier rows, and enforcement results.
- [ ] Tests/proof listed in this workpack and [test blueprint](../v0-5-native-apps-test-blueprint.md) are implemented or explicitly marked manual-required with reason.
- [ ] Required fixtures are present or N/A with reason for inventory, runtime, foreground, session, policy, enforcement, UI, malicious metadata, stale state, and manual-required state.
- [ ] Validation command outputs saved in the proof pack and summarized in [main checklist](../implementation-checklist.md).
- [ ] UI snapshots captured for every touched parent portal, child UX, policy authoring, approval, evidence drawer, dashboard, stale, degraded, or manual-required state; if no UI changed, `ui-not-applicable.md` records why.
- [ ] Security/no-claim negative proof captured where applicable: inventory is not usage, running is not foreground, foreground is not content, AI cannot enforce, manual-required cannot call adapters, and private paths/command lines do not leak.
- [ ] Manual platform proof captured for any claim stronger than observe-only, including OS/device version, authority tier, permission/enrollment setup, commands/UI steps, screenshots/logs, rollback, and cleanup.
- [ ] Platform limitations use capability status language: observe-only, permission-required, managed-device-required, admin/root-required, system-extension-required, supervised-device-required, manual-required, or not-claimed, with proof needed to move up.
- [ ] Evidence/proof artifact paths recorded in [main checklist](../implementation-checklist.md).
- [ ] Feature/expectation/product-checklist/README update decision recorded in [main checklist](../implementation-checklist.md).
- [ ] Known gaps, deferred items, and no-claim boundaries recorded before `DONE`.

## Completion Reconciliation

Completed on codex/app-plan-proof-reconciliation by mirroring the shared app/game proof spine into the native app plan.

- App-plan proof root: output/app-plan-proof/01-contract-boundary-and-effect-schemas
- Shared app/game proof root: output/app-game-plan-proof/01-contract-boundary-and-effect-schemas
- Current branch note: this historical reconciliation predates the plan-harness branch. On `codex/plan-harness-update`, treat it as prior proof evidence only; new edits must follow `workpacks/00-owner-boundary-proof-gate.md`, `WORKPACK_FAMILIES.md`, `TEST_PROOF_EXPECTATIONS.md`, and `PROOF_INDEX.md`.
- Product-doc decision: no feature doc, expectation doc, roadmap, or product capability checklist status moved because this reconciliation does not add new runtime, service, portal, policy, enforcement, or platform capability proof.
- Remaining boundary: app-plan follow-up work still owns app-only authority, taxonomy, sessionization, journal/read-model, portal, approval, policy, child UX, broad blocking, AI digest, install/purchase, performance, E2E, and rollout slices.

## 2026-07-28 Rust Runtime-Decision Integration Evidence

This narrow integration repaired the previously unmergeable PR #577 contract test and retained only Rust-owned runtime-decision behavior. The discarded TypeScript `schema-domain` contract files were a second product authority and are not part of this workpack's completed evidence.

- Rust owner: `crates/app-core`.
- Evidence: [`docs/proof/app-plan/01-contract-boundary-and-effect-schemas.md`](../../../proof/app-plan/01-contract-boundary-and-effect-schemas.md).
- Proven: canonical runtime ID prefixes, all 18 capability/foreground/classification decision tuples, the version-2 event contract, and a serialized `EventEnvelope` fixture.
- Negative boundaries: malformed/display-name IDs are rejected; missing capability does not publish AI or policy handoffs; inventory-only remains inventory recording.
- No product status moved: this does not prove a live OS inventory/process source, service/read model, portal rendering, policy execution, enforcement adapter, or platform capability.

## Manual-Required Gaps

Contracts alone do not create runtime, platform, UI, policy, AI, or enforcement claims.
