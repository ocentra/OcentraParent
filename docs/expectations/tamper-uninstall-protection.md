# Tamper And Uninstall Protection Expectations

Tamper and uninstall protection must be honest. It is allowed to protect parent
configuration and report removal attempts, but it must not become stealth,
privilege escalation, or hidden persistence.

## Parent Outcome

- Parent can see whether the child-device agent is installed, running, degraded,
  stopped, removed, outdated, or needs attention.
- Parent can receive an alert when the agent is disabled, uninstalled, loses
  required permission, or stops reporting.
- Parent can understand which protections are available on each platform.
- Parent can remove the product through documented support/admin paths.

## Child-Device Outcome

- The agent records service stop, permission loss, uninstall, restart,
  heartbeat, update, and degraded events where platform APIs allow it.
- The agent never hides from the OS, escalates privileges silently, or blocks
  legitimate parent/admin removal paths.

## Contract Boundary

Expected contract families:

- `AgentIntegrityStatus`
- `AgentHeartbeat`
- `PermissionLossEvent`
- `UninstallDetectionEvent`
- `TamperSignal`
- `IntegrityAlert`
- `AdminRemovalFlow`

## Acceptance

- Parent can distinguish offline, stale, stopped, removed, permission-denied,
  outdated, and unsupported states.
- Alerts carry minimal detail and authenticated drill-in.
- Tamper integrity audit logs carry only redaction-safe operational refs and
  status fields for stale/offline heartbeat, permission loss, stopped/removed,
  uninstall detection, tamper/manual-required, and admin removal flow states.
- Tamper integrity audit logs must not include raw child activity, raw evidence
  payloads, raw URLs, screenshots, command lines, private paths, or message
  contents.
- Audit proofs must keep stealth behavior, privilege escalation, hidden
  persistence, provider delivery, and admin-removal blocking as explicit
  non-claims until security/product/legal review and platform proof exist.
- Platform-specific protection claims cite real proof.
- Removal and support paths are documented.

## Validation Gates

- TypeScript schema tests for integrity, heartbeat, tamper, uninstall,
  permission, and alert states.
- Logging-domain contract proof for redaction-safe tamper/integrity audit rows
  and no-claim boundaries.
- Rust/service tests for heartbeat and degraded-state reporting.
- Real platform/manual proof for service manager, Device Owner, managed profile,
  Screen Time/Family Controls, launchd/systemd, or other OS capabilities before
  claiming protection.
- Security review before adding any persistence-hardening behavior.

## Non-Goals

- Do not add stealth behavior.
- Do not add privilege escalation.
- Do not block parent/admin removal without explicit product/legal/security
  design.
- Do not claim anti-tamper from heartbeat alone.
- Do not claim notification provider delivery from audit/log rows alone.

## Done Signal

A parent can see and act on agent integrity, permission loss, stale/offline, and
uninstall/tamper states with platform-specific proof and documented removal
paths.
