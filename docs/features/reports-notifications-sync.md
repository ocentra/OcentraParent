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

- Activity report persistence/family fanout/MIA context proof exists for the
  backend/read-model boundary.
- Saved Activity reports are stored as local JSON report documents and
  historical report queries now preserve scope, requested window, parsed report
  metadata, typed storage fallback states, parent-local custody/source labels,
  and `rawChildEvidenceIncluded=false`.
- Family Activity report aggregation preserves ready/offline/stale/unavailable,
  unreachable, and error source states for service-owned UI handoff without
  making Vite or portal UI the product-data owner.
- The Activity UI intent layer consumes service-backed report, history, and tab
  read-model adapter results for Reports, Screen, App Use, Browser, Games, and
  Network. Failed or absent adapter results stay explicit instead of creating
  fixture-backed devices.
- Parent Assistant/MIA evidence context can cite saved Activity report history
  with parent-owned report custody and `directEnforcementAllowed=false`; it is
  citation context only.
- Notification and sync/export expectation docs exist.
- V0.8 integrity alert/status bridge proof now creates minimal notification
  intent/status refs for permission-loss, stale heartbeat, stopped-or-removed,
  and tamper/manual-required enforcement integrity states, with authenticated
  drill-in refs back to audit evidence.
- V0.8 notification provider status boundary proof now represents queued,
  delivered, failed, unavailable, and manual-required provider status contract
  states, plus quiet-hours and escalation readiness refs, through the existing
  supported-adapter runtime proof event payload.
- Parent-facing reports, alert delivery, and connectors are incomplete.

## Current Gap

Need report UI polish, trend summaries, evidence citations in report sections,
cited assistant Q&A as a complete product flow, real notification provider
adapters, retry execution, provider receipt ingestion, parent controls,
connectors, retention, and delete/export controls.
The V0.8 integrity bridge and provider status boundary prove only minimal
notification intent/status/readiness references and audit drill-in, not provider
delivery or UI. Activity report persistence/family fanout/MIA context proof does
not claim physical household fanout, raw child evidence transfer, provider
notification delivery, policy writes, or child-device enforcement.

## Checklist

- [x] Report contract and storage/custody state.
- [x] Activity service adapter feeds the UI intent seam for reports and tab
      read-model states without Vite-owned product data.
- [ ] Evidence citations in reports.
- [ ] Parent-owned export/sync connector status.
- [ ] Notification rule contract.
- [x] Minimal payload and authenticated drill-in refs exist for V0.8 integrity
      alert/status bridge states.
- [x] Delivery/queued/failed/unavailable/manual-required provider status
      read-model proof exists without provider delivery claims.
- [x] Quiet-hours and escalation readiness read-model proof exists without
      parent controls or provider delivery.
- [ ] Retention/delete controls.

## Next AI Instructions

Do not put sensitive child detail in notification payloads. Do not store reports
in Ocentra-hosted systems by default. Keep report source and custody labels
visible. Treat `scripts/test/v0-8-integrity-alert-status-bridge.mjs` as
notification intent/status proof only; require provider artifacts before claiming
delivery. Treat `scripts/test/v0-8-notification-provider-status-boundary.mjs`
as provider status/readiness proof only; require provider adapter, receipt,
retry, and parent-control artifacts before claiming notification delivery.
