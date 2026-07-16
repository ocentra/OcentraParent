# 23 E2E And Manual Proof Artifacts

<!-- agent-capsule -->

> Agent Capsule
> Plan: `browser-plan`
> Doc: `23 E2E And Manual Proof Artifacts`
> Kind: proof reference; read only when validating matching claim.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Where We Are

Existing scripts write proof under `test-results/managed-browser-*` and related
proof directories. The browser plan needs a consistent artifact expectation for
real browser claims.

## Where We Want To Be

Every real browser claim has JSON, journal/read-model, screenshot, and manual
proof artifacts with unsupported/manual-required gaps recorded.

## Scope

- Managed Edge evidence.
- Managed Chrome/Chrome for Testing evidence.
- Unmanaged Chrome bypass.
- Bridge disconnect stale.
- Policy dry-run.
- Managed block page.
- URL/video intelligence classification with evidence, model/provider, policy,
  decision, action, and degraded-state artifacts.
- Social signup/account/feed proof with evidence, approval, child hold screen,
  parent decision, policy audit, and degraded/manual-required artifacts.
- Browser-game/cloud-gaming proof with URL shape, runtime signals, metadata,
  AI result, policy decision, approval/time-budget action, screenshots, and
  degraded/manual-required artifacts.
- Unsupported Firefox/later-adapter state.
- Windows manual proof matrix.
- macOS/Linux/Android/iOS manual matrices when started.

## Touched Paths

- `scripts/test/browser-plan-e2e-manual-proof-artifacts.mjs`
- `output/browser-plan-proof/23-e2e-and-manual-proof-artifacts/`
- `test-results/browser-plan-e2e-manual-proof-artifacts/proof.json`
- `docs/plans/browser-plan/workpacks/23-e2e-and-manual-proof-artifacts.md`
- `docs/plans/browser-plan/implementation-checklist.md`
- `docs/features/browser-web-control.md`
- `docs/expectations/browser-evidence.md`

## Tests And Proof

- `cmd /c node --check scripts/test/browser-plan-e2e-manual-proof-artifacts.mjs`
- `cmd /c node scripts/test/browser-plan-e2e-manual-proof-artifacts.mjs`
- `cmd /c npx prettier --write docs/plans/browser-plan/workpacks/23-e2e-and-manual-proof-artifacts.md docs/plans/browser-plan/implementation-checklist.md docs/features/browser-web-control.md docs/expectations/browser-evidence.md output/browser-plan-proof/23-e2e-and-manual-proof-artifacts/*.md`
- `cmd /c npm run format:check`
- `git diff --check`
- The generated manifest records artifact-present rows for managed Edge,
  managed Chrome/Chrome for Testing, unmanaged Chrome bypass, policy dry-run,
  managed block page, and Windows manual proof matrix when prior proof artifacts
  exist.
- The generated manifest records partial/manual-required rows for URL/video
  intelligence, social signup/account/feed, and browser-game/cloud-gaming
  because existing route/screenshot artifacts do not prove model/provider,
  parent decision, runtime-signal, or cloud-session behavior.
- Bridge disconnect stale state and unsupported Firefox/later-adapter state now
  have runtime/read-model/protocol proof in
  `browser-runtime-stale-unsupported-proof`; real cross-platform browser support
  and macOS/Linux/Android/iOS matrices remain manual-required until those real
  artifacts exist.

## AI Worker Checklist

Fill this before reporting `DONE` or PR-ready:

- [ ] Confirm source docs read: [folder README](../README.md), [source index](../source-index.md), [current snapshot](../current-browser-snapshot.md), [full scope plan](../v0-5-managed-browser-full-scope-plan.md), [test blueprint](../v0-5-managed-browser-test-blueprint.md), [UI/UX guide](../ui-ux-requirements-guide.md), [main checklist](../implementation-checklist.md), and this workpack.
- [ ] Check enhancement overlap: URL/video AI intelligence, social platform/account/feed gating, and browser games/cloud gaming.
- [ ] Hub lock covers this workpack and exact implementation/docs paths.
- [ ] Existing source layout inspected; no parallel browser truth created.
- [ ] Before-state source snapshot recorded in `output/browser-plan-proof/23-e2e-and-manual-proof-artifacts/00-source-snapshot.md`.
- [ ] Contracts updated first where this workpack changes behavior; WP23 adds a proof manifest script and no new runtime contract.
- [ ] Rust/service/portal parity updated only after contracts exist; no Rust/service/portal runtime code changed for WP23.
- [ ] Raw evidence artifacts captured where applicable: manifest indexes existing JSON, screenshots, and manual-required gaps from current proof folders.
- [ ] Tests/proof listed in this workpack are implemented or explicitly marked manual-required with reason.
- [ ] Validation command outputs saved in the proof pack and summarized in [main checklist](../implementation-checklist.md).
- [ ] UI snapshots captured for every touched parent portal, child UX, block/warn, policy authoring, or dashboard state; WP23 indexes existing screenshots and records no UI source changes in `ui-artifact-index.md`.
- [ ] Security/no-claim negative proof captured where applicable: no default profile attach, no unowned bridge, no unmanaged exact URL claim, no raw debugger URL exposure, and no AI direct enforcement.
- [ ] Manual platform proof captured for real browser/OS claims, including OS/browser version, command steps, screenshots/logs, and manual-required labels.
- [ ] Evidence/proof artifact paths recorded in [main checklist](../implementation-checklist.md).
- [ ] Feature/expectation/product-checklist/README update decision recorded in [main checklist](../implementation-checklist.md).
- [ ] Known gaps, deferred items, and no-claim boundaries recorded before `DONE`.

## Manual-Required Gaps

Manual platform proof is required for real OS app-control, cross-platform
browser support, mobile browser behavior, stores/signing, and active-tab claim
upgrades. Manual model/runtime proof is required before claiming video semantic
classification quality. Manual platform proof is required before claiming
specific social platform support or specific browser-game/cloud-gaming platform
support.

WP23 records those gaps in a generated manifest instead of treating missing
cross-platform, model/provider, parent-decision, runtime-signal, and
cloud-session artifacts as proof. The stale/unsupported runtime proof narrows
only the service/read-model/protocol gap; it does not upgrade real browser
platform support, host blocking, exact active-tab enforcement, browser mutation,
child intervention execution, final policy execution, or enforcement.
