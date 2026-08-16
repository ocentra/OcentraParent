<!-- agent-capsule -->

> Agent Capsule
> Doc: Validation Flow
> Kind: agent flow documentation; read only when selected by root AGENTS or TASK_ROUTER.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# Validation Flow

Use this before reporting `DONE`, `PR_READY`, PR refresh, CI readiness, or when
working on tests/validation.

## Choose validation by risk

Read `TEST_PROOF_DECISION_MATRIX.md` after the plan/workpack is known. Select
tests from the touched risk surface, not from habit. A docs-only routing change
does not need the same proof as an authenticated protocol or platform adapter
change; an authenticated persistent runtime change needs more than unit tests.

For Rust-first parent architecture packets, validate the owned slice before any
broad repo gate. Use crate/package/file-scoped commands while the worktree is
dirty or other lanes own adjacent files. Do not run repo-wide validation from a
subagent unless the main lane explicitly authorizes it.

## Root gate

```bash
npm run validate
```

The root gate covers release version alignment, CodeQL changed-file checks,
schema/source-boundary checks, Turbo lint/type-check/test tasks, Rust format,
Rust clippy/check/test, integration smoke, local portal smoke, and Playwright UI
coverage against the real Rust service.

Use the root gate near integration/PR readiness, not as the first validation for
per-crate Rust-first packets.

## Before DONE or PR_READY

Pre-commit passing is not enough. Run the heavier gate for each touched
workspace/package/crate:

- lane/hub guards;
- schema/source boundary checks;
- focused tests for touched behavior;
- `lint`, `type-check`, and `build` for every touched TypeScript workspace;
- `cargo check` and focused Rust tests for every touched Rust crate;
- relevant E2E/proof command when touching portal, protocol, runtime, or proof
  behavior.

Every report must list exact commands run. If a heavier command is skipped,
mark the risk explicitly.

## Proof families to consider

Use only the rows that apply: unit, integration, e2e, invariant,
property-based, mutation-style negative, differential, contract,
consumer-driven, version skew, authN/authZ matrix, privilege escalation, token
lifecycle, replay, dedicated security tests, API/schema fuzzing, rate limit,
abuse, CORS/origin/header/host/redirect, concurrency/race/idempotency/ordering,
load/spike/soak/resource exhaustion, migration/rollback/schema drift, chaos,
clock skew/DST/expiry, AI prompt-injection/output invariant/safety regression,
logging/metrics/tracing/alerting, canary/rollback, and human misuse flows.

## Test standard

When writing or changing tests, also read
`.ocentra-ai/rules/ocentra-parent-test-rules.mdc`. Tests must use real
contracts, parsers, services, transports, or UI paths. Test doubles are
forbidden.

Rust test structure must stay visible and honest: create only applicable
category folders/harnesses such as `unit`, `integration`, `contract`,
`security`, `property`, `fuzz`, `replay`, `concurrency`, `migration`,
`compatibility`, `observability`, `performance`, `chaos`, `ai`, or `e2e`.
Remove or ignore empty placeholder folders until a real claimed packet fills
them. Inline Rust tests are allowed only for tiny private invariants.
