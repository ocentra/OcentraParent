# V0.6 Local AI Contracts Plan

<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `V0.6 Local AI Contracts Plan`
> Kind: plan reference document; read only when routed by AGENTS, DOC_INDEX, or workpack.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

## Goal

Define the shared AI contract layer before runtime behavior expands. This is the
foundation for every browser, app/game, tracking, screen, network, LAN, memory,
policy, and parent assistant AI consumer.

## Required Contracts

- `LocalAiEvaluationInput`
- `LocalAiSafetyResult`
- `LocalModelRuntimeStatus`
- `LocalProviderCapability`
- `LocalAiJob`
- `LocalAiProviderRoute`
- `LocalAiEvidenceContextBuildRequest`
- `LocalAiEvidenceContext`
- `LocalAiEvidenceContextBuildResult`
- `LocalMemoryReference`
- `LocalGraphReference`
- `PromptTemplateVersion`
- `AiResultJournalEntry`
- `ParentAiExplanation`
- `RemoteAssistantRequest`
- `RemoteAssistantResult`
- `HouseholdAiProviderAdvertisement`
- `HouseholdAiProviderHeartbeat`
- `HouseholdAiProviderCapabilitySnapshot`
- `HouseholdAiProviderSelection`
- `AiWorkItem`
- `AiWorkClaimRequest`
- `AiWorkClaimDecision`
- `AiWorkLease`
- `AiWorkResult`
- `AiWorkResultValidation`
- `AiWorkDeadLetter`
- `MeshTransportMessageEnvelope`

## Contract Rules

- Every input must cite stored evidence refs or parent-rule refs.
- Every result must cite evidence refs, parent-rule refs, runtime refs, and
  prompt/template version.
- Confidence values must be finite and inside `0..1`.
- Missing confidence on probabilistic claims is degraded or rejected.
- AI results cannot contain direct enforcement commands.
- Remote/API assistant contracts are separate from child-device local AI
  contracts.
- Household LAN provider contracts are separate from remote/API assistant
  contracts.
- AI work contracts must distinguish execution provider from policy authority.
- Every AI work item must declare the evidence-owning child agent.
- Every AI result must be validated by the evidence-owning child agent before
  policy consumption.
- Provider-originated results cannot contain policy decisions or enforcement
  commands.
- Memory and graph refs must cite stored evidence, policy versions, or parent
  actions.

## Owned Paths

Expected implementation paths:

- `packages/parent-domain/src/local-ai*.ts`
- `packages/parent-domain/src/local-ai-context*.ts`
- `packages/parent-domain/src/local-ai-references.ts`
- `packages/parent-domain/src/local-ai-provider-scheduler.ts`
- `packages/parent-domain/src/household-ai-provider-mesh*.ts`
- `packages/parent-domain/src/ai-work*.ts`
- `packages/parent-domain/src/ai-provider*.ts`
- `packages/parent-domain/src/ai-work-claim*.ts`
- `packages/parent-domain/src/ai-work-result*.ts`
- `packages/parent-domain/src/parent-assistant*.ts`
- `packages/parent-domain/tests/local-ai*.test.ts`
- `packages/parent-domain/tests/parent-assistant*.test.ts`

## Acceptance

- TypeScript schema tests cover valid and invalid inputs/results.
- Contract tests reject unsourced evidence, unsourced memory, invalid
  confidence, direct enforcement, invalid custody, and remote-child-safety
  payloads.
- Contract tests reject provider policy authority, provider enforcement
  commands, wrong-child-agent ownership, invalid claim/lease state,
  custody-mismatched provider routes, and remote/API child-safety payloads.
- Package exports expose the contracts without duplicate definitions.
- Feature docs/checklist are updated when contract status changes.
