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
- No product runtime claim exists yet.

## Current Gap

Ocentra still needs platform adapters, approved store/API proof or package-source
artifact attachments, child-device delivery for pending/result state, portal UX,
runtime report integration, and proof for each store/platform path. The current
parent-domain proof does not implement Google Play, Apple App Store, Microsoft
Store, billing entitlement, platform interception, portal, child-device
delivery, runtime reports, runtime app-blocking behavior, or real child-device
package-source artifact capture.

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
- [ ] Portal tests and platform proof before product claim.

## Next AI Instructions

Do not fold this into generic app blocking. The next proof should attach real
platform/store metadata artifacts, approved API/entitlement evidence, package
source artifacts, or portal/report runtime evidence before upgrading
manual-required source rows, child delivery, or report status. If the OS/store
does not allow interception, document the limitation and offer the closest safe
parent workflow.
