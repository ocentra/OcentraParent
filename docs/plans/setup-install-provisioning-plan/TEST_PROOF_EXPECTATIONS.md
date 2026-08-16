<!-- agent-capsule -->

> Agent Capsule
> Plan: `setup-install-provisioning-plan`
> Doc: `Setup Install Provisioning Plan Test Proof Expectations`
> Kind: command/test selector.
> Read when: selected workpack asks which commands or proof artifacts are expected.
> Stop rule: run focused commands first; do not jump to full validation unless required by the workpack or PR_READY.
> Proves: command expectations only.
> Does not prove: implementation completion without matching artifacts.

<!-- /agent-capsule -->

# Setup Install Provisioning Plan Test Proof Expectations

## General rule

Use focused commands first. Broader validation is allowed only after focused commands pass or a precise blocker is recorded.

If a required package/test path does not exist yet, write a blocker artifact and leave the checklist row open.

Run through `npm run agent:run --` when collecting proof if the logging/evidence wrapper is available.

## Common command set

Use the subset relevant to the selected workpack:

```bash
npm run build --workspace @ocentra-parent/family-domain
npm run build --workspace @ocentra-parent/portal-domain
npm run test --workspace @ocentra-parent/portal -- setup
npm run test:e2e --workspace @ocentra-parent/portal -- setup
npm run lint:architecture -- --files docs/plans/setup-install-provisioning-plan packages/family-domain packages/portal-domain apps/portal
```

## Command ownership notes

- `setup-install-provisioning-plan` owns setup journey state labels, route-state matrices, proof roots, and safe wording.
- `setup-domain` currently exports only `package-info`; internal setup-domain source/tests prove selected slices but are not public API readiness unless exports are added.
- `account-identity-family-plan` owns provider/session/household/invite/recovery authority.
- `parent-desktop-runtime-package-plan` and `child-agent-runtime-distribution-plan` own runtime package/distribution readiness.
- `lan-plan` and `device-trust-bootstrap-plan` own pairing protocol/trust proof.
- `data-custody-storage-plan`, `policy-control-plane-plan`, and `payment-subscription-plan` own custody, policy baseline, and entitlement readiness.
- `portal-ux-household-surfaces-plan` owns broader portal UX beyond selected setup route projection.

## Setup E2E meaning

Do not use one proof family to claim the whole onboarding path. For this plan, E2E has separate meanings:

```text
public-site E2E: route map -> public data boundary -> no private child activity collection -> deploy/custom-domain blocker or proof.
registration-handoff E2E: auth-entry route state -> account handoff -> invite/provider negative states -> no account/session authority claim.
parent-install-handoff E2E: parent bootstrap/install route -> platform/version/integrity labels -> package-owner handoff -> no signed installer claim.
child-install-permission E2E: child install route -> platform/permission/disclosure/recovery labels -> runtime-owner handoff -> installed/running/permissioned/paired/trusted/policy-ready separated.
pairing-readiness E2E: setup pairing lifecycle -> readiness matrix -> recovery/redacted logs -> no LAN/device-trust product claim.
first-run UI E2E: typed state machine -> rendered setup screens -> manual-required/degraded/handoff labels -> no production onboarding claim.
rollout aggregation E2E: WP01-WP05/WP07 proof roots -> platform readiness/manual-required register -> safe wording -> sibling blockers preserved.
```

A workpack can be complete for one tier while other tiers remain open. Record the non-claim instead of broad DONE.

## Structured harness logging expectations

Product/runtime-safe logging:

```text
redact child activity data, profile/device private data, pairing tokens, invite tokens, session identifiers, account provider secrets, install codes, package URLs with secrets, and support-private diagnostics
log workpack, route, actor role, household state, parent install state, child install state, permission state, pairing state, trust state, custody state, policy baseline state, platform state, sibling owner state, manual-required state, artifact pointer, and no-claim boundary when safe
separate public site, account handoff, parent install, child install, pairing, first-run UI, rollout aggregation, and sibling readiness states
never treat website-only, login-button-only, download-button-only, install-only, pairing-discovered-only, UI-only, or aggregation-only proof as production onboarding readiness
```

Local Codex/MCP/debug harness logging:

```text
prefer npm run agent:run -- <command> when available
store raw stdout/stderr by artifact pointer instead of pasting terminal walls into plan docs
write compact command summaries into 16-validation-commands.log
include run id, command id, workpack id, route, owner, exit code, result, artifact pointer, diagnostics summary, blocker note, and no-claim note when available
if the wrapper is unavailable, write wrapper: unavailable and keep the same compact command-log shape
```

## WP01 Family Web Info Site

Expected focused commands:

```bash
node -e "console.log('family-web-info-site-docs-only')"
npm run lint:architecture -- --files docs/plans/setup-install-provisioning-plan
```

Expected coverage:

```text
public route map
no private activity data on public pages
data collection matrix
privacy copy no-overclaim proof
download/register/support/privacy/status link map
Cloudflare Pages or Workers route/deploy shape
custom domain manual-required state if no deployment proof exists
```

## WP02 Registration Login Entry

Expected focused commands:

```bash
node -e "console.log('registration-login-entry-handoff')"
npm run lint:architecture -- --files docs/plans/setup-install-provisioning-plan docs/plans/account-identity-family-plan
```

Expected coverage:

```text
register/login/logout/invite/resume/recovery route map
handoff to account-identity provider/session model
expired/revoked/wrong-household invite states
provider unavailable state
no profile/device private data before household authority
redacted logging proof or blocker
```

## WP03 Parent Install Journey

Expected focused commands:

```bash
node -e "console.log('parent-install-journey-handoff')"
npm run lint:architecture -- --files docs/plans/setup-install-provisioning-plan docs/plans/parent-desktop-runtime-package-plan
```

Expected coverage:

```text
parent bootstrap code state machine
parent platform matrix
download/version/integrity display states
unsupported/manual-required/update-required states
runtime distribution handoff proof
no fake installed state
```

## WP04 Child Install Permission Journey

Expected focused commands:

```bash
node -e "console.log('child-install-permission-journey-handoff')"
npm run lint:architecture -- --files docs/plans/setup-install-provisioning-plan docs/plans/child-agent-runtime-distribution-plan docs/plans/app-plan
```

Expected coverage:

```text
child bootstrap code state machine
child platform matrix
permission matrix
installed/running/permissioned/paired/trusted/policy-ready separation
missing permission degraded state
disclosure visible
reinstall recovery state
runtime/package/platform owner handoffs
```

## WP05 Pairing Readiness Recovery

Expected focused commands:

```bash
node -e "console.log('pairing-readiness-recovery-handoff')"
npm run lint:architecture -- --files docs/plans/setup-install-provisioning-plan docs/plans/lan-plan docs/plans/account-identity-family-plan
```

Expected coverage:

```text
pairing lifecycle state machine
readiness matrix
wrong-household/wrong-device/stale/revoked/offline states
no fake ready state
offline device degraded state
permission missing degraded state
policy baseline missing state
data custody unavailable state
lost-parent-device and reinstall recovery states
redacted pairing/setup log proof or blocker
```

## WP07 First-Run Setup UI And State Machine

Expected focused commands:

```bash
npm run build --workspace @ocentra-parent/family-domain
npm run build --workspace @ocentra-parent/portal-domain
npm run test --workspace @ocentra-parent/portal -- setup
npm run test:e2e --workspace @ocentra-parent/portal -- setup
npm run lint:architecture -- --files packages/family-domain packages/portal-domain apps/portal docs/plans/setup-install-provisioning-plan
```

Expected coverage:

```text
first-run state machine
screen map for welcome/sign-in/household/parent-install/child-profile/child-install/pair/readiness/recovery/complete/blocked/manual-required
empty/error/degraded UI
manual-required visible
adjacent handoff visible
no fake ready state
source/custody labels
```

## WP06 Rollout Proof And Route Gate

Expected focused commands:

```bash
node -e "console.log('setup-rollout-route-gate')"
npm run lint:architecture -- --files docs/plans/setup-install-provisioning-plan
```

Expected aggregation:

```text
WP01 proof root exists or blocker recorded
WP02 proof root exists or blocker recorded
WP03 proof root exists or blocker recorded
WP04 proof root exists or blocker recorded
WP05 proof root exists or blocker recorded
WP07 proof root exists or blocker recorded
proof manifest written
route/index sync proof written
platform readiness matrix written
manual-required gap register written
product-status wording safe
```

Required negative states:

```text
public site cannot claim private activity custody
registration cannot own auth internals
parent install cannot claim signed package readiness without package proof
child install cannot claim readiness without permission/pairing proof
pairing cannot claim product readiness by itself
first-run UI cannot show setup complete without readiness matrix
rollout aggregation cannot erase sibling-owner blockers
```
