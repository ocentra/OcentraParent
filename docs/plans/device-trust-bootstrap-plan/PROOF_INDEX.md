<!-- agent-capsule -->

> Agent Capsule
> Plan: `device-trust-bootstrap-plan`
> Doc: `Device Trust Bootstrap Plan Proof Index`
> Kind: proof artifact router.
> Read when: selected workpack needs proof paths or PR_READY/DONE proof validation.
> Stop rule: use only the proof root for the selected workpack.
> Proves: proof location routing only.
> Does not prove: implementation completion by itself.
> Proof rule: proof artifacts are valid only after focused commands run or precise blockers are recorded.

<!-- /agent-capsule -->

# Device Trust Bootstrap Plan Proof Index

## Proof root

```text
output/device-trust-bootstrap-plan-proof/<workpack-file-stem>/
```

This is a local generated-output root. Files below it are transient harness evidence and must not be committed. A tracked output file is not canonical proof; durable review evidence lives in source, visible tests, and the current CI or harness run.

## Required universal proof files

```text
00-scope-summary.md
01-negative-case-proof.md
02-no-claim-boundary.md
03-platform-proof-status.md
16-validation-commands.log
17-blockers.md
```

WP01 currently has one narrow source-and-test-backed runtime slice:

```text
crates/family-identity-core/src/parent_presence*.rs
crates/family-identity-core/src/trust_bootstrap.rs
crates/family-identity-core/tests/unit/trust_bootstrap*.rs
crates/ocentra-eventing/src/journal/ndjson_io*.rs
crates/ocentra-eventing/tests/journal_replay/file.rs
```

The visible tests cover the Rust parent-presence custody slice, including transactional decision outbox delivery, correlated and redacted accepted/replay journal entries, real journal failure, restart recovery, and idempotent re-delivery. The eventing journal tests separately prove stable idempotent append behavior across reopen and reject identity collisions. These tests do not substitute for broader WP01 or WP09 closure, subscriber delivery, or a broader event-bus runtime. Windows production custody is valid only when the final file and every ancestor remain pinned by no-delete-share handles and the runtime capability probe confirms that the filesystem denies substitution. Unix production custody remains unavailable; debug-only Unix file-mode tests are not production custody proof.

## Command log format

```text
command: <exact command>
exit: <code>
result: pass | fail | blocked
artifact: <path or n/a>
blocker-class: real dependency blocker | external platform constraint | avoidable local execution gap | n/a
notes: <short note>
```

## Structured proof metadata

For new proof artifacts and new command-log entries, include structured metadata when available:

```text
plan: device-trust-bootstrap-plan
workpack: <workpack id and name>
owner: schema-domain | family-domain | lan-domain | agent-protocol | agent-service | setup-handoff | account-handoff | data-custody-handoff | payment-handoff | distribution-handoff | remote-handoff | policy-handoff | docs-only
trust_subject: parent-device | child-device | household | parent-account | child-profile | n/a
device_role: trusted-parent | trusted-child | approving-phone | desktop-controller | child-agent | n/a
actor_role: parent-owner | co-parent-guardian | observer | support-admin | child-device | n/a
trust_state: untrusted | pending | trusted | revoked | expired | reset-required | manual-required | n/a
sealed_key_state: not-tested | sealed | unavailable | revoked | wrong-user | wrong-device | wrong-key | recovery-required | manual-required | n/a
platform_store: dpapi | keychain | secure-enclave | android-keystore | linux-keyring | hardware-backed | unsupported | manual-required | n/a
step_up_state: not-tested | required | accepted | expired | wrong-household | wrong-account | wrong-action | wrong-device | replay-rejected | manual-required | n/a
qr_challenge_state: not-tested | issued | approved | expired | replay-rejected | wrong-household | wrong-target | blocked | manual-required | n/a
entitlement_binding_state: not-tested | signed | bound-to-device | expired | revoked | replay-rejected | license-only-blocked | manual-required | n/a
recovery_bundle_state: not-tested | encrypted | preview-only | applied | wrong-household | wrong-device | wrong-key | corrupt | blocked | manual-required | n/a
tamper_uninstall_state: not-tested | parent-authorized | revoked | blocked | residual-state | unsupported | manual-required | n/a
revocation_state: not-tested | pending | applied | stale-rejected | blocked | manual-required | n/a
replay_state: not-tested | accepted-once | replay-rejected | expired | n/a
platform_note: <os/device/proof constraint or n/a>
manual_required_note: <explicit manual-required gap or n/a>
run_id: <wrapper run id or n/a>
command_id: <wrapper command id or n/a>
correlation_id: <trust/action/recovery/approval/proof correlation id or n/a>
command: <exact command>
exit: <code>
result: pass | fail | blocked
artifact: <stdout/stderr artifact pointer, proof file, test result path, or n/a>
diagnostics_summary: <short unique failure or proof summary>
no_claim: <what this result does not prove>
```

The command log is a compact local index, not a raw terminal transcript or a tracked repository artifact. Store command output, test reports, proof JSON, platform output, route-sync reports, or long failure dumps under ignored artifact paths and reference them by pointer. If no wrapper exists, write `run_id: n/a` and `command_id: n/a`; do not omit the proof row.

## Platform proof status

`03-platform-proof-status.md` should say which of these were expected for the touched slice and which were not:

- Windows proof expected / not relevant
- Android proof expected / not relevant
- Linux proof expected / not relevant
- iOS proof external-platform-constraint / not relevant
- macOS proof external-platform-constraint / not relevant

Do not mark iOS or macOS as a local blocker from this Windows host when the missing proof is only an external-platform constraint.

## Blocker file

`17-blockers.md` should separate:

1. real dependency blockers
2. external platform constraints
3. avoidable local execution gaps

If there are no blockers in one category, say so explicitly.

## No fake-green proof rule

- A proof folder with only document assertions does not close a runtime claim.
- If a workpack is still docs-only, the proof must say so directly.
- If mocks were used, the proof must call them out and explain why the remaining real-behavior gap is acceptable for that slice.

## No-claim language

Do not claim:

```text
device trust ready
local key sealing ready
parent step-up ready
phone QR approval ready
entitlement unlock ready
recovery ready
child tamper/uninstall ready
dependency adopted
route gate ready
PR_READY
```

unless the selected proof root proves that exact claim and WP09 aggregates it when broad readiness is claimed.

## Legacy note

Older `docs/proof/device-trust-bootstrap-plan/*` references are legacy pointers. New runs may use ignored local files below `output/device-trust-bootstrap-plan-proof/`; those files are not committed proof.
