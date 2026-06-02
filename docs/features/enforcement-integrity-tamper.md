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
- Broad app/browser/domain/network enforcement is not product-complete.
- Tamper/uninstall protection is a tracked gap.

## Current Gap

Broad adapters, child-facing messages, integrity heartbeat, permission-loss
alerts, uninstall detection, and platform-specific proof remain.

## Checklist

- [ ] Typed enforcement intent/action/result/audit.
- [ ] Adapter capability status.
- [ ] Timer create/expire/recover/rollback.
- [ ] Child-facing status.
- [ ] Parent override.
- [ ] Integrity heartbeat.
- [ ] Permission-loss and uninstall/tamper alerts.
- [ ] No stealth or privilege-escalation behavior.

## Next AI Instructions

Do not block from a category label alone. Do not add anti-tamper behavior
without security/product design. Every enforcement claim needs real adapter
proof.
