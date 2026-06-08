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
- Displays the Screen settings/capability catalog proof and local writable
  screen-summary intent draft proof on the Settings route while leaving
  child-agent persistence and runtime setting application to a later service
  path.
- Displays LAN source-matrix diagnostics from the service-backed add-device read
  model so workpack/source proof status is visible in Devices/LAN and
  Activity/Network review.
- Displays the tracking service read-model event, hosted service-backed citation
  detail proof, hosted child-safe check-in proof, hosted child-runtime UI
  disclosure/consent proof, and
  unsupported/manual platform rendered states as narrow Policy Tracking route
  cards; this is not actual child-device delivery/runtime execution,
  physical-device execution, authority enrollment, provider delivery, or full
  parent/child tracking UI proof.
- Displays app/game source freshness counts and source-kind evidence summaries
  from the service-backed App/Game Sessions read model without adding policy or
  adapter claims.
- Exposes grouped App/Game Sessions source-panel intent sections for existing
  service-backed source rows; SVG rendering remains a follow-up while the
  portal surface is owned by another lane.
- Sends the app/game timer parent-surface parent preference setup request
  command from request-ready setup cards and shows the accepted service event
  through the command-result lane, without claiming durable preference
  mutation, notification rule writes, delivery, outbox runtime, adapter
  dispatch, or platform enforcement.
- Renders parent-safe action-result persistence, mutation receipt,
  child-runtime handoff, service-local child-runtime queue refs/status, and
  service-local child-runtime dispatch refs/status, and service-local
  child-runtime receipt-required refs/status for accepted app/game parent
  preference setup command results while keeping actual child delivery,
  provider delivery, receipt ingestion, durable outbox runtime, adapter
  dispatch, broad blocking,
  platform enforcement, raw target values, and private diagnostics unclaimed.
- Displays an App/Game Sessions route overlay for app/game notification
  parent-surface rows by projecting the live service notification-readiness
  event into manual/unavailable setup rows, and otherwise shows the missing
  service event without claiming provider delivery, preference mutation, child
  delivery, scheduler/outbox runtime, or adapter dispatch.
- Displays service-backed app/game policy readiness rows on App/Game Sessions
  without adding policy execution, persistence, adapter dispatch, or broad
  blocking claims.
- Displays Activity network evidence drawer platform/capability state,
  active/tombstone/exportable row counts, retention delete refs, and degraded
  adapter state from the service-backed network read model without adding local
  risk scoring, policy evaluation, adapter dispatch, or enforcement claims.
- Displays the Browser-route social dashboard shell for the current
  unavailable zero-row state while service-backed social snapshots, connector
  runtime, native app control, final policy execution, notifications, and
  enforcement remain unclaimed.
- Displays the Browser-route social audit explanation proof panel when a
  dedicated proof env bundle is supplied. The panel renders only schema-decoded
  SOCIAL-22 explanation snapshots and keeps service-backed explanation
  delivery, notification delivery, connector authorization, native app control,
  final policy execution, and enforcement unclaimed.
- Displays the Browser-route parent explanation proof panel when a dedicated
  proof env bundle is supplied. The panel renders only schema-decoded browser AI
  parent explanation bundles and keeps runtime service delivery, final policy
  authority, browser mutation, remote AI, raw page/prompt custody, and
  enforcement unclaimed.
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
- Render the dedicated App/Game Sessions source panel from the new intent seam,
  without promoting adapter or policy claims.
- Replace the Browser-route social dashboard unavailable shell with
  service-backed social rows only after the runtime snapshot path exists.
- Add live policy evaluator, authoring UI, persistence, notification/child UX,
  and platform adapter proof after readiness rendering.
