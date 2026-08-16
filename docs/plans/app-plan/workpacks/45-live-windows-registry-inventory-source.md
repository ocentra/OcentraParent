# WP45 - Live Windows Registry Inventory Source

## Scope

Cross-record the shared app/game Windows registry inventory source for the
native app plan.

This workpack proves that Windows Uninstall registry keys can produce native
app/game inventory-only rows with opaque source/path refs, while preserving the
same no-use and no-enforcement boundaries as the shared app/game plan.

It does not add service capture, portal UI, source freshness rows, policy
consumption, adapter execution, broad blocking, or platform support claims.

## Implementation

- Reuse the shared `agent-core` Windows registry source.
- Read `HKLM` and `HKCU` Uninstall roots on Windows through `winreg`.
- Exercise `.reg` export fixture parsing in focused core tests.
- Hash registry keys and install/uninstall path values into source/path refs.
- Filter hidden system-component entries.

## Proof

- `cargo test -p ocentra-parent-agent-core registry_inventory`
- `cargo fmt --all --check`
- `cmd /c npm run lint:schema-boundaries`
- `git diff --check`
- `cmd /c npm run lanes:guard`
- `cmd /c npm run hub:guard`

Proof artifacts live in:

```text
output/app-plan-proof/45-live-windows-registry-inventory-source
```

## No-Claim Boundaries

- Registry evidence is native app/game inventory evidence only.
- The source does not prove runtime use, foreground use, service capture, portal
  UI, policy decisions, adapter execution, broad app blocking, or platform
  support.
- Raw registry keys and install/uninstall paths remain hashed before becoming
  source/path refs.

## Product Doc Decision

`docs/product-capability-checklist.md` is intentionally unchanged. WP45 moves
core native app evidence forward, but product status should not move until
service capture, portal freshness, policy consumption, and adapter boundaries
are finished.
