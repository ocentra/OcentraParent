# 05 Cross-Platform Inventory Matrix

## Where We Are

The architecture docs define platform posture, but runtime/browser support is
Windows-first. macOS, Linux, Android, iOS, Safari, Firefox, and mobile browser
support remain platform-specific or manual-required.

2026-06-02 codex-d progress: `activity-domain` now exposes a typed browser
platform inventory matrix. It keeps Windows Edge/Chrome entries fixture-backed,
macOS/Linux Chromium candidates manual-required, Firefox/Safari/WebKit and
external mobile browsers unsupported until platform adapters prove them, and
Android owned-browser-shell support manual-required. The matrix rejects
non-Windows managed exact-URL or known-active claims. This is contract proof
only; it does not add macOS/Linux/mobile adapters or manual platform evidence.

2026-06-04 codex-d progress: `scripts/test/browser-platform-inventory-matrix-proof.mjs`
now replays the matrix as a proof gate. It validates row uniqueness, platform
coverage, manual-required proof requirements, unsupported exact-URL labels,
Windows-only fixture-backed status, and iOS unsupported boundaries, then writes
`test-results/browser-platform-inventory-matrix-proof/proof.json` and
`output/browser-plan-proof/05-cross-platform-inventory-matrix/11-proof-gate-manifest.md`.
This is still a no-claim gate; it does not add real macOS/Linux/mobile adapters
or product checklist status.

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
- `scripts/test/browser-platform-inventory-matrix-proof.mjs`
- `test-results/browser-platform-inventory-matrix-proof/`
- `output/browser-plan-proof/05-cross-platform-inventory-matrix/`
- platform README/docs when implementation starts.

## Tests And Proof

- Unit tests for matrix derivation.
- Fixture tests for platform states.
- `node scripts/test/browser-platform-inventory-matrix-proof.mjs`
- Manual platform proof tables when platform work starts.

## AI Worker Checklist

Fill this before reporting `DONE` or PR-ready:

- [x] Confirm source docs read: [folder README](../README.md), [source index](../source-index.md), [current snapshot](../current-browser-snapshot.md), [full scope plan](../v0-5-managed-browser-full-scope-plan.md), [test blueprint](../v0-5-managed-browser-test-blueprint.md), [UI/UX guide](../ui-ux-requirements-guide.md), [main checklist](../implementation-checklist.md), and this workpack.
- [x] Check enhancement overlap: URL/video AI intelligence, social platform/account/feed gating, and browser games/cloud gaming.
- [x] Hub lock covers this workpack and exact implementation/docs paths.
- [x] Existing source layout inspected; no parallel browser truth created.
- [x] Before-state source snapshot recorded in `output/browser-plan-proof/05-cross-platform-inventory-matrix/00-source-snapshot.md`.
- [x] Contracts updated first where this workpack changes behavior.
- [x] Rust/service/portal parity updated only after contracts exist; no Rust/service/portal surface changed in this contract-only slice.
- [x] Raw evidence artifacts captured or marked N/A: this slice is a platform support matrix contract and has no bridge/CDP, journal, SQLite, policy, or action runtime evidence.
- [x] Tests/proof listed in this workpack are implemented for matrix derivation, dishonest-state rejection, and the repeatable proof gate; live macOS/Linux/Android/iOS fixtures and manual proof remain manual-required.
- [x] Validation command outputs saved in the proof pack and summarized in [main checklist](../implementation-checklist.md).
- [x] UI snapshots captured for every touched parent portal, child UX, block/warn, policy authoring, or dashboard state; no UI changed, so `ui-not-applicable.md` records why.
- [x] Security/no-claim negative proof captured: non-Windows entries reject managed exact-URL and known-active claims; iOS remains unsupported; unsupported entries cannot keep exact URL available.
- [x] Manual platform proof captured for real browser/OS claims; no new real platform claim was made, so `09-manual-platform-proof.md` records the pending manual-required boundary.
- [x] Evidence/proof artifact paths recorded in [main checklist](../implementation-checklist.md), including `test-results/browser-platform-inventory-matrix-proof/proof.json` and `output/browser-plan-proof/05-cross-platform-inventory-matrix/11-proof-gate-manifest.md`.
- [x] Feature/expectation/product-checklist/README update decision recorded in [main checklist](../implementation-checklist.md).
- [x] Known gaps, deferred items, and no-claim boundaries recorded before `DONE`.

## Manual-Required Gaps

No non-Windows browser product claim until real platform proof exists.
Remaining work requires real macOS app bundle inventory, Linux desktop entry and
package inventory, Android owned browser shell/device-policy proof, iOS
FamilyControls/ManagedSettings/Safari-extension proof, and manual platform
artifact tables before this workpack can be marked complete.
