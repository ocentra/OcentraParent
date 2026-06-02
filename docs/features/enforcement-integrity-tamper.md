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
- Modules: `packages/parent-domain`, `crates/agent-core`,
  `crates/agent-service`, platform folders.

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
- Broad app/browser/domain/network enforcement is not product-complete.
- Tamper/uninstall protection is a tracked gap.

## Current Gap

Broad adapters, child-facing messages, integrity heartbeat, permission-loss
alerts, uninstall detection, and platform-specific proof remain. The integrity
runtime audit makes these states parent-visible in contracts/protocol/service
proof, but permission restoration, fresh heartbeat alerting, uninstall
detection artifacts, service restart timer persistence, and anti-tamper behavior
are still manual-required or unavailable. Tamper/uninstall is represented as
manual-required/rejected state, not as anti-tamper behavior. The broad-adapter,
supported-adapter, and integrity audit proofs give service visibility into
adapter gaps but do not prove anti-tamper behavior or notification delivery.

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
- [ ] Integrity heartbeat is represented as running/stale state; real alerting
      and freshness proof remain.
- [ ] Permission-loss and uninstall/tamper alerts are explicit states; real
      permission restoration, uninstall detection, and delivery proof remain.
- [ ] No stealth or privilege-escalation behavior.

## Next AI Instructions

Do not block from a category label alone. Do not add anti-tamper behavior
without security/product design. Use
`scripts/test/v0-8-enforcement-integrity-runtime-audit.mjs` as the current proof
for runtime audit visibility, and require new platform artifacts before
upgrading broad blocking, notification, mobile, service-persistence, or tamper
claims.
