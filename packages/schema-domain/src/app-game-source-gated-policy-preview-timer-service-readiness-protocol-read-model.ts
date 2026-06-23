import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from './effect';
import { AppGamePolicyPreviewTargetDomainSchema } from './app-game-policy-preview-handoff';
import { AppGameSourceFreshnessEvidenceRefSchema } from './app-game-source-freshness-policy-consumption';
import { AppGameSourceGatedPolicyPreviewTimerProofRefSchema } from './app-game-source-gated-policy-preview-timer-status';
import { AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffReadApiRefSchema } from './app-game-source-gated-policy-preview-timer-service-readiness-handoff';
import {
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffIdSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffRowIdSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolProofRefSchema,
  type AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffRow,
} from './app-game-source-gated-policy-preview-timer-service-readiness-protocol-handoff';
import {
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelNoClaimFlags,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelState,
  RequiredAppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelNonClaims,
  appGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelCountsMatch,
  appGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelHasNoRuntimeClaims,
  appGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelMatchesHandoff,
  type AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelStateValue,
} from './app-game-source-gated-policy-preview-timer-service-readiness-protocol-read-model-rules';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from './family-reference-primitives';

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelIdSchema =
  brandedNonEmptyStringSchema('AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelId');
export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelRowIdSchema =
  brandedNonEmptyStringSchema('AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelRowId');
export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelContractRefSchema =
  brandedNonEmptyStringSchema('AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelContractRef');
export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolSummaryRefSchema = brandedNonEmptyStringSchema(
  'AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolSummaryRef'
);

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelStateSchema = withParser(
  Schema.Literal(...Object.values(AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelState))
);
export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelNonClaimSchema = withParser(
  Schema.Literal(...RequiredAppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelNonClaims)
);

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelOptionsSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    readModelId: AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelIdSchema,
    generatedAt: ParentTimestampSchema,
    sourceContractRefs: Schema.Array(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelContractRefSchema
    ),
    protocolSummaryRef: AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolSummaryRefSchema,
  }).pipe(
    Schema.filter(
      (options) =>
        options.sourceContractRefs.length > 0 ||
        'Expected source-gated policy preview timer service-readiness protocol read model options to cite source contracts'
    )
  )
);

const AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelRowBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  rowId: AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelRowIdSchema,
  sourceProtocolHandoffRowId: AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffRowIdSchema,
  targetDomain: AppGamePolicyPreviewTargetDomainSchema,
  protocolReadModelState: AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelStateSchema,
  requiredProtocolProofRefs: Schema.Array(AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolProofRefSchema),
  inheritedServiceReadinessProofRefs: Schema.Array(AppGameSourceGatedPolicyPreviewTimerProofRefSchema),
  sourceEvidenceRefs: Schema.Array(AppGameSourceFreshnessEvidenceRefSchema),
  serviceReadApiRef: AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffReadApiRefSchema,
  protocolSummaryRef: AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolSummaryRefSchema,
  agentProtocolContractImplemented: Schema.Literal(false),
  rustProtocolMirrored: Schema.Literal(false),
  serviceCommandRegistered: Schema.Literal(false),
  serviceEventEmitted: Schema.Literal(false),
  serviceReadApiImplemented: Schema.Literal(false),
  serviceReadModelEventEmitted: Schema.Literal(false),
  portalUiRendered: Schema.Literal(false),
  policyEvaluatorRuntimeClaimed: Schema.Literal(false),
  timerRuntimeClaimed: Schema.Literal(false),
  timerScheduled: Schema.Literal(false),
  schedulerPersistenceRuntimeClaimed: Schema.Literal(false),
  durableSchedulerStorageClaimed: Schema.Literal(false),
  auditRuntimeClaimed: Schema.Literal(false),
  durableAuditLogClaimed: Schema.Literal(false),
  rollbackRuntimeClaimed: Schema.Literal(false),
  rollbackExecutionClaimed: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
  childDeliveryClaimed: Schema.Literal(false),
  platformEnforcementClaimed: Schema.Literal(false),
  rawPrivateSourceRowsIncluded: Schema.Literal(false),
  generatedAt: ParentTimestampSchema,
});

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelRowSchema = withParser(
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        row.protocolReadModelState !==
          AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelState.ProtocolReadModelProofRequired ||
        row.requiredProtocolProofRefs.length > 0 ||
        'Expected protocol read-model proof-required rows to name future protocol proof refs'
    )
  )
);

const AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  readModelId: AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelIdSchema,
  sourceProtocolHandoffId: AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffIdSchema,
  generatedAt: ParentTimestampSchema,
  sourceContractRefs: Schema.Array(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelContractRefSchema
  ),
  protocolSummaryRef: AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolSummaryRefSchema,
  rows: Schema.Array(AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelRowSchema),
  nativeAppRowCount: Schema.Number,
  nativeGameRowCount: Schema.Number,
  protocolReadModelProofRequiredCount: Schema.Number,
  blockedBySourceFreshnessCount: Schema.Number,
  blockedByCompilerDecisionCount: Schema.Number,
  protocolReadModelNonClaims: Schema.Array(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelNonClaimSchema
  ),
  agentProtocolContractImplemented: Schema.Literal(false),
  rustProtocolMirrored: Schema.Literal(false),
  serviceCommandRegistered: Schema.Literal(false),
  serviceEventEmitted: Schema.Literal(false),
  serviceReadApiImplemented: Schema.Literal(false),
  serviceReadModelEventEmitted: Schema.Literal(false),
  portalUiRendered: Schema.Literal(false),
  policyEvaluatorRuntimeClaimed: Schema.Literal(false),
  timerRuntimeClaimed: Schema.Literal(false),
  timerScheduled: Schema.Literal(false),
  schedulerPersistenceRuntimeClaimed: Schema.Literal(false),
  durableSchedulerStorageClaimed: Schema.Literal(false),
  auditRuntimeClaimed: Schema.Literal(false),
  durableAuditLogClaimed: Schema.Literal(false),
  rollbackRuntimeClaimed: Schema.Literal(false),
  rollbackExecutionClaimed: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
  childDeliveryClaimed: Schema.Literal(false),
  platformEnforcementClaimed: Schema.Literal(false),
  rawPrivateSourceRowsIncluded: Schema.Literal(false),
});

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelSchema = withParser(
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelBaseSchema.pipe(
    Schema.filter(
      (readModel) =>
        appGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelCountsMatch(readModel) ||
        'Expected source-gated policy preview timer service-readiness protocol read model counts to match row states'
    )
  ).pipe(
    Schema.filter(
      (readModel) =>
        appGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelHasNoRuntimeClaims(readModel) ||
        'Expected source-gated policy preview timer service-readiness protocol read model to avoid protocol, service, UI, timer, audit, rollback, adapter, and raw-source claims'
    )
  )
);

export type AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelOptions = Infer<
  typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelOptionsSchema
>;
export type AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelRow = Infer<
  typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelRowSchema
>;
export type AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModel = Infer<
  typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelSchema
>;

export function buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModel(
  optionsInput: unknown,
  handoffInput: unknown
): AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModel {
  const options =
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelOptionsSchema.parse(optionsInput);
  const handoff = AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffSchema.parse(handoffInput);
  const rows = handoff.rows.map((row) => buildProtocolReadModelRow(options, row));

  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelSchema.parse({
    schemaVersion: options.schemaVersion,
    readModelId: options.readModelId,
    sourceProtocolHandoffId: handoff.handoffId,
    generatedAt: options.generatedAt,
    sourceContractRefs: options.sourceContractRefs,
    protocolSummaryRef: options.protocolSummaryRef,
    rows,
    nativeAppRowCount: handoff.nativeAppRowCount,
    nativeGameRowCount: handoff.nativeGameRowCount,
    protocolReadModelProofRequiredCount: rows.filter(
      (row) =>
        row.protocolReadModelState ===
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelState.ProtocolReadModelProofRequired
    ).length,
    blockedBySourceFreshnessCount: rows.filter(
      (row) =>
        row.protocolReadModelState ===
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelState.BlockedBySourceFreshness
    ).length,
    blockedByCompilerDecisionCount: rows.filter(
      (row) =>
        row.protocolReadModelState ===
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelState.BlockedByCompilerDecision
    ).length,
    protocolReadModelNonClaims: RequiredAppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelNonClaims,
    ...AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelNoClaimFlags,
  });
}

function buildProtocolReadModelRow(
  options: AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelOptions,
  handoffRow: AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffRow
): AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelRow {
  const protocolReadModelState = readModelStateForHandoff(handoffRow);

  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelRowSchema.parse({
    schemaVersion: options.schemaVersion,
    rowId: `${handoffRow.rowId}:protocol-read-model`,
    sourceProtocolHandoffRowId: handoffRow.rowId,
    targetDomain: handoffRow.targetDomain,
    protocolReadModelState,
    requiredProtocolProofRefs: handoffRow.requiredProtocolProofRefs,
    inheritedServiceReadinessProofRefs: handoffRow.inheritedServiceReadinessProofRefs,
    sourceEvidenceRefs: handoffRow.sourceEvidenceRefs,
    serviceReadApiRef: handoffRow.serviceReadApiRef,
    protocolSummaryRef: options.protocolSummaryRef,
    ...AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelNoClaimFlags,
    generatedAt: options.generatedAt,
  });
}

function readModelStateForHandoff(
  handoffRow: AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffRow
): AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelStateValue {
  for (const state of Object.values(AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelState)) {
    if (
      appGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelMatchesHandoff(
        handoffRow.protocolHandoffState,
        state
      )
    ) {
      return state;
    }
  }
  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelState.BlockedByCompilerDecision;
}

export const decodeAppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModel = Schema.decodeUnknownSync(
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelSchema
);

export { AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelState };
