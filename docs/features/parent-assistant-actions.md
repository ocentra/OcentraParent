<!-- agent-capsule -->

> Agent Capsule
> Doc: Parent Assistant Actions
> Kind: feature documentation; read only when selected by FEATURE_ROUTE_INDEX, PLAN_INDEX, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# Parent Assistant Actions

## Parent Outcome

A parent can ask Ocentra to explain activity, draft a rule, create a schedule,
preview a policy action, tune sensitivity, or prepare a report without learning
every setting first.

## Ocentra Requirement

The parent assistant is an operator over typed contracts. It can explain,
suggest, draft, preview, and request parent approval. It must not bypass
child-device validation or directly enforce.

## Roadmap And Expectations

- Roadmap: V4 reports and optional assistant, V5 parent policy product.
- Expectations: [parent assistant](../expectations/parent-assistant-chat.md),
  [AI](../expectations/ai.md), [policy](../expectations/policy.md),
  [data custody](../expectations/data-custody.md).
- Modules: `packages/parent-domain`, `packages/agent-protocol-domain`,
  `apps/portal`, `crates/agent-service`.

## Competitor Pressure

See [Competitor Capability Map](../competitor-capability-map.md), especially AI
assistant setup/control, reports/digests, and remote parent access.

Most competitors provide dashboards, alerts, and settings. Ocentra can
differentiate by letting high-tech and low-tech parents use natural language to
configure, explain, and preview rules.

## Current Ocentra State

- Parent assistant contracts and MIA/activity evidence context proof exist.
- Parent Assistant now uses the local service to load the latest saved Activity
  report history as evidence context when the command does not already carry a
  report payload, while preserving explicit report/history payload precedence.
- MIA/report-history evidence contexts now carry typed parent-owned custody and
  source labels, `rawChildEvidenceIncluded=false`, and
  `directEnforcementAllowed=false`.
- Parent Assistant answers and provider-status events now carry a typed
  provider route for local configured/degraded/unavailable and API
  authorized-unavailable/authorized-degraded states. The route keeps remote/API
  AI optional, cites saved report/read-model source refs, and preserves
  `childSafetyOrEnforcementUseAllowed=false`.
- `local-ai-parent-assistant-runtime-proof` records the focused local AI
  assistant runtime boundary: local answers use the shared provider scheduler,
  busy providers degrade or queue without duplicate runtime loads on the same
  physical device, unavailable states remain cited and explicit, API AI stays
  optional, and action preview/confirm cannot write policy or enforce directly.
  The provider scheduler proof now also shows independent Rust runtime lanes per
  physical device while preserving child-safety priority over queued assistant
  work on each device lane.
- Parent Assistant action preview/confirm now has a service-backed boundary that
  requires a preview id before confirmation, rejects raw assistant prose as an
  executable action, preserves cited source refs and audit reason, and returns
  explicit child-agent contract-required/unavailable state without writing
  policy or enforcing.
- Finished portal chat/action flow is not done.

## Current Gap

Assistant threads, provider status UI, parent approval, child-agent validation,
real API provider adapters, and rule-writing flow are incomplete as one product
flow. Saved report citations, cited answers, action previews, and backend
provider routing have runtime proof, the per-device local provider lane is
proved at the service boundary, and preview-before-confirm is now enforced at
the service boundary, but the portal chat surface still needs C-owned
integration. MIA evidence context remains read-model/report citation context
only; it does not transfer raw child evidence, write policy, enforce on child
devices, or validate actions through a real child-agent policy contract.

## Checklist

- [ ] Assistant thread contract.
- [x] Evidence-cited answer contract.
- [x] Action preview contract.
- [x] Preview-before-confirm backend boundary.
- [ ] Draft rule/schedule/approval actions.
- [x] Provider status and degraded state.
- [ ] Parent confirmation before write.
- [ ] Child-agent validation of actions.
- [ ] Portal chat and audit.

## Next AI Instructions

Do not create assistant-only policy. Every suggested action must become a typed
preview, then a parent-confirmed intent, then child-agent validation.
