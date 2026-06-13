import { describe, expect, it } from 'vitest';
import {
  EvidenceCustodyScope,
  EvidenceReferenceDecision,
  evaluateEvidenceCustodyReference,
} from '../../src/custody';

const availableEvidence = [
  {
    evidenceId: 'evidence-1',
    kind: 'local-db-row',
    digest: null,
    uri: null,
  },
] as const;

describe('evidence custody contracts', () => {
  it('accepts only available evidence references through a structured custody request', () => {
    const decision = evaluateEvidenceCustodyReference({
      evidenceId: 'evidence-1',
      allowedScope: EvidenceCustodyScope.LocalOnly,
      availableEvidence,
    });

    expect(decision.decision).toBe(EvidenceReferenceDecision.Accepted);
    expect(decision.evidence?.evidenceId).toBe('evidence-1');
  });

  it('rejects missing evidence references without substituting another reference', () => {
    const decision = evaluateEvidenceCustodyReference({
      evidenceId: 'evidence-2',
      allowedScope: EvidenceCustodyScope.LocalOnly,
      availableEvidence,
    });

    expect(decision.decision).toBe(EvidenceReferenceDecision.Missing);
    expect(decision.evidence).toBeNull();
  });
});
