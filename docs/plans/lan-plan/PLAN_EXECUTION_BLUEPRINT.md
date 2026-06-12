# LAN Plan � HID Execution Blueprint

## Execution objective

Implement trusted local device discovery/pairing with explicit replay-safe lease lifecycle and physical proof.

## Slice 01 � Contract and Evidence Models

### Acceptance

- Discovery, add-device, and pairing models are schema-backed and invalid cases fail.

### Tests

- `lan.peer.authn-authz-matrix`
- `lan.lease.token-lifecycle`

### Proof

- `docs/proof/lan-plan/slice-01-contract-evidence.md`

## Slice 02 � Discovery and Trust Surface

### Acceptance

- mDNS/ARP/scan listeners are bounded and reject spoofed sources and stale artifacts.

### Tests

- `lan.discovery.partial-outage`
- `lan.mesh.no-raw-sensitive-transfer`

### Proof

- `docs/proof/lan-plan/slice-02-discovery-trust.md`

## Slice 03 � Pairing, Revocation, and Audit

### Acceptance

- Pairing lifecycle supports revoke, double-submit rejection, and replay-safe tokens.

### Tests

- `lan.lease.token-lifecycle`
- `lan.audit.trace-completeness`

### Proof

- `docs/proof/lan-plan/slice-03-pairing-audit.md`

## Slice 04 � Physical Household Proof

### Acceptance

- Proof for 2-device/actual-network validation and environment details are attached before household claims.

### Tests

- `lan.two-device.physical-proof`

### Proof

- `docs/proof/lan-plan/slice-04-physical-hardware.md`

## Workpacks (execution lane)

### Slice-to-workpack binding

- Slice 01: docs/plans/lan-plan/workpacks/01-contract-boundary-and-effect-schemas.md
- Slice 02: docs/plans/lan-plan/workpacks/02-evidence-model-and-device-record.md
- Slice 03: docs/plans/lan-plan/workpacks/03-interface-detection.md
- Slice 04: docs/plans/lan-plan/workpacks/04-neighbor-table-ingestion.md

## PR-ready gate

- No household claim unless signed hello/heartbeat, pairing authZ, and replay safeguards are linked in proof.

## HID test floor (this plan)

### Required test families for closed slice

- Unit: discovery/trust contract validation
- Integration: pairing lifecycle and lease renewal/revocation
- E2E: two-device claim and live offline/online transitions
- Security: signed hello, replay, privilege split
- Non-functional: outage and retry behavior

### Mandatory slice evidence checks

- negative cases documented (at least one per slice)
- rollback/teardown proof recorded
- proof manifest references command output, artifacts, and manual review notes
