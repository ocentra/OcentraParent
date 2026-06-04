# 04 Windows Browser Inventory Adapter

## Where We Are

The repo has Windows-oriented managed browser proof scripts and process
snapshots, but not a full product inventory adapter that reconciles registry,
known paths, AppX/MSIX packages, shortcuts, and running processes.

2026-06-02 codex-d progress: `agent-core` now has a Windows browser inventory
observation adapter for known-path and process-snapshot fixtures. It classifies
managed Edge/Chrome/Chrome-for-Testing candidates, unproved Chromium forks
(Brave, Vivaldi, Opera, Chromium) as manual-required candidates, Firefox/Tor/
DuckDuckGo/Arc-style browsers as unsupported until a later adapter proves them,
and running browser processes as process-only evidence without URL claims. The
service can convert these observations into the existing browser inventory
read-model rows. This does not yet enumerate registry uninstall entries, Start
Menu shortcuts, live AppX/MSIX packages, signatures, hashes, or real Windows
manual inventory evidence.

2026-06-04 codex-d continuation: the Windows inventory path helper now
deduplicates caller-provided candidate roots before expanding the existing
managed/manual/unsupported browser path families from multiple host root
locations. It also proves `WindowsApps` path-shape classification as packaged
while keeping exact URL capability unavailable. This still does not enumerate
registry uninstall entries, Start Menu shortcuts, live AppX/MSIX packages,
signatures, hashes, service consumption of default roots, or real Windows manual
inventory evidence.

## Where We Want To Be

Windows inventory can detect supported, candidate, unsupported, packaged,
portable, and block-only browsers without collecting URLs.

## Scope

- Edge Stable/Beta/Dev/Canary.
- Chrome and Chrome for Testing.
- Brave, Vivaldi, Opera, Opera GX, Chromium.
- Firefox Stable/Developer/Nightly as later-adapter unless proved.
- Tor, DuckDuckGo, Arc, portable, AppX/MSIX, and browser-like processes.
- Registry, known paths, Start Menu, AppX/MSIX, process snapshot, signature/hash
  refs.

## Touched Paths

- `crates/agent-core/src/browser_managed_discovery.rs`
- `crates/agent-service/src/browser_runtime*.rs`
- `packages/activity-domain/src/browser*.ts`
- `crates/agent-protocol/src/browser*.rs`

## Tests And Proof

- Fixture-backed Windows inventory parser tests.
- Fake process snapshot integration tests.
- Manual Windows inventory proof artifact.

## AI Worker Checklist

Fill this before reporting `DONE` or PR-ready:

- [x] Confirm source docs read: [folder README](../README.md), [source index](../source-index.md), [current snapshot](../current-browser-snapshot.md), [full scope plan](../v0-5-managed-browser-full-scope-plan.md), [test blueprint](../v0-5-managed-browser-test-blueprint.md), [UI/UX guide](../ui-ux-requirements-guide.md), [main checklist](../implementation-checklist.md), and this workpack.
- [x] Check enhancement overlap: URL/video AI intelligence, social platform/account/feed gating, and browser games/cloud gaming.
- [x] Hub lock covers this workpack and exact implementation/docs paths.
- [x] Existing source layout inspected; no parallel browser truth created; managed launch identity now reuses the Windows inventory identity path.
- [x] Before-state source snapshot recorded in `output/browser-plan-proof/04-windows-browser-inventory-adapter/00-source-snapshot.md`.
- [x] Contracts updated first where this workpack changes behavior.
- [x] Rust/service parity updated only after contracts exist; portal parity remains deferred because no UI surface changed.
- [x] Raw evidence artifacts captured or marked N/A for this fixture-backed/root-expansion slice: known-path fixtures, deduplicated candidate roots, packaged path-shape fixtures, and process observations feed the adapter; no registry, shortcut, live AppX/MSIX enumeration, signature/hash, journal, SQLite, policy, or action behavior changed.
- [x] Tests/proof listed in this workpack are implemented for known-path/process fixture parsing, deduplicated root expansion, packaged path-shape classification, and service row conversion; registry, shortcut, live AppX/MSIX enumeration, signatures, hashes, service default-root consumption, and manual platform proof remain manual-required.
- [x] Validation command outputs saved in the proof pack and summarized in [main checklist](../implementation-checklist.md).
- [x] UI snapshots captured for every touched parent portal, child UX, block/warn, policy authoring, or dashboard state; no UI changed, so `ui-not-applicable.md` records why.
- [x] Security/no-claim negative proof captured for this slice: unmanaged running processes remain process-only, unproved Chromium forks are manual-required, unsupported browsers stay unsupported, and no URL collection path was added.
- [x] Manual platform proof captured for real browser/OS claims; no new real OS/browser claim was made, so `09-manual-platform-proof.md` records the pending manual-required boundary.
- [x] Evidence/proof artifact paths recorded in [main checklist](../implementation-checklist.md).
- [x] Feature/expectation/product-checklist/README update decision recorded in [main checklist](../implementation-checklist.md).
- [x] Known gaps, deferred items, and no-claim boundaries recorded before `DONE`.

## Manual-Required Gaps

Detecting a browser does not claim exact URL or app-control blocking.
Remaining adapter work requires registry uninstall entry enumeration, Start Menu
shortcut parsing, live AppX/MSIX package enumeration, publisher/signature/hash
refs, service consumption of the default roots, actual Windows manual capture,
and portal/read-model consumption before this workpack can be marked complete.
