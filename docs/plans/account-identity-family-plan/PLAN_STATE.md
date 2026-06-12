# Account Identity Family Plan State

Status: first-pass plan created because login/user/household authority was not owned by a dedicated plan.

Research status: incomplete. This plan requires a full follow-up research pass against existing portal-domain, parent-domain, local API, agent protocol, games Cloudflare/Firebase auth, current official provider docs, and Sujan's account/privacy choices before implementation claims.

Current direction:

- Cloudflare-first app and custody architecture.
- Firebase Auth may be evaluated as a pragmatic identity provider/token issuer, not as the family product data store.
- Cloudflare D1/Durable Objects/R2/KV roles must be deliberate: D1 for relational account/household state, Durable Objects for live coordination, R2 for encrypted blobs/artifacts, KV for non-authoritative cache/rate limits.
- Cloudflare Access is not a consumer family identity product by itself.

Open gaps:

- No provider decision record.
- No household role/device authority model.
- No session/token lifecycle proof matrix.
- No invite/recovery/delete/transfer state machine.
- No cross-family authorization test inventory.

## HID Execution Guard (added 2026-06-12)

- Scope and completion source:
  - follow [PLAN_HID_MATRIX.md](../../PLAN_HID_MATRIX.md) execution slice, then this plan's assigned WORKPACK_INDEX.md and NEXT_ACTIONS.md.
  - do not mark this plan complete from checklist deltas alone.
- Before any checked update, attach:
  - a real test run log (or explicit known blocker) from the assigned implementation boundary,
  - a proof manifest under docs/proof/account-identity-family-plan/.
- Required proof manifest names:
  - docs/proof/account-identity-family-plan/slice-01-\*.md
  - docs/proof/account-identity-family-plan/slice-02-\*.md
  - docs/proof/account-identity-family-plan/slice-03-\*.md
  - each proof file must include commands, pass/fail,
    negative-cases, and manual-required notes.
- Failure rule: no PR-ready claim until replay/idempotency, authZ/replay, and rollback/teardown proofs are present for the assigned slice.

## HID execution blueprint

Continue execution from: [PLAN_EXECUTION_BLUEPRINT.md](PLAN_EXECUTION_BLUEPRINT.md).
Update this plan only via the blueprint and matching workpack checklist.
