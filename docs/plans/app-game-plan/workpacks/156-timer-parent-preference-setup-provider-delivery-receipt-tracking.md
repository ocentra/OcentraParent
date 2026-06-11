# WP156 - Timer parent preference setup provider delivery receipt tracking

## Scope

Extend the parent preference setup provider-delivery local queue seam with
parent-safe provider receipt-required and receipt-pending tracking.

## Implementation

- Agent protocol-domain and Rust protocol results carry provider receipt
  requirement and receipt-pending refs/status.
- Agent-service setup persistence writes local provider receipt-required and
  receipt-pending audit rows after the provider local queue row.
- Durable setup outbox JSONL records serialize the provider receipt tracking
  IDs/statuses.
- Portal-domain and portal tests render provider receipt-required and
  receipt-pending refs/status in the parent command-result detail surface.

## No-Claim Boundary

This is still a local receipt tracking seam. It does not claim provider
delivery execution, external provider receipt ingestion, adapter dispatch,
broad blocking, platform enforcement, raw private source rows, raw target
values, or private diagnostics.

## Validation

See
`output/app-game-plan-proof/156-timer-parent-preference-setup-provider-delivery-receipt-tracking/10-validation-commands.log`.
