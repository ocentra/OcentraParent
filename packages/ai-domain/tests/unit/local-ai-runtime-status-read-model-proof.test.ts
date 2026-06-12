import { describe, expect, it } from 'vitest';
import {
  LocalAiRuntimeStatusSurfaceReadModel,
  LocalAiRuntimeStatusSurfaceReadModelSchema,
  LocalAiRuntimeStatusSurfaceRowSchema,
} from '../../src/local-ai-runtime-status-read-model-proof';

describe('local AI runtime status read-model proof', () => {
  it('projects provider proof entries into parent-visible runtime status rows', () => {
    const readModel = LocalAiRuntimeStatusSurfaceReadModelSchema.parse(LocalAiRuntimeStatusSurfaceReadModel);

    expect(readModel.rows).toHaveLength(8);
    expect(new Set(readModel.rows.map((row) => row.rowId)).size).toBe(readModel.rows.length);
    expect(readModel.sourceReadModelIds).toEqual(['local-ai-runtime-provider-proof']);
    expect(readModel.readyVisibleCount).toBe(3);
    expect(readModel.queuedVisibleCount).toBe(3);
    expect(readModel.degradedVisibleCount).toBe(1);
    expect(readModel.unavailableVisibleCount).toBe(1);
    expect(readModel.manualSetupRequiredCount).toBe(0);
    expect(readModel.rows.every((row) => row.parentVisible)).toBe(true);
  });

  it('preserves local runtime/model identifiers and child-safety priority visibility', () => {
    const priorityRow = rowFor('local-ai-proof-child-safety-priority');
    const duplicateRow = rowFor('local-ai-proof-no-duplicate-model-load');

    expect(priorityRow).toMatchObject({
      providerId: 'local-provider-llama-cli',
      runtimeReferenceId: 'local-ai-runtime-local-llama-cli',
      modelId: 'gemma-4-e2b-it-q4-k-m',
      modelReference: 'artifact:gemma_4_e2b_it_q4_k_m',
      surfaceState: 'queued-visible',
      childSafetyPriorityVisible: true,
      queueDepth: 2,
    });
    expect(duplicateRow.childSafetyPriorityVisible).toBe(true);
    expect(duplicateRow.runtimeReferenceId).toBe(priorityRow.runtimeReferenceId);
  });

  it('keeps degraded and unavailable states visible without overclaiming runtime capability', () => {
    const degradedRow = rowFor('local-ai-proof-queued-degraded-lifecycle');
    const unavailableRow = rowFor('local-ai-proof-status-contract-hardening');

    expect(degradedRow).toMatchObject({
      surfaceState: 'degraded-visible',
      degradedState: 'overloaded',
      unavailableReason: null,
      portalRuntimeRenderingClaimed: false,
      remoteApiClaimed: false,
      policyAuthorityClaimed: false,
      enforcementClaimed: false,
    });
    expect(unavailableRow).toMatchObject({
      surfaceState: 'unavailable-visible',
      unavailableReason: 'local-ai-provider-unconfigured',
      remoteApiClaimed: false,
    });
  });

  it('rejects hidden rows, remote/API claims, policy authority, and dishonest counts', () => {
    const readyRow = rowFor('local-ai-proof-single-provider-role');

    expect(() => LocalAiRuntimeStatusSurfaceRowSchema.parse({ ...readyRow, parentVisible: false })).toThrow();
    expect(() => LocalAiRuntimeStatusSurfaceRowSchema.parse({ ...readyRow, remoteApiClaimed: true })).toThrow();
    expect(() => LocalAiRuntimeStatusSurfaceRowSchema.parse({ ...readyRow, policyAuthorityClaimed: true })).toThrow();
    expect(() =>
      LocalAiRuntimeStatusSurfaceReadModelSchema.parse({
        ...LocalAiRuntimeStatusSurfaceReadModel,
        readyVisibleCount: 99,
      })
    ).toThrow();
  });
});

function rowFor(sourceRuntimeProviderProofEntryId: string) {
  const row = LocalAiRuntimeStatusSurfaceReadModel.rows.find(
    (candidate) => candidate.sourceRuntimeProviderProofEntryId === sourceRuntimeProviderProofEntryId
  );
  if (row === undefined) {
    throw new Error(`Missing local AI runtime status row: ${sourceRuntimeProviderProofEntryId}`);
  }
  return row;
}
