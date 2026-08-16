# Parent Authoring UI Expectations

This doc defines the required parent-facing UI states for policy creation, preview, conflict handling, and approval.

## Required screens

```text
Policy home
Create rule
Choose child/device target
Choose app/site/category/location/screen target
Choose schedule/time budget
Choose action: allow/warn/ask-parent/time-limit/block/manual-review
Preview impact
Conflict resolution
Confirm policy
Delivery status
Ask-parent requests
Bonus time / override
Audit history
Rollback/supersede
Manual-required gaps
```

## Required UI states

```text
draft
previewLoading
previewReady
previewFailed
conflictDetected
unsupportedTarget
manualRequired
staleDevice
offlineChild
scheduleAmbiguous
scheduleInvalid
confirmationRequired
confirmed
queued
delivered
acknowledged
active
partiallyActive
rejected
rolledBack
superseded
expired
```

## UI language constraints

Use:

```text
Draft
Preview only
Confirmation required
Queued
Delivered
Acknowledge by device
Active on this device
Partially active
Manual proof required
Unsupported on this platform
```

Do not use:

```text
Blocked
Protected
Active everywhere
Saved and enforced
AI applied it
Works on all devices
Remote ready
```

unless proof exists.

## Required UI behavior

- Parent can author rules without editing files or understanding device internals.
- Parent can preview before save.
- Conflicts must be visible.
- Unsupported and manual-required states must be visible.
- AI draft remains preview-only.
- Mobile and accessibility states must be considered, not implied.

## Negative cases

```text
rule saves without preview
preview enforces runtime behavior
unsupported target hidden
offline child shown as ready
stale device shown as successful
manual-required hidden behind green check
cancelled draft still changes policy
assistant draft saves without confirmation
```

## Proof expectation

The UI doc closes only when the proof inventory shows the required screens, the required states, the copy constraints, and the preview-only AI boundary.
