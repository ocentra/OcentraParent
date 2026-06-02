# 05 Cross-Platform Inventory Matrix

## Where We Are

The architecture docs define platform posture, but runtime/browser support is
Windows-first. macOS, Linux, Android, iOS, Safari, Firefox, and mobile browser
support remain platform-specific or manual-required.

## Where We Want To Be

The browser inventory/read model can show platform-specific capability states
without claiming desktop CDP where it does not apply.

## Scope

- macOS app bundle inventory and CDP candidate browsers.
- Linux desktop entries, packages, PATH, Snap/Flatpak, and CDP candidates.
- Firefox WebDriver BiDi or managed extension later-adapter state.
- Safari/WebKit platform-specific state.
- Android owned browser shell, VPN/DNS, UsageStats, Accessibility, Device Owner,
  managed profile, managed configurations.
- iOS FamilyControls, ManagedSettings, Safari extension, and manual-required
  states.

## Touched Paths

- `packages/parent-domain/src/browser-control-*.ts`
- `packages/activity-domain/src/browser*.ts`
- platform README/docs when implementation starts.

## Tests And Proof

- Unit tests for matrix derivation.
- Fixture tests for platform states.
- Manual platform proof tables when platform work starts.

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

No non-Windows browser product claim until real platform proof exists.
