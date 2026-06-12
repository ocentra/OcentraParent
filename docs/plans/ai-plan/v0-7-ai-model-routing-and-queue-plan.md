# V0.7 AI Model Routing And Queue Plan

<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `V0.7 AI Model Routing And Queue Plan`
> Kind: plan reference document; read only when routed by AGENTS, DOC_INDEX, or workpack.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

## Goal

Create an event-driven AI work queue and provider router so each task uses the
cheapest safe lane first, can delegate execution to trusted household providers
when allowed, and always returns results to the evidence-owning child agent for
validation and policy handoff.

## Worker Lanes

1. Deterministic classifier.
2. Same-device local text LLM.
3. Same-device OCR.
4. Same-device guided VLM.
5. Same-device embedding/memory worker.
6. Trusted household desktop/laptop AI provider.
7. Limited/dormant mobile fallback provider.
8. Parent-approved remote assistant for reports/explanations only.

## Routing Rules

- Deterministic classifiers run before model calls.
- Sensitive jobs prefer same-device execution or minimized/redacted payloads.
- Heavy OCR/VLM/text jobs prefer trusted desktop/laptop providers when
  available and custody-compatible.
- Mobile providers are dormant by default and do not claim heavy jobs while
  desktop/laptop providers are available.
- Remote/API assistant is unavailable for normal child safety.
- Provider route selection must consider trust, capability, custody, resource
  state, queue depth, battery, thermal state, route freshness, and parent
  policy.
- Provider route selection does not grant authority; it selects execution only.
- The evidence-owning child agent validates every provider result before policy
  consumes it.

## Queue Requirements

- job id;
- dedupe key;
- aggregate key;
- source evidence refs;
- parent-rule refs;
- child-agent authority ref;
- task scope;
- job kind;
- required capability;
- provider route;
- provider class;
- payload mode;
- custody policy;
- model/runtime ref;
- timeout/deadline/TTL;
- cancellation;
- retry policy;
- max attempts;
- resource class;
- lease state;
- result journal ref.

## Claim And Lease Requirements

- only one active lease per job;
- duplicate claims are rejected idempotently;
- expired leases requeue or dead-letter according to max attempts;
- wrong-provider, wrong-claim, expired-lease, stale-provider, revoked-provider,
  unsupported-capability, and custody-mismatch results are rejected;
- duplicate `dedupeKey` work cannot execute twice;
- accepted results require child-agent validation before policy.

## Validation

- Queue parser tests.
- Provider route selection tests.
- Backpressure tests.
- Cancellation tests.
- Remote disabled-by-default tests.
- Invalid route rejection tests.
- Resource limit proof on child-device runtime.
- Provider claim/lease tests.
- Duplicate dedupe key tests.
- Result validation tests.
- Mobile dormant/fallback tests.
- Mesh bridge event mapping tests.
