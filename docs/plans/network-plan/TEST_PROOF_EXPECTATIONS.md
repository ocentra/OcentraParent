# Network Plan Test and Proof Expectations

<!-- agent-capsule -->

> Agent Capsule
> Plan: `network-plan`
> Doc: `Network Plan Test and Proof Expectations`
> Kind: plan-local test and proof decision tree.
> Read when: After the assigned workpack/checklist row is known; use to choose required tests/proof.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: This file defines required local tests/proof; missing tests keep rows open.

<!-- /agent-capsule -->

Use this after the network task/checklist row is known. Network proof must separate observation, classification, policy, notification, enforcement, and adapter execution.

## Where tests should live

When the network implementation crate/package exists, tests belong under its test tree and proof output under its proof folder. Until then, colocate with the owning network/domain/runtime package and record paths in `PROOF_INDEX.md`.

## Decision Tree

| If the assigned work is...                         | Read next                        | Expected tests or proof                                                                                                |
| -------------------------------------------------- | -------------------------------- | ---------------------------------------------------------------------------------------------------------------------- |
| Packet/DNS/router/log import                       | `CHECKLIST_INDEX.md` exact row   | parser fuzzing, malformed input, custody/source authority, no content overclaim.                                       |
| Broker/family-hub/provider delivery                | exact row                        | authZ, token lifecycle, replay, retry/dead-letter, partial outage, remote ack/delete/export proof.                     |
| AI classification                                  | exact row; AI plan only if named | prompt injection, output invariants, no AI enforcement authority, redaction proof.                                     |
| Policy/notification                                | exact row                        | policy matrix, rate-limit/abuse, notification retry, audit trail, manual-required states.                              |
| Enforcement adapters/firewall/VPN/NetworkExtension | exact row                        | capability matrix, reversible lab proof, rollback/unblock, privilege escalation negatives, platform manual proof.      |
| Parent rule UX                                     | exact row                        | Playwright/e2e screenshots, empty/error/degraded states, authZ and stale state negatives.                              |
| Production SLO/security                            | exact row                        | load/spike/soak/resource exhaustion, smuggling/desync/cache poisoning if request surfaces change, alert/metrics proof. |

## Expected test/proof inventory

Use these names as proof intent labels in the assigned workpack/proof note. Implementers choose the actual crate/package test names after the owning implementation boundary exists.

- `network.metadata.schema-fuzz`: DNS/flow/request metadata schemas reject malformed, oversized, and ambiguous inputs.
- `network.request.origin-header-host`: origin, header, host, redirect, and URL hijack cases fail closed where request paths are touched.
- `network.security.smuggling-desync-cache`: smuggling, desync, request splitting, and cache poisoning are covered where applicable.
- `network.rate-limit.dos-boundary`: abuse, brute force, spike, and connection exhaustion paths are bounded.
- `network.privacy.no-payload-claim`: metadata proof does not imply decrypted content inspection.
- `network.policy.signal-staleness`: stale/partial network signals do not trigger unsupported policy or enforcement claims.
- `network.read-model.retention-delete`: read models honor retention, deletion, replay, and tombstone behavior.

## Required proof contents

- Clear boundary: observe, classify, decide, notify, or enforce.
- Negative security cases for public/localhost/request surfaces.
- Rollback and manual-required evidence for enforcement.
- Logs/metrics/traces when runtime or SLO claims move.

## Failure conditions

Do not claim DONE or PR_READY if any apply:

- The expected test/proof row for the touched work type is missing.
- The implementation crate/package test folder does not exist and the missing expected location is not recorded.
- Only happy-path tests pass for a trust, policy, persistence, protocol, UI, AI, platform, security, performance, or observability boundary.
- A product/checklist row moved without command logs and proof artifact path.
- A manual-required/platform limitation was converted into a runtime capability claim.
- A proof artifact lacks negative cases, logs/traces where relevant, or exact workpack/checklist linkage.
- A sibling plan or broad source tree was read without a route reason recorded in the workpack/proof note.
