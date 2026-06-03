# V0.6 Local AI Contracts Plan

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

## Contract Rules

- Every input must cite stored evidence refs or parent-rule refs.
- Every result must cite evidence refs, parent-rule refs, runtime refs, and
  prompt/template version.
- Confidence values must be finite and inside `0..1`.
- Missing confidence on probabilistic claims is degraded or rejected.
- AI results cannot contain direct enforcement commands.
- Remote/API assistant contracts are separate from child-device local AI
  contracts.
- Memory and graph refs must cite stored evidence, policy versions, or parent
  actions.

## Owned Paths

Expected implementation paths:

- `packages/parent-domain/src/local-ai*.ts`
- `packages/parent-domain/src/local-ai-context*.ts`
- `packages/parent-domain/src/local-ai-references.ts`
- `packages/parent-domain/src/local-ai-provider-scheduler.ts`
- `packages/parent-domain/src/parent-assistant*.ts`
- `packages/parent-domain/tests/local-ai*.test.ts`
- `packages/parent-domain/tests/parent-assistant*.test.ts`

## Acceptance

- TypeScript schema tests cover valid and invalid inputs/results.
- Contract tests reject unsourced evidence, unsourced memory, invalid
  confidence, direct enforcement, invalid custody, and remote-child-safety
  payloads.
- Package exports expose the contracts without duplicate definitions.
- Feature docs/checklist are updated when contract status changes.
