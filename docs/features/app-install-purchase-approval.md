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

## Current Gap

Ocentra still needs implemented platform adapters, approved provider/store API
execution proof, production child-device package-source adapter execution
beyond proof-backed local/manual/blocked/unavailable adapter execution rows,
real child-device delivery for pending/result state, portal UX, runtime report
writer/delivery, and proof for each store/platform path. The current
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
delivery envelope/manual-required rows, but do not implement Google Play, Apple
App Store, Microsoft Store, billing entitlement, provider/store execution,
platform interception, runtime status reader, portal approval/report UI,
runtime action writer execution, runtime writer delivery, parent action runtime
delivery, production child-device package-source adapter execution,
child-device delivery, runtime report delivery, runtime app-blocking behavior,
or Ocentra-hosted family data custody.

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
- [x] Package-source adapter execution proof linking package-source capture/
      status rows to local Windows, manual macOS, unavailable Linux, and
      blocked Android/iOS adapter execution states without provider/store
      execution, portal approval UI, production platform adapters, child
      delivery, report delivery, custody, interception, or app blocking claims.
- [ ] Portal tests and platform proof before product claim.

## Next AI Instructions

Do not fold this into generic app blocking. The next proof should add real
portal approval/report UI, parent action runtime delivery, runtime writer
execution/delivery, real child delivery, provider/store API execution proof, or
actual platform adapters before upgrading manual-required source rows, child
delivery, parent action, store status, capture status, adapter execution, or
report status. If the OS/store does not allow interception, document the
limitation and offer the closest safe parent workflow.
