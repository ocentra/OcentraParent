# Workpack 02: Parent Authoring Preview

Goal: define nontechnical policy authoring and preview.

Expected shape:

- Parents choose goals, templates, schedules, exceptions, and target children/devices/apps/sites/places.
- Preview explains impact before confirmation.
- Conflicts, unsupported targets, manual-required states, and stale device states are visible.

Expected proof:

- UI state proof.
- Conflict and validation proof.
- No fake green proof.
- Accessibility and mobile proof when UI is touched.

Failure: policy UI that saves ambiguous rules without preview, explanation, or conflict handling.

## Execution Detail

Minimum context:

- `docs/plans/portal-ux-household-surfaces-plan/workpacks/05-policy-authoring-control-center.md`
- `docs/plans/portal-ux-household-surfaces-plan/workpacks/06-schedules-time-budgets-and-templates.md`
- `docs/features/policy-schedules-approvals.md`

Required UX outcomes:

- Parent can understand target, time, condition, action, and exception before saving.
- Preview explains expected effect and unsupported/manual-required states.
- Conflict handling covers overlapping schedules, timezone/DST, device offline, unsupported platform, and domain capability gaps.
- Accessibility and mobile behavior are not optional for parent-critical controls.

Expected tests/proof names:

- `policy-authoring.preview-before-save`
- `policy-authoring.conflict-visible`
- `policy-authoring.dst-boundary`
- `policy-authoring.unsupported-target-visible`
- `policy-authoring.no-fake-green`

Proof artifact expectations:

- Screenshot proof for preview/conflict/error/degraded states.
- Fixture cases for schedule and conflict.
- Copy review for parent-readable explanations.
