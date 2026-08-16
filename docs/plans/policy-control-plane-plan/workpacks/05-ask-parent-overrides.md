# Workpack 05: Ask Parent Overrides

Goal: define child requests, parent approvals, bonus time, exceptions, and assistant-drafted actions.

Owns: child request lifecycle, parent approval lifecycle, bonus time, temporary allow/block, exception, override expiry, double-submit and replay defense, notification handoff, audit/history, and assistant-drafted action preview.

Handoff: AI may draft but cannot approve. Parent confirmation is required before any override becomes policy.

## Ownership boundary

```text
policy-control-plane-plan owns ask-parent and override state machines, approval contract, expiry, replay defense, scope limits, audit, and no-claim boundaries.
account-identity-family-plan owns parent/observer/revoked role authority.
device-trust-bootstrap-plan owns high-risk parent presence/step-up when selected.
ai-plan owns draft suggestions only; AI cannot confirm or apply policy.
notification plan owns notification delivery handoff.
domain/enforcement plans own runtime apply behavior after confirmed policy handoff.
```

## Production-code audit — 2026-08-16

The Rust command path exists, but the portal/provider handoff is incomplete. `apps/portal/src/portal-actions.ts` and `apps/portal/src/portal-runtime-controller-actions.ts` expose `requestPolicyRequestParentResolution`, while `apps/portal/src/PolicyPreviewRoutePanel.tsx` only renders draft staging and assistant confirmation; no rendered parent-resolution surface invokes the callback. `crates/parent-runtime-core/src/parent_ui_bridge/snapshot_overlay/command.rs` maps the action to the Rust command, but the parent-resolution payload is still caller-supplied rather than projected from a Rust-owned parent actor/provider context. The agent-service resolution modules validate the canonical confirmed request and delivery binding, but the account/identity-owned actor context and notification-provider dispatch composition are not present at this boundary. The smallest next production slice is a Rust-owned typed parent-resolution staging/preview handoff once the account/identity provider supplies exact actor identity/role/state; until then the portal must fail closed/manual-required and must not mint approval or contract JSON.

## Required lifecycle

```text
requestCreated
parentNotified
parentViewedContext
granted
denied
modified
expired
replayedRejected
doubleSubmitIgnored
superseded
applied
rolledBack
manualRequired
```

## Required proof fields

The selected proof must name, at minimum:

```text
request_id
child_actor_state
parent_actor_state
observer_state
revoked_parent_state
assistant_draft_state
parent_confirmation_state
expiry_state
double_submit_state
replay_state
override_scope_state
bonus_time_state
notification_handoff_state
audit_ref
rollback_state
manual_required_state
no_claim
```

These are proof-routing fields, not implementation code prescriptions.

## Required proof IDs

- `ask-parent.request-state-machine`
- `ask-parent.parent-confirmation-required`
- `ask-parent.child-cannot-self-approve`
- `ask-parent.double-submit-safe`
- `ask-parent.replay-rejected`
- `ask-parent.expired-request-denied`
- `ask-parent.bonus-time-expiry`
- `ask-parent.override-scope-limited`
- `ask-parent.override-audited`
- `ask-parent.notification-handoff`
- `ask-parent.assistant-draft-preview-only`
- `ask-parent.wrong-parent-denied`
- `ask-parent.observer-denied`
- `ask-parent.revoked-parent-denied`

## Rules

- Assistant may draft; parent confirms.
- Child cannot self-approve.
- Observer cannot approve.
- Revoked parent cannot approve.
- Double-submit and replay cannot grant twice.
- Overrides expire and are visible in audit/history.
- Notification handoff is part of the contract, not a hidden assumption.

## Negative cases

```text
child request self-approves
double-submit grants double time
replay grants old approval
expired request still applies
observer grants override
revoked parent grants override
assistant action writes policy
override has no expiry
```

## Failure

Do not let the AI or child request path write policy or enforcement state without parent confirmation and audit. Keep WP05 open until parent confirmation, assistant/portal integration, notification handoff, expiry/replay proof, and audit proof exist or are carried as explicit blockers.
