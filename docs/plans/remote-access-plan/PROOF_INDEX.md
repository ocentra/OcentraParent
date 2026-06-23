<!-- agent-capsule -->

> Agent Capsule
> Plan: `remote-access-plan`
> Doc: `Remote Access Plan Proof Index`
> Kind: proof artifact router.
> Read when: selected workpack needs proof paths or PR_READY/DONE proof validation.
> Stop rule: use only the proof root for the selected workpack.
> Proves: proof location routing only.
> Does not prove: implementation completion by itself.
> Proof rule: proof artifacts are valid only after focused commands run or precise blockers are recorded.

<!-- /agent-capsule -->

# Remote Access Plan Proof Index

## Deterministic proof root

```text
output/remote-access-plan-proof/<workpack-file-stem>/
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
plan: remote-access-plan
workpack: <workpack id and name>
owner: remote-access | screen-handoff | lan-handoff | account-handoff | device-trust-handoff | data-custody-handoff | portal-handoff | eventing-handoff | protocol-service | docs-only
capability_type: live-view | screenshot-request | diagnostic | remote-control-deferred | support-assist | n/a
actor_role: parent-owner | co-parent | observer | support-admin | child-agent | system | n/a
household_ref: <household ref or n/a>
child_device_ref: <child device ref or n/a>
session_ref: <session ref or n/a>
pairing_state: not-tested | requested | confirmed | paired | denied | failed | n/a
standing_access_state: not-tested | active | paused | stopped | revoked | removed | expired | n/a
grant_state: not-tested | requested | authorized | active | denied | failed | superseded | n/a
revocation_state: not-tested | revoked | reconnect-denied | cache-denied | n/a
removed_device_state: not-tested | removed | reconnect-denied | cache-denied | n/a
relay_state: not-tested | connecting | active | degraded | unavailable | retrying | failed | n/a
control_state: deferred | not-claimed | attempted-blocked | explicit-control-slice | n/a
protected_surface_state: not-tested | allowed | blocked | redacted | n/a
retention_state: not-tested | no-raw-retention | screenshot-opt-in | recording-disabled | custody-required | n/a
child_disclosure_state: not-tested | visible | missing | not-applicable
abuse_state: not-tested | rate-limited | backpressure | replay-denied | cross-household-denied | alerting | n/a
diagnostic_redaction_state: not-tested | redacted | raw-payload-blocked | leak-detected | n/a
support_admin_state: not-tested | parent-visible-grant | denied | hidden-access-blocked | n/a
manual_required_note: <manual-required gap or n/a>
run_id: <wrapper run id or n/a>
command_id: <wrapper command id or n/a>
command: <exact command>
exit: <code>
result: pass | fail | blocked
artifact: <stdout/stderr artifact pointer, proof file, test result path, or n/a>
diagnostics_summary: <short unique failure or proof summary>
no_claim: <what this result does not prove>
```

The command log is a compact index, not a raw terminal transcript. Store command output, logs, redacted diagnostics, screenshots, or failure dumps under artifact paths and reference them by pointer. If no wrapper exists, write `run_id: n/a` and `command_id: n/a`; do not omit the proof row.

## Required proof themes

```text
account/role authority handoff
device-trust handoff
explicit grant and expiry
revocation/remove-device/manual stop
remote transport/source labels
private payload boundary
audit/log redaction
degraded/manual-required states
child disclosure state
relay abuse and cross-household isolation
no-control no-claim for current pass
```

## No-claim language

Do not claim:

```text
remote access ready
remote live view ready
relay production ready
standing access ready
remote control ready
retention/custody ready
support/admin remote access ready
PR_READY
```

unless the selected workpack proof root proves the claim or carries the exact blocker. Remote input/control remains deferred unless WP03 is explicitly opened in a future control slice.
