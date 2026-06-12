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

Status: first-pass plan created because the repo has install, LAN, portal, package, and roadmap pieces but no single owner for the end-to-end first-run journey.

Research status: incomplete. This plan requires a full follow-up research pass against existing package scripts, portal routes, LAN pairing state, installer artifacts, family-site deployment options, and current product decisions with Sujan before implementation claims.

## Current Truth

- Existing detailed owners remain valid: package mechanics in `parent-desktop-runtime-package-plan`, LAN pairing in `lan-plan`, portal surfaces in `portal-ux-household-surfaces-plan`, account identity in `account-identity-family-plan`.
- This plan owns the customer path through those systems.
- `family.ocentra.ca` is a public informational entry surface by default. It may collect registration/contact/auth data only through explicit account flows; it must not collect child activity data on marketing/info pages.
- A successful setup claim requires observable readiness state, not just installer files or a portal page.

## Open Gaps

- No dedicated Vite/Cloudflare deployment plan for `family.ocentra.ca`.
- No single setup state machine tying account, parent device, child device, permissions, pairing, and recovery.
- No platform-specific install UX matrix for parent and child devices.
- No proof manifest defining screenshots/logs/artifacts for first-run success and failure states.
- No clear handoff from public website registration into account/household identity.

## Default Next Action

Start with `workpacks/01-family-web-info-site.md` or `workpacks/02-registration-login-entry.md` unless the assignment names an installer, child permission, pairing, or rollout proof slice.
