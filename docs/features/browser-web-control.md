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
- The V0.8 product-control spine now exposes managed browser session control,
  unmanaged process-only fallback, and managed/unmanaged exact URL gaps as
  separate parent-visible action states for downstream policy/device screens.
- The V0.8 product-control runtime path now exposes those browser states through
  a Rust service WebSocket read model and typed agent-protocol adapter that link
  back to browser/domain adapter proof and keep exact URL control
  manual-required or not-claimed.
- The V0.8 policy-dispatch proof now carries unmanaged browser process fallback
  as report-only with real evidence references, child reason codes, audit refs,
  and service-backed source state instead of exact URL claims.
- The V0.8 broad-adapter proof now exposes a service-backed WebSocket read
  model and typed protocol adapter where managed browser session support is an
  implemented boundary, managed exact URL remains manual-required, and
  unmanaged exact evidence remains not-claimed.
- The V0.8 supported-adapter runtime proof now keeps exact active-tab
  enforcement explicitly not-claimed while app/game and network observe-only
  supported boundaries are represented separately.
- Unmanaged browser states can be represented as possible bypass and
  process-only fallback, not exact URL/tab proof.
- The raw browser setting inventory and reduced questionnaire forest are now
  preserved as design inputs, not product-complete implementation proof.

## Current Gap

Managed-browser exact URL action, category filtering, warning delivery,
unmanaged fallback UX, and parent-facing rule UX are not product-complete.
The broad-adapter proof adds runtime visibility for those states but does not
upgrade exact URL, unmanaged exact evidence, or host domain blocking claims.
Policy dispatch and supported-adapter runtime proof preserve the report-only or
not-claimed boundary, not active tab enforcement.

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
