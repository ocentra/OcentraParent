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
- Provider routing and local AI scheduler work is in progress/proof-backed.
- Finished portal chat/action flow is not done.

## Current Gap

Assistant threads, cited answers, action previews, rule-writing flow, provider
status UI, parent approval, and child-agent validation are incomplete as one
product flow. Saved report citations have runtime proof, but the portal chat
surface still needs C-owned integration.

## Checklist

- [ ] Assistant thread contract.
- [ ] Evidence-cited answer contract.
- [ ] Action preview contract.
- [ ] Draft rule/schedule/approval actions.
- [ ] Provider status and degraded state.
- [ ] Parent confirmation before write.
- [ ] Child-agent validation of actions.
- [ ] Portal chat and audit.

## Next AI Instructions

Do not create assistant-only policy. Every suggested action must become a typed
preview, then a parent-confirmed intent, then child-agent validation.
