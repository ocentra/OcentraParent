# Workpack 12: Payment Plan Handoff Gate

## Goal

Define exactly what `payment-subscription-plan` may assume from the shared Cloudflare module and what remains blocked.

## First-touch surface

- `output/cloudflare-control-plane-plan-proof/12-payment-plan-handoff-gate/payment-handoff-proof.md`

## Read inputs

- [PLAN_STATE.md](../PLAN_STATE.md)
- [PARENT_CLOUDFLARE_MODULE_SPEC.md](../PARENT_CLOUDFLARE_MODULE_SPEC.md)
- [SOURCE_SURFACE_STATUS_MATRIX.md](../SOURCE_SURFACE_STATUS_MATRIX.md)
- [WORKPACK_FAMILIES.md](../WORKPACK_FAMILIES.md)
- [PROOF_INDEX.md](../PROOF_INDEX.md)

## Output files

- `output/cloudflare-control-plane-plan-proof/12-payment-plan-handoff-gate/`
- [SOURCE_SURFACE_STATUS_MATRIX.md](../SOURCE_SURFACE_STATUS_MATRIX.md)

## Ownership boundary

```text
cloudflare-control-plane-plan owns shared Worker module readiness and handoff facts.
payment-subscription-plan owns billing semantics, provider behavior, subscription lifecycle, invoice/grace/referral qualification, and payment runtime status.
account-identity-family-plan owns parent session, household, admin/support authority, and account-provider decisions.
device-trust-bootstrap-plan owns trusted-parent-device authority.
portal-ux-household-surfaces-plan owns consumer UI.
data-custody-storage-plan owns retention/export/deletion policy.
```

## Required handoff artifact fields

The handoff artifact must name, at minimum:

```text
handoff_id
cloudflare_module_state
accepted_proof_roots
missing_proof_roots
carried_blockers
payment_may_assume
payment_must_not_assume
auth_account_dependency_state
trusted_device_dependency_state
provider_webhook_dependency_state
storage_queue_dependency_state
portal_smoke_state
deployment_promotion_state
data_custody_state
downstream_payment_ack_state
no_claim
```

These are field requirements for proof routing, not implementation code prescriptions.

## Acceptance

- Payment knows whether the module exists.
- Payment knows whether auth boundary exists or is blocked.
- Payment knows whether local dev/test runner exists or is blocked.
- Payment knows whether no-provider-secrets and portal-to-worker boundaries are explicit.
- Payment knows which proof roots are accepted and which roots are missing or carried as blockers.
- Payment knows which assumptions it may consume and which assumptions it must not make.
- Payment knows whether account/session, trusted-device, provider-webhook, deployment, storage/queue, and data-custody dependencies are ready, blocked, or manual-required.

## Proof IDs

- `cloudflare-control.payment-plan-handoff`

## Validation

- Aggregate accepted WP03-WP11 proof roots and record downstream payment assumptions plus blockers.
- Record exact command output or blocker rows under the WP12 proof root.
- Keep payment runtime work blocked if any required upstream root is missing without an accepted carried-blocker decision.

## Negative cases

- Reject unblocking payment from docs alone when core Cloudflare blockers remain.
- Reject unblocking payment from source presence or route manifest presence alone.
- Reject unblocking payment from local dev proof as production deploy proof.
- Reject unblocking payment from auth adapter proof as production account/trusted-device authority proof.
- Reject unblocking payment from billing handler presence as payment semantics readiness.

## Failure conditions

- Do not mark payment unblocked without explicit handoff proof.
- Do not mark payment unblocked without naming accepted proof roots and carried blockers.
- Do not mark payment unblocked without downstream payment-plan acknowledgment.
- Do not collapse account, trusted-device, provider, deployment, or data-custody blockers into Cloudflare readiness.
- Do not store the handoff proof inside this plan folder; use the output proof root.
