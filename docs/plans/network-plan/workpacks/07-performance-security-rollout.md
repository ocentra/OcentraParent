# WP07 Performance Security Rollout

Scope: prove performance, resource, privacy, compliance, deployment, support, and staged rollout readiness for network work.

Source rows: `03-network-implementation-checklist-and-workpacks.md` rows 49-50.

Read next:

- `../02-network-tests-proof-and-validation-blueprint.md`
- `../PROOF_INDEX.md`
- `../../agent/PR_DONE_FLOW.md` only for PR/DONE reporting

Expected outcome:

- Performance and latency claims have high-concurrency, spike, soak, resource, memory, FD, and connection-exhaustion proof where relevant.
- Security/privacy proof covers redaction, raw artifact custody, auth, rate limit, abuse, DoS, CORS/origin/header/host/redirect where request paths exist, and no-overclaim guards.
- Deployment/support proof covers staged rollout, rollback, diagnostics, privacy copy, and support limitation text.

Expected tests/proof:

- `network.performance.high-concurrency`
- `network.performance.spike-soak-resource`
- `network.security.schema-fuzz`
- `network.security.rate-limit-abuse-dos`
- `network.privacy.redaction-custody`
- `network.rollout.rollback-support-proof`
- Proof includes command logs, thresholds, alerts/metrics sanity, and residual risk.

Failure conditions:

- Do not claim real-time or production readiness from local happy-path tests.
- Do not keep raw traffic proof without custody and delete/export boundaries.
- Do not ship support/privacy claims that exceed the proved authority tier.

## Current code-drafted slice — tests deferred

The recurring Windows activity-capture path no longer leaks one heap
allocation per capture interval. The capture operation owns one bounded
timestamp `String`, while a lifetime-bound `ObservedAtText` borrows it during
current event-batch construction; the capture limits remain the existing
process/network limits.
This is a production resource-safety correction, not rollout or performance
completion.

Shutdown coordination, high-concurrency/soak measurements, deployment
rollback, platform adapter readiness, and retained privacy/security proof
remain open.
