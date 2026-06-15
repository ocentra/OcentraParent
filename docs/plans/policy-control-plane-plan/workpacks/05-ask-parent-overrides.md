# Workpack 05: Ask Parent Overrides

Goal: define child requests, parent approvals, bonus time, exceptions, and assistant-drafted actions.

Owns: child request lifecycle, parent approval lifecycle, bonus time, temporary allow/block, exception, override expiry, double-submit and replay defense, notification handoff, audit/history, and assistant-drafted action preview.

Handoff: AI may draft but cannot approve. Parent confirmation is required before any override becomes policy.

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

Do not let the AI or child request path write policy or enforcement state without parent confirmation and audit.
