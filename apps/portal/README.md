# Portal App

Dev portal for local and LAN parent visibility.

This surface is presentation only. It consumes Rust-owned snapshots and actions
through the HostBridge and the generated/thin parent UI bridge. It is not allowed to
become the source of truth for child-device capture, AI safety decisions,
policy evaluation, timers, enforcement, or product contracts.

## Ownership

- Renders parent-facing surfaces for devices, activity, policy, data, AI, and
  account areas.
- Consumes Rust-owned snapshots and actions through the HostBridge and the
  generated parent UI bridge.
- Displays service-backed capability status, custody labels, evidence refs, and
  degraded states.
- Displays service-backed app/game policy readiness rows on App/Game Sessions.
- Provides Playwright proof for real portal-to-Rust behavior.

## Must Not Own

- Runtime policy evaluation.
- Local AI model execution.
- Timer recovery.
- OS enforcement adapters.
- Child-device capture.
- Unvalidated fake normal data paths.

## Connected Docs

- [Portal expectations](../../docs/expectations/portal.md)
- [Real evidence proof](../../docs/expectations/real-evidence-proof.md)
- [Product capability checklist](../../docs/product-capability-checklist.md)

## Gaps To Fill

- Keep replacing UI-check data with Rust-backed snapshots and actions.
- Make every route label live/local/LAN/relay/cache/unavailable source state
  clearly.
- Keep LAN source-matrix labels tied to service read models.
- Render the dedicated App/Game Sessions source panel from the Rust-backed
  intent seam.
- Replace the Browser-route social dashboard unavailable shell with
  service-backed social rows only after the runtime snapshot path exists.
