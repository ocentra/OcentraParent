# App Install And Purchase Approval

## Parent Outcome

Parents can review new app installs, purchases, subscriptions, and sensitive app
permissions where the platform exposes a safe control path. They can approve,
deny, time-box, or mark review-needed with audit history.

## Ocentra Requirement

Install approval is separate from app blocking. Ocentra must not imply Google
Play, Apple App Store, Microsoft Store, or device-management parity unless the
platform path is proved. Unsupported platforms need explicit unavailable states.

## Roadmap And Expectations

- Roadmap: V5 parent policy product, V6 mobile agents.
- Expectations:
  [app install and purchase approval](../expectations/app-install-purchase-approval.md),
  [policy](../expectations/policy.md),
  [platforms](../expectations/platforms.md).
- Modules: `packages/parent-domain`, `packages/portal-domain`,
  `platforms/android`, `platforms/ios`.

## Competitor Pressure

See [Competitor Capability Map](../competitor-capability-map.md), especially
install approval and purchases, app inventory, app block/app limits, and
platform restrictions.

Google and Apple own app-store approval deeply. Microsoft has family purchase
and app controls. Ocentra needs a platform-specific answer instead of pretending
generic app blocking covers this concern.

## Current Ocentra State

- Expectation doc exists.
- Policy and platform docs now track the requirement.
- `packages/parent-domain/src/app-install-purchase-approval.ts` now defines
  contract-only install requests, purchase/subscription requests, store
  metadata freshness/source states, approval decisions, approval expiry and
  review-needed states, audit event refs, platform
  support/manual-required/unavailable rows, platform-source metadata limitation
  rows, package-source artifact requirement rows, child-facing pending/result
  state rows, audit/report integration status rows, and explicit non-claims.
- `scripts/test/app-install-purchase-approval-contract-proof.mjs` records the
  contract proof and non-claims under
  `test-results/app-install-purchase-approval-contract-proof/proof.json` when
  run.
- `scripts/test/app-install-package-source-artifact-proof.mjs` records the
  package-source artifact row proof under
  `test-results/app-install-package-source-artifact-proof/proof.json` when run.
- `packages/parent-domain/src/app-install-purchase-runtime-proof.ts` now records
  a runtime-boundary proof that links platform/store metadata artifact
  requirements, package-source artifact requirements, child pending/result
  delivery rows, report integration rows, and status runtime readiness rows
  while keeping store/provider, runtime status reader, child-device delivery,
  runtime report delivery, and app-blocking behavior unclaimed.
- `scripts/test/app-install-purchase-runtime-proof.mjs` records that runtime
  boundary under `test-results/app-install-purchase-runtime-proof/proof.json`
  when run.
- `packages/parent-domain/src/app-install-purchase-platform-artifact-proof.ts`
  now attaches parent-owned platform/store metadata artifact refs and
  report-runtime evidence refs to the runtime boundary while preserving
  no-provider-API, no-store-integration, no-platform-adapter,
  no-child-device-delivery, no-runtime-report-delivery, no-interception, and
  not-generic-app-blocking non-claims.
- `scripts/test/app-install-purchase-platform-artifact-proof.mjs` records that
  platform artifact proof under
  `test-results/app-install-purchase-platform-artifact-proof/proof.json` when
  run.
- `packages/parent-domain/src/app-install-purchase-child-artifact-delivery-proof.ts`
  now links child package-source artifact refs to platform/report artifact
  proof rows and child pending/result delivery boundaries while preserving
  no-runtime-capture, no-child-device-delivery, no-provider-API,
  no-platform-adapter, no-runtime-report-delivery, no-interception,
  no-child-activity-data, and not-generic-app-blocking non-claims.
- `scripts/test/app-install-purchase-child-artifact-delivery-proof.mjs` records
  that child artifact/delivery boundary proof under
  `test-results/app-install-purchase-child-artifact-delivery-proof/proof.json`
  when run.
- `packages/parent-domain/src/app-install-purchase-approved-api-entitlement-proof.ts`
  now attaches approved store API evidence refs, entitlement evidence refs, and
  limitation/audit refs to the child artifact proof rows while preserving
  no-provider-API-execution, no-store-integration, no-platform-adapter,
  no-child-device-delivery, no-runtime-report-delivery, no-interception,
  no-child-activity-data, and not-generic-app-blocking non-claims.
- `scripts/test/app-install-purchase-approved-api-entitlement-proof.mjs`
  records that approved API/entitlement evidence proof under
  `test-results/app-install-purchase-approved-api-entitlement-proof/proof.json`
  when run.
- `packages/parent-domain/src/app-install-purchase-report-runtime-proof.ts`
  now links app-install report surfaces and child artifact refs to the existing
  stateless report compiler status/result contract while preserving no portal
  report UI, no runtime report delivery, no provider/store execution, no
  platform adapter, no child-device delivery, no child activity data, no app
  blocking, and no Ocentra-hosted family data custody claims.
- `scripts/test/app-install-purchase-report-runtime-proof.mjs` records that
  report-runtime status proof under
  `test-results/app-install-purchase-report-runtime-proof/proof.json` when run.
- `packages/parent-domain/src/app-install-purchase-platform-adapter-boundary-proof.ts`
  now links approved API/entitlement evidence rows and report-runtime refs to
  platform adapter readiness/manual/unavailable boundary rows while preserving
  no platform adapter implementation, no provider/store execution, no
  child-device delivery, no runtime report delivery, no interception, no child
  activity data, no app blocking, and no Ocentra-hosted family data custody
  claims.
- `scripts/test/app-install-purchase-platform-adapter-boundary-proof.mjs`
  records that platform adapter boundary proof under
  `test-results/app-install-purchase-platform-adapter-boundary-proof/proof.json`
  when run.
- `packages/parent-domain/src/app-install-purchase-parent-review-action-proof.ts`
  now links approval decision actions to approved API/entitlement evidence refs
  and report-runtime refs while preserving no portal approval UI, no parent
  action runtime delivery, no provider/store execution, no platform adapter, no
  child-device delivery, no child activity data, no app blocking, and no
  Ocentra-hosted family data custody claims.
- `scripts/test/app-install-purchase-parent-review-action-proof.mjs` records
  that parent review action proof under
  `test-results/app-install-purchase-parent-review-action-proof/proof.json`
  when run.
- `packages/parent-domain/src/app-install-purchase-parent-action-runtime-handoff-proof.ts`
  now links parent review actions to runtime handoff status rows and platform
  adapter boundary refs while preserving no portal approval UI, runtime action
  writer implementation, parent action runtime delivery, provider/store
  execution, platform adapter implementation, child-device delivery, runtime
  report delivery, child activity data, app blocking, or Ocentra-hosted family
  data custody claims.
- `scripts/test/app-install-purchase-parent-action-runtime-handoff-proof.mjs`
  records that parent action runtime handoff proof under
  `test-results/app-install-purchase-parent-action-runtime-handoff-proof/proof.json`
  when run.
- `packages/parent-domain/src/app-install-purchase-store-status-handoff-proof.ts`
  now links parent action runtime handoff rows and platform adapter boundary
  rows to per-store status handoff states while preserving no provider/store
  execution, no store integration, no platform adapter implementation, no
  parent action runtime delivery, no child-device delivery, no runtime report
  delivery, no interception, no child activity data, no app blocking, and no
  Ocentra-hosted family data custody claims.
- `scripts/test/app-install-purchase-store-status-handoff-proof.mjs` records
  that store status handoff proof under
  `test-results/app-install-purchase-store-status-handoff-proof/proof.json`
  when run.
- `packages/parent-domain/src/app-install-purchase-runtime-writer-delivery-proof.ts`
  now links parent action runtime handoff rows and per-store status handoff rows
  to writer-envelope-ready/manual-required proof states while preserving no
  runtime writer implementation, no runtime writer delivery, no parent action
  runtime delivery, no provider/store execution, no platform adapter
  implementation, no child-device delivery, no runtime report delivery, no
  interception, no child activity data, no app blocking, and no Ocentra-hosted
  family data custody claims.
- `scripts/test/app-install-purchase-runtime-writer-delivery-proof.mjs` records
  that runtime writer delivery boundary proof under
  `test-results/app-install-purchase-runtime-writer-delivery-proof/proof.json`
  when run.
- `packages/parent-domain/src/app-install-purchase-package-source-capture-status-proof.ts`
  now links child package-source artifact refs and store status handoff rows to
  captured, blocked, manual-required, and unavailable package-source capture
  status rows with artifact, audit, report, and platform limitation refs while
  preserving no provider/store execution, no store integration, no portal
  approval UI, no platform adapter implementation, no child-device delivery, no
  runtime report delivery, no interception, no child activity data, no app
  blocking, and no Ocentra-hosted family data custody claims.
- `scripts/test/app-install-purchase-package-source-capture-status-proof.mjs`
  records that package-source capture status proof under
  `test-results/app-install-purchase-package-source-capture-status-proof/proof.json`
  when run.
- `packages/parent-domain/src/app-install-purchase-child-device-delivery-runtime-writer-proof.ts`
  now links runtime writer delivery rows and package-source capture/status rows
  to child-device delivery envelope/manual-required rows while preserving no
  runtime writer execution, runtime writer delivery, parent action runtime
  delivery, provider/store execution, platform adapter implementation,
  child-device delivery, runtime report delivery, interception, child activity
  data, app blocking, or Ocentra-hosted family data custody claims.
- `scripts/test/app-install-purchase-child-device-delivery-runtime-writer-proof.mjs`
  records that child-device delivery runtime writer proof under
  `test-results/app-install-purchase-child-device-delivery-runtime-writer-proof/proof.json`
  when run.
- `packages/parent-domain/src/app-install-purchase-child-device-delivery-readiness-proof.ts`
  now links child-device delivery runtime-writer envelope rows,
  package-source adapter execution rows, and platform limitation action rows
  into child delivery readiness/manual/unavailable/policy-blocked evidence
  states while preserving no child-device delivery, runtime writer execution or
  delivery, provider/store execution, platform adapter implementation, app
  blocking, child activity data, or Ocentra-hosted family data custody claims.
- `scripts/test/app-install-purchase-child-device-delivery-readiness-proof.mjs`
  records that child-device delivery readiness proof under
  `test-results/app-install-purchase-child-device-delivery-readiness-proof/proof.json`
  when run.
- `packages/parent-domain/src/app-install-purchase-package-source-adapter-execution-proof.ts`
  now links package-source capture/status rows to local Windows, manual macOS,
  unavailable Linux, and blocked Android/iOS package-source adapter execution
  states while preserving no provider/store execution, no store integration, no
  portal approval UI, no production platform adapter, no child-device delivery,
  no runtime report delivery, no interception, no child activity data, no app
  blocking, and no Ocentra-hosted family data custody claims.
- `scripts/test/app-install-purchase-package-source-adapter-execution-proof.mjs`
  records that package-source adapter execution proof under
  `test-results/app-install-purchase-package-source-adapter-execution-proof/proof.json`
  when run.
- `packages/parent-domain/src/app-install-purchase-parent-action-delivery-readiness-proof.ts`
  now links parent action runtime handoff rows to child-device delivery
  runtime-writer envelope rows so parent actions can be classified as
  delivery-ready or manual-review-required while preserving no parent action
  runtime delivery, runtime writer execution/delivery, provider/store
  execution, platform adapters, child-device delivery, runtime report delivery,
  interception, child activity data, app blocking, or Ocentra-hosted family data
  custody claims.
- `scripts/test/app-install-purchase-parent-action-delivery-readiness-proof.mjs`
  records that parent action delivery readiness proof under
  `test-results/app-install-purchase-parent-action-delivery-readiness-proof/proof.json`
  when run. The proof records the public package export and product checklist
  row as pending lock-gated deltas because other lanes owned those files during
  this slice.
- `packages/parent-domain/src/app-install-purchase-provider-store-execution-readiness-proof.ts`
  now links approved API/entitlement evidence, per-store status handoff,
  package-source adapter execution, and parent action delivery readiness rows
  into provider/store execution-ready, manual-required, and unavailable states
  while preserving no Google Play, Apple App Store, Microsoft Store,
  billing/provider contact, provider/store execution, platform interception,
  platform adapters, child-device delivery, runtime writer delivery, app
  blocking, child activity data, or Ocentra-hosted family data custody claims.
- `scripts/test/app-install-purchase-provider-store-execution-readiness-proof.mjs`
  records that provider/store execution readiness proof under
  `test-results/app-install-purchase-provider-store-execution-readiness-proof/proof.json`
  when run. The proof records the public package export and product checklist
  row as pending lock-gated deltas because other lanes owned those files during
  this slice.
- `packages/parent-domain/src/app-install-purchase-provider-store-report-status-runtime-proof.ts`
  now links provider/store execution readiness rows to parent-owned runtime
  report writer delivery receipts so store-specific report status runtime rows
  can be classified as ready, manual-required, or unavailable while preserving
  no provider/store execution, portal report UI, external runtime report
  delivery, platform adapter, child-device delivery, app blocking, child
  activity data, or Ocentra-hosted family data custody claims.
- `scripts/test/app-install-purchase-provider-store-report-status-runtime-proof.mjs`
  records that provider/store report status runtime proof under
  `test-results/app-install-purchase-provider-store-report-status-runtime-proof/proof.json`
  when run. The proof records the public package export and product checklist
  row as pending lock-gated deltas because other lanes owned those files during
  this slice.
- `packages/parent-domain/src/app-install-purchase-approval-report-domain-proof.ts`
  now links parent review actions and report-runtime rows into approval/report
  domain rows while preserving no portal approval UI, no portal report UI,
  no runtime report delivery, no provider/store execution, no platform adapter,
  no child-device delivery, no interception, no app blocking, no child activity
  data, and no Ocentra-hosted family data custody claims.
- `scripts/test/app-install-purchase-approval-report-domain-proof.mjs` records
  that approval/report domain proof under
  `test-results/app-install-purchase-approval-report-domain-proof/proof.json`
  when run.
- `packages/parent-domain/src/app-install-purchase-limitation-summary-proof.ts`
  now aggregates provider/store report status rows and report status read-model
  rows into parent-visible ready, manual-required, and unavailable limitation
  summary buckets while preserving no portal approval/report UI, no external
  runtime report delivery, no provider/store execution, no billing provider
  contact, no platform adapters, no child-device delivery, no app blocking, no
  child activity data, and no Ocentra-hosted family data custody claims.
- `scripts/test/app-install-purchase-limitation-summary-proof.mjs` records that
  limitation summary proof under
  `test-results/app-install-purchase-limitation-summary-proof/proof.json` when
  run.
- `packages/parent-domain/src/app-install-purchase-platform-proof-readiness.ts`
  now links merged limitation-summary rows to per-platform manual evidence
  requirements for Windows, macOS, Linux, Android, and iOS before product
  claims while preserving no Google Play, Apple App Store, Microsoft Store,
  provider/store execution, store integration, platform adapter, child-device
  delivery, runtime report delivery, app blocking, child activity data, or
  hosted family data custody claims.
- `scripts/test/app-install-purchase-platform-proof-readiness.mjs` records that
  platform proof readiness proof under
  `test-results/app-install-purchase-platform-proof-readiness/proof.json` when
  run.
- `packages/parent-domain/src/app-install-purchase-store-manual-evidence-proof.ts`
  now links platform proof readiness rows to Microsoft Store, Mac App Store,
  Linux package manager, Google Play, and Apple App Store manual evidence
  states while preserving no provider/store execution, no store integration, no
  platform adapter implementation, no runtime writer/report delivery, no
  child-device delivery, no app blocking, no child activity data, and no
  Ocentra-hosted family data custody claims.
- `scripts/test/app-install-purchase-store-manual-evidence-proof.mjs` records
  that store manual evidence proof under
  `test-results/app-install-purchase-store-manual-evidence-proof/proof.json`
  when run, with the product checklist row updated for the same proof movement.
- `packages/parent-domain/src/app-install-purchase-provider-store-manual-evidence-packet-proof.ts`
  now links platform proof readiness rows and provider/store preflight rows into
  parent-owned manual evidence packet-ready, manual-review-required, and
  provider-unavailable rows while preserving no provider/store execution, no
  store integration, no platform adapter implementation, no runtime
  writer/report delivery, no child-device delivery, no app blocking, no child
  activity data, and no Ocentra-hosted family data custody claims.
- `scripts/test/app-install-purchase-provider-store-manual-evidence-packet-proof.mjs`
  records that provider/store manual evidence packet proof under
  `test-results/app-install-purchase-provider-store-manual-evidence-packet-proof/proof.json`
  when run. The proof records the public package export as pending behind the
  active `packages/parent-domain/package.json` lock and the parent-domain
  README delta as pending behind the active README lock.
- `packages/parent-domain/src/app-install-purchase-product-claim-gate-proof.ts`
  now links provider/store manual evidence packet rows to product-claim gate
  rows that deny, keep manual-required, or block app-install product claims
  until portal approval/report tests, child delivery proof, provider/store API
  execution proof, and platform adapter proof are present. It preserves no
  portal approval/report UI, provider/store execution, store integration,
  platform adapters, child-device delivery, runtime writer/report delivery, app
  blocking, child activity data, or hosted family data custody claims.
- `scripts/test/app-install-purchase-product-claim-gate-proof.mjs` records
  that product-claim gate proof under
  `test-results/app-install-purchase-product-claim-gate-proof/proof.json` when
  run.
- `packages/parent-domain/src/app-install-purchase-product-claim-safe-parent-workflow-proof.ts`
  now consumes product-claim gate rows into safe parent review,
  manual-parent-review-required, and unsupported-store-workflow-blocked rows so
  parents have a closest safe workflow without upgrading product claims. It
  preserves no portal approval/report UI, provider/store execution, store
  integration, platform adapters, child-device delivery, runtime writer/report
  delivery, app blocking, child activity data, or hosted family data custody
  claims.
- `packages/parent-domain/src/app-install-purchase-product-claim-portal-test-readiness-proof.ts`
  now consumes product-claim gate rows into portal-test-ready,
  manual-portal-test-required, and unsupported-portal-test-blocked rows that
  name portal approval/report test refs required before product claims. It
  preserves no portal approval/report UI, provider/store execution, store
  integration, platform adapters, child-device delivery, runtime writer/report
  delivery, app blocking, child activity data, or hosted family data custody
  claims.
- `scripts/test/app-install-purchase-product-claim-safe-parent-workflow-proof.mjs`
  records that safe parent workflow proof under
  `test-results/app-install-purchase-product-claim-safe-parent-workflow-proof/proof.json`
  when run.
- `scripts/test/app-install-purchase-product-claim-portal-test-readiness-proof.mjs`
  records that portal test readiness proof under
  `test-results/app-install-purchase-product-claim-portal-test-readiness-proof/proof.json`
  when run.
- `packages/parent-domain/src/app-install-purchase-product-claim-provider-store-proof.ts`
  now links product-claim gate rows to provider/store execution preflight rows
  so provider/store proof stays required, manual-required, or unsupported before
  any product claim. It preserves no Google Play, Apple App Store, Microsoft
  Store, billing/provider contact, provider API execution, store integration,
  platform interception/adapters, runtime device delivery, child-device
  delivery, portal approval/report UI, app blocking, child activity data, or
  hosted family data custody claims.
- `scripts/test/app-install-purchase-product-claim-provider-store-proof.mjs`
  records that product-claim provider/store proof under
  `test-results/app-install-purchase-product-claim-provider-store-proof/proof.json`
  when run and validates the public parent-domain package export.
- `packages/parent-domain/src/app-install-purchase-product-claim-store-handoff-proof.ts`
  now links product-claim safe parent workflow rows with provider/store manual
  evidence packet rows into parent-visible store handoff review-ready,
  manual-required, and unavailable rows. It preserves no product-claim
  approval, portal approval/report UI, provider/store execution, store
  integration, platform adapters, child-device delivery, runtime writer/report
  delivery, app blocking, child activity data, or hosted family data custody
  claims.
- `scripts/test/app-install-purchase-product-claim-store-handoff-proof.mjs`
  records that product-claim store handoff proof under
  `test-results/app-install-purchase-product-claim-store-handoff-proof/proof.json`
  when run and validates the public parent-domain package export.
- `packages/parent-domain/src/app-install-purchase-product-claim-store-upgrade-readiness-proof.ts`
  now links product-claim gate, portal test readiness, and provider/store proof
  rows into product-claim store upgrade blocked, manual-required, and
  unsupported states. It preserves no product claim approval, provider/store
  execution, store integration, billing/provider contact, platform
  interception/adapters, runtime writer/report delivery, child-device delivery,
  portal approval/report UI, app blocking, child activity data, or hosted family
  data custody claims.
- `scripts/test/app-install-purchase-product-claim-store-upgrade-readiness-proof.mjs`
  records that product-claim store upgrade readiness proof under
  `test-results/app-install-purchase-product-claim-store-upgrade-readiness-proof/proof.json`
  when run and validates the public parent-domain package export.
- `packages/parent-domain/src/app-install-purchase-product-claim-platform-preclaim-proof.ts`
  now links product-claim portal test readiness rows with platform proof
  readiness rows into manual-required or unsupported preclaim rows before any
  product claim while preserving no product-claim approval, no portal
  approval/report UI, no provider/store execution, no store integration, no
  platform adapter implementation, no child-device delivery, no runtime
  writer/report delivery, no app blocking, no child activity data, and no
  Ocentra-hosted family data custody claims.
- `scripts/test/app-install-purchase-product-claim-platform-preclaim-proof.mjs`
  records that platform preclaim proof under
  `test-results/app-install-purchase-product-claim-platform-preclaim-proof/proof.json`
  when run and validates the public parent-domain package export.
- `packages/parent-domain/src/app-install-purchase-product-claim-platform-limitation-fallback-proof.ts`
  now links product-claim platform preclaim rows, safe parent workflow rows, and
  platform limitation action rows into fallback-ready, manual-required, and
  unsupported rows so parents retain the closest safe workflow while product
  claims remain blocked or manual. It preserves no product-claim approval,
  portal approval/report UI, Google Play, Apple App Store, Microsoft Store,
  billing provider contact, provider API execution, store integration, platform
  interception/adapters, child-device delivery, runtime writer/report delivery,
  app blocking, child activity data, or hosted custody claims.
- `scripts/test/app-install-purchase-product-claim-platform-limitation-fallback-proof.mjs`
  records that platform limitation fallback proof under
  `test-results/app-install-purchase-product-claim-platform-limitation-fallback-proof/proof.json`
  when run and validates the public parent-domain package export.
- `packages/parent-domain/src/app-install-purchase-provider-store-api-execution-proof.ts`
  now links product-claim provider/store rows with platform limitation fallback
  rows into provider/store API execution-ready, manual-required, unavailable,
  and blocked-before-claim evidence states. It preserves no product-claim
  approval, Google Play, Apple App Store, Microsoft Store, billing provider
  contact, provider API execution, store integration, platform
  interception/adapters, child-device delivery, runtime writer/report delivery,
  portal approval/report UI, app blocking, child activity data, or hosted
  custody claims.
- `scripts/test/app-install-purchase-provider-store-api-execution-proof.mjs`
  records that provider/store API execution proof under
  `test-results/app-install-purchase-provider-store-api-execution-proof/proof.json`
  when run and validates the public parent-domain package export.
- `packages/parent-domain/src/app-install-purchase-runtime-writer-execution-delivery-proof.ts`
  now converts runtime writer delivery rows and parent action delivery
  readiness rows into deterministic parent-owned runtime writer envelope and
  delivery result receipt rows for approve, deny, and time-box decisions while
  keeping review-needed manual-required. It preserves no provider/store
  execution, store integration, platform interception, platform adapter
  implementation, child-device delivery, runtime report delivery, real
  interception, app blocking, child activity data, or Ocentra-hosted family
  data custody claims.
- `scripts/test/app-install-purchase-runtime-writer-execution-delivery-proof.mjs`
  records that runtime writer execution delivery proof under
  `test-results/app-install-purchase-runtime-writer-execution-delivery-proof/proof.json`
  when run. The proof records the public package export as pending behind the
  active `packages/parent-domain/package.json` lock and the parent-domain
  README delta as pending behind the active README lock.
- `packages/parent-domain/src/app-install-purchase-provider-store-execution-preflight-proof.ts`
  links provider/store execution readiness rows and runtime writer execution
  delivery receipts into parent-owned provider/store execution preflight ready,
  manual-required, and unavailable rows without provider/store execution,
  provider API execution, store integration, billing/provider contact, platform
  interception/adapters, runtime device delivery, child-device delivery, app
  blocking, child activity data, or hosted family data custody claims.
- `scripts/test/app-install-purchase-provider-store-execution-preflight-proof.mjs`
  records that provider/store execution preflight proof under
  `test-results/app-install-purchase-provider-store-execution-preflight-proof/proof.json`
  when run. The proof records the product checklist row as pending behind the
  current E-C lock on `docs/product-capability-checklist.md`.
- `packages/parent-domain/src/app-install-purchase-runtime-report-writer-delivery-proof.ts`
  now links runtime writer execution delivery receipts and report-runtime
  compiler output rows into parent-owned runtime report writer delivery-ready
  rows and report receipts while keeping review-needed manual-required. It
  preserves no portal report UI, no external runtime report delivery, no
  provider/store execution, no platform interception/adapters, no child-device
  delivery, no app blocking, no child activity data, and no Ocentra-hosted
  family data custody claims.
- `scripts/test/app-install-purchase-runtime-report-writer-delivery-proof.mjs`
  records that runtime report writer delivery proof under
  `test-results/app-install-purchase-runtime-report-writer-delivery-proof/proof.json`
  when run and validates the public parent-domain package export.
- `packages/parent-domain/src/app-install-purchase-report-status-read-model-handoff-proof.ts`
  now links approval/report domain rows and runtime report writer delivery rows
  into parent-visible report status ready/manual-required rows while preserving
  no portal report UI, external runtime report delivery, provider/store
  execution, platform adapters, child-device delivery, app blocking, child
  activity data, or Ocentra-hosted family data custody claims.
- `scripts/test/app-install-purchase-report-status-read-model-handoff-proof.mjs`
  records that report status read-model handoff proof under
  `test-results/app-install-purchase-report-status-read-model-handoff-proof/proof.json`
  when run. The proof records the public package export, product checklist row,
  and parent-domain README deltas as pending behind the active shared locks.
- `packages/parent-domain/src/app-install-purchase-platform-limitation-action-proof.ts`
  now links provider/store report status rows and report status read-model rows
  into parent-visible limitation follow-up action rows for ready,
  manual-required, and unavailable platforms while preserving no portal
  approval/report UI, no external runtime report delivery, no provider/store
  execution, no billing provider contact, no platform adapters, no child-device
  delivery, no app blocking, no child activity data, and no Ocentra-hosted
  family data custody claims.
- `scripts/test/app-install-purchase-platform-limitation-action-proof.mjs`
  records that platform limitation action proof under
  `test-results/app-install-purchase-platform-limitation-action-proof/proof.json`
  when run.

## Current Gap

Ocentra still needs implemented platform adapters, approved provider/store API
execution proof, production child-device package-source adapter execution
beyond proof-backed local/manual/blocked/unavailable adapter execution rows,
real child-device delivery for pending/result state, portal UX, runtime report
writer/delivery, public package export for the runtime writer execution
delivery proof, and proof for each store/platform path. The current
parent-domain proofs attach parent-owned
platform/report artifact refs, child package-source artifact refs, approved
API/entitlement evidence refs, stateless report compiler status/result refs, and
platform adapter readiness refs to runtime boundary rows, add child-facing
status runtime readiness rows, classify package-source capture requests as
captured, blocked, manual-required, or unavailable with artifact/audit/report
refs and platform limitation states, link parent approval actions to those
evidence refs, link those package-source capture rows to adapter execution
proof states, map the actions into runtime handoff status rows, and link
per-store status handoff rows to adapter readiness/manual/unavailable states,
attach those handoffs to runtime writer envelope/manual-required rows, and link
runtime writer rows plus package-source capture/status rows to child-device
delivery envelope/manual-required rows, and classify parent action delivery
readiness against child delivery envelope rows, and classify provider/store
execution readiness/manual/unavailable states across approved API evidence,
store status, package-source adapter, and parent action readiness rows, and link
provider/store readiness rows to runtime report writer receipts for
store-specific report status runtime rows, and link parent review actions and
report-runtime rows into approval/report domain
rows, and
convert parent-owned runtime writer delivery rows into deterministic envelope
and delivery result receipt rows, and link those receipts to parent-owned
runtime report writer delivery-ready rows and report receipts, expose
parent-visible report status read-model rows for ready/manual-required handoff,
aggregate those rows into limitation summaries, and name platform manual
evidence requirements before any platform product claim, and deny product claims
through a parent-domain gate until portal tests, child delivery proof,
provider/store API execution proof, and platform adapter proof are present, and
links provider/store preflight rows back to that gate so provider/store proof
stays required/manual/unsupported before product claims, and route those
denied/manual/unsupported product-claim states through parent-visible store
handoff rows that still require the same proof before any claim upgrade, but do
not
implement Google Play, Apple App Store, Microsoft Store, billing entitlement,
provider/store execution, provider contact, platform interception, runtime
status reader, portal approval/report UI, external runtime writer delivery,
production child-device package-source adapter execution, child-device delivery,
external runtime report delivery, runtime app-blocking behavior, or
Ocentra-hosted family data custody.

## Checklist

- [x] Install request contract.
- [x] Purchase/subscription request contract.
- [x] Store metadata source, rating, category, publisher, freshness.
- [x] Parent approval/deny/time-box/review-needed flow contract.
- [x] Child-facing pending/result state contract with manual-required delivery
      until platform proof exists.
- [x] Platform-specific unavailable/manual-required states.
- [x] Platform-source metadata limitation rows for Google Play, Apple App Store,
      Mac App Store, Microsoft Store, and Linux package manager.
- [x] Package-source artifact requirement rows for Windows, macOS, Linux,
      Android, and iOS with no attached child-device artifacts or interception
      claim.
- [x] Audit and report integration status contract with no portal/runtime report
      claim.
- [x] Runtime-boundary proof linking platform metadata, package-source artifact,
      child delivery, and report rows without provider/store/runtime overclaims.
- [x] Status runtime readiness proof linking child-facing statuses to runtime
      status-reader readiness rows without reader implementation, child
      delivery, runtime report delivery, store integration, platform adapters,
      or app-blocking claims.
- [x] Platform/report artifact proof attaching parent-owned platform/store
      metadata artifact refs and report-runtime evidence refs without
      provider/store, adapter, child-delivery, report-delivery, interception, or
      app-blocking claims.
- [x] Child artifact/delivery boundary proof linking child package-source
      artifact refs to pending/result delivery rows without runtime capture,
      provider/store, adapter, child delivery, report delivery, interception,
      child activity data, or app-blocking claims.
- [x] Approved API/entitlement evidence proof linking store API, entitlement,
      limitation, and audit refs to child artifact rows without provider
      execution, store integration, platform adapters, child delivery, report
      delivery, interception, child activity data, or app-blocking claims.
- [x] Report-runtime status proof linking app-install report surfaces and child
      artifact refs to stateless report compiler status/result refs without
      portal report UI, runtime report delivery, provider/store execution,
      platform adapters, child delivery, child activity data, app blocking, or
      Ocentra-hosted family data custody claims.
- [x] Platform adapter boundary proof linking approved API/entitlement evidence
      and report-runtime refs to adapter readiness/manual/unavailable rows
      without platform adapter implementation, provider/store execution, child
      delivery, report delivery, interception, child activity data, app
      blocking, or Ocentra-hosted family data custody claims.
- [x] Parent review action proof linking approve/deny/time-box/review-needed
      decisions to approved API/entitlement evidence refs and report-runtime
      refs without portal approval UI, parent action runtime delivery,
      provider/store execution, platform adapters, child delivery, child
      activity data, app blocking, or Ocentra-hosted family data custody claims.
- [x] Parent action runtime handoff proof linking parent review actions to
      runtime handoff status rows and platform adapter boundary refs without
      portal approval UI, runtime action writer implementation, parent action
      runtime delivery, provider/store execution, platform adapter
      implementation, child delivery, runtime report delivery, child activity
      data, app blocking, or Ocentra-hosted family data custody claims.
- [x] Store status handoff proof linking parent action runtime handoff rows and
      platform adapter boundary rows to per-store status handoff states without
      provider/store execution, store integration, platform adapter
      implementation, parent action runtime delivery, child delivery, report
      delivery, interception, child activity data, app blocking, or
      Ocentra-hosted family data custody claims.
- [x] Runtime writer delivery proof linking parent action runtime handoff rows
      and per-store status handoff rows to writer envelope/manual-required
      states without runtime writer implementation, runtime writer delivery,
      parent action runtime delivery, provider/store execution, platform adapter
      implementation, child delivery, report delivery, interception, child
      activity data, app blocking, or Ocentra-hosted family data custody claims.
- [x] Package-source capture/status proof linking child package-source artifact
      refs and store status handoff rows to captured, blocked, manual-required,
      and unavailable status rows without provider/store execution, store
      integration, portal approval UI, platform adapter implementation, child
      delivery, report delivery, custody, interception, or app blocking claims.
- [x] Child-device delivery runtime writer proof linking runtime writer delivery
      rows and package-source capture/status rows to child delivery
      envelope/manual-required rows without writer execution, writer delivery,
      parent action runtime delivery, provider/store execution, platform
      adapters, child delivery, report delivery, custody, interception, or app
      blocking claims.
- [x] Child-device delivery readiness proof linking child-device delivery
      runtime-writer envelope rows, package-source adapter execution rows, and
      platform limitation action rows into readiness/manual/unavailable/
      policy-blocked evidence states without child delivery, writer delivery,
      provider/store execution, adapters, app blocking, custody, or hosted
      family data claims.
- [x] Package-source adapter execution proof linking package-source capture/
      status rows to local Windows, manual macOS, unavailable Linux, and
      blocked Android/iOS adapter execution states without provider/store
      execution, portal approval UI, production platform adapters, child
      delivery, report delivery, custody, interception, or app blocking claims.
- [x] Parent action delivery readiness proof linking parent action runtime
      handoff rows to child delivery runtime-writer envelope rows without
      parent action runtime delivery, runtime writer execution/delivery,
      provider/store execution, platform adapters, child delivery, report
      delivery, custody, interception, or app blocking claims.
- [x] Provider/store execution readiness proof linking approved API evidence,
      store status handoff, package-source adapter execution, and parent action
      delivery readiness rows into readiness/manual/unavailable states without
      Google Play, Apple App Store, Microsoft Store, billing/provider contact,
      provider/store execution, platform interception, child delivery, runtime
      writer delivery, app blocking, custody, or hosted family data claims.
- [x] Provider/store report status runtime proof linking provider/store
      execution readiness rows and runtime report writer receipts into
      store-specific ready/manual/unavailable report status rows without
      provider/store execution, portal report UI, external report delivery,
      platform adapters, child delivery, app blocking, custody, or hosted family
      data claims.
- [x] Approval/report domain proof linking parent review action decisions and
      report-runtime refs into approval/report rows without portal approval UI,
      portal report UI, runtime report delivery, provider/store execution,
      platform adapters, child delivery, interception, app blocking, custody, or
      hosted family data claims.
- [x] Runtime writer execution delivery proof linking runtime writer delivery
      rows and parent action delivery readiness rows into deterministic
      parent-owned runtime writer envelope and delivery result receipt rows
      without provider/store execution, platform interception/adapters,
      child-device delivery, runtime report delivery, app blocking, child
      activity data, or hosted family data custody claims.
- [x] Provider/store execution preflight proof linking provider/store execution
      readiness rows and runtime writer execution delivery receipts into
      parent-owned ready/manual/unavailable preflight rows without provider API
      execution, store integration, billing/provider contact, platform
      interception/adapters, runtime device delivery, child-device delivery, app
      blocking, child activity data, or hosted family data custody claims.
- [x] Runtime report writer delivery proof linking runtime writer execution
      delivery receipts and report-runtime compiler output rows into
      parent-owned report delivery-ready rows and report receipts without
      portal report UI, external runtime report delivery, provider/store
      execution, platform interception/adapters, child-device delivery, app
      blocking, child activity data, or hosted family data custody claims.
- [x] Report status read-model handoff proof linking approval/report domain rows
      and runtime report writer delivery rows into parent-visible ready/manual
      status rows without portal report UI, external runtime report delivery,
      provider/store execution, platform adapters, child-device delivery, app
      blocking, child activity data, or hosted family data custody claims.
- [x] Limitation summary proof aggregating provider/store report status rows and
      report status read-model rows into ready/manual/unavailable parent-visible
      buckets without portal approval/report UI, external report delivery,
      provider/store execution, billing contact, platform adapters, child
      delivery, app blocking, child activity data, or hosted custody claims.
- [x] Platform limitation action proof linking provider/store report status
      rows and report status read-model rows into parent-visible follow-up
      action rows without portal approval/report UI, external report delivery,
      provider/store execution, billing contact, platform adapters, child
      delivery, app blocking, child activity data, or hosted custody claims.
- [x] Platform proof readiness proof linking limitation summary rows to
      Windows/macOS/Linux/Android/iOS manual evidence requirements before
      product claims without Google Play, Apple App Store, Microsoft Store,
      provider/store execution, platform adapters, child delivery, report
      delivery, app blocking, child activity data, or hosted custody claims.
- [x] Store manual evidence proof linking platform proof readiness rows to
      Microsoft Store, Mac App Store, Linux package manager, Google Play, and
      Apple App Store manual evidence states without provider/store execution,
      store integration, platform adapters, runtime writer/report delivery,
      child delivery, app blocking, child activity data, or hosted custody
      claims.
- [x] Provider/store manual evidence packet proof linking platform proof
      readiness rows and provider/store preflight rows into packet-ready,
      manual-review-required, and provider-unavailable parent-owned packet rows
      without provider/store execution, store integration, platform adapters,
      runtime writer/report delivery, child delivery, app blocking, child
      activity data, or hosted custody claims.
- [x] Product-claim gate proof linking provider/store manual evidence packet
      rows to denied/manual-required/blocked product-claim gate rows until
      portal approval/report tests, child delivery proof, provider/store API
      execution proof, and platform adapter proof exist, without portal UI,
      provider/store execution, platform adapters, child delivery, app
      blocking, child activity data, or hosted custody claims.
- [x] Product-claim safe parent workflow proof converting product-claim gate
      rows into safe parent review/manual-required/unsupported workflow rows
      without portal approval/report UI, provider/store execution, store
      integration, platform adapters, child delivery, runtime writer/report
      delivery, app blocking, child activity data, or hosted custody claims.
- [x] Product-claim portal test readiness proof linking product-claim gate rows
      to portal approval/report test refs without portal approval/report UI,
      provider/store execution, platform adapters, child delivery, app
      blocking, child activity data, or hosted custody claims.
- [x] Product-claim provider/store proof linking product-claim gate rows to
      provider/store execution preflight rows so provider/store proof stays
      required/manual/unsupported before product claims without provider API
      execution, store integration, billing/provider contact, platform
      interception/adapters, runtime device delivery, child-device delivery,
      portal approval/report UI, app blocking, child activity data, or hosted
      custody claims.
- [x] Product-claim store handoff proof linking safe parent workflow rows and
      provider/store manual evidence packets into review-ready/manual-required/
      unavailable store handoff rows without approving product claims or
      claiming portal UI, provider/store execution, platform adapters, child
      delivery, runtime writer/report delivery, app blocking, child activity
      data, or hosted custody.
- [x] Product-claim store upgrade readiness proof linking product-claim gate,
      portal test readiness, and provider/store proof rows into
      blocked/manual/unsupported upgrade states without product claim approval,
      provider/store execution, store integration, portal approval/report UI,
      platform adapters, child delivery, app blocking, child activity data, or
      hosted custody claims.
- [x] Portal tests and platform proof before product claim, represented by
      platform preclaim rows that link portal approval/report test readiness
      with platform proof readiness while keeping product claims manual or
      unsupported until real portal UI tests, platform adapters, child delivery,
      and provider/store execution proof exist.
- [x] Product-claim platform limitation fallback proof linking platform
      preclaim, safe parent workflow, and platform limitation action rows into
      fallback-ready/manual/unsupported rows while keeping product claims,
      portal UI, provider/store execution, platform interception/adapters,
      child delivery, runtime delivery, app blocking, child activity data, and
      hosted custody unclaimed.
- [x] Provider/store API execution proof linking product-claim provider/store
      rows and platform limitation fallback rows into execution-ready,
      manual-required, unavailable, and blocked-before-claim evidence states
      without product-claim approval, Google Play, Apple App Store, Microsoft
      Store, billing/provider contact, provider API execution, store
      integration, platform interception/adapters, child delivery, runtime
      delivery, portal UI, app blocking, child activity data, or hosted custody
      claims.

## Next AI Instructions

Do not fold this into generic app blocking. The next proof should add real
portal approval/report UI, external runtime writer delivery to a device, real
child delivery, real provider/store API execution with credentials/evidence,
or actual platform adapters before upgrading manual-required source rows, child
delivery, parent action, store status, capture status, adapter execution,
report status, or product claim preclaim rows. If the OS/store does not allow
interception, document the limitation and offer the closest safe parent
workflow.
