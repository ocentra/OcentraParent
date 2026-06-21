import { expect, it } from 'vitest';
import {
  LocalAiRuntimeProviderProofEntrySchema,
  LocalAiRuntimeProviderProofReadModel,
  LocalAiRuntimeProviderProofReadModelSchema,
  LocalAiRuntimeProviderProofRequirementValues,
} from '@ocentra-parent/schema-domain/local-ai-runtime-provider-proof';

it('captures every local AI provider proof requirement with stable ids and singleton runtime loads', () => {
  const readModel = LocalAiRuntimeProviderProofReadModelSchema.parse(LocalAiRuntimeProviderProofReadModel);
  const statusCounts = countBy(readModel.entries.map((entry) => entry.proofStatus));
  const lifecycleCounts = countBy(readModel.entries.map((entry) => entry.schedulerLifecycle));

  expect(readModel.entries).toHaveLength(8);
  expect(new Set(readModel.entries.map((entry) => entry.proofEntryId)).size).toBe(readModel.entries.length);
  expect(new Set(readModel.entries.map((entry) => entry.requirement))).toEqual(
    new Set(LocalAiRuntimeProviderProofRequirementValues)
  );
  expect(statusCounts).toEqual({ proved: 6, degraded: 1, unavailable: 1 });
  expect(lifecycleCounts).toEqual({ idle: 2, queued: 3, degraded: 1, running: 1, unavailable: 1 });
  expect(readModel.entries.every((entry) => entry.runtimeLoadCount <= 1)).toBe(true);
  expect(readModel.entries.every((entry) => entry.runtimeAccessLaneCount === 1)).toBe(true);
  expect(readModel.entries.every((entry) => entry.singletonScope === 'physical-device')).toBe(true);
});

it('proves parent and child roles share the provider instead of loading duplicate runtimes', () => {
  const sharedProvider = entryFor('shared-parent-child-provider');
  const duplicateBlocked = entryFor('no-duplicate-local-model-load');

  expect(sharedProvider.participatingRoles).toEqual(['parent-controller', 'child-agent', 'ai-provider']);
  expect(sharedProvider.providerId).toBe(duplicateBlocked.providerId);
  expect(sharedProvider.runtimeReferenceId).toBe(duplicateBlocked.runtimeReferenceId);
  expect(sharedProvider.physicalDeviceId).toBe(duplicateBlocked.physicalDeviceId);
  expect(duplicateBlocked).toMatchObject({
    runtimeAccessLaneCount: 1,
    runtimeLoadCount: 1,
    duplicateRuntimeBlocked: true,
    childSafetyPriorityProved: true,
    parentAssistantSubmissionAllowed: true,
  });
});

it('keeps child-safety priority ahead of parent assistant on the shared queue', () => {
  const priority = entryFor('child-safety-priority');

  expect(priority.schedulerLifecycle).toBe('queued');
  expect(priority.sourceSchedulerStatus.currentJobClass).toBe('parent-report');
  expect(priority.queue).toMatchObject({
    childSafetyQueued: 1,
    parentAssistantQueued: 1,
    parentReportQueued: 0,
  });
  expect(priority.childSafetyPriorityProved).toBe(true);
  expect(priority.claimBoundary).toContain('not a child-safety model accuracy claim');
});

it('preserves queued, degraded, unavailable, and parent-assistant submission boundaries', () => {
  const degraded = entryFor('queued-degraded-unavailable-lifecycle');
  const parentAssistant = entryFor('parent-assistant-submits-when-allowed');
  const unavailable = entryFor('provider-status-contract-hardening');

  expect(degraded).toMatchObject({
    proofStatus: 'degraded',
    schedulerLifecycle: 'degraded',
    degradedState: 'overloaded',
    parentAssistantSubmissionAllowed: true,
  });
  expect(parentAssistant).toMatchObject({
    schedulerLifecycle: 'running',
    parentAssistantSubmissionAllowed: true,
    runtimeLoadCount: 1,
  });
  expect(parentAssistant.acceptedJobClasses).toEqual(['parent-assistant']);
  expect(unavailable).toMatchObject({
    proofStatus: 'unavailable',
    schedulerLifecycle: 'unavailable',
    runtimeLoadCount: 0,
    unavailableReason: 'local-ai-provider-unconfigured',
  });
});

it('rejects duplicate model loads, invalid child priority, and unavailable status without reason', () => {
  const noDuplicate = entryFor('no-duplicate-local-model-load');
  const priority = entryFor('child-safety-priority');
  const unavailable = entryFor('provider-status-contract-hardening');

  expect(() =>
    LocalAiRuntimeProviderProofEntrySchema.parse({
      ...noDuplicate,
      proofEntryId: 'invalid-duplicate-model-load',
      runtimeLoadCount: 2,
    })
  ).toThrow();
  expect(() =>
    LocalAiRuntimeProviderProofEntrySchema.parse({
      ...noDuplicate,
      proofEntryId: 'invalid-missing-runtime-access-lane',
      runtimeAccessLaneCount: 0,
    })
  ).toThrow();
  expect(() =>
    LocalAiRuntimeProviderProofEntrySchema.parse({
      ...priority,
      proofEntryId: 'invalid-priority-claim',
      childSafetyPriorityProved: false,
    })
  ).toThrow();
  expect(() =>
    LocalAiRuntimeProviderProofEntrySchema.parse({
      ...unavailable,
      proofEntryId: 'invalid-unavailable-without-reason',
      unavailableReason: null,
      sourceSchedulerStatus: {
        ...unavailable.sourceSchedulerStatus,
        unavailableReason: null,
      },
    })
  ).toThrow();
});

function entryFor(requirement: string) {
  const entry = LocalAiRuntimeProviderProofReadModel.entries.find((candidate) => candidate.requirement === requirement);
  if (entry === undefined) {
    throw new Error(`Missing local AI runtime provider proof entry: ${requirement}`);
  }
  return entry;
}

function countBy(values: readonly string[]) {
  return values.reduce<Record<string, number>>((counts, value) => {
    counts[value] = (counts[value] ?? 0) + 1;
    return counts;
  }, {});
}
