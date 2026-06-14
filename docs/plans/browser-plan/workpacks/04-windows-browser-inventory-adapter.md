# 04 Windows Browser Inventory Adapter

<!-- agent-capsule -->

> Agent Capsule
> Plan: `browser-plan`
> Doc: `04 Windows Browser Inventory Adapter`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

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

2026-06-05 codex-d continuation: `scripts/test/browser-windows-live-inventory-proof.mjs`
now runs against the local Windows host and records redacted live evidence for
known-path, registry uninstall, Start Menu shortcut, and running-process browser
inventory inputs. The proof captured real Chrome, Edge, and Firefox executable
rows with file SHA-256 refs, Authenticode status, source counts, and hashed path
refs while storing no raw paths, raw URLs, page titles, page bodies, cookies,
tokens, tabs, or profile data. This improves the manual Windows platform proof,
but it still does not apply AppLocker/App Control, parse `.lnk` files inside the
Rust adapter, claim exact URL/tab visibility, capture browser content, or prove
blocking/enforcement.

2026-06-06 codex-d continuation: the Rust browser inventory service path now
has a browser-owned live Windows registry source. On Windows it enumerates
uninstall registry DisplayIcon and InstallLocation entries, feeds them through
the existing browser candidate-path normalization, and then uses the existing
browser observation/read-model conversion. Focused Rust tests prove
fixture-backed registry display-icon/install-location candidates, shortcut
target ingress through the same helper, and service default rows while allowing
additional live host registry rows. This moves registry enumeration into the
Rust browser inventory path without adding exact URL/tab capture, page content,
AppLocker/App Control application, blocking, rollback, or enforcement claims.
Rust `.lnk` binary parsing, live AppX/MSIX enumeration, and OS policy
apply/rollback artifacts remain pending.

2026-06-06 codex-d continuation: the Rust browser inventory service path now
also has browser-owned Windows Store package manifest enumeration. It walks
WindowsApps/AppxManifest.xml roots, extracts package name, display label, and
AppUserModelId refs, maps browser packages into package-ref-only inventory
observations, and feeds those rows through the existing service read-model
conversion. Focused Rust tests prove fixture-backed package manifest ingestion
and service source conversion, while the live Windows proof captured 12
store-package rows with no executable, URL, tab, page content, or enforcement
claim. Rust `.lnk` binary parsing and OS policy apply/rollback artifacts remain
pending.

2026-06-06 codex-d continuation: the Rust browser inventory service path now
parses Start Menu `.lnk` files directly. The browser-owned shortcut source
recursively finds shortcut files, parses the Shell Link `LinkInfo` local base
path, feeds the target string through the existing browser candidate-path
normalizer, and then relies on the existing observation/read-model conversion.
Focused Rust proof builds a binary `.lnk` fixture with a local Edge executable
target and verifies the resulting managed candidate row still has exact URL
unavailable. OS policy apply/rollback artifacts remain pending.

2026-06-06 codex-d continuation: `scripts/test/browser-windows-inventory-adapter-completion-proof.mjs`
now closes the Windows inventory adapter checklist row by verifying the live
Windows inventory proof, the Browser-route portal/read-model consumption proof,
and the WP20 AppLocker/App Control state-representation artifact together. The
gate requires known-path, registry uninstall, Start Menu shortcut,
store-package, and running-process inventory evidence; verifies portal labels
for inventory, exact URL capability, active-tab proof, and not-claimed states;
and rejects accidental exact URL, active-tab, broad browser control, AppLocker
prevention, policy mutation, or rollback-execution claims. Real AppLocker/WDAC
policy creation, apply, rollback execution, launch prevention, browser content
capture, and enforcement remain unclaimed product/platform proof gaps.

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

- [ ] Confirm source docs read: [folder README](../README.md), [source index](../source-index.md), [current snapshot](../current-browser-snapshot.md), [full scope plan](../v0-5-managed-browser-full-scope-plan.md), [test blueprint](../v0-5-managed-browser-test-blueprint.md), [UI/UX guide](../ui-ux-requirements-guide.md), [main checklist](../implementation-checklist.md), and this workpack.
- [ ] Check enhancement overlap: URL/video AI intelligence, social platform/account/feed gating, and browser games/cloud gaming.
- [ ] Hub lock covers this workpack and exact implementation/docs paths.
- [ ] Existing source layout inspected; no parallel browser truth created; managed launch identity now reuses the Windows inventory identity path.
- [ ] Before-state source snapshot recorded in `output/browser-plan-proof/04-windows-browser-inventory-adapter/00-source-snapshot.md`.
- [ ] Contracts updated first where this workpack changes behavior.
- [ ] Rust/service parity updated only after contracts exist; portal parity remains deferred because no UI surface changed.
- [ ] Raw evidence artifacts captured or marked N/A for this fixture-backed/root-expansion slice: known-path fixtures, deduplicated candidate roots, packaged path-shape fixtures, process executable-path identity fixtures, duplicate installed/running executable collapse fixtures, caller-provided registry display-icon/install-location values, caller-provided shortcut target values including unquoted command targets with launch arguments and leading environment-variable prefixes, Rust live Windows registry DisplayIcon/InstallLocation enumeration for service default candidates, Rust Start Menu `.lnk` local-target parsing through a binary Shell Link fixture, Windows Store package manifest enumeration as package-ref-only rows, default-root service read-model consumption through temp `PROGRAMFILES` fixture roots, process observations feed the adapter, and the live Windows proof records redacted known-path, registry uninstall, Start Menu shortcut, store-package, running-process, file-hash, and signature-status evidence; no raw paths, raw URLs, page content, journal, SQLite, policy, or action behavior changed.
- [ ] Tests/proof listed in this workpack are implemented for known-path/process fixture parsing, deduplicated root expansion, packaged path-shape classification, process executable-path identity, duplicate installed/running executable collapse, registry display-icon/install-location target normalization, shortcut target normalization including unquoted command target arguments and leading environment-variable prefixes, Rust live registry source ingestion, Rust `.lnk` binary local-target parsing, AppX/MSIX package manifest ingestion, service default-root/live-registry/shortcut/package source consumption, service row conversion, and a live Windows proof harness that captures redacted registry/shortcut/store-package/process/file-hash/signature evidence; OS policy application, UI, and enforcement remain manual-required.
- [ ] Validation command outputs saved in the proof pack and summarized in [main checklist](../implementation-checklist.md).
- [ ] UI snapshots captured for every touched parent portal, child UX, block/warn, policy authoring, or dashboard state; no UI changed, so `ui-not-applicable.md` records why.
- [ ] Security/no-claim negative proof captured for this slice: unmanaged running processes remain process-only, unproved Chromium forks are manual-required, unsupported browsers stay unsupported, and no URL collection path was added.
- [ ] Manual platform proof captured for real browser/OS claims in `output/browser-plan-proof/04-windows-browser-inventory-adapter/09-manual-platform-proof.md`; the proof improves live Windows inventory evidence only and keeps exact URL/tab, browser content, AppLocker/App Control application, blocking, rollback, and enforcement unclaimed.
- [ ] Evidence/proof artifact paths recorded in [main checklist](../implementation-checklist.md).
- [ ] Feature/expectation/product-checklist/README update decision recorded in [main checklist](../implementation-checklist.md).
- [ ] Known gaps, deferred items, and no-claim boundaries recorded before `DONE`.

## Manual-Required Gaps

Detecting a browser does not claim exact URL or app-control blocking.
The browser inventory adapter row is complete with no-claim boundaries after
the completion gate verifies live Windows inventory evidence, portal/read-model
consumption, and AppLocker/App Control state artifacts. Real AppLocker/WDAC
policy creation, apply, rollback execution, launch prevention, browser content
capture, and enforcement remain unclaimed product/platform proof gaps.
