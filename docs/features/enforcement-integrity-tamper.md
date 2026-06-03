# Enforcement, Integrity, And Tamper

## Parent Outcome

Parents can see what was blocked, warned, time-limited, allowed, or sent for
approval, and whether the child-device agent is installed, running, degraded,
stopped, stale, permission-limited, or removed.

## Ocentra Requirement

Enforcement changes device behavior and requires a higher bar: typed policy
decision, adapter capability status, audit, rollback, timer recovery, and
parent-visible failure state. Integrity/tamper protection must be honest and
non-stealth.

## Roadmap And Expectations

- Roadmap: V0.8 enforcement adapters, V5 policy product, V6 mobile agents.
- Expectations: [enforcement](../expectations/enforcement.md),
  [tamper/uninstall](../expectations/tamper-uninstall-protection.md),
  [platforms](../expectations/platforms.md).
- Modules: `packages/parent-domain`, `packages/logging-domain`,
  `crates/agent-core`, `crates/agent-service`, platform folders.

## Competitor Pressure

See [Competitor Capability Map](../competitor-capability-map.md), especially
app block/app limits, web filtering/categories, screen time schedules, and
tamper/uninstall resistance.

Competitors can block apps/sites, enforce screen time, and sometimes resist or
report removal through platform controls. Ocentra must close enforcement parity
without fake anti-tamper claims.

## Current Ocentra State

- Enforcement contracts, audit, rollback, capability status, timer/recovery,
  and owned-process time-limit proof exist in progress form.
- Browser/domain adapter proof now preserves exact surface states for
  managed-session intervention, unmanaged process-only fallback, and
  manual-required or unavailable browser/domain gaps.
- Product-control spine contracts now expose parent-visible action states for
  observe, warn, time-limit, scoped process block, ask-parent, dry-run preview,
  and report-only surfaces without upgrading broad app, network/domain, exact
  URL, notification, or tamper claims.
- Product-control runtime proof now wires the merged spine into a Rust service
  WebSocket read model and typed agent-protocol adapter that link
  cross-platform, browser/domain, and OS-adapter proof sources while preserving
  manual-required and not-claimed states.
- Policy-dispatch proof now adds a schema-backed dispatch read model and
  WebSocket event for parent-authored intents, service validation, adapter
  capability matrix state, app/game evidence handoff, timer recovery,
  approval/override audit refs, child reason codes, and deterministic proof JSON.
- Broad-adapter proof now adds a schema-backed ten-entry runtime read model,
  WebSocket command/event, TypeScript protocol adapter, and proof JSON that
  separates implemented-boundary, manual-required, unavailable, and not-claimed
  outcomes without upgrading broad app/browser/domain or mobile privilege claims.
- Supported-adapter runtime proof now adds a schema-backed ten-entry read model,
  WebSocket command/event, TypeScript protocol adapter, and proof JSON that
  distinguishes implemented-boundary, manual-required, unavailable,
  not-claimed, unsupported, and degraded states without upgrading broad app,
  network/domain, exact active-tab, notification, tamper, mobile, or unsupported
  platform behavior claims.
- Enforcement integrity runtime audit now adds a schema-backed 14-entry read
  model, Rust protocol parity, service event payload, TypeScript protocol
  adapter parsing, and proof harness for supported action results, dry-run,
  observe-only, stale/wrong-device rejection, manual-required, unavailable,
  recovery-needed, unsupported, permission-loss, heartbeat, rollback,
  parent-override, child-status, and tamper/manual states.
- Integrity alert/status bridge proof now nests a schema-backed four-entry read
  model inside the integrity runtime audit payload for permission-loss, stale
  heartbeat, stopped-or-removed, and tamper/manual-required parent-visible
  states. It carries notification intent/status refs, audit refs, integrity refs,
  and drill-in refs while preserving provider-delivery, anti-tamper, broad
  blocking, mobile enforcement, stealth, and privilege-escalation non-claims.
- Tamper integrity audit logging proof now adds a logging-domain contract and
  proof JSON for stale/offline heartbeat, permission loss, stopped service,
  removed agent, uninstall detection, tamper/manual-required state, and
  documented admin-removal flow refs. It limits payloads to redaction-safe
  operational refs and status fields while preserving no stealth, no privilege
  escalation, no provider delivery, no admin-removal blocking, and no raw child
  data/evidence claims.
- Tamper uninstall artifact status proof now adds a parent-domain read model and
  proof JSON for desktop service/package removal artifacts, Android package and
  device-owner/managed-profile artifacts, iOS Family Controls/DeviceActivity
  artifacts, and documented admin-removal flow refs. It keeps desktop artifacts
  manual-required, mobile artifacts device-proof-required, and admin removal
  documented-only without claiming uninstall detection, anti-tamper behavior,
  provider delivery, privilege escalation, stealth persistence, admin-removal
  blocking, or raw child data custody.
- Broad app/browser/domain/network enforcement is not product-complete.
- Tamper/uninstall protection is a tracked gap.

## Current Gap

Broad adapters, finished child-facing messages, delivered notifications,
uninstall detection artifacts, and platform-specific proof remain. The integrity
runtime audit and alert/status bridge make permission loss, stale heartbeat,
stopped-or-removed, and tamper/manual-required states parent-visible in
contracts/protocol/service proof, but permission restoration, fresh heartbeat
delivery, uninstall detection artifacts, service restart timer persistence, and
anti-tamper behavior are still manual-required or unavailable. Tamper/uninstall
is represented as manual-required/rejected state, not as anti-tamper behavior.
The broad-adapter, supported-adapter, integrity audit, and alert/status bridge
proofs plus the logging-domain tamper integrity audit proof and parent-domain
tamper uninstall artifact status proof give service/logging/product visibility
into adapter gaps but do not prove anti-tamper behavior, real uninstall artifact
capture, admin-removal blocking, or notification provider delivery.

## Checklist

- [x] Typed enforcement intent/action/result/audit read-model proof for V0.8
      supported, rejected, unavailable, manual-required, and unsupported states.
- [ ] Adapter capability status.
- [ ] Timer create/expire/recover/rollback is represented in audit proof; real
      restart persistence beyond recovery-needed still needs proof.
- [ ] Child-facing status is referenced in audit proof; finished child UX still
      needs implementation.
- [ ] Parent override is represented as auditable intent refs; finished approval
      UX remains.
- [x] Integrity heartbeat is represented as running/stale state with
      parent-visible alert/status refs; real provider delivery and freshness
      proof remain.
- [x] Permission-loss, stopped/removed, and uninstall/tamper alerts are explicit
      parent-visible states with notification intent/status and audit refs; real
      permission restoration, uninstall detection, and delivery proof remain.
- [x] Tamper integrity audit logs keep stale/offline heartbeat, permission loss,
      stopped/removed, uninstall detection, tamper/manual-required, and admin
      removal flow references redaction-safe without claiming stealth, privilege
      escalation, provider delivery, or admin-removal blocking.
- [x] Tamper uninstall artifact status rows distinguish desktop
      manual-required artifacts, mobile device-proof-required artifacts, and
      documented admin-removal flow refs without claiming artifact capture,
      stealth, privilege escalation, provider delivery, or removal blocking.
- [ ] No stealth or privilege-escalation behavior.

## Next AI Instructions

Do not block from a category label alone. Do not add anti-tamper behavior
without security/product design. Use
`scripts/test/v0-8-enforcement-integrity-runtime-audit.mjs` and
`scripts/test/v0-8-integrity-alert-status-bridge.mjs` as the current proof for
runtime audit and alert/status visibility. Use
`scripts/test/tamper-integrity-audit-contract-proof.mjs` as the logging-domain
proof for redaction-safe tamper/integrity audit rows. Use
`scripts/test/tamper-uninstall-artifact-status-proof.mjs` as the parent-domain
proof for manual-required/device-proof-required uninstall artifact statuses, and
require new platform artifacts before upgrading broad blocking, notification
delivery, mobile, service-persistence, uninstall detection, admin-removal
blocking, or tamper claims.
