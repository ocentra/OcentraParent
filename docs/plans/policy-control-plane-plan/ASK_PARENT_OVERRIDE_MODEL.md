# Ask Parent Override Model

Ask-parent is a temporary policy request path. It is a typed, auditable mutation path, not a free-form chat shortcut.

## Required request fields

```text
request id
household
child profile
child device
target
requested action
reason
evidence refs
policy version
expiry
delivery route
audit ref
```

## Required parent response values

```text
grant
deny
modify
grant bonus time
grant one-time allow
block
expire
manualRequired
```

## Required states

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

## Required rules

- Child cannot self-approve.
- Observer cannot approve.
- Revoked parent cannot approve.
- Assistant cannot approve.
- Double-submit cannot grant twice.
- Replay cannot re-grant.
- Override must expire.
- Audit must show why and who approved.
- Notification handoff is part of the contract, but delivery runtime is still owned by the delivery model.

## AI boundary

AI can draft the request or suggested response, but the output stays preview-only until an authorized parent confirms a typed action.

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

## Proof expectation

The override model is closed only when the proof inventory shows state-machine coverage, double-submit and replay defense, parent confirmation, bonus-time expiry, assistant preview-only behavior, and override audit history.
