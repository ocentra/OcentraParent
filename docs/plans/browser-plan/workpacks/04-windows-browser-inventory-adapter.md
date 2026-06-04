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

2026-06-04 codex-d continuation: running-process inventory identity now prefers
the captured executable path when available before falling back to the process
name. Fixture proof covers Chrome for Testing and Tor-style process paths while
preserving process-only or unsupported state and making no exact URL claim. This
still does not enumerate registry uninstall entries, Start Menu shortcuts, live
AppX/MSIX packages, signatures, hashes, service inventory read-model default
root consumption, or real Windows manual inventory evidence.

2026-06-04 codex-d continuation: when a captured process executable path exactly
matches a discovered candidate executable, the adapter now collapses the
candidate path row and process row into one installed-and-running observation.
The merged row preserves the path-derived install state while marking the live
process as unmanaged process-only evidence with exact URL still not-claimed.
Process-only rows without an exact candidate path still remain
`candidate-running`. This still does not enumerate registry uninstall entries,
Start Menu shortcuts, live AppX/MSIX packages, signatures, hashes, service
inventory read-model default-root consumption, or real Windows manual inventory
evidence.

2026-06-04 codex-d continuation: caller-provided registry
display-icon/install-location values and shortcut target strings now normalize
into known browser executable candidate paths before feeding the same Windows
inventory observation adapter. This proves registry/shortcut target ingress at
the parser boundary without live registry enumeration, Start Menu enumeration,
`.lnk` binary parsing, URL/title/content/account capture, AppX/MSIX package
enumeration, signature/hash refs, UI, or enforcement claims.

2026-06-04 codex-d continuation: unquoted registry/shortcut command targets
that include known browser executable paths plus trailing launch arguments now
trim back to the executable path before classification. This covers
caller-provided command target strings such as managed profile launch arguments
without claiming live shortcut parsing, registry enumeration, URL/title/content
capture, or default-profile attachment.

2026-06-04 codex-d continuation: registry/shortcut command targets with a
leading Windows environment-variable segment now expand that caller-provided
prefix before the same known-executable filter runs. This covers path strings
such as environment-rooted browser launch targets while still requiring the
resolved executable fixture to exist and still making no live registry,
Start Menu, `.lnk`, URL/title/content/account, or default-profile attachment
claim.

2026-06-04 codex-d continuation: the service browser inventory read-model
loader now feeds default Windows candidate roots through the existing inventory
path helper before process observations. Focused service proof uses a temp
`PROGRAMFILES` root and shows a default-root Edge executable becomes an
installed managed candidate row with exact URL still unavailable. This still
does not enumerate live registry entries, Start Menu shortcuts, `.lnk` files,
AppX/MSIX packages, signatures, hashes, UI, enforcement actions, or real Windows
manual inventory evidence.

2026-06-04 codex-d continuation: the service default browser inventory path now
collects bounded Windows Uninstall registry display-icon/install-location
sources and bounded Start Menu shortcut targets before feeding the same
candidate-path helper and existing inventory observation adapter. The new
`browser_windows_inventory_sources` module reads live Windows registry and Start
Menu sources on Windows, returns empty live sources on unsupported platforms,
and keeps shortcut parsing limited to known browser executable targets. Focused
proof covers UTF-16 `.lnk` target extraction into a Chrome inventory row and a
host-aware service read-model scan where real registry rows may be present but
all rows still satisfy `claim_boundary_is_honest`. This still does not claim
full `.lnk` shell parsing, AppX/MSIX enumeration, signature/hash extraction,
portal UI, enforcement actions, or product-complete manual Windows inventory
evidence.

2026-06-04 proof hardening: the generated proof JSON now records redacted live
host evidence counts from this Windows machine instead of relying only on
fixture tests. It captured 3 readable registry roots, 958 scanned uninstall
entries, 6 browser-like registry entries, 2 readable Start Menu roots, 270
shortcut files, and 5 browser-named shortcuts. Raw local paths are not written,
and the counts do not upgrade the manual-required gaps listed below.

## Where We Want To Be

Windows inventory can detect supported, candidate, unsupported, packaged,
portable, and block-only browsers without collecting URLs.

## Scope

- Edge Stable/Beta/Dev/Canary.
- Chrome and Chrome for Testing.
- Brave, Vivaldi, Opera, Opera GX, Chromium.
- Firefox Stable/Developer/Nightly as later-adapter unless proved.
- Tor, DuckDuckGo, Arc, portable, AppX/MSIX, and browser-like processes.
- Registry, known paths, bounded Start Menu shortcut target extraction,
  AppX/MSIX, process snapshot, signature/hash refs.

## Touched Paths

- `crates/agent-core/src/browser_managed_discovery.rs`
- `crates/agent-service/src/browser_runtime*.rs`
- `packages/activity-domain/src/browser*.ts`
- `crates/agent-protocol/src/browser*.rs`

## Tests And Proof

- Fixture-backed Windows inventory parser tests.
- Bounded live-source tests for registry/Start Menu source ingestion.
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
- [x] Raw evidence artifacts captured or marked N/A for this fixture-backed/root-expansion/live-source slice: known-path fixtures, deduplicated candidate roots, packaged path-shape fixtures, process executable-path identity fixtures, duplicate installed/running executable collapse fixtures, caller-provided registry display-icon/install-location values, caller-provided shortcut target values including unquoted command targets with launch arguments and leading environment-variable prefixes, bounded live Windows Uninstall registry source collection, bounded Start Menu shortcut target extraction, redacted live host registry and Start Menu source counts, default-root service read-model consumption through temp `PROGRAMFILES` fixture roots, and process observations feed the adapter; no full `.lnk` shell parsing, live AppX/MSIX enumeration, signature/hash, journal, SQLite, policy, or action behavior changed.
- [x] Tests/proof listed in this workpack are implemented for known-path/process fixture parsing, deduplicated root expansion, packaged path-shape classification, process executable-path identity, duplicate installed/running executable collapse, registry display-icon/install-location target normalization, shortcut target normalization including unquoted command target arguments and leading environment-variable prefixes, bounded Start Menu UTF-16 `.lnk` known-target extraction, service default-root/live-source candidate consumption, and service row conversion; full `.lnk` shell parsing, live AppX/MSIX enumeration, signatures, hashes, and product-complete manual platform proof remain manual-required.
- [x] Validation command outputs saved in the proof pack and summarized in [main checklist](../implementation-checklist.md).
- [x] UI snapshots captured for every touched parent portal, child UX, block/warn, policy authoring, or dashboard state; no UI changed, so `ui-not-applicable.md` records why.
- [x] Security/no-claim negative proof captured for this slice: unmanaged running processes remain process-only, unproved Chromium forks are manual-required, unsupported browsers stay unsupported, and no URL collection path was added.
- [x] Manual platform proof captured for real browser/OS claims; no new real OS/browser claim was made, so `09-manual-platform-proof.md` records the pending manual-required boundary.
- [x] Evidence/proof artifact paths recorded in [main checklist](../implementation-checklist.md).
- [x] Feature/expectation/product-checklist/README update decision recorded in [main checklist](../implementation-checklist.md).
- [x] Known gaps, deferred items, and no-claim boundaries recorded before `DONE`.

## Manual-Required Gaps

Detecting a browser does not claim exact URL or app-control blocking.
Remaining adapter work requires product-complete manual Windows capture,
full Start Menu shell shortcut parsing evidence beyond known executable target
extraction, live AppX/MSIX package enumeration, publisher/signature/hash refs,
and portal/manual platform artifact review before this workpack can be marked
complete.
