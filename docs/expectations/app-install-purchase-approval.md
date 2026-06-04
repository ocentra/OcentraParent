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
- Parent can see which store/source metadata fields still require platform API,
  entitlement, package-source, or manual proof before product support is claimed.
- Parent can see which package-source artifact rows still require host/device
  package identity, installer-source, receipt, signing, entitlement, or package
  manager proof before product support is claimed.
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
- Child-facing pending/result states cite approval state, audit refs, and report
  refs without claiming child-device delivery until platform proof exists.
- Audit/report integration status is explicit and does not imply portal or
  runtime report delivery.
- Platform-source metadata limitation rows name the store source, required
  artifacts, unavailable/manual-required state, limitation report ref, and
  explicit no-store-integration/no-interception boundary.
- Package-source artifact rows name the package identity/source fields, required
  artifacts, manual/device-proof/unavailable state, limitation report ref, and
  explicit no-attached-artifact/no-store-integration/no-platform-adapter/no-real
  interception/no-child-activity-data boundary.
- Runtime proof rows link the platform/store metadata artifact requirement,
  package-source artifact requirement, child pending/result delivery boundary,
  and report integration boundary without claiming provider/store integration,
  child-device delivery, runtime report delivery, or app blocking.
- Platform artifact proof rows attach parent-owned platform/store metadata
  artifact refs and report-runtime evidence refs to the existing runtime
  boundary without claiming provider APIs, store integration, platform adapters,
  child-device delivery, runtime report delivery, real interception, or app
  blocking.

## Validation Gates

- TypeScript schema tests for requests, store metadata, approvals,
  child-facing states, audit/report status, policy rules, and audit events.
- TypeScript schema tests for platform-source metadata limitation rows and
  missing-platform-row rejection.
- TypeScript schema tests and a proof harness for package-source artifact rows,
  missing package-source row rejection, missing field coverage rejection, and
  adapter/store/interception overclaim rejection.
- TypeScript schema tests and a proof harness for runtime-boundary rows, missing
  platform/package artifact coverage rejection, child delivery/report runtime
  overclaim rejection, and required non-claim coverage.
- TypeScript schema tests and a proof harness for platform artifact rows,
  missing platform/report artifact coverage rejection, provider/API/adapter/
  delivery/report/app-blocking overclaim rejection, and required non-claim
  coverage.
- Platform proof for Google Play, Apple App Store, Microsoft Store, or other
  store hooks before claiming integration.
- Portal tests for approval flow when UI exists.

## Non-Goals

- Do not bypass store policy.
- Do not imply Apple/Google ecosystem parity without approved APIs.
- Do not let billing entitlements decide child safety approvals.
- Do not treat contract-only child-facing/report rows as delivered runtime UX.
- Do not treat package-source artifact requirement rows as captured host/device
  artifacts.
- Do not treat runtime-boundary proof rows as provider/store integration,
  child-device delivery, runtime report delivery, or app blocking.
- Do not treat parent-owned platform artifact proof refs as approved store APIs,
  platform adapters, child-device package captures, runtime report delivery, or
  app blocking.

## Done Signal

A parent can review install/purchase requests with platform-backed metadata and
typed approval/audit behavior, or the product clearly states that the platform
does not allow the capability.
