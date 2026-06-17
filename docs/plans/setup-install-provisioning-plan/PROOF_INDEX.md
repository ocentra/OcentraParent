<!-- agent-capsule -->

> Agent Capsule
> Plan: `setup-install-provisioning-plan`
> Doc: `Setup Install Provisioning Plan Proof Index`
> Kind: proof artifact router.
> Read when: selected workpack needs proof paths or PR_READY/DONE proof validation.
> Stop rule: use only the proof root for the selected workpack.
> Proves: proof location routing only.
> Does not prove: implementation completion by itself.
> Proof rule: proof artifacts are valid only after focused commands run or precise blockers are recorded.

<!-- /agent-capsule -->

# Setup Install Provisioning Plan Proof Index

## Proof roots

```text
output/setup-install-provisioning-plan-proof/01-family-web-info-site/
output/setup-install-provisioning-plan-proof/02-registration-login-entry/
output/setup-install-provisioning-plan-proof/03-parent-install-journey/
output/setup-install-provisioning-plan-proof/04-child-install-permission-journey/
output/setup-install-provisioning-plan-proof/05-pairing-readiness-recovery/
output/setup-install-provisioning-plan-proof/07-first-run-setup-ui-and-state-machine/
output/setup-install-provisioning-plan-proof/06-rollout-proof-and-route-gate/
```

## Test result roots

```text
test-results/setup-install-provisioning-plan-family-web-info-site/
test-results/setup-install-provisioning-plan-registration-login-entry/
test-results/setup-install-provisioning-plan-parent-install-journey/
test-results/setup-install-provisioning-plan-child-install-permission-journey/
test-results/setup-install-provisioning-plan-pairing-readiness-recovery/
test-results/setup-install-provisioning-plan-first-run-ui/
test-results/setup-install-provisioning-plan-rollout-route-gate/
```

## Required proof files per workpack

### WP01 Family Web Info Site

```text
00-public-site-route-map-proof.md
01-no-private-activity-data-proof.md
02-data-collection-matrix.md
03-privacy-copy-no-overclaim-proof.md
04-link-accessibility-proof.md
05-deploy-preview-proof-or-blocker.md
16-validation-commands.log
```

### WP02 Registration Login Entry

```text
00-registration-route-state-proof.md
01-auth-handoff-contract-proof.md
02-invite-negative-state-proof.md
03-no-sensitive-data-before-household-proof.md
04-registration-ui-state-proof.md
05-provider-unavailable-state-proof.md
16-validation-commands.log
```

### WP03 Parent Install Journey

```text
00-parent-bootstrap-code-state-proof.md
01-parent-platform-matrix-proof.md
02-download-integrity-proof.md
03-unsupported-platform-proof.md
04-update-rollback-handoff-proof.md
05-parent-install-ui-proof.md
16-validation-commands.log
```

### WP04 Child Install Permission Journey

```text
00-child-bootstrap-code-state-proof.md
01-child-platform-matrix-proof.md
02-permission-matrix-proof.md
03-missing-permission-degraded-proof.md
04-child-disclosure-proof.md
05-reinstall-recovery-proof.md
06-child-install-ui-proof.md
16-validation-commands.log
```

### WP05 Pairing Readiness Recovery

```text
00-pairing-state-machine-proof.md
01-pairing-negative-proof.md
02-readiness-matrix-proof.md
03-no-fake-ready-state-proof.md
04-recovery-flow-proof.md
05-redacted-pairing-log-proof.md
16-validation-commands.log
```

### WP07 First-Run Setup UI And State Machine

```text
00-first-run-state-machine-proof.md
01-first-run-ui-screen-map.md
02-empty-error-degraded-ui-proof.md
03-manual-required-visible-proof.md
04-adjacent-handoff-visible-proof.md
05-no-fake-ready-state-proof.md
06-source-custody-label-proof.md
16-validation-commands.log
```

### WP06 Rollout Proof And Route Gate

```text
00-rollout-proof-pack.md
01-route-sync-proof.md
02-platform-readiness-matrix.md
03-public-private-boundary-proof.md
04-manual-required-gap-register.md
05-product-status-safe-wording-proof.md
16-validation-commands.log
```

## Command log format

Every proof root must include:

```text
command: <exact command>
exit: <code>
result: pass | fail | blocked
artifact: <path or n/a>
notes: <short note>
```

If blocked:

```text
blocker:
required environment:
why this does not prove completion:
next command:
```

## No-claim language

Do not claim:

```text
public site deployed
registration implemented
parent installer ready
child installer ready
pairing ready
first-run setup ready
platform support ready
production onboarding ready
```

unless the selected workpack proof root proves the claim and WP06 aggregates it when broad readiness is claimed.

Use narrow wording:

```text
public route map drafted
Cloudflare preview blocker recorded
registration handoff proof passed
parent platform matrix proof passed
child permission matrix proof passed
pairing readiness matrix proof passed
first-run UI smoke passed
rollout proof gate blocked by package/distribution proof
```
