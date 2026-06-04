# Portal App

Vite development portal for local and LAN parent visibility.

This is the fast HMR surface for proving parent workflows against the real Rust
service. It is not allowed to become the source of truth for child-device
capture, AI safety decisions, policy evaluation, timers, or enforcement.

The product parent portal should become a packaged parent-owned desktop/mobile
surface. Tauri is the preferred desktop-shell candidate unless a later
architecture decision replaces it. `family.ocentra.ca` is for downloads, account,
subscription, docs, status, and optional stateless report compilation, not for
default custody of child activity data.

```powershell
cmd /c npm run dev:agent
cmd /c npm run dev:portal
```

For cross-device LAN testing, run from the repo root:

```powershell
cmd /c npm run dev:lan
```

Run the real browser UI check from the repo root:

```powershell
cmd /c npm run playwright:install
cmd /c npm run test:e2e --workspace @ocentra-parent/portal
```

The Playwright check starts the Rust agent and Vite portal on the scaffold smoke ports, verifies WebSocket connection state, clicks command buttons, checks rendered event output, and fails on browser console or page errors.

## Ownership

- Renders parent-facing surfaces for devices, activity, policy, data, AI, and
  account areas.
- Sends typed intents and queries through `@ocentra-parent/agent-protocol-domain`.
- Displays service-backed capability status, custody labels, evidence refs, and
  degraded states.
- Displays the read-only Screen settings/capability catalog proof on the
  Settings route while leaving writable opt-in and retention controls to the
  product settings flow.
- Displays LAN source-matrix diagnostics from the service-backed add-device read
  model so workpack/source proof status is visible in Devices/LAN and
  Activity/Network review.
- Displays the tracking service read-model event as a narrow Policy Tracking
  route summary; this is not full parent/child tracking UI proof.
- Provides Playwright proof for real portal-to-Rust behavior.

## Must Not Own

- Runtime policy evaluation.
- Local AI model execution.
- Timer recovery.
- OS enforcement adapters.
- Child-device capture.
- Unvalidated fake "normal" data paths.

## Connected Docs

- [Portal expectations](../../docs/expectations/portal.md)
- [Real evidence proof](../../docs/expectations/real-evidence-proof.md)
- [Product capability checklist](../../docs/product-capability-checklist.md)

## Gaps To Fill

- Complete first-run setup, child profiles, policy authoring, schedules,
  reports, notifications, and AI action previews.
- Keep replacing UI-check data with service-backed read models.
- Make every route label live/local/LAN/relay/cache/unavailable source state
  clearly.
- Keep LAN source-matrix labels tied to service read models; do not add
  portal-only completion claims for unimplemented discovery adapters.
