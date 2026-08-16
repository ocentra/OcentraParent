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

The Rust command path exists, but the portal/provider handoff is incomplete. `apps/portal/src/portal-actions.ts` and `apps/portal/src/portal-runtime-controller-actions.ts` expose `requestPolicyRequestParentResolution`, while `apps/portal/src/PolicyPreviewRoutePanel.tsx` only renders draft staging and assistant confirmation; no rendered parent-resolution surface invokes the callback. `crates/parent-runtime-core/src/parent_ui_bridge/snapshot_overlay/command.rs` maps the action to the Rust command, and the new parent-runtime staging boundary projects a typed request only from trusted preview context plus local controller authority. The agent-service resolution modules validate the canonical confirmed request and delivery binding, but the account/identity-owned actor context and notification-provider dispatch composition are not present at this boundary. The portal must therefore remain fail-closed/manual-required and must not mint approval or contract JSON; the drafted staging slice does not remove those dependency blockers.

## Production-code pass status — 2026-08-16

The WP05 policy lane now has a narrow Rust-owned typed parent-resolution staging/relay slice in:

```text
crates/parent-runtime-core/src/parent_ui_bridge/action_dispatch.rs
crates/parent-runtime-core/src/parent_ui_bridge/policy_preview.rs
crates/parent-runtime-core/src/parent_ui_bridge/policy_preview/resolution.rs
```

The action boundary accepts only a strict decision input. Rust projects approval, actor, request, delivery-binding, expiry, and audit fields from the trusted preview context, binds the actor to the local active-controller record, and retains one-shot relay-attempt state so a failed/deferred attempt cannot be replayed as a new approval. `Modify` does not accept arbitrary caller-selected changes; its approved action is projected from the trusted preview context. Missing or ambiguous account/identity context fails closed for manual review.

This is code drafted and unvalidated only. The rendered portal parent-resolution surface, account/identity provider composition, notification-provider dispatch, WP11 pre-action/post-action durability, concrete runtime composition, tests, and proof remain open. This slice does not claim approval, notification, delivery, enforcement, rollback, or workpack completion.

The resolution service keeps `notification_handoff_claim_state` explicitly `Unclaimed` until the notification owner composes a real provider dispatch. A resolved policy request is not evidence that a parent notification was sent.

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
