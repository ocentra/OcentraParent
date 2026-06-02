# 17 Cross-Platform Unavailable States

Sources: [20-step plan](../v0-8-enforcement-control-20-step-plan.md),
[test blueprint](../v0-8-enforcement-control-test-blueprint.md), and
[folder README](../README.md).

## Where We Are

Windows has the first real adapter direction. Other platforms have scaffold or
manual-required states for many privileged capabilities.

## Where We Want To Be

Every enforcement surface states platform coverage honestly across Windows,
macOS, Linux, Android, iOS, and web.

## Requirement Checklist

- [ ] Split parent desktop, parent mobile, child Windows, child Android, and
      child iOS claims.
- [ ] Attach unavailable or manual-required reasons.
- [ ] Preserve web as authoring/visibility only.
- [ ] Add platform matrix tests or proof output rows.
- [ ] Update platform docs/checklist when status changes.

## Acceptance And Proof

Proof output and docs agree on which platform can do what and why unsupported
states remain unsupported.

## Parallel Ownership Notes

D owns package/runtime platform proof. A owns enforcement capability truth.
Coordinate on shared checklist rows.
