# Reports, Notifications, And Sync

## Parent Outcome

Parents get useful alerts, summaries, history, and reports without Ocentra
becoming the default store for child activity data.

## Ocentra Requirement

Reports cite local or parent-owned evidence. Notifications carry minimal
detail and link to authenticated parent surfaces. Sync/export goes to
parent-selected storage unless a remote path is explicitly authorized.

## Roadmap And Expectations

- Roadmap: V2 remote access, V3 notifications, V4 reports/assistant.
- Expectations: [notifications](../expectations/notifications.md),
  [sync/export](../expectations/sync-export.md),
  [data custody](../expectations/data-custody.md),
  [roadmap V4](../expectations/roadmap-v4-parent-owned-reports-optional-assistant.md).
- Modules: `packages/activity-domain`, `packages/parent-domain`,
  `packages/logging-domain`, `crates/agent-service`.

## Competitor Pressure

See [Competitor Capability Map](../competitor-capability-map.md), especially
notifications/alerts, reports/digests, remote parent access, and local-first
privacy.

Mature competitors provide alerts, weekly reports, activity history, and remote
parent review. Ocentra must match usefulness while preserving local-first data
custody.

## Current Ocentra State

- Activity report persistence/family fanout/MIA context proof is in progress.
- Saved Activity reports are stored as local JSON report documents and
  historical report queries now preserve scope, requested window, parsed report
  metadata, and typed storage fallback states.
- Notification and sync/export expectation docs exist.
- V0.8 integrity alert/status bridge proof now creates minimal notification
  intent/status refs for permission-loss, stale heartbeat, stopped-or-removed,
  and tamper/manual-required enforcement integrity states, with authenticated
  drill-in refs back to audit evidence.
- Parent-facing reports, alert delivery, and connectors are incomplete.

## Current Gap

Need report UI, trend summaries, cited assistant Q&A as a complete product
flow, notification providers, quiet hours, escalation, delivered/failed provider
status, connectors, retention, and delete/export controls. The V0.8 integrity
bridge proves only minimal notification intent/status references and audit
drill-in, not provider delivery or UI.

## Checklist

- [ ] Report contract and storage/custody state.
- [ ] Evidence citations in reports.
- [ ] Parent-owned export/sync connector status.
- [ ] Notification rule contract.
- [x] Minimal payload and authenticated drill-in refs exist for V0.8 integrity
      alert/status bridge states.
- [ ] Delivery/queued/failed provider status.
- [ ] Quiet hours and escalation.
- [ ] Retention/delete controls.

## Next AI Instructions

Do not put sensitive child detail in notification payloads. Do not store reports
in Ocentra-hosted systems by default. Keep report source and custody labels
visible. Treat `scripts/test/v0-8-integrity-alert-status-bridge.mjs` as
notification intent/status proof only; require provider artifacts before claiming
delivery.
