# WP01 validation commands

plan: cloudflare-control-plane-plan
workpack: 01-cloudflare-module-scaffold
owner: infra-cloudflare
environment: local
route_key: n/a
auth_state: n/a
binding_family: n/a
storage_family: n/a
queue_state: not-applicable
secret_custody_state: not-applicable
provider_webhook_state: not-applicable
deployment_state: not-tested
consumer_handoff_state: not-tested
payment_handoff_state: blocked
run_id: n/a
command_id: n/a
correlation_id: n/a

```text
command: npm --prefix packages/logging-domain run build
exit: 0
result: pass
artifact: n/a
diagnostics_summary: required local logging-domain export build completed before Cloudflare TypeScript lint
rollback_or_teardown_note: after validation, verify packages/logging-domain/dist is an ignored non-reparse-point build directory, then run Remove-Item -LiteralPath packages/logging-domain/dist -Recurse -Force if this pass created it
dependency_blocker_note: the local logging-domain build is validation preparation only, not a Cloudflare product change
no_claim: local dependency preparation does not prove runtime or consumer behavior

command: npm --prefix infra/cloudflare install --ignore-scripts --no-audit --no-fund --no-package-lock
exit: 0
result: pass
artifact: 03-package-dependency-graph.md
diagnostics_summary: clean module resolver install after pinned Wrangler and Workers-types declarations
rollback_or_teardown_note: 02-rollback-or-teardown-proof.md
dependency_blocker_note: none for WP01 graph; local logging-domain build is required before module lint resolves file-dependency exports
no_claim: no runtime, deployment, authority, payment, or WP07 proof claim

command: npm --prefix infra/cloudflare ls wrangler @cloudflare/workers-types
exit: 0
result: pass
artifact: 03-package-dependency-graph.md
diagnostics_summary: wrangler@4.118.0 and @cloudflare/workers-types@5.20260804.1 deduped
rollback_or_teardown_note: 02-rollback-or-teardown-proof.md
dependency_blocker_note: none for WP01 graph
no_claim: no runtime, deployment, authority, payment, or WP07 proof claim

command: npm --prefix infra/cloudflare run lint
exit: 0
result: pass
artifact: n/a
diagnostics_summary: TypeScript no-emit passed after building the declared local logging-domain dependency
rollback_or_teardown_note: 02-rollback-or-teardown-proof.md
dependency_blocker_note: local package build is validation preparation, not a Cloudflare source change
no_claim: lint does not prove runtime or consumer behavior

command: npm --prefix infra/cloudflare run test:unit
exit: 0
result: pass
artifact: n/a
diagnostics_summary: 49 tests across 7 suites passed
rollback_or_teardown_note: 02-rollback-or-teardown-proof.md
dependency_blocker_note: none
no_claim: unit tests do not prove WP07, deployment, account authority, or payment readiness

command: npm run lint:architecture -- --files infra/cloudflare
exit: 0
result: pass
artifact: n/a
diagnostics_summary: architecture-policy and generated-artifacts passed
rollback_or_teardown_note: n/a
dependency_blocker_note: none
no_claim: source-policy validation does not prove runtime readiness

teardown_command: Remove-Item -LiteralPath packages/logging-domain/dist -Recurse -Force
teardown_precondition: verify the exact ignored build directory is not a reparse point and was created only for this validation pass
teardown_result: not run; the build directory pre-existed this pass and remains disposable ignored local validation state
teardown_no_claim: retaining or removing ignored local build output does not change committed source or proof

command: rg -n 'packages/billing-domain/src' infra/cloudflare
exit: 1
result: pass
artifact: 01-negative-case-proof.md
diagnostics_summary: no obsolete private billing-domain source import found
rollback_or_teardown_note: n/a
dependency_blocker_note: none
no_claim: absence of the stale import is not payment readiness
```
