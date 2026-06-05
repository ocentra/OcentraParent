# WP04 Live Windows Inventory Source Snapshot

Branch: `codex/browser-windows-inventory-live-proof`

Base before edits: `origin/main` at `62dee64f35f8080281c90fdd9eac6c681aa28b83`

Before-state gap: WP04 had fixture-backed Windows inventory proof for
known-path, process, registry-target, shortcut-target, default-root, and service
row conversion behavior, but its manual platform proof still recorded live
Windows inventory evidence as pending.

Source paths inspected:

- `docs/plans/browser-plan/README.md`
- `docs/plans/browser-plan/source-index.md`
- `docs/plans/browser-plan/implementation-checklist.md`
- `docs/plans/browser-plan/workpacks/04-windows-browser-inventory-adapter.md`
- `docs/features/browser-web-control.md`
- `crates/agent-core/src/browser_windows_inventory.rs`
- `crates/agent-core/src/browser_windows_inventory_paths.rs`
- `crates/agent-core/src/browser_windows_inventory_tests.rs`
- `scripts/test/browser-platform-inventory-matrix-proof.mjs`

Existing behavior: the Rust adapter already classifies supported managed
Chromium candidates, manual-required Chromium forks, unsupported browsers, and
running browser processes without URL claims. The new proof harness records live
Windows input evidence for that existing adapter boundary and stores only
redacted refs, file hashes, signature status, source counts, and capability
labels.

No-claim boundary: this source snapshot does not claim live Rust registry
enumeration, `.lnk` parsing, AppX/MSIX enumeration, exact URL/tab evidence,
page content capture, AppLocker/App Control application, blocking, rollback, or
enforcement.
