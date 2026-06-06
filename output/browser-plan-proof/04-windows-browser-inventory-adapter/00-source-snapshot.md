# WP04 Live Windows Inventory Source Snapshot

Branch: `codex/browser-child-intervention-endpoint-flow`

Base before edits: `origin/main` at `d9a96339`

Before-state gap: WP04 had fixture-backed Windows inventory proof for known
paths, process rows, caller-provided registry targets, caller-provided shortcut
targets, default-root service row conversion, and a Node live Windows proof, but
the Rust browser inventory service path did not enumerate live Windows registry
install entries before deriving service default candidate paths.

Source paths inspected:

- `docs/plans/browser-plan/README.md`
- `docs/plans/browser-plan/source-index.md`
- `docs/plans/browser-plan/implementation-checklist.md`
- `docs/plans/browser-plan/workpacks/04-windows-browser-inventory-adapter.md`
- `docs/features/browser-web-control.md`
- `crates/agent-core/src/browser_windows_inventory.rs`
- `crates/agent-core/src/browser_windows_inventory_paths.rs`
- `crates/agent-core/src/browser_windows_inventory_source.rs`
- `crates/agent-core/src/browser_windows_inventory_tests.rs`
- `crates/agent-core/src/browser_windows_inventory_source_tests.rs`
- `crates/agent-service/src/browser_runtime_paths.rs`
- `crates/agent-service/src/browser_inventory_read_model_tests.rs`
- `scripts/test/browser-platform-inventory-matrix-proof.mjs`
- `scripts/test/browser-windows-live-inventory-proof.mjs`

Existing behavior: the Rust adapter already classifies supported managed
Chromium candidates, manual-required Chromium forks, unsupported browsers, and
running browser processes without URL claims. The live proof harness records
live Windows input evidence for that adapter boundary and stores only redacted
refs, file hashes, signature status, source counts, and capability labels.

2026-06-06 refresh: the Rust service default browser candidate helper now uses
`live_windows_browser_inventory_candidate_paths_with_limit`, which reads live
Windows uninstall registry DisplayIcon and InstallLocation entries on Windows,
feeds them through the existing browser-owned path normalization, and then uses
the existing browser inventory observation/read-model conversion. Focused Rust
tests cover fixture-backed registry display-icon/install-location candidates,
shortcut-target ingress through the same helper, and service default rows while
allowing additional live host registry rows.

No-claim boundary: this source snapshot does not claim Rust `.lnk` binary
parsing, live AppX/MSIX enumeration, exact URL/tab evidence, page content
capture, AppLocker/App Control application, blocking, rollback, or enforcement.
