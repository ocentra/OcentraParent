# Source Snapshot

- Branch: `codex/browser-plan-implementation`
- Rebase state: rebased on latest `origin/main` after main advanced to `51afaf8`.
- Before-state gap: managed browser discovery only checked limited Edge/Chrome paths for launch and first unmanaged browser process fallback. There was no Windows inventory observation adapter for known paths/process fixtures.
- Existing source inspected: `crates/agent-core/src/browser_managed_discovery.rs`, `crates/agent-core/src/process_capture.rs`, `crates/agent-service/src/browser_runtime_paths.rs`, `crates/agent-service/src/browser_inventory_read_model.rs`, and browser runtime tests.
- Current validated slice: added `browser_windows_inventory` and `browser_windows_inventory_paths` in `agent-core`, reused that identity from managed discovery, expanded runtime candidate path generation, and added service conversion into browser inventory rows.
- 2026-06-04 continuation: `browser_windows_inventory_paths` now deduplicates
  caller-provided Windows inventory roots before candidate expansion and still
  generates the managed/manual/unsupported browser path families from multiple
  roots. Tests also prove `WindowsApps` paths classify as packaged without
  upgrading exact URL capability.
- 2026-06-04 continuation: `browser_windows_inventory_paths` now normalizes
  caller-provided registry display-icon/install-location values and shortcut
  target strings into known browser executable candidate paths. The existing
  observation adapter remains the only classifier, so registry/shortcut inputs
  still cannot claim exact URL, page title, content, account, blocking, or live
  OS enumeration.
- 2026-06-04 continuation: unquoted caller-provided registry/shortcut command
  targets that include known browser executables plus trailing launch arguments
  now trim back to the executable path before classification. The adapter still
  requires the executable path to exist before emitting an observation and still
  makes no URL/title/content/default-profile claim.
- 2026-06-04 continuation: caller-provided registry/shortcut command targets
  with a leading Windows environment-variable segment now expand that prefix
  before the same known-executable filter runs. The focused fixture uses an
  environment-rooted Chrome target with launch arguments and proves the
  resolved executable row remains installed/not-running with exact URL
  unavailable.
- 2026-06-04 continuation: service inventory read-model default-root
  consumption now runs through `crates/agent-service/src/activity_api.rs` and
  `crates/agent-service/src/browser_runtime_paths.rs`. The service scan feeds
  default Windows candidate paths into the existing Windows inventory adapter
  before process observations; focused service proof uses a temp
  `PROGRAMFILES` root and keeps exact URL unavailable.
- 2026-06-04 continuation on branch
  `codex/browser-windows-registry-start-menu-proof`: added
  `crates/agent-core/src/browser_windows_inventory_sources.rs` for bounded live
  Windows Uninstall registry source collection and bounded Start Menu shortcut
  target extraction. `crates/agent-service/src/browser_runtime_paths.rs` now
  feeds those live source candidates into the same browser inventory adapter
  before process observations. Focused proof covers UTF-16 `.lnk` target
  extraction, host-aware service rows when live registry rows are present, and
  claim-boundary honesty for every returned row.
- Not claimed: full shell `.lnk` parsing, live AppX/MSIX enumeration,
  signature/hash refs, product-complete Windows manual inventory capture,
  portal display, URL visibility, or app-control blocking.
