# V5 Parent Policy Product Expectations

This is the milestone-specific expectation file for V5 in `docs/product-roadmap.md`.

Supporting expectation files: [policy](policy.md), [portal](portal.md),
[family setup](family-setup.md), [social and video control](social-video-control.md),
[location and geofence](location-geofence.md),
[app install and purchase approval](app-install-purchase-approval.md),
[tamper and uninstall protection](tamper-uninstall-protection.md),
[sync and export](sync-export.md), and [billing](billing.md).

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
