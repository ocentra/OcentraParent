# 16 Policy Target Compiler

<!-- agent-capsule -->

> Agent Capsule
> Plan: `browser-plan`
> Doc: `16 Policy Target Compiler`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Where We Are

Browser policy contracts exist, and V0.8 enforcement proof keeps unsupported
browser/domain actions manual-required or not claimed.

## Where We Want To Be

Parent browser rules compile only against evidence and action capabilities that
exist on the selected device/source.

## Scope

- Exact URL target requires managed exact URL capability.
- Domain target may compile against managed URL/domain evidence or network
  domain proof with different strength labels.
- Category target requires classifier proof.
- Search query and video URL/channel require URL shape and metadata proof.
- Social platform, route kind, account creation, unknown account, secondary
  account, feed, short-video feed, messaging, upload/post, livestream, and
  unknown social site targets require typed social evidence and policy approval
  state.
- Browser-game targets such as all browser games, game platform, game portal,
  specific game URL, educational games, cloud gaming, WebGL/canvas games,
  multiplayer/UGC, game chat, purchases, loot boxes, unknown games, and
  unblocked game sites require typed game evidence and capability state.
- Managed Chrome/Edge policy outputs must compile only from typed policy-writer
  inputs and adapter capability proof; unsupported browser policy keys stay
  manual-required or unavailable.
- AI classification can supply candidate category/risk/benefit evidence, but
  final allow/warn/ask/time-limit/block/unknown must come from deterministic
  parent policy.
- Unmanaged browser target requires process detection.
- Block/warn/terminate/OS block actions require corresponding adapter proof.
- Observe/dry-run never execute adapters.

## Touched Paths

- `packages/parent-domain/src/browser-control-values.ts`
- `packages/parent-domain/src/browser-control-catalog-values.ts`
- `packages/parent-domain/src/browser-control-policy.ts`
- `packages/parent-domain/tests/browser-control-contracts.test.ts`
- `crates/agent-protocol/src/browser_policy_values.rs`
- `crates/agent-protocol/src/browser_policy_catalog_values.rs`
- `crates/agent-protocol/src/browser_policy_model.rs`
- `crates/agent-protocol/src/browser_policy_tests.rs`
- `crates/agent-protocol/src/constants.rs`
- `crates/agent-service/src/browser_policy_compiler.rs`
- `crates/agent-service/src/browser_policy_compiler_assessment.rs`
- `crates/agent-service/src/browser_policy_compiler_tests.rs`
- `crates/agent-service/src/browser_policy_request.rs`
- `crates/agent-service/src/browser_policy_runtime_support.rs`
- `crates/agent-service/src/browser_policy_api_tests.rs`
- `crates/agent-service/src/browser_policy_manifest_patch_tests.rs`
- `crates/agent-service/src/main.rs`

## Tests And Proof

- Policy compile tests cover exact URL, domain, category, search metadata,
  social route evidence, browser-game runtime signal, and process-detection
  proof requirements.
- AI-recommendation-to-policy proof shows AI suggestions/summaries compile as
  candidate-only authority and cannot directly enforce.
- Social target compile proof covers ready social evidence only when the policy
  is approved, with unknown social targets staying manual-required.
- Browser-game target compile proof covers runtime-signal-ready and unknown game
  manual-required states.
- Managed Chrome/Edge policy-writer proof keeps policy writing manual-required
  until browser policy integration, concrete writer controls, and adapter proof
  are present.
- Dry-run and observe no-execution tests prove adapters are not executed.
- Manual-required and adapter-ready action tests cover blocking action gates.

## AI Worker Checklist

Fill this before reporting `DONE` or PR-ready:

- [ ] Confirm source docs read: [folder README](../README.md), [source index](../source-index.md), [current snapshot](../current-browser-snapshot.md), [full scope plan](../v0-5-managed-browser-full-scope-plan.md), [test blueprint](../v0-5-managed-browser-test-blueprint.md), [UI/UX guide](../ui-ux-requirements-guide.md), [main checklist](../implementation-checklist.md), and this workpack.
- [ ] Check enhancement overlap: URL/video AI intelligence, social platform/account/feed gating, and browser games/cloud gaming.
- [ ] Hub lock covers this workpack and exact implementation/docs paths.
- [ ] Existing source layout inspected; no parallel browser truth created; the compiler extends the existing browser-policy contract/protocol/service path.
- [ ] Before-state source snapshot recorded in `output/browser-plan-proof/16-policy-target-compiler/00-source-snapshot.md`.
- [ ] Contracts updated first where this workpack changes behavior: target/proof/action/AI authority vocabulary and effective-rule result fields were added in `parent-domain`.
- [ ] Rust/service parity updated only after contracts exist; no portal visual source changed for WP16.
- [ ] Raw evidence artifacts captured where applicable: policy decisions and action-result states are captured through compiler tests and `05-policy-action-proof.json`; no bridge/CDP/session/journal/SQLite path changed.
- [ ] Tests/proof listed in this workpack are implemented or explicitly marked manual-required with reason.
- [ ] Validation command outputs saved in the proof pack and summarized in [main checklist](../implementation-checklist.md).
- [ ] UI snapshots captured for every touched parent portal, child UX, block/warn, policy authoring, or dashboard state; no UI changed and `output/browser-plan-proof/16-policy-target-compiler/06-ui-snapshots/ui-not-applicable.md` records why.
- [ ] Security/no-claim negative proof captured where applicable: no unmanaged exact URL claim, no browser policy writer claim, no adapter execution in observe/dry-run, and no AI direct enforcement.
- [ ] Manual platform proof captured for real browser/OS claims: marked manual-required because WP16 compiles target/action requirements but does not prove platform policy writes or real browser blocking.
- [ ] Evidence/proof artifact paths recorded in [main checklist](../implementation-checklist.md).
- [ ] Feature/expectation/product-checklist/README update decision recorded in [main checklist](../implementation-checklist.md).
- [ ] Known gaps, deferred items, and no-claim boundaries recorded before `DONE`.

## Manual-Required Gaps

Compiler output must not upgrade host/domain blocking or exact active-tab
enforcement without separate proof.

WP16 keeps managed browser policy writing, exact active-tab enforcement,
child-facing warning/block delivery, unmanaged exact URL evidence, and real
platform adapter execution manual-required or not-claimed until later
workpacks provide adapter/platform proof.
