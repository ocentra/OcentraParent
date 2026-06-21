import { AppGameSchemaVersion } from './app-game-primitives';
import { ActivityEvidenceRefSchema } from './evidence-contracts';
import { type Infer, NonEmptyStringSchema, Schema, withParser } from './effect';

const BoundaryCount = Schema.Number.pipe(Schema.nonNegative(), Schema.int());

export const AgentAppGameBoundaryReadModelKind = {
  EvidenceClaim: 'evidenceClaim',
  Identity: 'identity',
  ApprovalAuthority: 'approvalAuthority',
  ApprovalActionResult: 'approvalActionResult',
  PlatformAuthorityMatrix: 'platformAuthorityMatrix',
  PlatformAuthorityRow: 'platformAuthorityRow',
  AiClassifierResult: 'aiClassifierResult',
} as const;

export const AgentAppGameBoundaryReadModelRowSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AppGameSchemaVersion),
    rowId: NonEmptyStringSchema,
    boundaryKind: Schema.Literal(
      AgentAppGameBoundaryReadModelKind.EvidenceClaim,
      AgentAppGameBoundaryReadModelKind.Identity,
      AgentAppGameBoundaryReadModelKind.ApprovalAuthority,
      AgentAppGameBoundaryReadModelKind.ApprovalActionResult,
      AgentAppGameBoundaryReadModelKind.PlatformAuthorityMatrix,
      AgentAppGameBoundaryReadModelKind.PlatformAuthorityRow,
      AgentAppGameBoundaryReadModelKind.AiClassifierResult
    ),
    rowCount: BoundaryCount,
    evidenceReferenceIds: Schema.Array(NonEmptyStringSchema),
    evidence: Schema.Array(ActivityEvidenceRefSchema),
  })
);

export const AgentAppGameBoundaryReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AppGameSchemaVersion),
    generatedAt: NonEmptyStringSchema,
    custodyLabel: NonEmptyStringSchema,
    capabilityStatus: NonEmptyStringSchema,
    returned: BoundaryCount,
    evidenceClaimRowCount: BoundaryCount,
    identityRowCount: BoundaryCount,
    approvalAuthorityRowCount: BoundaryCount,
    approvalActionResultRowCount: BoundaryCount,
    platformAuthorityMatrixCount: BoundaryCount,
    platformAuthorityRowCount: BoundaryCount,
    aiClassifierResultRowCount: BoundaryCount,
    rows: Schema.Array(AgentAppGameBoundaryReadModelRowSchema),
  })
);

export type AgentAppGameBoundaryReadModelKind = Infer<typeof AgentAppGameBoundaryReadModelRowSchema>['boundaryKind'];
export type AgentAppGameBoundaryReadModelRow = Infer<typeof AgentAppGameBoundaryReadModelRowSchema>;
export type AgentAppGameBoundaryReadModel = Infer<typeof AgentAppGameBoundaryReadModelSchema>;
