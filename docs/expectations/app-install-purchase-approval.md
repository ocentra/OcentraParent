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
- Child-device delivery readiness proof rows link child-device delivery
  runtime-writer envelope rows, package-source adapter execution rows, and
  platform limitation action rows into delivery evidence ready, manual proof
  required, platform unavailable, and policy blocked states without claiming
  child-device delivery, runtime writer execution, runtime writer delivery,
  provider API execution, store integration, platform adapter implementation,
  app blocking, child activity data custody, or Ocentra-hosted family data
  custody.
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
- Provider/store report status runtime proof rows link provider/store execution
  readiness rows to parent-owned runtime report writer receipts so store
  report statuses can be ready, manual-required, or unavailable without
  claiming provider API execution, store integration, portal report UI,
  external runtime report delivery, platform adapter implementation,
  child-device delivery, app blocking, child activity data custody, or
  Ocentra-hosted family data custody.
- Approval/report domain proof rows link parent review action decisions and
  report-runtime refs into approval/report-ready or manual-review rows without
  claiming portal approval UI, portal report UI, runtime report delivery,
  provider/store execution, platform adapter implementation, child-device
  delivery, real interception, app blocking, child activity data custody, or
  Ocentra-hosted family data custody.
- Runtime writer execution delivery proof rows link runtime writer delivery
  rows and parent action delivery readiness rows into deterministic
  parent-owned runtime writer envelopes and delivery result receipts without
  claiming provider API execution, store integration, platform interception,
  platform adapter implementation, child-device delivery, runtime report
  delivery, real interception, app blocking, child activity data custody, or
  Ocentra-hosted family data custody.
- Provider/store execution preflight proof rows link provider/store execution
  readiness rows and runtime writer execution delivery receipts into
  parent-owned execution-ready, manual-required, and unavailable preflight states
  without claiming Google Play execution, Apple App Store execution, Microsoft
  Store execution, billing/provider contact, provider API execution, store
  integration, platform interception, platform adapter implementation, runtime
  device delivery, child-device delivery, app blocking, child activity data
  custody, or Ocentra-hosted family data custody.
- Runtime report writer delivery proof rows link runtime writer execution
  delivery receipts and report-runtime compiler output rows into parent-owned
  report delivery-ready rows and report receipts without claiming portal report
  UI, external runtime report delivery, provider API execution, store
  integration, platform interception, platform adapter implementation,
  child-device delivery, app blocking, child activity data custody, or
  Ocentra-hosted family data custody.
- Report status read-model handoff proof rows link approval/report domain rows
  and runtime report writer delivery rows into parent-visible report status
  ready/manual-required rows without claiming portal report UI, external runtime
  report delivery, provider API execution, store integration, platform adapter
  implementation, child-device delivery, app blocking, child activity data
  custody, or Ocentra-hosted family data custody.
- Limitation summary proof rows aggregate provider/store report status rows and
  report status read-model rows into parent-visible ready, manual-required, and
  unavailable buckets without claiming portal approval UI, portal report UI,
  external runtime report delivery, provider API execution, store integration,
  billing provider contact, platform adapter implementation, child-device
  delivery, app blocking, child activity data custody, or Ocentra-hosted family
  data custody.
- Platform limitation action proof rows link provider/store report status rows
  and report status read-model rows into parent-visible follow-up action rows
  for ready, manual-required, and unavailable platforms without claiming portal
  approval UI, portal report UI, external runtime report delivery, provider API
  execution, store integration, billing provider contact, platform adapter
  implementation, child-device delivery, app blocking, child activity data
  custody, or Ocentra-hosted family data custody.
- Platform proof readiness rows link limitation-summary proof rows to
  platform-specific manual evidence requirements for Windows, macOS, Linux,
  Android, and iOS without claiming Google Play execution, Apple App Store
  execution, Microsoft Store execution, provider API execution, store
  integration, platform adapter implementation, child-device delivery, runtime
  report delivery, app blocking, child activity data custody, or
  Ocentra-hosted family data custody.
- Store manual evidence proof rows map platform proof readiness rows to
  Microsoft Store, Mac App Store, Linux package manager, Google Play, and Apple
  App Store manual evidence states without claiming provider API execution,
  store integration, platform adapter implementation, runtime writer delivery,
  runtime report delivery, child-device delivery, app blocking, child activity
  data custody, or Ocentra-hosted family data custody.
- Provider/store manual evidence packet proof rows link platform proof
  readiness rows and provider/store preflight rows into parent-owned
  packet-ready, manual-review-required, and provider-unavailable packet states
  without claiming provider API execution, store integration, platform adapter
  implementation, runtime writer delivery, runtime report delivery,
  child-device delivery, app blocking, child activity data custody, or
  Ocentra-hosted family data custody.
- Product-claim gate proof rows link provider/store manual evidence packet rows
  to denied, manual-required, or blocked product-claim gate states until portal
  approval/report tests, child-device delivery proof, provider/store API
  execution proof, and platform adapter proof are present without claiming
  portal approval UI, portal report UI, provider API execution, store
  integration, platform adapter implementation, child-device delivery, runtime
  writer delivery, runtime report delivery, app blocking, child activity data
  custody, or Ocentra-hosted family data custody.
- Product-claim safe parent workflow rows consume product-claim gate rows into
  safe parent review, manual-parent-review-required, and unsupported-store
  workflow states while product claims remain unapproved and portal approval UI,
  portal report UI, provider API execution, store integration, platform adapter
  implementation, child-device delivery, runtime writer/report delivery, app
  blocking, child activity custody, and Ocentra-hosted family data custody stay
  unclaimed.
- Product-claim portal test readiness rows consume product-claim gate rows into
  portal-test-ready, manual-portal-test-required, and unsupported portal test
  states that name portal approval/report test refs before product claims while
  portal approval UI, portal report UI, provider API execution, store
  integration, platform adapter implementation, child-device delivery, runtime
  writer/report delivery, app blocking, child activity custody, and
  Ocentra-hosted family data custody stay unclaimed.
- Product-claim provider/store rows consume product-claim gate rows and
  provider/store execution preflight rows so provider/store proof remains
  required, manual-required, or unsupported before any product claim while
  Google Play execution, Apple App Store execution, Microsoft Store execution,
  billing/provider contact, provider API execution, store integration, platform
  interception, platform adapter implementation, runtime device delivery,
  child-device delivery, portal approval/report UI, app blocking, child
  activity custody, and Ocentra-hosted family data custody stay unclaimed.
- Product-claim store handoff rows link safe parent workflow rows with
  provider/store manual evidence packet rows into parent-visible store handoff
  review-ready, manual-required, and unavailable states while product claims
  remain unapproved and portal approval UI, portal report UI, provider API
  execution, store integration, platform adapter implementation,
  child-device delivery, runtime writer/report delivery, app blocking, child
  activity custody, and Ocentra-hosted family data custody stay unclaimed.
- Product-claim store upgrade readiness rows consume product-claim gate, portal
  test readiness, and provider/store proof rows so upgrade readiness remains
  blocked, manual-required, or unsupported before any product claim while
  product claim approval, Google Play execution, Apple App Store execution,
  Microsoft Store execution, billing/provider contact, provider API execution,
  store integration, platform interception, platform adapter implementation,
  runtime writer/report delivery, child-device delivery, portal approval/report
  UI, app blocking, child activity custody, and Ocentra-hosted family data
  custody stay unclaimed.
- Product-claim platform preclaim rows consume portal-test readiness rows and
  platform-proof readiness rows so product claims remain manual-required or
  unsupported before any claim while product claim approval, portal
  approval/report UI, Google Play execution, Apple App Store execution,
  Microsoft Store execution, provider API execution, store integration,
  platform adapter implementation, child-device delivery, runtime writer/report
  delivery, app blocking, child activity custody, and Ocentra-hosted family data
  custody stay unclaimed.

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
- TypeScript schema tests and a proof harness for provider/store report status
  runtime rows, missing provider/store readiness or runtime report writer
  receipt coverage rejection, provider/store execution/store integration/portal
  report UI/external report delivery/platform adapter/child delivery/
  app-blocking/custody overclaim rejection, and required non-claim coverage.
- TypeScript schema tests and a proof harness for approval/report domain rows,
  missing parent review action/report-runtime coverage rejection, portal
  approval/report UI/runtime report delivery/provider/store/adapter/child
  delivery/interception/app-blocking/custody overclaim rejection, public package
  export visibility, and required non-claim coverage.
- TypeScript schema tests and a proof harness for runtime writer execution
  delivery rows, missing runtime writer delivery/parent action readiness/
  envelope/receipt/audit/report coverage rejection, provider/store/platform/
  child delivery/report delivery/app-blocking/custody overclaim rejection, and
  required non-claim coverage. Public package export visibility remains a
  required follow-up once `packages/parent-domain/package.json` is available.
- TypeScript schema tests and a proof harness for provider/store execution
  preflight rows, missing provider/store readiness or runtime writer execution
  delivery receipt coverage rejection, provider/store execution/store
  integration/billing contact/platform adapter/runtime device delivery/child
  delivery/app-blocking/custody overclaim rejection, and required non-claim
  coverage. Product checklist visibility remains pending while the current
  checklist lock is owned by E-C.
- TypeScript schema tests and a proof harness for child-device delivery
  readiness rows, missing child delivery runtime-writer/package adapter/platform
  limitation/status refs, child delivery/provider/store/adapter/app-blocking/
  custody overclaim rejection, and required non-claim coverage.
- TypeScript schema tests and a proof harness for runtime report writer
  delivery rows, missing runtime writer execution/report-runtime/output/
  receipt/audit coverage rejection, provider/store/platform/child delivery/
  portal report UI/report delivery/app-blocking/custody overclaim rejection,
  public package export visibility, and required non-claim coverage.
- TypeScript schema tests and a proof harness for report status read-model
  handoff rows, missing approval/report domain and runtime report writer refs,
  missing parent-visible status refs, portal report UI/external delivery/
  provider/store/platform adapter/child delivery/app-blocking/custody overclaim
  rejection, and required non-claim coverage. Public package export visibility
  remains a required follow-up once `packages/parent-domain/package.json` is
  available.
- TypeScript schema tests and a proof harness for limitation summary rows,
  missing ready/manual/unavailable bucket coverage rejection, source provider/
  store status mismatch rejection, missing parent-visible summary refs, portal
  approval/report UI/external delivery/provider/store/billing contact/platform
  adapter/child delivery/app-blocking/custody overclaim rejection, and required
  non-claim coverage.
- TypeScript schema tests and a proof harness for platform limitation action
  rows, missing ready/manual/unavailable follow-up coverage rejection, missing
  provider/store status and report status read-model refs, portal approval/
  report UI/external delivery/provider/store/billing contact/platform adapter/
  child delivery/app-blocking/custody overclaim rejection, and required
  non-claim coverage.
- TypeScript schema tests and a proof harness for platform proof readiness rows,
  missing platform coverage rejection, missing limitation-summary refs, missing
  manual evidence refs, provider/store/platform/child delivery/report delivery/
  app-blocking/custody overclaim rejection, and required non-claim coverage.
- TypeScript schema tests and a proof harness for store manual evidence rows,
  missing store coverage rejection, missing platform-proof-readiness source
  version rejection, missing manual evidence refs, provider/store/platform/
  runtime writer/report/child delivery/app-blocking/custody overclaim
  rejection, and required non-claim coverage.
- TypeScript schema tests and a proof harness for provider/store manual
  evidence packet rows, missing store coverage rejection, missing platform
  readiness or provider/store preflight refs, missing manual/provider evidence
  refs, provider/store/platform/runtime writer/report/child delivery/
  app-blocking/custody overclaim rejection, and required non-claim coverage.
- TypeScript schema tests and a proof harness for product-claim gate rows,
  missing store coverage rejection, missing portal test refs, missing child
  delivery refs, missing provider/store API execution refs, missing platform
  adapter refs, product-claim overclaim rejection, portal/provider/store/
  platform/child delivery/app-blocking/custody overclaim rejection, and
  required non-claim coverage.
- TypeScript schema tests and a proof harness for product-claim safe parent
  workflow rows, source gate linkage, workflow coverage, required follow-up
  refs, portal/provider/store/platform/child delivery/runtime/app-blocking/
  custody overclaim rejection, and required non-claim coverage.
- TypeScript schema tests and a proof harness for product-claim portal test
  readiness rows, source gate linkage, portal approval/report test refs,
  required child/provider/platform follow-up refs, portal/provider/store/
  platform/child delivery/runtime/app-blocking/custody overclaim rejection, and
  required non-claim coverage.
- TypeScript schema tests and a proof harness for product-claim provider/store
  rows, product-claim gate linkage, provider/store preflight linkage, required
  provider/store execution refs, provider evidence refs, runtime receipt refs,
  portal/provider/store/platform/child delivery/runtime/app-blocking/custody
  overclaim rejection, public package export visibility, and required non-claim
  coverage.
- TypeScript schema tests and a proof harness for product-claim store handoff
  rows, source safe parent workflow linkage, manual evidence packet linkage,
  required portal, child delivery, provider/store execution, platform adapter,
  manual evidence, audit, and report refs, product-claim approval/provider/
  portal/delivery/app-blocking/custody overclaim rejection, and required
  non-claim coverage.
- TypeScript schema tests and a proof harness for product-claim store upgrade
  readiness rows, product-claim gate linkage, portal test readiness linkage,
  provider/store product-claim linkage, required portal/provider/child/platform/
  runtime refs, product-claim approval/provider/store/platform/child delivery/
  portal/runtime/app-blocking/custody overclaim rejection, public package export
  visibility, and required non-claim coverage.
- TypeScript schema tests and a proof harness for product-claim platform
  preclaim rows, portal-test readiness linkage, platform-proof readiness
  linkage, required portal/manual-platform/child/provider/platform/report refs,
  product-claim approval, portal UI, provider/store/platform/child delivery/
  runtime/app-blocking/custody overclaim rejection, public package export
  visibility, and required non-claim coverage.
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
- Do not treat child-device delivery readiness proof refs as child-device
  delivery, runtime writer execution, runtime writer delivery, provider/store
  API execution, store integration, platform adapter implementation, app
  blocking, child activity custody, or Ocentra-hosted family data custody.
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
- Do not treat provider/store report status runtime proof refs as provider API
  execution, store integration, portal report UI, external runtime report
  delivery, platform adapter implementation, child-device delivery, app
  blocking, child activity custody, or Ocentra-hosted family data custody.
- Do not treat approval/report domain proof refs as portal approval UI, portal
  report UI, runtime report delivery, provider/store execution, platform
  adapter implementation, child-device delivery, interception, app blocking,
  child activity custody, or Ocentra-hosted family data custody.
- Do not treat runtime writer execution delivery proof refs as provider API
  execution, store integration, platform interception, platform adapter
  implementation, child-device delivery, runtime report delivery, real
  interception, app blocking, child activity custody, or Ocentra-hosted family
  data custody.
- Do not treat provider/store execution preflight proof refs as Google Play,
  Apple App Store, Microsoft Store, billing provider execution/contact,
  provider API execution, store integration, platform interception, platform
  adapter implementation, runtime device delivery, child-device delivery, app
  blocking, child activity custody, or Ocentra-hosted family data custody.
- Do not treat runtime report writer delivery proof refs as portal report UI,
  external runtime report delivery, provider API execution, store integration,
  platform interception, platform adapter implementation, child-device delivery,
  app blocking, child activity custody, or Ocentra-hosted family data custody.
- Do not treat report status read-model handoff proof refs as portal report UI,
  external runtime report delivery, provider API execution, store integration,
  platform adapter implementation, child-device delivery, app blocking, child
  activity custody, or Ocentra-hosted family data custody.
- Do not treat platform limitation action proof refs as portal approval UI,
  portal report UI, external runtime report delivery, provider API execution,
  store integration, billing provider contact, platform adapter implementation,
  child-device delivery, app blocking, child activity custody, or
  Ocentra-hosted family data custody.
- Do not treat platform proof readiness refs as Google Play, Apple App Store,
  Microsoft Store, provider API execution, store integration, platform adapter
  implementation, child-device delivery, runtime report delivery, app blocking,
  child activity custody, or Ocentra-hosted family data custody.
- Do not treat store manual evidence proof refs as Google Play, Apple App Store,
  Microsoft Store, provider API execution, store integration, platform adapter
  implementation, runtime writer delivery, runtime report delivery,
  child-device delivery, app blocking, child activity custody, or
  Ocentra-hosted family data custody.
- Do not treat provider/store manual evidence packet proof refs as Google Play,
  Apple App Store, Microsoft Store, provider API execution, store integration,
  platform adapter implementation, runtime writer delivery, runtime report
  delivery, child-device delivery, app blocking, child activity custody, or
  Ocentra-hosted family data custody.
- Do not treat product-claim gate proof refs as portal approval UI, portal
  report UI, provider API execution, store integration, platform adapter
  implementation, child-device delivery, runtime writer delivery, runtime
  report delivery, app blocking, child activity custody, Ocentra-hosted family
  data custody, or an approved product claim.
- Do not treat product-claim safe parent workflow refs as portal approval UI,
  portal report UI, provider API execution, store integration, platform adapter
  implementation, child-device delivery, runtime writer delivery, runtime
  report delivery, app blocking, child activity custody, Ocentra-hosted family
  data custody, or an approved product claim.
- Do not treat product-claim portal test readiness refs as portal approval UI,
  portal report UI, provider API execution, store integration, platform adapter
  implementation, child-device delivery, runtime writer delivery, runtime
  report delivery, app blocking, child activity custody, Ocentra-hosted family
  data custody, or an approved product claim.
- Do not treat product-claim provider/store proof refs as Google Play, Apple
  App Store, Microsoft Store, or billing provider execution/contact, provider
  API execution, store integration, platform interception, platform adapter
  implementation, runtime device delivery, child-device delivery, portal
  approval/report UI, app blocking, child activity custody, Ocentra-hosted
  family data custody, or an approved product claim.
- Do not treat product-claim store handoff refs as approved product claims,
  portal approval UI, portal report UI, provider API execution, store
  integration, platform adapter implementation, child-device delivery, runtime
  writer delivery, runtime report delivery, app blocking, child activity
  custody, Ocentra-hosted family data custody, or store integration proof.
- Do not treat product-claim store upgrade readiness proof refs as product claim
  approval, Google Play, Apple App Store, Microsoft Store, or billing provider
  execution/contact, provider API execution, store integration, platform
  interception, platform adapter implementation, runtime writer/report delivery,
  child-device delivery, portal approval/report UI, app blocking, child activity
  custody, Ocentra-hosted family data custody, or a product-claim upgrade.
- Do not treat product-claim platform preclaim proof refs as product claim
  approval, portal approval UI, portal report UI, Google Play execution, Apple
  App Store execution, Microsoft Store execution, provider API execution, store
  integration, platform adapter implementation, child-device delivery, runtime
  writer/report delivery, app blocking, child activity custody,
  Ocentra-hosted family data custody, or proof that platform/store claims are
  product-ready.

## Done Signal

A parent can review install/purchase requests with platform-backed metadata and
typed approval/audit behavior, or the product clearly states that the platform
does not allow the capability.
