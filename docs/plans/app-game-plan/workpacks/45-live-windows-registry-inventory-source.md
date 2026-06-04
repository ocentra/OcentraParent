# WP45 - Live Windows Registry Inventory Source

## Scope

Add a bounded core source for Windows installed-app registry evidence.

This workpack proves that Windows Uninstall registry keys can be read through the
Windows registry API, mapped into app/game inventory rows, and converted into
journal events without exposing raw registry keys or install paths.

It does not add service capture, portal UI, source freshness rows, policy
consumption, adapter execution, broad blocking, or platform support claims.

## Implementation

- Add a bounded Windows registry source module in `agent-core`.
- Read `HKLM` and `HKCU` Uninstall roots on Windows through `winreg`.
- Exercise the same parser path with `.reg` export fixtures in tests.
- Hash registry keys and install/uninstall path values into source/path refs
  before rows enter the journal boundary.
- Filter hidden system-component entries so OS internals are not promoted as
  parent-visible inventory.
- Keep default live source optional on unsupported platforms and
  permission-limited hosts.

## Proof

- `cargo test -p ocentra-parent-agent-core registry_inventory`
- `cargo fmt --all --check`
- `cmd /c npm run lint:schema-boundaries`
- `git diff --check`
- `cmd /c npm run lanes:guard`
- `cmd /c npm run hub:guard`

Proof artifacts live in:

```text
output/app-game-plan-proof/45-live-windows-registry-inventory-source
```

## No-Claim Boundaries

- Registry evidence is inventory evidence only.
- The source does not prove runtime use, foreground use, content knowledge,
  service capture, portal UI, policy decisions, adapter execution, broad app
  blocking, or platform support.
- Raw registry keys and install/uninstall paths remain hashed before becoming
  source/path refs.

## Product Doc Decision

`docs/product-capability-checklist.md` is intentionally unchanged. WP45 moves
core Windows installed-app evidence forward, but product status should not move
until service capture, portal freshness, policy consumption, and adapter
boundaries are finished.
