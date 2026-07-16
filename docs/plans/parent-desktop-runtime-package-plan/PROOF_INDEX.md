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

`docs/proof/parent-desktop-runtime-package-plan/` is compatibility-only for old references. New proof should use the `output/parent-client-runtime-distribution-plan-proof/<workpack>/` root.

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

## Structured proof metadata

For new proof artifacts and new command-log entries, include structured metadata when available:

```text
plan: parent-client-runtime-distribution-plan
workpack: <workpack id and name>
owner: apps-portal | portal-domain | parent-domain | scripts-dev | scripts-release | setup-handoff | child-runtime-handoff | device-trust-handoff | docs-only
artifact_kind: web | desktop | parent-android | parent-ios | route-bridge | signing-store | update-rollback | launch-smoke | setup-handoff | release-gate | n/a
platform: web | windows | macos | linux | android | ios | cross-platform | n/a
package_state: not-tested | scaffold | built | packaged | installed | blocked | manual-required | n/a
signing_state: not-tested | unsigned | self-signed | signed | blocked | manual-required | n/a
store_state: not-tested | unpublished | uploaded | reviewed | published | blocked | manual-required | n/a
notarization_state: not-tested | not-notarized | submitted | notarized | blocked | manual-required | n/a
launch_state: not-tested | launched | degraded | failed | blocked | manual-required | n/a
route_bridge_state: not-tested | defined | connected | degraded | blocked | n/a
setup_handoff_state: not-tested | request-defined | response-defined | setup-owner-required | blocked | n/a
update_state: not-tested | channel-defined | checksum-proved | sbom-proved | blocked | manual-required | n/a
rollback_state: not-tested | proved | blocked | manual-required | n/a
manual_required_note: <manual-required gap or n/a>
run_id: <wrapper run id or n/a>
command_id: <wrapper command id or n/a>
command: <exact command>
exit: <code>
result: pass | fail | blocked
artifact: <stdout/stderr artifact pointer, proof file, test result path, package artifact path, or n/a>
diagnostics_summary: <short unique failure or proof summary>
no_claim: <what this result does not prove>
```

The command log is a compact index, not a raw terminal transcript. Store package logs, build output, signing/store notes, screenshots, or long failure dumps under artifact paths and reference them by pointer. If no wrapper exists, write `run_id: n/a` and `command_id: n/a`; do not omit the proof row.

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
