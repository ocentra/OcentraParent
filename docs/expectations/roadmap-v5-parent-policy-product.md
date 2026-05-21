# V5 Parent Policy Product Expectations

This is the milestone-specific expectation file for V5 in `docs/product-roadmap.md`.

Supporting expectation files: [policy](policy.md), [portal](portal.md), [sync and export](sync-export.md), and [billing](billing.md).

## Outcome

- Non-technical parents can configure household rules, schedules, child profiles, permissions, time budgets, reports, and audit history without editing files.
- Parent settings sync safely through local or parent-owned storage boundaries.
- Source/custody is clear for local, LAN, parent-owned storage, and Ocentra-hosted non-activity metadata.

## Acceptance

- Parent-authored rules remain the authority for allow, warn, time-limit, ask-parent, and block behavior.
- Rule previews explain evidence, local AI result, schedule, conflict resolution, and decision reason.
- Billing entitlements may gate paid convenience/product value but do not silently disable critical local safety behavior.

## Validation

- Run `npm run validate`.
- Include policy schema tests, portal rule-authoring tests, sync/conflict tests, and entitlement-boundary tests when billing surfaces exist.
