# Identity Authority Model

## Scope

This model separates user identity, household membership, child profiles, devices, and support access.

The current contract seeds in `packages/family-domain/src/household-authority.ts` are the source vocabulary for this plan:

- `HouseholdRole`: `parent-owner`, `co-parent-guardian`, `observer`, `child-profile`, `child-device-agent`, `support-admin`
- `HouseholdMembershipState`: `invited`, `pending`, `active`, `revoked`, `disabled`
- `DeviceRole`: `parent-controller`, `parent-observer`, `child-agent`
- `DeviceTrustState`: `pending`, `trusted`, `revoked`, `disabled`
- `DeviceAuthorityAction`: `pair-child-device`, `revoke-child-device`, `view-child-status`, `change-policy`, `start-remote-view`, `start-remote-control`, `export-delete-data`, `manage-billing`
- `ParentControllerLeaseState`: `active`, `expired`, `revoked`
- `ObserverPermissionScope`: `household-summary`, `child-status`, `device-source-state`
- `ObserverPermissionState`: `granted`, `revoked`, `disabled`

## Core Invariants

- User identity is not the same as household membership.
- Child profile is not the same as child device.
- A device can be trusted, revoked, or disabled independently of the account.
- A support-admin actor is not a household member by default and does not get household authority through support access alone.
- Cross-family reads and writes are denied unless an explicit household boundary has been established.

## Role / Membership / Device Model

| Entity | Source of truth | Can authorize? | Notes |
| --- | --- | --- | --- |
| User account | Identity provider + Cloudflare-owned family state | No | Identity establishes who the actor is, not which family they control. |
| Household profile | Cloudflare-owned family state | Yes | This is the family authority boundary. |
| Household membership | Cloudflare-owned family state | Yes, if active | Membership states must be typed and revocable. |
| Child profile | Cloudflare-owned family state | No | A child profile is descriptive, not authority-bearing. |
| Device registration | Cloudflare-owned family state | Yes, if trusted and scoped | Device role and trust state are independent. |
| Parent controller lease | Cloudflare-owned family state | Yes, for controller actions | Lease is time-bounded and action-specific. |
| Observer permission | Cloudflare-owned family state | Yes, read-only scope only | Observer permissions never imply write authority. |

## Authorization Rules

- Parent-owner can own the household, manage billing, export/delete data, and approve recovery where the recovery model allows it.
- Co-parent-guardian can pair and revoke devices, change policy, and start remote view/control only when the device and session rules also pass.
- Observer can view household and child status and device-source state, but cannot write policy, billing, export/delete, or recovery.
- Child-profile and child-device-agent are separate concepts; neither can grant parent authority.
- Support-admin is audited and minimized, and cannot act as household owner by default.

## Failure Conditions

- Any setup flow that treats login as household trust is wrong.
- Any setup flow that lets a child profile imply device trust is wrong.
- Any support flow that can silently cross into household-owner powers is wrong.
- Any cross-family authorization that passes because of an id guess is wrong.
