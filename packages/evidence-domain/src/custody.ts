import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ActivityEvidenceRefSchema } from './contracts';
import { ActivityEvidenceIdSchema } from '@ocentra-parent/evidence-domain/primitives';

export const EvidenceCustodyScopeLiteral = {
  LocalOnly: 'local-only',
  FamilyShared: 'family-shared',
  Exportable: 'exportable',
} as const;

export const EvidenceReferenceDecisionLiteral = {
  Accepted: 'accepted',
  Missing: 'missing',
  ScopeMismatch: 'scope-mismatch',
} as const;

export const EvidenceCustodyScopeSchema = withParser(
  Schema.Literal(
    EvidenceCustodyScopeLiteral.LocalOnly,
    EvidenceCustodyScopeLiteral.FamilyShared,
    EvidenceCustodyScopeLiteral.Exportable
  )
);

export const EvidenceReferenceDecisionSchema = withParser(
  Schema.Literal(
    EvidenceReferenceDecisionLiteral.Accepted,
    EvidenceReferenceDecisionLiteral.Missing,
    EvidenceReferenceDecisionLiteral.ScopeMismatch
  )
);

export const EvidenceCustodyRequestSchema = withParser(
  Schema.Struct({
    evidenceId: ActivityEvidenceIdSchema,
    allowedScope: EvidenceCustodyScopeSchema,
    availableEvidence: Schema.Array(ActivityEvidenceRefSchema),
  })
);

export const EvidenceCustodyDecisionSchema = withParser(
  Schema.Struct({
    evidenceId: ActivityEvidenceIdSchema,
    decision: EvidenceReferenceDecisionSchema,
    allowedScope: EvidenceCustodyScopeSchema,
    evidence: Schema.Union(ActivityEvidenceRefSchema, Schema.Null),
  })
);

export type EvidenceCustodyScope = Infer<typeof EvidenceCustodyScopeSchema>;
export type EvidenceReferenceDecision = Infer<typeof EvidenceReferenceDecisionSchema>;
export type EvidenceCustodyRequest = Infer<typeof EvidenceCustodyRequestSchema>;
export type EvidenceCustodyDecision = Infer<typeof EvidenceCustodyDecisionSchema>;

export const EvidenceCustodyScope = {
  LocalOnly: EvidenceCustodyScopeSchema.parse(EvidenceCustodyScopeLiteral.LocalOnly),
  FamilyShared: EvidenceCustodyScopeSchema.parse(EvidenceCustodyScopeLiteral.FamilyShared),
  Exportable: EvidenceCustodyScopeSchema.parse(EvidenceCustodyScopeLiteral.Exportable),
} as const;

export const EvidenceReferenceDecision = {
  Accepted: EvidenceReferenceDecisionSchema.parse(EvidenceReferenceDecisionLiteral.Accepted),
  Missing: EvidenceReferenceDecisionSchema.parse(EvidenceReferenceDecisionLiteral.Missing),
  ScopeMismatch: EvidenceReferenceDecisionSchema.parse(EvidenceReferenceDecisionLiteral.ScopeMismatch),
} as const;

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
