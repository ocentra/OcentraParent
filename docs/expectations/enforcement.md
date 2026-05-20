# Enforcement Feature Expectations

Enforcement features change device behavior and therefore need a higher bar.

## Expected Deliverables

- Adapter boundary per platform.
- Explicit policy decision input.
- Enforcement action event.
- Reason code.
- Evidence reference.
- Manual override or safe rollback path.
- Clear status when enforcement capability is unavailable.

## Acceptance

- Enforcement acts only after a typed policy decision.
- Every action is journaled.
- Parent can see what happened and why.
- Failure to enforce is reported.
- Enforcement tests cover allowed, blocked, timeout, unavailable, and rollback paths where feasible.

## Non-Goals

- Do not add stealth behavior.
- Do not add anti-tamper behavior.
- Do not add privilege escalation.
- Do not claim persistence-hardening without explicit product/security design.

## Done Signal

The feature can enforce one clearly scoped policy action, report success/failure through typed events, and leave an auditable journal trail.
