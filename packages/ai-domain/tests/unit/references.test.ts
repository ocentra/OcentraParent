import { describe, expect, it } from 'vitest';

import { LocalAiDegradedState } from '@ocentra-parent/schema-domain/ai-primitives';
import { LocalAiMemoryReferenceSchema } from '@ocentra-parent/schema-domain/ai-references';

describe('ai-domain references', () => {
  it('parses shared AI references through evidence-backed contracts', () => {
    const reference = LocalAiMemoryReferenceSchema.parse({
      memoryReferenceId: 'memory-alpha',
      kind: 'evidence-memory',
      sourceEvidenceReferences: [
        {
          evidenceReferenceId: 'evidence-alpha',
          kind: 'activity-event',
          observedAt: '2026-06-12T00:00:00.000Z',
        },
      ],
      sourcePolicyVersion: 'policy-v1',
      generatedAt: '2026-06-12T00:00:01.000Z',
      confidence: 0.8,
      derivedIndexVersion: 'derived-index-v1',
    });

    expect(reference.memoryReferenceId).toBe('memory-alpha');
    expect(LocalAiDegradedState.ProviderUnavailable).toBe('provider-unavailable');
  });
});
