# Checklist Index

> **Live-code audit (2026-07-17):** [Project Progress Matrix](../../PLAN_CODE_STATUS_MATRIX.md) records current implementation, blockers, dependencies, and next unblocker. Rows remain proof-gated; this audit does not check unsupported work.

Status: engineering-spec complete / implementation-open.

Use this file to point at the active workpack and its close condition. It is not
the proof store.

| Checklist row | Owning workpack | Close when |
| --- | --- | --- |
| PSP-00 | WP00 | Cloudflare handoff assumptions, blockers, route/auth/test-shape dependencies, and payment no-claim boundaries are explicit. |
| PSP-01 | WP01 | Starter bundle, seat expansion, over-limit grace, and rejected game-economy semantics are explicit. |
| PSP-02 | WP02 | Hosted checkout, portal, redirect safety, and browser-secret boundaries are explicit. |
| PSP-03 | WP03 | Webhook signature, replay, idempotency, dead-letter, reconciliation, and test/live separation are explicit. |
| PSP-04 | WP04 | Billing, referral, and entitlement ledgers plus signed snapshots and device gates are explicit. |
| PSP-05 | WP05 | Invoice, tax, refund, dispute, cancel, resume, and grace semantics are explicit. |
| PSP-06 | WP06 | Metadata allow/deny, secret handling, privacy, observability, and abuse controls are explicit. |
| PSP-07 | WP07 | Route sync, proof manifest, validation-log shape, negative case, and rollback path are explicit. |
| PSP-08 | WP08 | Provider adapter boundaries, normalized event contract, and provider-lock escape are explicit. |
| PSP-09 | WP09 | Regional rollout, local-method assumptions, and manual-required gaps are explicit. |
| PSP-10 | WP10 | Referral qualification, abuse rejection, grace, lost-credit recalculation, and no-data-delete behavior are explicit. |
| PSP-11 | WP11 | Parent billing dashboard visibility, allow/deny fields, wrong-household denial, and targeted parent proof requirement are explicit. |
| PSP-12 | WP12 | Support/admin billing operations, role limits, audit requirements, and no-child-private-data boundary are explicit. |
