import { AppGameSchemaVersion } from '@ocentra-parent/activity-domain/app-game';
import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AgentEvent, AgentProtocolDefaults, isAgentProtocolLogText, type AgentEventEnvelope } from './contracts';

const PolicyPreviewText = Schema.String.pipe(Schema.minLength(1));
const PolicyPreviewCount = Schema.Number.pipe(Schema.nonNegative(), Schema.int());

export const AgentAppGamePolicyPreviewTargetDomain = {
  NativeApp: 'native-app',
  NativeGame: 'native-game',
  NotAppGame: 'not-app-game',
} as const;

export const AgentAppGamePolicyPreviewUnavailableReason = {
  SourceTargetKindNotPersisted: 'source-target-kind-not-persisted',
} as const;

const PolicyPreviewTargetSchema = Schema.Struct({
  targetId: PolicyPreviewText,
  targetType: PolicyPreviewText,
  targetValue: PolicyPreviewText,
});

const PolicyPreviewDecisionSchema = Schema.Struct({
  schemaVersion: PolicyPreviewText,
  decisionId: PolicyPreviewText,
  action: PolicyPreviewText,
  reasonCodes: Schema.Array(PolicyPreviewText),
  evidenceReferences: Schema.Array(Schema.Unknown),
  ruleIds: Schema.Array(PolicyPreviewText),
  localAiResultId: Schema.Union(PolicyPreviewText, Schema.Null),
  dryRun: Schema.Boolean,
  enforcementHandoffState: PolicyPreviewText,
  expiresAt: Schema.Union(PolicyPreviewText, Schema.Null),
});

const PolicyPreviewServiceRowSchema = Schema.Struct({
  previewId: PolicyPreviewText,
  sourceEventId: PolicyPreviewText,
  observedAt: PolicyPreviewText,
  target: PolicyPreviewTargetSchema,
  evidenceReferences: Schema.Array(Schema.Unknown),
  parentRuleContextReferences: Schema.Array(Schema.Unknown),
  decision: PolicyPreviewDecisionSchema,
});

const PolicyPreviewServiceReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: PolicyPreviewText,
    generatedAt: PolicyPreviewText,
    custody: PolicyPreviewText,
    limit: PolicyPreviewCount,
    returned: PolicyPreviewCount,
    capabilityStatus: PolicyPreviewText,
    rows: Schema.Array(PolicyPreviewServiceRowSchema),
  }).pipe(
    Schema.filter(
      (readModel) =>
        readModel.rows.every(
          (row) => row.decision.dryRun && row.decision.enforcementHandoffState === 'disabled'
        ) || 'Expected app/game policy preview service rows to stay dry-run with disabled enforcement handoff'
    )
  )
);

export const AgentAppGamePolicyPreviewRowSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AppGameSchemaVersion),
    rowId: PolicyPreviewText,
    sourcePreviewId: PolicyPreviewText,
    sourceEventId: PolicyPreviewText,
    observedAt: PolicyPreviewText,
    targetDomain: Schema.Literal(
      AgentAppGamePolicyPreviewTargetDomain.NativeApp,
      AgentAppGamePolicyPreviewTargetDomain.NativeGame,
      AgentAppGamePolicyPreviewTargetDomain.NotAppGame
    ),
    targetType: PolicyPreviewText,
    targetValue: PolicyPreviewText,
    policyDecisionId: PolicyPreviewText,
    policyAction: PolicyPreviewText,
    reasonCodes: Schema.Array(PolicyPreviewText),
    ruleIds: Schema.Array(PolicyPreviewText),
    evidenceReferenceCount: PolicyPreviewCount,
    parentRuleContextReferenceCount: PolicyPreviewCount,
    dryRun: Schema.Literal(true),
    enforcementHandoffState: Schema.Literal('disabled'),
    policyEvaluatorRuntimeClaimed: Schema.Literal(false),
    timerRuntimeClaimed: Schema.Literal(false),
    adapterDispatchClaimed: Schema.Literal(false),
    childDeliveryClaimed: Schema.Literal(false),
    platformEnforcementClaimed: Schema.Literal(false),
  })
);

export const AgentAppGamePolicyPreviewReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AppGameSchemaVersion),
    generatedAt: PolicyPreviewText,
    custodyLabel: PolicyPreviewText,
    capabilityStatus: PolicyPreviewText,
    returned: PolicyPreviewCount,
    nativeAppPreviewRowCount: PolicyPreviewCount,
    nativeGamePreviewRowCount: PolicyPreviewCount,
    notAppGameRowCount: PolicyPreviewCount,
    nativeGamePromotionClaimed: Schema.Literal(false),
    nativeGameUnavailableReason: Schema.Literal(
      AgentAppGamePolicyPreviewUnavailableReason.SourceTargetKindNotPersisted
    ),
    policyEvaluatorRuntimeClaimed: Schema.Literal(false),
    timerRuntimeClaimed: Schema.Literal(false),
    adapterDispatchClaimed: Schema.Literal(false),
    childDeliveryClaimed: Schema.Literal(false),
    platformEnforcementClaimed: Schema.Literal(false),
    rows: Schema.Array(AgentAppGamePolicyPreviewRowSchema),
  }).pipe(
    Schema.filter(
      (readModel) =>
        readModel.nativeAppPreviewRowCount ===
          countRows(readModel.rows, AgentAppGamePolicyPreviewTargetDomain.NativeApp) &&
        readModel.nativeGamePreviewRowCount ===
          countRows(readModel.rows, AgentAppGamePolicyPreviewTargetDomain.NativeGame) &&
        readModel.notAppGameRowCount ===
          countRows(readModel.rows, AgentAppGamePolicyPreviewTargetDomain.NotAppGame) ||
        'Expected app/game policy preview service counts to match rows'
    )
  )
);

export type AgentAppGamePolicyPreviewTargetDomain = Infer<
  typeof AgentAppGamePolicyPreviewRowSchema
>['targetDomain'];
export type AgentAppGamePolicyPreviewRow = Infer<typeof AgentAppGamePolicyPreviewRowSchema>;
export type AgentAppGamePolicyPreviewReadModel = Infer<typeof AgentAppGamePolicyPreviewReadModelSchema>;

export type AgentAppGamePolicyPreviewFailureReason =
  | 'wrong-event'
  | 'missing-json-field'
  | 'invalid-json'
  | 'invalid-payload';

export type AgentAppGamePolicyPreviewResult =
  | {
      readonly ok: true;
      readonly value: AgentAppGamePolicyPreviewReadModel;
    }
  | {
      readonly ok: false;
      readonly reason: AgentAppGamePolicyPreviewFailureReason;
    };

export function parseAgentAppGamePolicyPreviewEvent(event: AgentEventEnvelope): AgentAppGamePolicyPreviewResult {
  if (event.event !== AgentEvent.PolicyPreviewReadModelReported) {
    return adapterFailure('wrong-event');
  }

  const raw = event.payload[AgentProtocolDefaults.Field.Payload];
  if (!isAgentProtocolLogText(raw)) {
    return adapterFailure('missing-json-field');
  }

  let decoded: unknown;
  try {
    decoded = JSON.parse(raw);
  } catch {
    return adapterFailure('invalid-json');
  }

  const parsed = PolicyPreviewServiceReadModelSchema.safeParse(decoded);
  if (!parsed.success || parsed.data === undefined) {
    return adapterFailure('invalid-payload');
  }

  return {
    ok: true,
    value: buildAppGamePolicyPreviewReadModel(parsed.data),
  };
}

function buildAppGamePolicyPreviewReadModel(
  serviceReadModel: Infer<typeof PolicyPreviewServiceReadModelSchema>
): AgentAppGamePolicyPreviewReadModel {
  const rows = serviceReadModel.rows.map(buildAppGamePolicyPreviewRow);
  return AgentAppGamePolicyPreviewReadModelSchema.parse({
    schemaVersion: AppGameSchemaVersion,
    generatedAt: serviceReadModel.generatedAt,
    custodyLabel: serviceReadModel.custody,
    capabilityStatus: serviceReadModel.capabilityStatus,
    returned: serviceReadModel.returned,
    nativeAppPreviewRowCount: countRows(rows, AgentAppGamePolicyPreviewTargetDomain.NativeApp),
    nativeGamePreviewRowCount: 0,
    notAppGameRowCount: countRows(rows, AgentAppGamePolicyPreviewTargetDomain.NotAppGame),
    nativeGamePromotionClaimed: false,
    nativeGameUnavailableReason: AgentAppGamePolicyPreviewUnavailableReason.SourceTargetKindNotPersisted,
    policyEvaluatorRuntimeClaimed: false,
    timerRuntimeClaimed: false,
    adapterDispatchClaimed: false,
    childDeliveryClaimed: false,
    platformEnforcementClaimed: false,
    rows,
  });
}

function buildAppGamePolicyPreviewRow(
  row: Infer<typeof PolicyPreviewServiceRowSchema>
): AgentAppGamePolicyPreviewRow {
  return AgentAppGamePolicyPreviewRowSchema.parse({
    schemaVersion: AppGameSchemaVersion,
    rowId: row.previewId,
    sourcePreviewId: row.previewId,
    sourceEventId: row.sourceEventId,
    observedAt: row.observedAt,
    targetDomain: appGamePolicyPreviewTargetDomain(row.target.targetType),
    targetType: row.target.targetType,
    targetValue: row.target.targetValue,
    policyDecisionId: row.decision.decisionId,
    policyAction: row.decision.action,
    reasonCodes: row.decision.reasonCodes,
    ruleIds: row.decision.ruleIds,
    evidenceReferenceCount: row.evidenceReferences.length,
    parentRuleContextReferenceCount: row.parentRuleContextReferences.length,
    dryRun: row.decision.dryRun,
    enforcementHandoffState: row.decision.enforcementHandoffState,
    policyEvaluatorRuntimeClaimed: false,
    timerRuntimeClaimed: false,
    adapterDispatchClaimed: false,
    childDeliveryClaimed: false,
    platformEnforcementClaimed: false,
  });
}

function appGamePolicyPreviewTargetDomain(targetType: string): AgentAppGamePolicyPreviewTargetDomain {
  if (targetType === 'app' || targetType === 'process' || targetType === 'window') {
    return AgentAppGamePolicyPreviewTargetDomain.NativeApp;
  }
  return AgentAppGamePolicyPreviewTargetDomain.NotAppGame;
}

function countRows(rows: readonly AgentAppGamePolicyPreviewRow[], targetDomain: AgentAppGamePolicyPreviewTargetDomain) {
  return rows.filter((row) => row.targetDomain === targetDomain).length;
}

function adapterFailure(reason: AgentAppGamePolicyPreviewFailureReason): AgentAppGamePolicyPreviewResult {
  return {
    ok: false,
    reason,
  };
}
