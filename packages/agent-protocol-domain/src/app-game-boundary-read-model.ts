import { AppGameSchemaVersion } from '@ocentra-parent/activity-domain/app-game';
import { ActivityEvidenceRefSchema } from '@ocentra-parent/activity-domain/contracts';
import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AgentEvent, AgentProtocolDefaults, isAgentProtocolLogText, type AgentEventEnvelope } from './contracts';

const BoundaryText = Schema.String.pipe(Schema.minLength(1));
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
    rowId: BoundaryText,
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
    evidenceReferenceIds: Schema.Array(BoundaryText),
    evidence: Schema.Array(ActivityEvidenceRefSchema),
  })
);

export const AgentAppGameBoundaryReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AppGameSchemaVersion),
    generatedAt: BoundaryText,
    custodyLabel: BoundaryText,
    capabilityStatus: BoundaryText,
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

export type AgentAppGameBoundaryReadModelKind = Infer<
  typeof AgentAppGameBoundaryReadModelRowSchema
>['boundaryKind'];
export type AgentAppGameBoundaryReadModelRow = Infer<typeof AgentAppGameBoundaryReadModelRowSchema>;
export type AgentAppGameBoundaryReadModel = Infer<typeof AgentAppGameBoundaryReadModelSchema>;

export type AgentAppGameBoundaryReadModelFailureReason =
  | 'wrong-event'
  | 'missing-json-field'
  | 'invalid-json'
  | 'invalid-payload';

export type AgentAppGameBoundaryReadModelResult =
  | {
      readonly ok: true;
      readonly value: AgentAppGameBoundaryReadModel;
    }
  | {
      readonly ok: false;
      readonly reason: AgentAppGameBoundaryReadModelFailureReason;
    };

export function parseAgentAppGameBoundaryReadModelEvent(
  event: AgentEventEnvelope
): AgentAppGameBoundaryReadModelResult {
  if (event.event !== AgentEvent.ActivityAppGameBoundaryReadModelReported) {
    return adapterFailure('wrong-event');
  }

  const raw = event.payload[AgentProtocolDefaults.Field.ActivityAppGameBoundaryReadModel];
  if (!isAgentProtocolLogText(raw)) {
    return adapterFailure('missing-json-field');
  }

  let decoded: unknown;
  try {
    decoded = JSON.parse(raw);
  } catch {
    return adapterFailure('invalid-json');
  }

  const parsed = AgentAppGameBoundaryReadModelSchema.safeParse(decoded);
  if (!parsed.success || parsed.data === undefined) {
    return adapterFailure('invalid-payload');
  }

  return {
    ok: true,
    value: parsed.data,
  };
}

function adapterFailure(reason: AgentAppGameBoundaryReadModelFailureReason): AgentAppGameBoundaryReadModelResult {
  return {
    ok: false,
    reason,
  };
}
