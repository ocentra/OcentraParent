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
- Working plan:
  [browser plan](../plans/browser-plan/README.md), including the
  [source index](../plans/browser-plan/source-index.md),
  [current snapshot](../plans/browser-plan/current-browser-snapshot.md),
  [full scope plan](../plans/browser-plan/v0-5-managed-browser-full-scope-plan.md),
  [URL/video AI intelligence plan](../plans/browser-plan/v0-5-browser-url-video-ai-intelligence-plan.md),
  [social platform account/feed gating plan](../plans/browser-plan/v0-5-social-platform-account-feed-gating-plan.md),
  [browser games/cloud gaming gating plan](../plans/browser-plan/v0-5-browser-games-cloud-gaming-gating-plan.md),
  [test blueprint](../plans/browser-plan/v0-5-managed-browser-test-blueprint.md),
  [UI/UX guide](../plans/browser-plan/ui-ux-requirements-guide.md), and
  [workpacks](../plans/browser-plan/workpacks/01-contract-boundary-and-effect-schemas.md).
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
- The V0.8 enforcement integrity runtime audit now carries browser/web-related
  non-execution states through the supported-adapter event path: dry-run and
  observe-only states do not execute adapters, stale/wrong-device intents reject
  before execution, and exact active-tab enforcement remains unclaimed.
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
not-claimed boundary, not active tab enforcement. The integrity runtime audit
adds proof that dry-run, observe-only, rejected, unavailable, and manual-required
states stay non-executing, but it still does not prove managed exact URL or
active-tab enforcement.

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
policy contracts, enforcement status, and portal source labels together. Any
future exact active-tab claim must add new managed-browser artifacts beyond the
current integrity runtime audit proof. Use the browser plan folder for
implementation sequencing and workpack ownership; do not recreate browser
contracts, URL/video intelligence, policy catalogs, or UI surfaces outside the
existing package/crate layout unless an ownership boundary changes.
