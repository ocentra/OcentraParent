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
  delivery rows, and report integration rows while keeping store/provider,
  child-device delivery, runtime report delivery, and app-blocking behavior
  unclaimed.
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

## Current Gap

Ocentra still needs implemented platform adapters, approved provider/store API
execution proof, production child-device package-source artifact capture, real
child-device delivery for pending/result state, portal UX, runtime report
writer/delivery, and proof for each store/platform path. The current
parent-domain proofs attach parent-owned platform/report artifact refs, child
package-source artifact refs, approved API/entitlement evidence refs, stateless
report compiler status/result refs, and platform adapter readiness refs to
runtime boundary rows but do not implement Google Play, Apple App Store,
Microsoft Store, billing entitlement, provider/store execution, platform
interception, portal, child-device runtime capture, child-device delivery,
runtime report delivery, runtime app-blocking behavior, or production
child-device package-source artifact capture.

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
- [ ] Portal tests and platform proof before product claim.

## Next AI Instructions

Do not fold this into generic app blocking. The next proof should add production
child-device package-source capture, real portal report UI/runtime writer
delivery, real child delivery, provider/store API execution proof, or actual
platform adapters before upgrading manual-required source rows, child delivery,
or report status. If the OS/store does not allow interception, document the
limitation and offer the closest safe parent workflow.
