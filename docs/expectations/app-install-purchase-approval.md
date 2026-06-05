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
- Status runtime readiness rows link each child-facing status to runtime status
  reader readiness without claiming a runtime status reader, child-device
  delivery, runtime report delivery, store integration, platform adapters, or
  app blocking.
- Platform artifact proof rows attach parent-owned platform/store metadata
  artifact refs and report-runtime evidence refs to the existing runtime
  boundary without claiming provider APIs, store integration, platform adapters,
  child-device delivery, runtime report delivery, real interception, or app
  blocking.
- Child artifact/delivery proof rows attach child package-source artifact refs
  to platform/report artifact proof rows and child pending/result delivery
  boundaries without claiming child-device runtime capture, child-device
  delivery, provider APIs, store integration, platform adapters, runtime report
  delivery, real interception, child activity data custody, or app blocking.
- Approved API/entitlement proof rows attach approved store API evidence refs,
  entitlement evidence refs, limitation report refs, and audit refs to child
  artifact rows without claiming provider API execution, store integration,
  platform adapters, child delivery, runtime report delivery, real
  interception, child activity data custody, or app blocking.
- Report-runtime status proof rows link app-install report surfaces and child
  artifact refs to stateless report compiler status/result refs without claiming
  portal report UI, runtime report delivery, provider/store execution, platform
  adapters, child-device delivery, child activity data custody, app blocking, or
  Ocentra-hosted family data custody.
- Platform adapter boundary proof rows link approved API/entitlement evidence
  and report-runtime refs to adapter readiness/manual/unavailable rows without
  claiming platform adapter implementation, provider API execution, store
  integration, child-device delivery, runtime report delivery, real
  interception, child activity data custody, app blocking, or Ocentra-hosted
  family data custody.
- Parent review action proof rows link approve, deny, time-box, and
  review-needed decision actions to approved API/entitlement evidence refs and
  report-runtime refs without claiming portal approval UI, parent action runtime
  delivery, provider/store execution, platform adapters, child-device delivery,
  child activity data custody, app blocking, or Ocentra-hosted family data
  custody.
- Parent action runtime handoff proof rows link parent review actions to
  runtime handoff status and platform adapter boundary refs without claiming
  portal approval UI, runtime action writer implementation, parent action
  runtime delivery, provider/store execution, platform adapter implementation,
  child-device delivery, runtime report delivery, child activity data custody,
  app blocking, or Ocentra-hosted family data custody.
- Store status handoff proof rows link parent action runtime handoff refs and
  platform adapter readiness/manual/unavailable rows to per-store status
  handoff states without claiming provider API execution, store integration,
  platform adapter implementation, parent action runtime delivery,
  child-device delivery, runtime report delivery, real interception, child
  activity data custody, app blocking, or Ocentra-hosted family data custody.
- Runtime writer delivery proof rows link parent action runtime handoff refs and
  store status handoff refs to writer-envelope-ready/manual-required states
  without claiming runtime writer implementation, runtime writer delivery,
  parent action runtime delivery, provider/store execution, platform adapter
  implementation, child-device delivery, runtime report delivery, real
  interception, child activity data custody, app blocking, or Ocentra-hosted
  family data custody.
- Package-source capture status proof rows link child package-source artifact
  refs and store status handoff rows to captured, blocked, manual-required, and
  unavailable package-source capture status rows with artifact, audit, report,
  and platform limitation refs without claiming provider API execution, store
  integration, portal approval UI, platform adapter implementation,
  child-device delivery, runtime report delivery, real interception, child
  activity data custody, app blocking, or Ocentra-hosted family data custody.
- Child-device delivery runtime writer proof rows link runtime writer delivery
  rows and package-source capture/status rows to child delivery
  envelope/manual-required rows without claiming runtime writer execution,
  runtime writer delivery, parent action runtime delivery, provider API
  execution, store integration, platform adapter implementation, child-device
  delivery, runtime report delivery, real interception, child activity data
  custody, app blocking, or Ocentra-hosted family data custody.
- Package-source adapter execution proof rows link package-source capture/
  status rows to local Windows, manual macOS, unavailable Linux, and blocked
  Android/iOS adapter execution states with artifact, audit, report, and
  required-proof refs without claiming provider API execution, store
  integration, portal approval UI, production platform adapters, child-device
  delivery, runtime report delivery, real interception, child activity data
  custody, app blocking, or Ocentra-hosted family data custody.
- Parent action delivery readiness proof rows link parent action runtime handoff
  rows to child-device delivery runtime-writer envelope rows without claiming
  parent action runtime delivery, runtime writer execution, runtime writer
  delivery, provider API execution, store integration, platform adapter
  implementation, child-device delivery, runtime report delivery, real
  interception, child activity data custody, app blocking, or Ocentra-hosted
  family data custody.
- Provider/store execution readiness proof rows link approved API/entitlement
  evidence, store status handoff, package-source adapter execution, and parent
  action delivery readiness rows into execution-ready, manual-required, and
  unavailable states without claiming Google Play execution, Apple App Store
  execution, Microsoft Store execution, billing/provider contact, provider API
  execution, store integration, platform interception, platform adapter
  implementation, child-device delivery, runtime writer delivery, app blocking,
  child activity data custody, or Ocentra-hosted family data custody.

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
- TypeScript schema tests and a proof harness for status runtime readiness rows,
  missing child status coverage rejection, runtime status-reader overclaim
  rejection, and required non-claim coverage.
- TypeScript schema tests and a proof harness for platform artifact rows,
  missing platform/report artifact coverage rejection, provider/API/adapter/
  delivery/report/app-blocking overclaim rejection, and required non-claim
  coverage.
- TypeScript schema tests and a proof harness for child artifact/delivery rows,
  missing child artifact/delivery coverage rejection, runtime capture/provider/
  adapter/delivery/report/app-blocking overclaim rejection, and required
  non-claim coverage.
- TypeScript schema tests and a proof harness for approved API/entitlement
  evidence rows, missing platform/evidence-state coverage rejection,
  provider-execution/adapter/delivery/report/custody/interception/app-blocking
  overclaim rejection, public package export visibility, and required non-claim
  coverage.
- TypeScript schema tests and a proof harness for report-runtime status rows,
  missing report surface rejection, missing report compiler lifecycle coverage,
  portal/provider/store/adapter/delivery/custody/app-blocking overclaim
  rejection, public package export visibility, and required non-claim coverage.
- TypeScript schema tests and a proof harness for platform adapter boundary
  rows, missing platform/evidence-state coverage rejection, adapter/provider/
  store/delivery/report/interception/custody/app-blocking overclaim rejection,
  public package export visibility, and required non-claim coverage.
- TypeScript schema tests and a proof harness for parent review action rows,
  missing approval action rejection, missing approved API evidence/report
  runtime refs, portal/provider/store/adapter/delivery/custody/interception/
  app-blocking overclaim rejection, public package export visibility, and
  required non-claim coverage.
- TypeScript schema tests and a proof harness for parent action runtime handoff
  rows, missing parent review action/platform adapter boundary/audit/report
  coverage rejection, portal/runtime-writer/provider/store/adapter/delivery/
  custody/interception/app-blocking overclaim rejection, public package export
  visibility, and required non-claim coverage.
- TypeScript schema tests and a proof harness for store status handoff rows,
  missing platform/status coverage rejection, missing parent action runtime
  refs, missing platform adapter evidence/report refs, provider/store/adapter/
  parent action delivery/child delivery/report delivery/custody/interception/
  app-blocking overclaim rejection, public package export visibility, and
  required non-claim coverage.
- TypeScript schema tests and a proof harness for runtime writer delivery rows,
  missing parent action runtime/store status/audit/report coverage rejection,
  runtime-writer/provider/store/adapter/parent-action-delivery/child-delivery/
  report-delivery/custody/interception/app-blocking overclaim rejection, public
  package export visibility, and required non-claim coverage.
- TypeScript schema tests and a proof harness for package-source capture status
  rows, missing platform/status coverage rejection, missing child artifact/store
  status refs, missing artifact/audit/report refs, provider/store/portal/
  adapter/child delivery/report delivery/custody/interception/app-blocking
  overclaim rejection, and required non-claim coverage.
- TypeScript schema tests and a proof harness for child-device delivery runtime
  writer rows, missing runtime writer/package-source/audit/report coverage
  rejection, runtime writer/provider/store/adapter/parent action/child
  delivery/report delivery/custody/interception/app-blocking overclaim
  rejection, and required non-claim coverage.
- TypeScript schema tests and a proof harness for package-source adapter
  execution rows, missing adapter state coverage rejection, missing capture
  status/artifact/audit/attempt/report refs, provider/store/portal/production
  adapter/child delivery/report delivery/custody/interception/app-blocking
  overclaim rejection, public package export visibility, and required non-claim
  coverage.
- TypeScript schema tests and a proof harness for parent action delivery
  readiness rows, missing parent action handoff/child envelope/audit/report
  coverage rejection, parent action delivery/runtime writer/provider/store/
  adapter/child delivery/report delivery/custody/interception/app-blocking
  overclaim rejection, and required non-claim coverage.
- TypeScript schema tests and a proof harness for provider/store execution
  readiness rows, missing approved API/store handoff/package adapter/parent
  action readiness coverage rejection, Google Play/Apple App Store/Microsoft
  Store/provider contact/provider execution/store integration/platform
  interception/adapter/child delivery/runtime writer/app-blocking/custody
  overclaim rejection, and required non-claim coverage.
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
- Do not treat status runtime readiness rows as a runtime status reader,
  child-device delivery, runtime report delivery, store integration, platform
  adapter implementation, or app blocking.
- Do not treat parent-owned platform artifact proof refs as approved store APIs,
  platform adapters, child-device package captures, runtime report delivery, or
  app blocking.
- Do not treat child artifact/delivery proof refs as production child-device
  package capture, provider/store integration, platform adapters, child-device
  delivery, runtime report delivery, child activity custody, interception, or
  app blocking.
- Do not treat approved API/entitlement evidence refs as provider API
  execution, store integration, platform adapter implementation, child-device
  delivery, runtime report delivery, child activity custody, interception, or
  app blocking.
- Do not treat report-runtime status proof refs as portal report UI, runtime
  report writer/delivery, provider/store execution, platform adapters,
  child-device delivery, child activity custody, app blocking, or
  Ocentra-hosted family data custody.
- Do not treat platform adapter boundary refs as implemented platform adapters,
  provider API execution, store integration, child-device delivery, runtime
  report delivery, interception, child activity custody, app blocking, or
  Ocentra-hosted family data custody.
- Do not treat parent review action proof refs as portal approval UI, parent
  action runtime delivery, provider/store execution, platform adapters,
  child-device delivery, child activity custody, interception, app blocking, or
  Ocentra-hosted family data custody.
- Do not treat parent action runtime handoff proof refs as portal approval UI,
  runtime action writer implementation, parent action runtime delivery,
  provider/store execution, platform adapters, child-device delivery, runtime
  report delivery, child activity custody, interception, app blocking, or
  Ocentra-hosted family data custody.
- Do not treat store status handoff proof refs as provider/store status API
  execution, store integration, platform adapter implementation, parent action
  runtime delivery, child-device delivery, runtime report delivery,
  interception, child activity custody, app blocking, or Ocentra-hosted family
  data custody.
- Do not treat runtime writer delivery proof refs as runtime writer
  implementation, runtime writer delivery, parent action runtime delivery,
  provider/store execution, platform adapter implementation, child-device
  delivery, runtime report delivery, interception, child activity custody, app
  blocking, or Ocentra-hosted family data custody.
- Do not treat package-source capture status proof refs as provider/store API
  execution, store integration, portal approval UI, platform adapter
  implementation, child-device delivery, runtime report delivery, child
  activity custody, interception, app blocking, or Ocentra-hosted family data
  custody.
- Do not treat child-device delivery runtime writer proof refs as runtime
  writer execution, runtime writer delivery, parent action runtime delivery,
  provider/store API execution, store integration, platform adapter
  implementation, child-device delivery, runtime report delivery, child
  activity custody, interception, app blocking, or Ocentra-hosted family data
  custody.
- Do not treat package-source adapter execution proof refs as provider/store
  API execution, store integration, portal approval UI, production platform
  adapters, child-device delivery, runtime report delivery, child activity
  custody, interception, app blocking, or Ocentra-hosted family data custody.
- Do not treat parent action delivery readiness proof refs as parent action
  runtime delivery, runtime writer execution, runtime writer delivery,
  provider/store API execution, store integration, platform adapter
  implementation, child-device delivery, runtime report delivery, child
  activity custody, interception, app blocking, or Ocentra-hosted family data
  custody.
- Do not treat provider/store execution readiness proof refs as Google Play,
  Apple App Store, Microsoft Store, or billing provider execution/contact,
  provider API execution, store integration, platform interception, platform
  adapter implementation, child-device delivery, runtime writer delivery, app
  blocking, child activity custody, or Ocentra-hosted family data custody.

## Done Signal

A parent can review install/purchase requests with platform-backed metadata and
typed approval/audit behavior, or the product clearly states that the platform
does not allow the capability.
