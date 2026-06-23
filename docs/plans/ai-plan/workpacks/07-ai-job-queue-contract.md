# 07 - AI Job Queue Contract

<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `07 - AI Job Queue Contract`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: AI job queue contract/readiness only for the selected contract tier after focused tests and proof exist.
> Does not prove: provider mesh runtime, local model execution, remote assistant readiness, policy readiness, enforcement readiness, sibling plan completion, PR readiness, or broad DONE.
> Proof rule: Before DONE, apply `workpacks/00-owner-boundary-proof-gate.md`, select tests in TEST_PROOF_EXPECTATIONS.md, and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

AI jobs are event-driven, bounded, prioritized, cancellable, auditable, source-referenced, lease-aware, deduplicated, replayable, and safe under local and household mesh backpressure.

## Where We Are

Provider scheduler proof exists and now proves child-safety priority, same-device parent/child sharing, queued/degraded/unavailable provider states, and one independent runtime access lane per physical device. A broader cross-slice AI job contract still needs to own task scope, evidence refs, parent rule refs, provider route, timeout, result journal refs, claim/lease state, idempotency, and child-agent authority.

## Owner Path

```text
Primary contract owner: packages/schema-domain
Local helper/projection consumer: packages/ai-domain when selected
Rust/runtime consumer: crates/child-ai-core when selected
Wire/service consumer: crates/agent-protocol and crates/agent-service when selected
LAN/remote provider mesh: handoff only unless the selected workpack names provider-mesh runtime proof
```

Do not place canonical `AiWorkItem`, state, lease, result, or dead-letter shapes in browser, screen, app-game, tracking, LAN, remote, portal, policy, or enforcement owners. Those plans should consume typed job/request/result handoffs.

## Checklist

- [ ] Define `AiWorkItem` contract in a neutral shared boundary.
- [ ] Define `AiWorkState` state machine.
- [ ] Define deterministic `dedupeKey` rules.
- [ ] Define aggregate key rules for ordered work transitions.
- [ ] Define idempotency key rules for duplicate jobs, claims, and results.
- [ ] Define provider scheduler queue state and child-safety priority for the local runtime lane.
- [ ] Add timeout, cancellation, retry, TTL, deadline, and max attempts.
- [ ] Define payload mode and custody policy.
- [ ] Require evidence refs, parent-rule refs, and child-agent authority refs.
- [ ] Journal queue, claim, lease, start, complete, fail, validate, accept, reject, requeue, and dead-letter states.
- [ ] Prove no direct capture-to-worker call path.

## Required Proof

```text
proof root: output/ai-plan-proof/07-ai-job-queue-contract/
required files:
  00-scope-summary.md
  01-negative-case-proof.md
  02-no-claim-boundary.md
  16-validation-commands.log
```

Focused proof should include:

```bash
npm run build --workspace @ocentra-parent/schema-domain
npm run test --workspace @ocentra-parent/schema-domain -- ai
npm run build --workspace @ocentra-parent/ai-domain
npm run test --workspace @ocentra-parent/ai-domain
```

If Rust/wire/service consumers are touched, add the focused commands from `TEST_PROOF_EXPECTATIONS.md`.

## Negative Cases

- Invalid job shape is rejected before provider selection.
- Missing evidence refs block the job from claiming content understanding.
- Missing parent-rule refs block policy-adjacent claims.
- Duplicate dedupe/idempotency keys do not execute the job twice.
- Expired or cancelled jobs cannot produce accepted results.
- Lease mismatch or stale provider result is rejected.
- Payload mode cannot include raw private evidence unless the selected custody rule allows it.
- Direct capture-to-worker calls are rejected; evidence must flow through stored refs or typed digests.

## No-Claim Boundary

This workpack can prove queue contract and replay semantics for the selected tier. It does not prove local model execution, provider mesh discovery, household LAN execution, remote assistant readiness, policy/enforcement decision readiness, or product-ready AI.
