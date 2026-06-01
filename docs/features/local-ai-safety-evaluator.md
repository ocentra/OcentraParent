# Local AI Safety Evaluator

## Parent Outcome

Parents can get context-aware decisions and explanations from local evidence:
allow, warn, time-limit, ask-parent, block, or unknown, with evidence refs and
confidence/degraded status.

## Ocentra Requirement

Child-safety AI runs in the child-device safety path. It consumes typed evidence
and parent rules, produces typed results, and feeds deterministic policy. It
does not directly enforce.

## Roadmap And Expectations

- Roadmap: V0.6 local AI contracts, V0.7 local AI policy evaluator, V5 policy
  product.
- Expectations: [AI](../expectations/ai.md), [policy](../expectations/policy.md),
  [data custody](../expectations/data-custody.md).
- Modules: `packages/parent-domain`, `crates/agent-service`,
  `crates/agent-core`.

## Competitor Pressure

See [Competitor Capability Map](../competitor-capability-map.md), especially AI
assistant setup/control, video safety, social app controls, and local-first
privacy.

Competitors often rely on static categories, platform ratings, or cloud ML.
Ocentra's differentiator is local, evidence-backed, parent-rule-aware AI. That
claim needs proof, not slogans.

## Current Ocentra State

- Local AI runtime/provider status and provider scheduler proof exist.
- Dry-run policy evaluator and evidence context builder exist in proof form.
- Model quality, video/social coverage, and enforcement handoff remain
  incomplete.

## Current Gap

Ocentra needs product-grade model configuration, local model artifacts,
confidence handling, degraded states, social/video evidence coverage, parent
explanations, and validation against real evidence.

## Checklist

- [ ] Runtime/provider status.
- [ ] Evidence context builder.
- [ ] Parent-rule context.
- [ ] Local result contract with confidence/degraded state.
- [ ] Deterministic policy integration.
- [ ] Social/video and screen summary handling.
- [ ] Parent explanation and audit.
- [ ] Tests with real stored evidence.

## Next AI Instructions

Keep AI as input to policy, not authority. If local model execution is not
available, expose unavailable/degraded state and deterministic fallback.
