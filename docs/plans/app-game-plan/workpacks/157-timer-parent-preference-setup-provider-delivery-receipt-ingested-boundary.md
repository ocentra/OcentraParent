# WP157 - Timer parent preference setup provider delivery receipt-ingested boundary

## Scope

Extend the parent preference setup provider-delivery receipt tracking seam with a
parent-safe local provider receipt-ingested boundary.

## Implementation

- Agent protocol-domain and Rust protocol results carry provider receipt-ingested
  refs/status.
- Agent-service setup persistence writes a local provider receipt-ingested audit
  row after provider receipt-pending tracking.
- Durable setup outbox JSONL records serialize the provider receipt-ingested
  ID/status.
- Portal-domain and portal tests render provider receipt-ingested refs/status in
  the parent command-result detail surface.

## No-Claim Boundary

This is still a local provider receipt boundary. It does not claim provider
delivery execution, external provider receipt ingestion, adapter dispatch,
broad blocking, platform enforcement, raw private source rows, raw target
values, or private diagnostics.

## Validation

See
`output/app-game-plan-proof/157-timer-parent-preference-setup-provider-delivery-receipt-ingested-boundary/10-validation-commands.log`.
