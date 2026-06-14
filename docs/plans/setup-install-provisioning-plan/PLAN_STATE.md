<!-- agent-capsule -->

> Agent Capsule
> Plan: `setup-install-provisioning-plan`
> Doc: PLAN_STATE
> Kind: current state and gap map.
> Read when: Immediately after AGENTS.
> Stop rule: Use this to choose one next action; do not scan historical docs.
> Proves: current planning state only.
> Does not prove: product completion or implementation readiness.

<!-- /agent-capsule -->

# Setup Install Provisioning Plan State

Status: execution-grade route and proof inventory are being rewritten for the end-to-end first-run journey.

Research status: incomplete. This plan requires a follow-up pass against existing package scripts, portal routes, LAN pairing state, installer artifacts, family-site deployment options, and current product decisions with Sujan before implementation claims.

## Current Truth

- Existing detailed owners remain valid: package mechanics in `parent-client-runtime-distribution-plan`, LAN pairing in `lan-plan`, portal surfaces in `portal-ux-household-surfaces-plan`, account identity in `account-identity-family-plan`.
- This plan owns the customer path through those systems.
- `family.ocentra.ca` is a public informational entry surface by default. It may collect registration/contact/auth data only through explicit account flows; it must not collect child activity data on marketing/info pages.
- A successful setup claim requires observable readiness state, not just installer files or a portal page.
- Parent install bootstrap and child pairing bootstrap are separate code-gated flows.

## Open Gaps

- No dedicated Vite/Cloudflare deployment plan for `family.ocentra.ca`.
- No single setup state machine tying account, parent device, child device, permissions, pairing, bootstrap codes, and recovery.
- No platform-specific install UX matrix for parent and child devices.
- No proof manifest defining screenshots/logs/artifacts for first-run success and failure states.
- No clear handoff from public website invite/code entry into account/household identity.
- No explicit separation between parent bootstrap code and child pairing bootstrap code in the state model.

## Default Next Action

Start with `workpacks/01-family-web-info-site.md` or `workpacks/02-registration-login-entry.md` unless the assignment names an installer, child permission, pairing, or rollout proof slice.

## HID Execution Guard (added 2026-06-12)

- Scope and completion source:
  - follow [PLAN_HID_MATRIX.md](../../PLAN_HID_MATRIX.md) execution slice, then this plan's assigned WORKPACK_INDEX.md and NEXT_ACTIONS.md.
  - do not mark this plan complete from checklist deltas alone.
- Before any checked update, attach:
  - a real test run log (or explicit known blocker) from the assigned implementation boundary,
  - a proof manifest under docs/proof/setup-install-provisioning-plan/.
- Required proof manifest names:
  - docs/proof/setup-install-provisioning-plan/01-*.md
  - docs/proof/setup-install-provisioning-plan/02-*.md
  - docs/proof/setup-install-provisioning-plan/03-*.md
  - docs/proof/setup-install-provisioning-plan/04-*.md
  - docs/proof/setup-install-provisioning-plan/05-*.md
  - docs/proof/setup-install-provisioning-plan/06-*.md
  - docs/proof/setup-install-provisioning-plan/07-*.md
  - each proof file must include commands, pass/fail,
    negative-cases, and manual-required notes.
- Failure rule: no PR-ready claim until replay/idempotency, authZ/replay, rollback/teardown, and bootstrap code proofs are present for the assigned slice.

## HID execution blueprint

Continue execution from: [PLAN_EXECUTION_BLUEPRINT.md](PLAN_EXECUTION_BLUEPRINT.md).
Update this plan only via the blueprint and matching workpack checklist.
