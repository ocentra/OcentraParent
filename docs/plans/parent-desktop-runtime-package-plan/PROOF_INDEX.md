<!-- agent-capsule -->

> Agent Capsule
> Plan: `parent-client-runtime-distribution-plan`
> Doc: `Parent Client Runtime Distribution Proof Index`
> Kind: proof artifact router.
> Read when: selected workpack needs proof paths or PR_READY/DONE proof validation.
> Stop rule: use only the proof root for the selected workpack.
> Proves: proof location routing only.
> Does not prove: implementation completion by itself.
> Proof rule: proof artifacts are valid only after focused commands run or precise blockers are recorded.

<!-- /agent-capsule -->

# Parent Client Runtime Distribution Proof Index

## Proof roots

```text
output/parent-client-runtime-distribution-plan-proof/01-parent-client-scope-and-route-boundary/
output/parent-client-runtime-distribution-plan-proof/02-parent-web-portal-distribution/
output/parent-client-runtime-distribution-plan-proof/03-parent-desktop-shell-package/
output/parent-client-runtime-distribution-plan-proof/04-parent-android-package/
output/parent-client-runtime-distribution-plan-proof/05-parent-ios-package/
output/parent-client-runtime-distribution-plan-proof/06-parent-local-service-route-bridge/
output/parent-client-runtime-distribution-plan-proof/07-parent-client-signing-store-matrix/
output/parent-client-runtime-distribution-plan-proof/08-parent-client-update-rollback/
output/parent-client-runtime-distribution-plan-proof/09-parent-client-launch-smoke-matrix/
output/parent-client-runtime-distribution-plan-proof/10-setup-handoff-contracts/
output/parent-client-runtime-distribution-plan-proof/11-proof-ci-release-gate/
```

## Required universal proof files

Every proof root needs:

```text
00-scope-summary.md
01-negative-case-proof.md
02-manual-required-gap-register.md
16-validation-commands.log
```

## Command log format

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
web portal production ready
desktop package ready
mobile package ready
signing/store ready
update/rollback ready
setup handoff ready
release ready
PR_READY
```

unless the selected proof root proves the claim and WP11 aggregates it when broad readiness is claimed.
