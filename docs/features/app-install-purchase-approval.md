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
- No product runtime claim exists yet.

## Current Gap

Ocentra still needs request contracts, store metadata source/freshness,
age-rating/category handling, parent approval states, platform limitations,
portal UX, and proof for each store/platform path.

## Checklist

- [ ] Install request contract.
- [ ] Purchase/subscription request contract.
- [ ] Store metadata source, rating, category, publisher, freshness.
- [ ] Parent approval/deny/time-box/review-needed flow.
- [ ] Child-facing pending/result state where platform allows it.
- [ ] Platform-specific unavailable/manual-required states.
- [ ] Audit and report integration.
- [ ] Portal tests and platform proof before product claim.

## Next AI Instructions

Do not fold this into generic app blocking. Start with platform capability
research and contracts, then UI and runtime proof. If the OS/store does not
allow interception, document the limitation and offer the closest safe parent
workflow.
