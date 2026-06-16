<!-- agent-capsule -->

> Agent Capsule
> Plan: `child-agent-runtime-distribution-plan`
> Doc: `Child Agent Runtime Distribution Proof Index`
> Kind: proof artifact router.
> Read when: selected workpack needs proof paths or PR_READY/DONE proof validation.
> Stop rule: use only the proof root for the selected workpack.
> Proves: proof location routing only.
> Does not prove: implementation completion by itself.
> Proof rule: proof artifacts are valid only after focused commands run or precise blockers are recorded.

<!-- /agent-capsule -->

# Child Agent Runtime Distribution Proof Index

## Proof roots

```text
output/child-agent-runtime-distribution-plan-proof/01-child-agent-scope-and-route-boundary/
output/child-agent-runtime-distribution-plan-proof/02-child-windows-service-package/
output/child-agent-runtime-distribution-plan-proof/03-child-macos-service-package/
output/child-agent-runtime-distribution-plan-proof/04-child-linux-service-package/
output/child-agent-runtime-distribution-plan-proof/05-child-android-agent-package/
output/child-agent-runtime-distribution-plan-proof/06-child-ios-agent-capability-package/
output/child-agent-runtime-distribution-plan-proof/07-child-managed-service-respawn/
output/child-agent-runtime-distribution-plan-proof/08-child-parent-authorized-uninstall/
output/child-agent-runtime-distribution-plan-proof/09-child-signing-store-device-owner-matrix/
output/child-agent-runtime-distribution-plan-proof/10-setup-device-trust-handoff/
output/child-agent-runtime-distribution-plan-proof/11-proof-ci-release-gate/
```

## Required universal proof files

```text
00-scope-summary.md
01-negative-case-proof.md
02-no-claim-boundary.md
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

## No-claim language

Do not claim child package, service, restart, platform, setup handoff, or release readiness unless the selected proof root proves that exact claim and WP11 aggregates it.
