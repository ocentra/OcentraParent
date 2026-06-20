import { type Infer, Schema, withParser } from './effect';
import { ActivityEvidenceRefSchema } from './evidence-contracts';
import { ActivityEvidenceIdSchema } from './evidence-primitives';

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
