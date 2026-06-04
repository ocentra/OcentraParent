import { expect, it } from 'vitest';
import {
  LocalAiParentAssistantRuntimeProofEntrySchema,
  LocalAiParentAssistantRuntimeProofReadModelSchema,
  LocalAiParentAssistantRuntimeProofRequirementValues,
} from '../src/local-ai-parent-assistant-runtime-proof';
import { LocalAiParentAssistantRuntimeProofReadModel } from '../src/local-ai-parent-assistant-runtime-proof-values';

it('captures every local AI parent assistant runtime proof requirement', () => {
  const readModel = LocalAiParentAssistantRuntimeProofReadModelSchema.parse(
    LocalAiParentAssistantRuntimeProofReadModel
  );

  expect(readModel.entries).toHaveLength(6);
  expect(new Set(readModel.entries.map((entry) => entry.proofEntryId)).size).toBe(readModel.entries.length);
  expect(new Set(readModel.entries.map((entry) => entry.requirement))).toEqual(
    new Set(LocalAiParentAssistantRuntimeProofRequirementValues)
  );
  expect(countBy(readModel.entries.map((entry) => entry.proofStatus))).toEqual({
    proved: 3,
    degraded: 1,
    unavailable: 1,
    'not-claimed': 1,
  });
});

it('ties local assistant answers to the shared provider runtime lane', () => {
  const entry = entryFor('local-provider-answer-uses-shared-runtime');

  expect(entry.parentAssistantAnswer?.answerState).toBe('answered');
  expect(entry.parentAssistantAnswer?.providerRoute.routingState).toBe('local-provider-ready');
  expect(entry.providerStatus?.schedulerStatus.providerId).toBe(entry.parentAssistantAnswer?.providerId);
  expect(entry.providerStatus?.schedulerStatus.runtimeReferenceId).toBeTruthy();
  expect(entry.localProviderSelected).toBe(true);
  expect(entry.apiProviderSelected).toBe(false);
  expect(entry.parentAssistantAnswer?.citations).toHaveLength(1);
});

it('keeps degraded and queued assistant states behind the singleton provider boundary', () => {
  const degraded = entryFor('busy-provider-degrades-without-extra-runtime');
  const priority = entryFor('child-safety-priority-keeps-assistant-queued');

  expect(degraded.parentAssistantAnswer?.answerState).toBe('degraded');
  expect(degraded.providerStatus?.busy).toBe(true);
  expect(degraded.providerStatus?.schedulerStatus.duplicateRuntimeBlocked).toBe(true);
  expect(priority.parentAssistantAnswer?.answerState).toBe('queued');
  expect(priority.providerStatus?.schedulerStatus.queue).toMatchObject({
    childSafetyQueued: 1,
    parentAssistantQueued: 1,
  });
});

it('preserves explicit unavailable and optional API boundaries without child-safety or enforcement use', () => {
  const unavailable = entryFor('provider-unavailable-is-explicit-and-cited');
  const apiBoundary = entryFor('api-provider-remains-optional-parent-authorized-boundary');

  expect(unavailable.parentAssistantAnswer?.answerText).toBeNull();
  expect(unavailable.parentAssistantAnswer?.unavailableReason).toBe('local-ai-provider-unconfigured');
  expect(unavailable.providerStatus?.providerRoute.routingState).toBe('no-provider-available');
  expect(apiBoundary.proofStatus).toBe('not-claimed');
  expect(apiBoundary.providerStatus?.apiProviderBoundary.authorizationState).toBe('authorized');
  expect(apiBoundary.providerStatus?.providerRoute.routingState).toBe('api-provider-authorized-degraded');
  expect(apiBoundary.providerStatus?.providerRoute.remoteAiOptional).toBe(true);
  expect(apiBoundary.providerStatus?.providerRoute.childSafetyOrEnforcementUseAllowed).toBe(false);
});

it('keeps action preview and confirmation contract-required and non-enforcing', () => {
  const action = entryFor('action-preview-confirm-requires-child-contract');

  expect(action.actionPreviewResult).toMatchObject({
    enforcementApplied: false,
    policyWritten: false,
    previewRequired: true,
    previewSatisfied: true,
    rawAssistantProseAccepted: false,
  });
  expect(action.actionConfirmResult).toMatchObject({
    childAgentContractRequired: true,
    confirmState: 'contract-required',
    enforcementApplied: false,
    policyWritten: false,
    previewRequired: true,
    previewSatisfied: true,
    rawAssistantProseAccepted: false,
  });
});

it('rejects source mismatches, remote overclaims, and direct action writes', () => {
  const local = entryFor('local-provider-answer-uses-shared-runtime');
  const action = entryFor('action-preview-confirm-requires-child-contract');

  expect(
    LocalAiParentAssistantRuntimeProofEntrySchema.safeParse({
      ...local,
      sourceProviderProofRequirement: 'provider-status-contract-hardening',
    }).success
  ).toBe(false);
  expect(
    LocalAiParentAssistantRuntimeProofEntrySchema.safeParse({
      ...local,
      remoteAiOptional: false,
    }).success
  ).toBe(false);
  expect(
    LocalAiParentAssistantRuntimeProofEntrySchema.safeParse({
      ...action,
      actionConfirmResult: {
        ...action.actionConfirmResult!,
        policyWritten: true,
      },
    }).success
  ).toBe(false);
});

function entryFor(requirement: (typeof LocalAiParentAssistantRuntimeProofRequirementValues)[number]) {
  const entry = LocalAiParentAssistantRuntimeProofReadModel.entries.find(
    (candidate) => candidate.requirement === requirement
  );
  if (entry === undefined) {
    throw new Error(`Missing local AI parent assistant runtime proof entry: ${requirement}`);
  }
  return entry;
}

function countBy(values: readonly string[]) {
  return values.reduce<Record<string, number>>((counts, value) => {
    counts[value] = (counts[value] ?? 0) + 1;
    return counts;
  }, {});
}
