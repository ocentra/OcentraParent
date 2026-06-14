# UI Expectations

## Scope

This plan owns the first-run parent account and family setup UI. The UI is a parent-facing control and status surface, not a child-device execution boundary.

## Required Screens

| Screen | Must show | Must not imply |
| --- | --- | --- |
| Welcome / sign in | Auth provider state, recovery entry, source labels | Login equals household trust |
| Create or join household | Household creation/join path, owner role, audit hint | Household membership exists automatically |
| Create child profile | Child profile state, profile binding status | Child profile equals child device trust |
| Add child device | Device role, trust state, source/custody state | A device is trusted before pairing |
| Pair child device | Pairing intent, expiry, single-use state | Pairing can be reused |
| Devices and roles | Parent-controller, parent-observer, child-agent, trust/revocation state | Roles are interchangeable |
| Invite co-parent | Purpose, target role, expiry, single-use | Invite can be forwarded or reused |
| Invite observer | Read-only role, scope, expiry | Observer has write authority |
| Recovery and revoke | Recovery kind, approval state, custody handoff | Recovery bypasses owner approval |
| Account / security settings | Session, revocation, recovery, provider state | Security state is hidden behind generic UI |
| Support access | Audited, minimized, explicit scope | Support gets household-owner authority |

## Source And Custody Labels

Every first-run screen must show one of these source labels when relevant:

- live local
- LAN
- parent cache
- parent-owned storage
- stale
- degraded
- unavailable
- manual-required

## No-Claim Language

- Authentication is not authorization.
- A valid login does not prove household membership, parent authority, child-device trust, or export/delete authority.
- Parent setup UI must not expose raw protocol fields or internal token values.
- Support access is not household ownership.
- Observer is read-only.
- Child-device authority remains on the child device.

## Layout And Behavior

- The UI must remain readable on common desktop and mobile widths.
- Empty, loading, degraded, and unavailable states must be visible and explicit.
- The UI must make it obvious whether a view is live local, LAN, cached, parent-owned storage, or unavailable.
- The UI must not create a polished fake dashboard before the underlying typed state exists.

## Failure Conditions

- A screen that looks successful while the source is unavailable is wrong.
- A screen that hides custody state is wrong.
- A screen that implies support can act as the household owner is wrong.
