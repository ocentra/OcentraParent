# App Install And Purchase Approval Expectations

Install and purchase approval is separate from app blocking. Platform stores
already expose this in ecosystem-specific ways, so Ocentra needs an explicit
answer before claiming parity.

## Parent Outcome

- Parent can see when a child requests a new app, purchase, subscription, or
  permission where the platform allows it.
- Parent can approve, deny, time-box, or mark review-needed.
- Parent can see app age rating, category, publisher, platform source, and known
  risk/context where available.
- Parent can see when the platform does not allow Ocentra to intercept or
  control install/purchase flow.

## Child-Device Outcome

- Child-device agent records install/purchase request events only from approved
  platform sources.
- Policy decisions and approvals are typed and audited.
- Unsupported platforms degrade honestly.

## Contract Boundary

Expected contract families:

- `InstallRequest`
- `PurchaseRequest`
- `AppStoreMetadata`
- `InstallApprovalDecision`
- `PurchaseApprovalDecision`
- `InstallPolicyRule`
- `InstallAuditEvent`

## Acceptance

- App install and purchase approvals are not confused with app runtime blocking.
- Platform support matrix is explicit.
- Parent approvals expire or carry scope.
- Store metadata has source and freshness status.

## Validation Gates

- TypeScript schema tests for requests, store metadata, approvals, policy rules,
  and audit events.
- Platform proof for Google Play, Apple App Store, Microsoft Store, or other
  store hooks before claiming integration.
- Portal tests for approval flow when UI exists.

## Non-Goals

- Do not bypass store policy.
- Do not imply Apple/Google ecosystem parity without approved APIs.
- Do not let billing entitlements decide child safety approvals.

## Done Signal

A parent can review install/purchase requests with platform-backed metadata and
typed approval/audit behavior, or the product clearly states that the platform
does not allow the capability.
