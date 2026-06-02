# 16 Policy Target Compiler

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

- `packages/parent-domain/src/browser-control-policy.ts`
- `crates/agent-service/src/browser_policy_compiler.rs`
- `crates/agent-service/src/browser_policy_runtime*.rs`
- `crates/agent-protocol/src/browser_policy*.rs`

## Tests And Proof

- Policy compile tests for every target/action/capability state.
- AI-recommendation-to-policy tests that prove model output cannot directly
  enforce.
- Social target compile tests for account creation, secondary account,
  feed/short-video, messaging, upload/post, livestream, and unknown social site.
- Browser-game target compile tests for cloud gaming, educational games,
  unknown games, purchases, loot boxes, unblocked portals, and runtime-signal
  targets.
- Managed Chrome/Edge policy-writer compile tests for incognito, guest/profile,
  history deletion, safe search, restricted mode, and URL allow/block list
  targets.
- Dry-run no-execution tests.
- Manual-required and unavailable state tests.

## AI Worker Checklist

Fill this before reporting `DONE` or PR-ready:

- [ ] Confirm source docs read: [folder README](../README.md), [source index](../source-index.md), [current snapshot](../current-browser-snapshot.md), [full scope plan](../v0-5-managed-browser-full-scope-plan.md), [test blueprint](../v0-5-managed-browser-test-blueprint.md), [UI/UX guide](../ui-ux-requirements-guide.md), [main checklist](../implementation-checklist.md), and this workpack.
- [ ] Check enhancement overlap: URL/video AI intelligence, social platform/account/feed gating, and browser games/cloud gaming.
- [ ] Hub lock covers this workpack and exact implementation/docs paths.
- [ ] Existing source layout inspected; no parallel browser truth created.
- [ ] Before-state source snapshot recorded in `output/browser-plan-proof/<workpack-id>/00-source-snapshot.md` or explicit docs-only N/A reason.
- [ ] Contracts updated first where this workpack changes behavior.
- [ ] Rust/service/portal parity updated only after contracts exist.
- [ ] Raw evidence artifacts captured where applicable: bridge/CDP payloads, managed session state, unmanaged process rows, journal entries, SQLite/read-model rows, policy decisions, and action results.
- [ ] Tests/proof listed in this workpack are implemented or explicitly marked manual-required with reason.
- [ ] Validation command outputs saved in the proof pack and summarized in [main checklist](../implementation-checklist.md).
- [ ] UI snapshots captured for every touched parent portal, child UX, block/warn, policy authoring, or dashboard state; if no UI changed, `ui-not-applicable.md` records why.
- [ ] Security/no-claim negative proof captured where applicable: no default profile attach, no unowned bridge, no unmanaged exact URL claim, no raw debugger URL exposure, and no AI direct enforcement.
- [ ] Manual platform proof captured for real browser/OS claims, including OS/browser version, command steps, screenshots/logs, and manual-required labels.
- [ ] Evidence/proof artifact paths recorded in [main checklist](../implementation-checklist.md).
- [ ] Feature/expectation/product-checklist/README update decision recorded in [main checklist](../implementation-checklist.md).
- [ ] Known gaps, deferred items, and no-claim boundaries recorded before `DONE`.

## Manual-Required Gaps

Compiler output must not upgrade host/domain blocking or exact active-tab
enforcement without separate proof.
