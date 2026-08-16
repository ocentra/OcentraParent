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
plan: child-agent-runtime-distribution-plan
workpack: <workpack id and name>
owner: schema-domain | child-runtime-domain | release-script | agent-protocol | agent-service | setup-handoff | device-trust-handoff | platform-proof | docs-only
platform: windows | macos | linux | android | ios | cross-platform | n/a
artifact_kind: service-package | installer | archive | apk | simulator-app | manifest | checksum | sbom | signing-record | store-record | proof-json | n/a
package_path: <artifact path/ref or n/a>
checksum_ref: <checksum artifact/ref or n/a>
sbom_ref: <sbom artifact/ref or n/a>
signing_state: unsigned | debug | signed | notarized | store-signed | manual-required | not-applicable | unknown
install_state: not-tested | installed | blocked | failed | manual-required | not-applicable
runtime_state: not-tested | running | stopped | degraded | failed | manual-required | not-applicable
respawn_state: not-tested | proved | unsupported | failed | manual-required | not-applicable
uninstall_state: not-tested | parent-authorized | revoked | removed | residual-state | failed | manual-required | not-applicable
device_owner_state: not-tested | enrolled | unsupported | failed | manual-required | not-applicable
managed_profile_or_supervision_state: not-tested | enrolled | supervised | unsupported | failed | manual-required | not-applicable
setup_trust_handoff_ref: <typed handoff ref or n/a>
run_id: <wrapper run id or n/a>
command_id: <wrapper command id or n/a>
correlation_id: <artifact/proof/setup/device/runtime correlation id or n/a>
command: <exact command>
exit: <code>
result: pass | fail | blocked
artifact: <raw stdout/stderr artifact pointer, package artifact, proof file, platform output path, or n/a>
diagnostics_summary: <short unique failure or proof summary>
platform_note: <os/version/service-manager/device/provisioning/capability/manual-required note or n/a>
manual_required_note: <explicit manual-required gap or n/a>
no_claim: <what this result does not prove>
```

The command log is a compact index, not a raw terminal transcript. Store raw stdout/stderr, package logs, platform logs, checksums, SBOMs, proof JSON, service traces, or long failure dumps under artifact paths and reference them by pointer. If no wrapper exists, write `run_id: n/a` and `command_id: n/a`; do not omit the proof row.

## Runtime and local harness split

Runtime/product-safe proof must show artifact custody, platform state, setup-trust handoff, capability state, and no-claim boundaries. Local harness proof may include richer diagnostics, but it still stores raw logs by pointer and keeps plan docs compact.

```text
runtime-safe: no account tokens, pairing secrets, signing keys, certificate private material, provisioning secrets, store credentials, raw device identifiers unless required and redacted, or child private activity payloads.
local harness: enough file/line/command/artifact/platform/setup/ref context for Codex/MCP/humans to debug without reading terminal walls.
```

## No-claim language

Do not claim child package, service, restart, platform, setup handoff, or release readiness unless the selected proof root proves that exact claim and WP11 aggregates it.

Do not claim broad child distribution readiness from package scripts alone, parent client proof, setup UI proof, debug APK proof, simulator proof, checksum/signing proof alone, empty proof directories, stale legacy proof paths, scaffold/manual-required rows, or a proof root for another platform/workpack.
