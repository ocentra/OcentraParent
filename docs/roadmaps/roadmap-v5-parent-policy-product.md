<!-- agent-capsule -->

> Agent Capsule
> Doc: V5 Parent Policy Product Expectations
> Kind: expectation/acceptance documentation; read only when selected by feature doc, plan route, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# V5 Parent Policy Product Expectations

This is the milestone-specific expectation file for V5 in `docs/product-roadmap.md`.

Supporting expectation files: [policy](../expectations/policy.md), [portal](../expectations/portal.md),
[family setup](../expectations/family-setup.md), [social and video control](../expectations/social-video-control.md),
[location and geofence](../expectations/location-geofence.md),
[app install and purchase approval](../expectations/app-install-purchase-approval.md),
[tamper and uninstall protection](../expectations/tamper-uninstall-protection.md),
[sync and export](../expectations/sync-export.md), and [billing](../expectations/billing.md).

## Outcome

- Non-technical parents can configure household rules, schedules, child profiles, permissions, time budgets, reports, and audit history without editing files.
- Parents can configure app/site/category, social/video, location/geofence,
  install/purchase approval, and agent-integrity expectations where the platform
  supports them.
- Parent settings sync safely through local or parent-owned storage boundaries.
- Source/custody is clear for local, LAN, parent-owned storage, and Ocentra-hosted non-activity metadata.

## Acceptance

- Parent-authored rules remain the authority for allow, warn, time-limit, ask-parent, and block behavior.
- Rule previews explain evidence, local AI result, schedule, conflict resolution, and decision reason.
- Social/video, location, install approval, and integrity controls use explicit
  capability states instead of implying generic parity.
- Billing entitlements may gate paid convenience/product value but do not silently disable critical local safety behavior.

## Validation

- Run `npm run validate`.
- Include policy schema tests, portal rule-authoring tests, setup/profile tests,
  social/video target tests, location/geofence contract tests,
  install/purchase approval tests, integrity/tamper-state tests,
  sync/conflict tests, and entitlement-boundary tests when billing surfaces
  exist.
