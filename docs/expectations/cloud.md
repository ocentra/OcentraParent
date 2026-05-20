# Cloud Feature Expectations

Cloud features support parent-away-from-home use cases.

## Expected Deliverables

- Cloudflare control-plane boundary.
- Authenticated parent identity.
- Authenticated device identity.
- Device heartbeat.
- Rule/query/approval event relay.
- Sync queue.
- Retry/backoff behavior.
- Conflict handling.
- Local-first fallback.

## Acceptance

- Local operation works when cloud is unavailable.
- Remote rule updates, queries, approvals, and device events are authenticated and auditable.
- Device state cannot be overwritten silently by stale cloud state.
- Cloud logs do not leak sensitive child activity beyond intended product data.
- Cloud behavior reuses shared contracts instead of inventing parallel payloads.

## Non-Goals

- Do not replace local evidence storage with cloud-only storage.
- Do not add paid provider requirements to local development.
- Do not route production family data through unauthenticated dev endpoints.

## Done Signal

A parent can remotely see device health or send a scoped rule, query, or approval intent through authenticated cloud routing while the child-device agent remains local-first and owns execution.
