<!-- agent-capsule -->

> Agent Capsule
> Plan: `cloudflare-control-plane-plan`
> Doc: `Cloudflare Control Plane Proof Index`
> Kind: proof artifact router.
> Read when: selected workpack needs proof paths or PR_READY/DONE proof validation.
> Stop rule: use only the proof root for the selected workpack.
> Proves: proof location routing only.
> Does not prove: implementation completion by itself.
> Proof rule: proof artifacts are valid only after focused commands run or precise blockers are recorded.

<!-- /agent-capsule -->

# Cloudflare Control Plane Proof Index

## Proof roots

```text
output/cloudflare-control-plane-plan-proof/00-games-infra-parity-extraction/
output/cloudflare-control-plane-plan-proof/01-cloudflare-module-scaffold/
output/cloudflare-control-plane-plan-proof/02-wrangler-env-bindings/
output/cloudflare-control-plane-plan-proof/03-worker-entrypoint-runtime-guards/
output/cloudflare-control-plane-plan-proof/04-route-manifest-and-domain-contracts/
output/cloudflare-control-plane-plan-proof/05-auth-admin-support-boundary/
output/cloudflare-control-plane-plan-proof/06-storage-do-d1-kv-r2-queue-bindings/
output/cloudflare-control-plane-plan-proof/07-local-dev-seeding-and-fixtures/
output/cloudflare-control-plane-plan-proof/08-testing-runner-and-test-pyramid/
output/cloudflare-control-plane-plan-proof/09-portal-to-worker-e2e-smoke/
output/cloudflare-control-plane-plan-proof/10-security-fuzz-property-observability/
output/cloudflare-control-plane-plan-proof/11-deployment-and-environment-promotion/
output/cloudflare-control-plane-plan-proof/12-payment-plan-handoff-gate/
```

Raw command output and generated bundles under `output/` remain ignored build
evidence. A compact tracked receipt is required when workpack state or a
cross-plan handoff must survive worktree cleanup and remain reviewable from the
remote branch:

```text
docs/proof/cloudflare-control-plane-plan/<workpack-id>.md
```

The receipt records exact commit, commands, results, accepted roots, missing
roots, blockers, and no-claim boundaries. It must not embed raw logs, secrets,
provider payloads, environment values, or generated output files.

## Required universal proof files

Every proof root needs:

```text
00-scope-summary.md
01-negative-case-proof.md
02-rollback-or-teardown-proof.md
16-validation-commands.log
```

Additional workpack-specific files are named inside the selected workpack and `REQUIRED_TEST_ASSERTION_MATRIX.md` for WP08/WP10.

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
plan: cloudflare-control-plane-plan
workpack: <workpack id and name>
owner: infra-cloudflare | route-manifest | auth-boundary | storage-bindings | local-dev | test-runner | portal-smoke | security-observability | deployment | payment-handoff | docs-only
environment: local | development | production | cross-environment | n/a
route_key: <route handler key/proof id family or n/a>
auth_state: public | parent-session-required | trusted-parent-device-required | admin-required | support-required | provider-webhook-signature-required | internal-queue-only | n/a
binding_family: durable-object | d1 | kv | r2 | queue | analytics | mixed | n/a
storage_family: idempotency | ledger | read-model | cache | rate-limit | retry | dead-letter | audit-export | n/a
queue_state: not-tested | enqueued | retried | dead-lettered | blocked | manual-required | not-applicable
secret_custody_state: local-example-only | configured-outside-repo | placeholder | blocked | not-applicable
provider_webhook_state: syntax-only | verified | blocked | manual-required | not-applicable
deployment_state: not-tested | local-only | dev-deployed | production-deployed | rollback-proved | blocked | manual-required | not-applicable
consumer_handoff_state: not-tested | portal-smoke-proved | payment-blocked | payment-handoff-ready | blocked | not-applicable
payment_handoff_state: blocked | assumptions-listed | accepted-proof-roots-listed | downstream-acknowledged | not-applicable
run_id: <wrapper run id or n/a>
command_id: <wrapper command id or n/a>
correlation_id: <request/proof/deploy/queue/handoff correlation id or n/a>
command: <exact command>
exit: <code>
result: pass | fail | blocked
artifact: <raw stdout/stderr artifact pointer, proof file, deploy output path, test result path, or n/a>
diagnostics_summary: <short unique failure or proof summary>
rollback_or_teardown_note: <rollback/teardown proof or n/a>
dependency_blocker_note: <account/device/payment/provider/deployment blocker or n/a>
no_claim: <what this result does not prove>
```

The command log is a compact index, not a raw terminal transcript. Store raw stdout/stderr, Wrangler output, test reports, deployment output, queue traces, redaction reports, provider proof files, or long failure dumps under artifact paths and reference them by pointer. If no wrapper exists, write `run_id: n/a` and `command_id: n/a`; do not omit the proof row.

## Runtime and local harness split

Runtime/product-safe proof must show route, auth, binding, environment, redaction, dependency, rollback/teardown, and handoff boundaries. Local harness proof may include richer diagnostics, but it still stores raw logs by pointer and keeps plan docs compact.

```text
runtime-safe: no sensitive auth/provider material, private billing fields, support-private notes, environment-only values, raw provider bodies unless allowed, child telemetry, or raw child data.
local harness: enough file/line/command/artifact/route/env/binding/handoff context for Codex/MCP/humans to debug without reading terminal walls.
```

## No-claim language

Do not claim:

```text
Cloudflare runtime ready
payment handoff ready
production deploy ready
private/admin/support boundary ready
account/session authority ready
trusted-device authority ready
storage binding ready
portal-worker smoke ready
PR_READY
```

unless the selected proof root proves the claim and WP12 aggregates it when broad readiness is claimed.

Active proof routing uses ignored raw/generated evidence under
`output/cloudflare-control-plane-plan-proof/` plus compact retained receipts
under `docs/proof/cloudflare-control-plane-plan/` when cleanup-safe proof or a
cross-plan handoff is required.

Old `docs/proof/cloudflare-control-plane-plan/` files remain invalid unless
they use the retained-receipt contract above and cite current-head validation.
File presence alone is never proof.
