import {
  EvidenceCustodyDecisionSchema,
  EvidenceCustodyRequestSchema,
  EvidenceReferenceDecision,
  type EvidenceCustodyDecision,
  type EvidenceCustodyRequest,
} from '@ocentra-parent/schema-domain/evidence-custody-contracts';

export function evaluateEvidenceCustodyReference(input: EvidenceCustodyRequest): EvidenceCustodyDecision {
  const request = EvidenceCustodyRequestSchema.parse(input);
  const evidence = request.availableEvidence.find((candidate) => candidate.evidenceId === request.evidenceId) ?? null;
  const decision = evidence === null ? EvidenceReferenceDecision.Missing : EvidenceReferenceDecision.Accepted;

  return EvidenceCustodyDecisionSchema.parse({
    evidenceId: request.evidenceId,
    decision,
    allowedScope: request.allowedScope,
    evidence,
  });
}
