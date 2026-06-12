# Remote Access Plan � HID Execution Blueprint

## Execution objective

Define remote access as explicit consented session flow with strict control-view split and replay-safe session lifecycle.

## Slice 01 � Auth/AuthZ and Session Grant

### Acceptance

- Household-scoped grants and role checks block all cross-household/session abuse.

### Tests

- `remote-access.session.authn-authz`
- `remote-access.session.replay-idempotency`

### Proof

- `docs/proof/remote-access-plan/slice-01-authz-grant.md`

## Slice 02 � Relay and Transport Reliability

### Acceptance

- Reconnect/retry-storm behavior bounded with backpressure and timeout cleanup.

### Tests

- `remote-access.relay.reconnect-retry-storm`
- `remote-access.transport.timeout`

### Proof

- `docs/proof/remote-access-plan/slice-02-relay-transport.md`

## Slice 03 � Remote View vs Control Separation

### Acceptance

- Control paths require stronger explicit privileges than view-only.

### Tests

- `remote-access.view-control-separation`
- `remote-access.privilege-escalation`

### Proof

- `docs/proof/remote-access-plan/slice-03-view-control-separation.md`

## Slice 04 � Retention and Audit

### Acceptance

- Remote artifacts are handled by retention/delete rules and all actions logged with reason codes.

### Tests

- `remote-access.audit.log-safety`

### Proof

- `docs/proof/remote-access-plan/slice-04-retention-audit.md`

## Workpacks (execution lane)

### Slice-to-workpack binding

- Slice 01: docs/plans/remote-access-plan/workpacks/01-remote-capability-fabric.md
- Slice 02: docs/plans/remote-access-plan/workpacks/02-live-screen-relay.md
- Slice 03: docs/plans/remote-access-plan/workpacks/03-remote-input-control-authority.md
- Slice 04: docs/plans/remote-access-plan/workpacks/04-session-consent-grants.md

## PR-ready gate

- No remote-control claim without explicit privilege split and rollback/revocation evidence.

## HID test floor (this plan)

### Required test families for closed slice

- Unit: grant/session contracts
- Integration: relay separation and control/view lanes
- E2E: remote-control establishment and teardown
- Security: authN/authZ, replay and log redaction tests
- Non-functional: retry storm and partial-outage behavior

### Mandatory slice evidence checks

- negative cases documented (at least one per slice)
- rollback/teardown proof recorded
- proof manifest references command output, artifacts, and manual review notes
