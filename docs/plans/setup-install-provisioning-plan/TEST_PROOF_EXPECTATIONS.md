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

## Common command set

Use the subset relevant to the selected workpack:

```bash
npm run build --workspace @ocentra-parent/family-domain
npm run build --workspace @ocentra-parent/portal-domain
npm run test --workspace @ocentra-parent/portal -- setup
npm run test:e2e --workspace @ocentra-parent/portal -- setup
npm run lint:architecture -- --files docs/plans/setup-install-provisioning-plan packages/family-domain packages/portal-domain apps/portal
```

Run through `npm run agent:run --` when collecting proof if the logging/evidence wrapper is available.

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
```
