# Ocentra Parent

Ocentra Parent is a family-safety product from Ocentra, intended to live at `family.ocentra.ca`.

The product exists for a real parent problem: children live inside browsers, games, chats, short-form video, school apps, and social feeds, while parents often only see the outside story. A child can look like they are studying while bouncing between TikTok, Snapchat, Discord, games, adult content, or other distracting and unsafe parts of the internet. Parents need a way to understand what is happening, set sane boundaries, give permissions, enforce timeouts, and get alerted when something needs attention.

The goal is not to start with a flashy dashboard or vague AI promises. The first job is to build a trustworthy local recorder: a headless agent on the child device that can observe useful activity signals, normalize them into strict schemas, store raw evidence safely, and make that evidence queryable. Once that recorder is honest, policy, blocking, alerts, and AI-assisted guidance can be built on top of facts instead of guesses.

## What We Are Solving

Parents should not have to choose between blind trust and invasive guessing. Ocentra Parent aims to make family device safety practical by answering basic questions clearly:

- What apps and sites are active on the child device?
- Is the child spending time on school work, games, social media, video platforms, or adult content?
- Which domains and apps are consuming time and bandwidth?
- When should the parent permit, limit, timeout, or block an activity?
- What happened before an alert or policy decision?
- Can the system explain its evidence instead of producing a magic AI verdict?

The long-term product is an agentic safety system: local device agents gather evidence, parent portals expose control and visibility, and AI helps classify, explain, and recommend action. The v0 foundation is intentionally simpler: capture trustworthy events first.

## Architecture Direction

Ocentra Parent has two main product surfaces:

- Child devices run a headless local agent.
- Parents use a web/mobile control surface.

The first proof is Windows-focused and local-first. The Rust service hosts local development endpoints so the portal can command and observe the agent on the same machine or LAN. The repository now keeps real package scaffolds for Windows, Linux, macOS, Android, and iOS so platform build breakage shows up early, while feature implementation still lands honestly one platform at a time.

The core data pipeline is:

```text
capture -> NDJSON journal -> ingester -> SQLite query store -> portal/reports/policy
```

NDJSON is the append-only source of truth. It is easy to inspect, replay, rotate, and recover from. SQLite is the default cross-platform query/index layer for time windows, joins, summaries, and reports. The hot capture path should stay resilient and boring; analysis and policy should happen after events are safely written.

## V0 Milestone

The first real milestone is a Windows network/activity recorder.

Definition of done:

- Runs as a Windows background service.
- Starts through a normal MSI installer.
- Emits one schema-versioned event per observation.
- Writes append-only NDJSON with safe flushing and rotation.
- Ingests events into SQLite for local queries.
- Exposes a minimal local/LAN portal for visibility.
- Can summarize top processes, domains, time windows, and suspicious unknowns.
- Does no blocking, no AI classification, and no content inspection yet.

The event model is intent-first, not packet-first. We care about normalized activity such as `chrome.exe connected to youtube.com:443`, not raw TCP packets or decrypted HTTPS payloads.

## Current Repository State

This repository is currently in scaffold-first mode. The committed foundation includes workspace layout, domain boundaries, validation gates, test structure, Rust crate boundaries, local and LAN dev APIs, a minimal Vite portal, MSI release packaging, package-preview scaffolds for every target platform, signed updater scaffolding, dependency/security gates, and SBOM generation.

Not implemented yet:

- Windows Filtering Platform capture.
- Activity classification.
- Blocking or enforcement.
- Parent policy UI.
- Cloud sync.
- Notification delivery.
- AI guidance.
- Browser extension URL context.
- Mobile agents.
- Production mobile store distribution.

## Engineering Principles

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
- SQLite is the default local query store.
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
  validation, dependency policy, platform packaging, smoke, and git hook guardrails.
platforms/
  android/      Android APK scaffold with a foreground agent service.
  ios/          iOS simulator app scaffold with an Xcode project.
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

Source pushes to `main` run the CI gate and build package-preview artifacts for Windows, Linux, macOS, Android, and iOS simulator, but they do not publish GitHub Releases. README-only, Markdown-only, and `docs/**` pushes are ignored by CI.

Production releases happen from the `production` branch only. After `main` is green and the version is intentionally bumped, pushing/merging to `production` builds the signed Windows release, creates tag `v<version>`, and publishes GitHub Release assets. If the version tag already exists, the production workflow still runs its gates and package previews, then skips publishing. Package previews for Linux, macOS, Android, and iOS stay as CI artifacts until their signing/store/update paths are deliberately promoted.

Package previews are not just archive builds. CI now performs install or launch smoke checks for each scaffolded platform: MSI install/uninstall on Windows, DEB install/remove on Linux, PKG payload validation on macOS, APK install/launch in Android emulator, and app install/launch in iOS simulator.

Once a release exists, install on another Windows PC from an elevated PowerShell session:

Latest Windows MSI download:

https://github.com/ocentra/OcentraParent/releases/latest/download/ocentra-parent-agent-windows-x64-latest.msi

That URL is intended for a future `family.ocentra.ca` download button. A browser click downloads the MSI; Windows still requires the parent to open it and approve the installer.

Support/admin one-line install:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -Command "irm https://github.com/ocentra/OcentraParent/releases/latest/download/install-ocentra-parent-agent-windows.ps1 | iex"
```

See `docs/architecture/release-update.md` for the updater boundary. The MSI installs the headless service under `%ProgramFiles%\Ocentra\Ocentra Parent Agent`, registers it as `OcentraParentAgent`, starts it on install, and gives Windows a normal uninstall/upgrade entry. It also installs `OcentraParentUpdater`, a separate signed-manifest updater service that checks GitHub Release metadata and runs quiet MSI upgrades.

The production Windows update manifest requires `OCENTRA_PARENT_UPDATE_SIGNING_KEY_BASE64`. Future platform signing secrets for Authenticode, macOS, Android store, and Apple store distribution are documented in the repo but are not required until those release paths are implemented.

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
