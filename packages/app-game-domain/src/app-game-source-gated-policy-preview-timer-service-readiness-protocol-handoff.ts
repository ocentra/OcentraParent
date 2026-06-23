import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import { AppGamePolicyPreviewTargetDomainSchema } from './app-game-policy-preview-handoff';
import { AppGameSourceFreshnessEvidenceRefSchema } from '@ocentra-parent/schema-domain/app-game-source-freshness-policy-consumption';
import { AppGameSourceGatedPolicyPreviewTimerProofRefSchema } from './app-game-source-gated-policy-preview-timer-status';
import {
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffIdSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffReadApiRefSchema,
} from './app-game-source-gated-policy-preview-timer-service-readiness-handoff';
import {
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelIdSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelRowIdSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelSchema,
  type AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelRow,
} from './app-game-source-gated-policy-preview-timer-service-readiness-read-model';
import {
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffNoClaimFlags,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffState,
  RequiredAppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffNonClaims,
  appGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffCountsMatch,
  appGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffHasNoRuntimeClaims,
  appGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffMatchesReadModel,
  type AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffStateValue,
} from '@ocentra-parent/schema-domain/app-game-source-gated-policy-preview-timer-service-readiness-protocol-handoff-rules';
import {
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
} from '@ocentra-parent/schema-domain/family-reference-primitives';

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffIdSchema = brandedNonEmptyStringSchema(
  'AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffId'
);
export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffRowIdSchema =
  brandedNonEmptyStringSchema('AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffRowId');
export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffContractRefSchema =
  brandedNonEmptyStringSchema('AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffContractRef');
export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolProofRefSchema = brandedNonEmptyStringSchema(
  'AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolProofRef'
);

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffStateSchema = withParser(
  Schema.Literal(...Object.values(AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffState))
);
export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffNonClaimSchema = withParser(
  Schema.Literal(...RequiredAppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffNonClaims)
);

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffOptionsSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    handoffId: AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffIdSchema,
    generatedAt: ParentTimestampSchema,
    sourceContractRefs: Schema.Array(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffContractRefSchema
    ),
    protocolCommandContractProofRef: AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolProofRefSchema,
    protocolEventContractProofRef: AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolProofRefSchema,
    rustProtocolMirrorProofRef: AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolProofRefSchema,
    serviceHandlerProofRef: AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolProofRefSchema,
  }).pipe(
    Schema.filter(
      (options) =>
        options.sourceContractRefs.length > 0 ||
        'Expected source-gated policy preview timer service-readiness protocol handoff options to cite source contracts'
    )
  )
);

const AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffRowBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  rowId: AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffRowIdSchema,
  sourceServiceReadinessReadModelRowId: AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelRowIdSchema,
  targetDomain: AppGamePolicyPreviewTargetDomainSchema,
  protocolHandoffState: AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffStateSchema,
  requiredProtocolProofRefs: Schema.Array(AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolProofRefSchema),
  inheritedServiceReadinessProofRefs: Schema.Array(AppGameSourceGatedPolicyPreviewTimerProofRefSchema),
  sourceEvidenceRefs: Schema.Array(AppGameSourceFreshnessEvidenceRefSchema),
  serviceReadApiRef: AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffReadApiRefSchema,
  agentProtocolContractImplemented: Schema.Literal(false),
  rustProtocolMirrored: Schema.Literal(false),
  serviceCommandRegistered: Schema.Literal(false),
  serviceEventEmitted: Schema.Literal(false),
  serviceReadApiImplemented: Schema.Literal(false),
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

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffRowSchema = withParser(
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        row.protocolHandoffState !==
          AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffState.ProtocolProofRequired ||
        row.requiredProtocolProofRefs.length > 0 ||
        'Expected protocol-ready rows to name protocol command, event, Rust mirror, and service handler proof refs'
    )
  )
);

const AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  handoffId: AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffIdSchema,
  sourceServiceReadinessReadModelId: AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelIdSchema,
  sourceServiceReadinessHandoffId: AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffIdSchema,
  generatedAt: ParentTimestampSchema,
  sourceContractRefs: Schema.Array(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffContractRefSchema
  ),
  rows: Schema.Array(AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffRowSchema),
  nativeAppRowCount: Schema.Number,
  nativeGameRowCount: Schema.Number,
  protocolProofRequiredCount: Schema.Number,
  blockedBySourceFreshnessCount: Schema.Number,
  blockedByCompilerDecisionCount: Schema.Number,
  protocolHandoffNonClaims: Schema.Array(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffNonClaimSchema
  ),
  agentProtocolContractImplemented: Schema.Literal(false),
  rustProtocolMirrored: Schema.Literal(false),
  serviceCommandRegistered: Schema.Literal(false),
  serviceEventEmitted: Schema.Literal(false),
  serviceReadApiImplemented: Schema.Literal(false),
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

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffSchema = withParser(
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffBaseSchema.pipe(
    Schema.filter(
      (handoff) =>
        appGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffCountsMatch(handoff) ||
        'Expected source-gated policy preview timer service-readiness protocol handoff counts to match row states'
    )
  ).pipe(
    Schema.filter(
      (handoff) =>
        appGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffHasNoRuntimeClaims(handoff) ||
        'Expected source-gated policy preview timer service-readiness protocol handoff to avoid protocol, service, UI, timer, audit, rollback, adapter, and raw-source claims'
    )
  )
);

export type AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffOptions = Infer<
  typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffOptionsSchema
>;
export type AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffRow = Infer<
  typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffRowSchema
>;
export type AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoff = Infer<
  typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffSchema
>;

export function buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoff(
  optionsInput: unknown,
  readModelInput: unknown
): AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoff {
  const options = AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffOptionsSchema.parse(optionsInput);
  const readModel = AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelSchema.parse(readModelInput);
  const rows = readModel.rows.map((row) => buildProtocolHandoffRow(options, row));

  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffSchema.parse({
    schemaVersion: options.schemaVersion,
    handoffId: options.handoffId,
    sourceServiceReadinessReadModelId: readModel.readModelId,
    sourceServiceReadinessHandoffId: readModel.sourceServiceReadinessHandoffId,
    generatedAt: options.generatedAt,
    sourceContractRefs: options.sourceContractRefs,
    rows,
    nativeAppRowCount: readModel.nativeAppRowCount,
    nativeGameRowCount: readModel.nativeGameRowCount,
    protocolProofRequiredCount: rows.filter(
      (row) =>
        row.protocolHandoffState ===
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffState.ProtocolProofRequired
    ).length,
    blockedBySourceFreshnessCount: rows.filter(
      (row) =>
        row.protocolHandoffState ===
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffState.BlockedBySourceFreshness
    ).length,
    blockedByCompilerDecisionCount: rows.filter(
      (row) =>
        row.protocolHandoffState ===
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffState.BlockedByCompilerDecision
    ).length,
    protocolHandoffNonClaims: RequiredAppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffNonClaims,
    ...AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffNoClaimFlags,
  });
}

function buildProtocolHandoffRow(
  options: AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffOptions,
  readModelRow: AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelRow
): AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffRow {
  const protocolHandoffState = protocolStateForReadModel(readModelRow);

  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffRowSchema.parse({
    schemaVersion: options.schemaVersion,
    rowId: `${readModelRow.rowId}:protocol-handoff`,
    sourceServiceReadinessReadModelRowId: readModelRow.rowId,
    targetDomain: readModelRow.targetDomain,
    protocolHandoffState,
    requiredProtocolProofRefs:
      protocolHandoffState ===
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffState.ProtocolProofRequired
        ? [
            options.protocolCommandContractProofRef,
            options.protocolEventContractProofRef,
            options.rustProtocolMirrorProofRef,
            options.serviceHandlerProofRef,
          ]
        : [],
    inheritedServiceReadinessProofRefs: readModelRow.requiredProofRefs,
    sourceEvidenceRefs: readModelRow.sourceEvidenceRefs,
    serviceReadApiRef: readModelRow.serviceReadApiRef,
    ...AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffNoClaimFlags,
    generatedAt: options.generatedAt,
  });
}

function protocolStateForReadModel(
  readModelRow: AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelRow
): AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffStateValue {
  for (const state of Object.values(AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffState)) {
    if (
      appGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffMatchesReadModel(
        readModelRow.serviceReadinessReadModelState,
        state
      )
    ) {
      return state;
    }
  }
  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffState.BlockedByCompilerDecision;
}

export const decodeAppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoff = Schema.decodeUnknownSync(
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffSchema
);

export { AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffState };
