# AI Plan Test and Proof Expectations

<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `AI Plan Test and Proof Expectations`
> Kind: plan-local test and proof decision tree.
> Read when: After the assigned workpack/checklist row is known; use to choose required tests/proof.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: This file defines required local tests/proof; missing tests keep rows open.

<!-- /agent-capsule -->

Use this after the assigned AI workpack is known. The goal is to force local/remote AI work to prove custody, deterministic fallbacks, output invariants, and provider boundaries without reading unrelated browser, screen, or tracking plans.

## Where tests should live

When the AI implementation crate/package exists, AI tests belong in that implementation test tree and proof output under its proof folder. Until then, colocate with the owning AI/domain/runtime package and record the path in the workpack and `PROOF_INDEX.md`.

## Decision Tree

| If the assigned work is...                                                                 | Read next                                        | Expected tests or proof                                                                                                                              |
| ------------------------------------------------------------------------------------------ | ------------------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------- |
| WP02 snapshot/gap map                                                                      | `DOC_INDEX.md`, exact source docs only           | `ai-doc-link-sanity`, `ai-gap-map-no-product-claim-proof`.                                                                                           |
| WP03-WP04 contracts and Rust protocol parity                                               | owning domain README, source-boundary flow       | schema decode negatives, branded ids, TS/Rust serialization parity, version-skew tests.                                                              |
| WP05-WP08 runtime status, provider capability, job queue, provider routing                 | assigned workpack                                | provider-state matrix, queue idempotency/replay, duplicate job prevention, retry/dead-letter, fallback routing proof.                                |
| WP09-WP12 context builder, evidence refs, rule context, prompt registry                    | assigned workpack                                | evidence custody invariants, redaction tests, prompt template version tests, no raw private payload leakage.                                         |
| WP13-WP18 classifier, LLM adapter, output parser, degraded handling, policy evaluator      | assigned workpack                                | deterministic no-model regression, prompt injection, hallucination/output invariant tests, timeout/invalid-output handling, temperature sensitivity. |
| WP19-WP25 result journal, explanation, memory, graph refs                                  | assigned workpack                                | journal replay, migration/rollback, memory custody, graph edge invariants, stale evidence negatives.                                                 |
| WP26-WP29 TabAgent reuse candidates                                                        | exact workpack only                              | reuse audit proof, boundary mapping, no imported authority bypass, lifecycle/cache compatibility proof.                                              |
| WP30-WP39 OCR/VLM/domain lanes                                                             | assigned workpack plus owning plan only if named | cross-plan handoff proof, image/text fixture invariants, redaction, model-fit/resource proof.                                                        |
| WP40-WP47 model artifacts, settings, portal, provider API, remote assistant, security/perf | assigned workpack                                | artifact integrity/hash/cache tests, API auth custody, prompt-injection/redaction, resource/battery proof, performance limits.                       |
| WP48 rollout gate                                                                          | `PROOF_INDEX.md`                                 | complete proof manifest, selected risk rows, skipped-heavy-check risks, PR validation list.                                                          |

## Expected test/proof inventory

Use these names as proof intent labels in the assigned workpack/proof note. Implementers choose the actual crate/package test names after the owning implementation boundary exists.

- `ai.contract.schema-negative-decode`: invalid AI request/result/context shapes are rejected with safe diagnostics.
- `ai.provider.routing-capability-matrix`: provider choice respects local capability, custody, battery/resource, and unavailable states.
- `ai.output.invariant-regression`: model/classifier output stays schema-valid, bounded, and policy-neutral across regression fixtures.
- `ai.prompt-injection.boundary`: prompt/input attempts cannot override safety rules, custody, or parent policy authority.
- `ai.redaction.no-raw-sensitive-transfer`: remote/API paths receive only approved redacted summaries.
- `ai.result-journal.replay-idempotency`: duplicate/stale AI results do not create duplicate policy/audit state.
- `ai.parent-explanation.evidence-citation`: explanations cite evidence/rule/model refs and show degraded states.
- `ai.no-direct-enforcement`: AI evidence cannot execute or imply enforcement without deterministic policy handoff proof.

## Required proof contents

- Prompt/input/output fixtures with schema validation result.
- Safety and redaction assertions for every assistant/classifier output.
- Provider state, fallback, timeout, retry, and invalid-output evidence.
- Artifact integrity and cache/retention proof for local models.
- Exact command logs and proof path linked from the workpack.

## Failure conditions

Do not claim DONE or PR_READY if any apply:

- The expected test/proof row for the touched work type is missing.
- The implementation crate/package test folder does not exist and the missing expected location is not recorded.
- Only happy-path tests pass for a trust, policy, persistence, protocol, UI, AI, platform, security, performance, or observability boundary.
- A product/checklist row moved without command logs and proof artifact path.
- A manual-required/platform limitation was converted into a runtime capability claim.
- A proof artifact lacks negative cases, logs/traces where relevant, or exact workpack/checklist linkage.
- A sibling plan or broad source tree was read without a route reason recorded in the workpack/proof note.
