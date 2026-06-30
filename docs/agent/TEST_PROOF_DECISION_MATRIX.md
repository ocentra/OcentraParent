<!-- agent-capsule -->

> Agent Capsule
> Doc: Test and Proof Decision Matrix
> Kind: agent flow documentation; read only when selected by root AGENTS or TASK_ROUTER.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# Test and Proof Decision Matrix

Use this after the route, plan, and assigned workpack are known. Do not read
every test/proof doc first. Select rows whose trigger matches the touched
behavior, then run or cite the exact command/artifact.

## Baseline rules

- Tests must exercise real contracts, parsers, services, transports, or UI paths.
- Rust tests must be visible at the crate level for closure evidence. Use
  `tests/<category>/...` or a crate-level `tests/<category>.rs` harness with
  real test modules. Inline `#[cfg(test)]` blocks count only for tiny private
  invariants, not plan or product closure.
- Do not keep empty `.gitkeep` test category folders as proof. Add a category
  only when that crate has real tests or an immediate claimed packet to fill it.
- Test doubles, fake-green seams, broad mocks, spies, MSW, Nock, Sinon,
  `vi.mock`, `vi.fn`, and equivalent shortcuts are forbidden unless an owning
  rule explicitly permits a narrow adapter boundary.
- Happy-path tests alone are not enough for behavior that crosses trust,
  policy, persistence, protocol, scheduling, AI, or platform boundaries.
- If a risk row applies and no test/proof exists, keep the checklist row open and
  report the missing proof instead of claiming completion.
- Use explicit families: unit, integration, contract, and e2e tests where each is
  the minimum boundary needed for the claim. Invariant and fuzz tests are required
  for parsers, external input, and schema boundaries. Mutation and differential
  tests are mandatory for security/policy/entitlement-risk paths.
- For Rust-first parent architecture packets, prove the Rust owner first:
  serde/encoded-shape tests for `crates/schema`, route/action/read-model tests
  for `crates/parent-runtime-core`, and domain crate tests for product logic.
  TypeScript tests should prove rendering generated snapshots or dispatching
  generated actions, not reconstructing product truth.

## Test selection by risk

| Trigger in assigned work                                               | Required test/proof family                                                                                                                                   |
| ---------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Pure parser, schema, brand, route helper, constant map                 | Unit plus invariant tests; schema fuzzing when input is external or untrusted.                                                                               |
| Shared domain contract consumed by multiple packages/crates            | Contract tests, consumer-driven examples where available, version-skew/backward-compatibility checks.                                                        |
| Protocol, WebSocket, API, GraphQL, command/event shape                 | Integration tests with real parser/transport; API/schema fuzzing; depth/size/replay negatives where applicable.                                              |
| AuthN, authZ, family/device role, policy authority, privilege boundary | Auth matrix, privilege-escalation negatives, token lifecycle, replay, stale-session, and cross-family isolation proof.                                       |
| Browser/network/request surface                                        | CORS/origin/header/host/redirect/URL-hijack tests; header injection, request splitting, smuggling/desync, cache poisoning where relevant.                    |
| Rate limit, abuse, brute force, lockout, quota, retry                  | Abuse tests, rate-limit proof, brute-force negatives, retry-storm and DoS boundary checks.                                                                   |
| Persistence, journal, SQLite, migrations, read models                  | Migration/rollback, idempotency, ordering, replay, schema drift, backward compatibility, and recovery proof.                                                 |
| Concurrency, queue, scheduler, sync, retry, distributed ordering       | Race, idempotency, replay, ordering, partial outage, slow dependency, retry storm, and clock-skew tests.                                                     |
| Time windows, expiry, schedules, quiet hours, geofence timing          | Clock skew, expiry boundary, DST, timezone, double-submit, refresh-abuse, and replay proof.                                                                  |
| Portal or child UI behavior                                            | Real UI/e2e or component path without fake data; beyond-happy-path states, permission states, loading/error/empty states, accessibility-relevant assertions. |
| Platform adapter, OS service, device capability, installer             | Real service/adapter smoke where possible; manual-required proof for platform gaps; rollback/uninstall/cleanup proof.                                        |
| AI prompt, model output, classifier, assistant, safety summary         | Prompt-injection, hallucination regression, output invariants, safety boundary, temperature sensitivity, redaction, and human-review proof.                  |
| Logging, metrics, tracing, alerting, incident workflow                 | Logging assertion, metrics sanity, tracing completeness, alert firing, error-budget burn, and redaction proof.                                               |
| Performance-sensitive service, stream, worker, or long-running process | Load, spike, soak, memory, file descriptor, connection exhaustion, cancellation, and cleanup proof.                                                          |
| Security-sensitive public or localhost boundary                        | Dedicated security test plus smuggling/desync/cache poisoning/origin/header/redirect checks as applicable.                                                   |
| CI, release, package, canary, rollback                                 | CI dependency-kill or failure-mode proof, package smoke, canary/rollback validation, version/tag alignment.                                                  |
| Flaky or brittle behavior                                              | Flaky detection, repeated-run proof, mutation score or mutation-style negative where useful.                                                                 |
| Human workflow or parent action                                        | Misuse, double submit, stale screen, refresh abuse, audit trail, and rollback proof.                                                                         |

## Proof artifact router

| Claim type                       | Proof expected                                                                                          |
| -------------------------------- | ------------------------------------------------------------------------------------------------------- |
| Contract exists                  | Source path, schema/brand decode tests, negative decode cases, consumer path if shared.                 |
| Runtime works                    | Real service/transport/adapter output, journal or read-model evidence, and focused integration command. |
| UI works                         | Screenshot or Playwright/browser proof for every touched state, plus the command that produced it.      |
| Security boundary holds          | Negative proof showing forbidden actor/input/path is rejected and logged safely.                        |
| AI boundary holds                | Prompt/output fixture, invariant check, redaction/safety proof, and regression command.                 |
| Platform capability is real      | Device/OS/version, permission/enrollment state, adapter output, limitations, manual-required notes.     |
| Performance or reliability holds | Load/soak/spike or retry/partial-outage output, resource limits, cleanup results.                       |
| Docs/status changed              | Exact feature/checklist/plan rows changed, evidence path, and why no product status moved if none did.  |

## Escalation rule

When several rows apply, choose the strictest one. For example, an authenticated
policy API with persistence needs auth matrix, schema/API fuzzing, replay or
idempotency negatives, persistence rollback/read-model proof, and logging
assertions. Do not substitute a single unit test for a multi-boundary claim.
