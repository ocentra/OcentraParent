# Workpack 05: Ask Parent Overrides

Goal: define child requests, parent approvals, bonus time, exceptions, and assistant-drafted actions.

Expected shape:

- Child requests are typed, scoped, expiring, and linked to policy context.
- Parent approval can grant, deny, modify, or expire.
- Assistant-drafted actions remain preview-only until parent confirmation.
- Abuse cases include repeated requests, double submit, replay, and stale approval.

Expected proof:

- Request/approval lifecycle tests.
- Replay/double-submit proof.
- Parent role authorization proof.
- Notification/audit handoff proof.

Failure: AI or child request path writes policy or enforcement state without parent confirmation and audit.

## Execution Detail

Minimum context:

- `docs/features/parent-assistant-actions.md`
- `docs/expectations/parent-assistant-chat.md`
- `docs/plans/ai-plan/AGENTS.md`
- `docs/plans/portal-ux-household-surfaces-plan/workpacks/07-parent-requests-and-approvals.md`

Required lifecycle:

- Child request created.
- Parent notified.
- Parent views context.
- Parent grants, denies, modifies, or lets expire.
- Domain compiler updates policy or temporary override.
- Enforcement/result state is audited.

Rules:

- Assistant may draft; parent confirms.
- Child cannot self-approve.
- Double-submit/replay cannot grant extra time.
- Overrides expire and are visible in audit/history.

Expected tests/proof names:

- `ask-parent.double-submit-safe`
- `ask-parent.replay-rejected`
- `ask-parent.expired-request-denied`
- `ask-parent.parent-confirmation-required`
- `ask-parent.assistant-draft-preview-only`

Proof artifact expectations:

- Request/approval state machine.
- Notification handoff.
- Audit and UI screenshots when rendered.
