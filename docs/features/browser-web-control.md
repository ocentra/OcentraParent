# Browser And Web Control

## Parent Outcome

Parents can see and control web activity by site, URL, domain, category,
schedule, and exception where the browser/source boundary is proved.

## Ocentra Requirement

Exact URL/tab knowledge requires a managed browser or proved browser bridge.
Process/window or network metadata alone cannot claim exact page activity.
Blocking requires typed policy decisions and adapter proof.

## Roadmap And Expectations

- Roadmap: V0.5.1 browser evidence, V0.8 enforcement, V5 policy product.
- Expectations: [browser evidence](../expectations/browser-evidence.md),
  [policy](../expectations/policy.md),
  [enforcement](../expectations/enforcement.md).
- Supporting docs:
  [raw 1,057-setting inventory](../browser-control-1057-settings-inventory.md)
  and
  [questionnaire forest v1](../browser-policy-questionnaire-forest-v1.md).
- Modules: `packages/activity-domain`, `packages/parent-domain`,
  `packages/agent-protocol-domain`, `crates/agent-service`.

## Competitor Pressure

See [Competitor Capability Map](../competitor-capability-map.md), especially
web filtering/categories, search/platform restrictions, and video safety.

Google, Apple, Microsoft, Qustodio, Norton, Net Nanny, Canopy, Bark, and others
offer web filtering or content restrictions. Ocentra must match parent-visible
control while being more honest about managed versus unmanaged sources.

## Current Ocentra State

- Managed browser URL/tab evidence direction and status contracts exist.
- Browser/domain adapter proof now uses surface-specific contract guards so
  managed-session intervention, managed exact-URL manual-required state,
  unmanaged process-only termination/warn state, and network/domain manual or
  unavailable state cannot be drifted into stronger claims by direct parsing.
- Unmanaged browser states can be represented as possible bypass and
  process-only fallback, not exact URL/tab proof.
- The raw browser setting inventory and reduced questionnaire forest are now
  preserved as design inputs, not product-complete implementation proof.

## Current Gap

Managed-browser enforcement, category filtering, exact URL action, unmanaged
fallback, and parent-facing rule UX are not product-complete.

## Checklist

- [ ] Managed browser launch/profile state.
- [ ] Exact URL/tab evidence.
- [ ] Unmanaged-browser bypass status.
- [ ] Site/domain/category rule targets.
- [ ] Schedule and exception support.
- [ ] Dry-run preview with evidence refs.
- [ ] Adapter capability status.
- [ ] Real blocking/terminate proof where claimed.
- [ ] Exact active-tab enforcement and host domain blocking proof before any
      managed URL or network/domain claim upgrade.

## Next AI Instructions

Keep managed and unmanaged browser claims separate. Do not claim page semantics
from network metadata. If adding web control, update browser expectations,
policy contracts, enforcement status, and portal source labels together.
