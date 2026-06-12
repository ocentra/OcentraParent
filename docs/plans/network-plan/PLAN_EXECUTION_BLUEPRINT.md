# Network Plan � HID Execution Blueprint

## Execution objective

Split observation vs policy vs enforcement and prove each boundary independently with safe request/network handling.

## Slice 01 � Metadata Parsing and Schema Fuzzing

### Acceptance

- Parser rejects malformed flow metadata and schema ambiguities.

### Tests

- `network.metadata.schema-fuzz`

### Proof

- `docs/proof/network-plan/slice-01-metadata-schema.md`

## Slice 02 � AuthN/AuthZ and Request Surface

### Acceptance

- Request metadata and remote controls are authorized per household and role.

### Tests

- `network.request.origin-header-host`
- `network.security.smuggling-desync-cache`

### Proof

- `docs/proof/network-plan/slice-02-request-security.md`

## Slice 03 � Delivery and Policy Boundary

### Acceptance

- Policy/notification outputs are separate from classification observation.

### Tests

- `network.policy.delivery`
- `network.rate-limit.dos-boundary`

### Proof

- `docs/proof/network-plan/slice-03-delivery-boundary.md`

## Slice 04 � Adapter + Production Readiness

### Acceptance

- Intervention adapter limits, retry storms, and delete/read-model retention are validated.

### Tests

- `network.read-model.retention-delete`
- `network.rate-limit.dos-boundary`
- `network.partial-outage.resilience`

### Proof

- `docs/proof/network-plan/slice-04-production-slo.md`

## Workpacks (execution lane)

### Slice-to-workpack binding

- Slice 01: docs/plans/network-plan/workpacks/01-foundation-contracts-and-eventing.md
- Slice 02: docs/plans/network-plan/workpacks/02-passive-capture-and-parsing.md
- Slice 03: docs/plans/network-plan/workpacks/03-classification-and-correlation.md
- Slice 04: docs/plans/network-plan/workpacks/04-cross-slice-cascade-and-parent-surface.md

## PR-ready gate

- Never claim enforcement behavior without adapter capability proof and transport audit.

## HID test floor (this plan)

### Required test families for closed slice

- Unit: metadata and route schema checks
- Integration: service-policy handoff and adapter boundaries
- E2E: enforcement fallback and policy routing paths
- Security: header origin validation, request smuggling/desync probes
- Non-functional: resource pressure and connection handling

### Mandatory slice evidence checks

- negative cases documented (at least one per slice)
- rollback/teardown proof recorded
- proof manifest references command output, artifacts, and manual review notes
