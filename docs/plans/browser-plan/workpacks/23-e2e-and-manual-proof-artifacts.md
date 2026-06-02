# 23 E2E And Manual Proof Artifacts

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

- `scripts/test/managed-browser-profile-matrix.mjs`
- `scripts/test/managed-browser-service-proof.mjs`
- `scripts/test/managed-browser-intervention-proof.mjs`
- `scripts/test/windows-managed-unmanaged-browser-enforcement-proof.mjs`
- `output/browser-proof/**` or `test-results/**`

## Tests And Proof

- Existing focused proof scripts.
- `npm run test:e2e` when portal proof changes.
- Artifact path listed in DONE/PR reports.
- Manual model validation and parent/child UI screenshots for intelligence
  claims.
- Platform proof artifacts under `output/social-proof/**` for social claims.
- Platform proof artifacts under `output/browser-game-proof/**` for browser-game
  claims.

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

Manual platform proof is required for real OS app-control, cross-platform
browser support, mobile browser behavior, stores/signing, and active-tab claim
upgrades. Manual model/runtime proof is required before claiming video semantic
classification quality. Manual platform proof is required before claiming
specific social platform support or specific browser-game/cloud-gaming platform
support.
