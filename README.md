# Ocentra Parent

Ocentra Parent is the family-safety product line for Ocentra, intended to live at `family.ocentra.ca`.

This repository is currently in scaffold-first mode. The first milestone is infrastructure and visibility only: workspace layout, domain boundaries, validation gates, test structure, Rust crate boundaries, local and LAN dev APIs, a minimal Vite portal, and documentation. No Windows capture logic, policy engine, product portal UI, AI classification, blocking, notification delivery, or cloud runtime is implemented yet.

## Product Direction

- Child devices run a headless local agent.
- Parents use a web/mobile control surface.
- The first proof is Windows-focused and local-first.
- The Rust service hosts local development endpoints so the portal can command and observe the agent on the same machine or LAN.
- Cloudflare, remote sync, notifications, AI, and enforcement come after the recorder foundation is measured.

## Scaffold Principles

- Domain packages own shared contracts.
- Runtime apps and Rust crates consume contracts instead of inventing local strings.
- Effect Schema is the TypeScript validation standard.
- Branded strings must come from schema decoders.
- Runtime source does not own inline string literals; domains own text, ids, routes, commands, events, fields, and protocol names.
- Runtime app TypeScript does not annotate values as raw `string`; app code receives branded domain values or parses external `unknown` input at the boundary.
- Source files and functions have shape budgets. Validation warns at 80% and fails when a module becomes too large.
- Test doubles are forbidden. Tests must exercise real contracts, parsers, localhost services, and transport boundaries.
- Tests are required for every source workspace and Rust crate from the start.
- Rust service execution is async and Tokio multithreaded by default.
- NDJSON is the append-only evidence journal.
- DuckDB is the local query warehouse.
- Tests and validation gates are part of the scaffold, not an afterthought.

## Current Shape

```text
apps/
  portal/        Minimal Vite dev portal for local and LAN agent visibility.
  local-api/     Reserved local query/control API package placeholder.
packages/
  schema-domain/     Shared Effect Schema helpers.
  endpoint-domain/   Endpoint/path/header brand boundaries.
  agent-protocol-domain/ WebSocket command/event contracts.
  text-domain/       Schema-backed display text tokens.
  portal-domain/     Portal route, DOM, and dev command contracts.
  parent-domain/     Parent product contract placeholder.
  activity-domain/   Device activity contract placeholder.
  logging-domain/    Effect Schema operational logging contracts.
crates/
  agent-core/      Agent core placeholder.
  agent-protocol/  Rust protocol structs matching shared contracts.
  agent-service/   Rust local API smoke service.
scripts/
  validation and git hook guardrails.
docs/
  architecture and decisions.
```

## Commands

```powershell
npm install
npm run hooks:install
npm run validate
```

Use `cmd /c npm ...` on Windows if PowerShell execution policy blocks npm shims.

## Release Scaffold

Releases are part of the scaffold because the Windows agent needs a repeatable install/update path from the beginning.

```powershell
cmd /c npm run release:version
cmd /c npm run release:package:windows
```

Pushes to `main` run the CI gate first. If validation and build pass, the release job builds the Windows x64 MSI, creates tag `v<version>`, and publishes GitHub Release assets.

Once a release exists, install on another Windows PC from an elevated PowerShell session:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -Command "irm https://github.com/SujanMishra/OcentraParent/releases/latest/download/install-ocentra-parent-agent-windows.ps1 | iex"
```

See `docs/architecture/release-update.md` for the updater boundary. The MSI installs the headless service under `%ProgramFiles%\Ocentra\Ocentra Parent Agent`, registers it as `OcentraParentAgent`, starts it on install, and gives Windows a normal uninstall/upgrade entry. The automatic update helper is still the next implementation slice.

## Local Dev Loop

Run the loopback-only stack:

```powershell
cmd /c npm run dev
```

Or run the pieces separately:

```powershell
cmd /c npm run dev:agent
cmd /c npm run dev:portal
```

The portal connects to `ws://127.0.0.1:4477/api/dev/ws`, sends typed command envelopes from `@ocentra-parent/agent-protocol-domain`, and validates returned events through Effect Schema. The Rust service still exposes `http://127.0.0.1:4477/api/dev/log-snapshot` as a plain HTTP smoke endpoint.

Local development uses fixed Ocentra Parent ports:

- Rust agent service: `127.0.0.1:4477`
- Vite portal: `127.0.0.1:4478`

Use `npm run dev`, `npm run dev:agent`, or `npm run dev:portal` so `scripts/dev/*` can reclaim only stale Ocentra Parent processes. Do not run the portal on generic Vite ports like `5173` or the Ocentra Games asset-editor port `5174`.

## LAN Dev Loop

Run the same scaffold over your local network:

```powershell
cmd /c npm run dev:lan
```

LAN mode keeps the same ports but binds both dev surfaces to the network:

- Rust agent service bind: `0.0.0.0:4477`
- Vite portal bind: `0.0.0.0:4478`
- Portal URL from another device: `http://<this-pc-lan-ip>:4478/#/commands`
- Agent WebSocket URL from the portal: `ws://<this-pc-lan-ip>:4477/api/dev/ws`

The managed scripts auto-detect the first non-internal IPv4 address. If Windows has multiple active network adapters, set the host explicitly:

```powershell
$env:OCENTRA_PARENT_LAN_HOST = "192.168.1.25"
cmd /c npm run dev:lan
```

LAN mode is explicit because it exposes the agent to other devices on the network. The Rust service refuses non-loopback binds unless `OCENTRA_PARENT_AGENT_LOCAL_NETWORK_ENABLED=true`, and browser origins are restricted through `OCENTRA_PARENT_AGENT_ALLOWED_ORIGINS`.
