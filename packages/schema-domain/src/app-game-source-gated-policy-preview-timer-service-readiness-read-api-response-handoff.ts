import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from './effect';
import { AppGamePolicyPreviewTargetDomainSchema } from './app-game-policy-preview-handoff';
import { AppGameSourceFreshnessEvidenceRefSchema } from './app-game-source-freshness-policy-consumption';
import { AppGameSourceGatedPolicyPreviewTimerProofRefSchema } from './app-game-source-gated-policy-preview-timer-status';
import { AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffReadApiRefSchema } from './app-game-source-gated-policy-preview-timer-service-readiness-handoff';
import { AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolProofRefSchema } from './app-game-source-gated-policy-preview-timer-service-readiness-protocol-handoff';
import {
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandRefSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolEventRefSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolServiceHandlerRefSchema,
} from './app-game-source-gated-policy-preview-timer-service-readiness-protocol-command-handoff';
import {
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffIdSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffRowIdSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffSchema,
  type AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffRow,
} from './app-game-source-gated-policy-preview-timer-service-readiness-read-api-handoff';
import { AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiProofRefSchema } from './app-game-source-gated-policy-preview-timer-service-readiness-service-handler-handoff';
import {
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffNoClaimFlags,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffState,
  RequiredAppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffNonClaims,
  appGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffCountsMatch,
  appGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffHasNoRuntimeClaims,
  appGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffMatchesReadApiHandoff,
  type AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffStateValue,
} from './app-game-source-gated-policy-preview-timer-service-readiness-read-api-response-handoff-rules';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from './family-reference-primitives';

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffIdSchema =
  brandedNonEmptyStringSchema('AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffId');
export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffRowIdSchema =
  brandedNonEmptyStringSchema('AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffRowId');
export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffContractRefSchema =
  brandedNonEmptyStringSchema('AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffContractRef');
export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseProofRefSchema =
  brandedNonEmptyStringSchema('AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseProofRef');
export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseSummaryRefSchema =
  brandedNonEmptyStringSchema('AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseSummaryRef');

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffStateSchema = withParser(
  Schema.Literal(...Object.values(AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffState))
);
export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffNonClaimSchema = withParser(
  Schema.Literal(...RequiredAppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffNonClaims)
);

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffOptionsSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    readApiResponseHandoffId: AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffIdSchema,
    generatedAt: ParentTimestampSchema,
    sourceContractRefs: Schema.Array(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffContractRefSchema
    ),
    readApiResponseProofRefs: Schema.Array(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseProofRefSchema
    ),
    readApiResponseSummaryRef: AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseSummaryRefSchema,
  }).pipe(
    Schema.filter(
      (options) =>
        (options.sourceContractRefs.length > 0 && options.readApiResponseProofRefs.length > 0) ||
        'Expected read API response handoff options to cite source contracts and future response proof refs'
    )
  )
);

const AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffRowBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  rowId: AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffRowIdSchema,
  sourceReadApiHandoffRowId: AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffRowIdSchema,
  targetDomain: AppGamePolicyPreviewTargetDomainSchema,
  readApiResponseHandoffState: AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffStateSchema,
  inheritedProtocolProofRefs: Schema.Array(AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolProofRefSchema),
  inheritedAgentProtocolCommandRefs: Schema.Array(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandRefSchema
  ),
  inheritedAgentProtocolEventRefs: Schema.Array(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolEventRefSchema
  ),
  inheritedServiceHandlerRefs: Schema.Array(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolServiceHandlerRefSchema
  ),
  inheritedServiceReadApiProofRefs: Schema.Array(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiProofRefSchema
  ),
  requiredReadApiResponseProofRefs: Schema.Array(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseProofRefSchema
  ),
  inheritedServiceReadinessProofRefs: Schema.Array(AppGameSourceGatedPolicyPreviewTimerProofRefSchema),
  sourceEvidenceRefs: Schema.Array(AppGameSourceFreshnessEvidenceRefSchema),
  serviceReadApiRef: AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffReadApiRefSchema,
  readApiResponseSummaryRef: AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseSummaryRefSchema,
  serviceCommandRegistered: Schema.Literal(false),
  serviceHandlerImplemented: Schema.Literal(false),
  serviceReadApiImplemented: Schema.Literal(false),
  serviceReadApiResponseImplemented: Schema.Literal(false),
  serviceEventEmitted: Schema.Literal(false),
  agentProtocolImplemented: Schema.Literal(false),
  rustProtocolMirrored: Schema.Literal(false),
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

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffRowSchema = withParser(
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        row.readApiResponseHandoffState !==
          AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffState.ReadApiResponseProofRequired ||
        (row.inheritedServiceReadApiProofRefs.length > 0 && row.requiredReadApiResponseProofRefs.length > 0) ||
        'Expected read API response proof-required rows to preserve read API proof refs and name response proof refs'
    )
  )
);

const AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  readApiResponseHandoffId: AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffIdSchema,
  sourceReadApiHandoffId: AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffIdSchema,
  generatedAt: ParentTimestampSchema,
  sourceContractRefs: Schema.Array(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffContractRefSchema
  ),
  readApiResponseSummaryRef: AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseSummaryRefSchema,
  rows: Schema.Array(AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffRowSchema),
  nativeAppRowCount: Schema.Number,
  nativeGameRowCount: Schema.Number,
  readApiResponseProofRequiredCount: Schema.Number,
  blockedBySourceFreshnessCount: Schema.Number,
  blockedByCompilerDecisionCount: Schema.Number,
  readApiResponseHandoffNonClaims: Schema.Array(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffNonClaimSchema
  ),
  serviceCommandRegistered: Schema.Literal(false),
  serviceHandlerImplemented: Schema.Literal(false),
  serviceReadApiImplemented: Schema.Literal(false),
  serviceReadApiResponseImplemented: Schema.Literal(false),
  serviceEventEmitted: Schema.Literal(false),
  agentProtocolImplemented: Schema.Literal(false),
  rustProtocolMirrored: Schema.Literal(false),
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

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffSchema = withParser(
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffBaseSchema.pipe(
    Schema.filter(
      (handoff) =>
        appGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffCountsMatch(handoff) ||
        'Expected read API response handoff counts to match row states'
    )
  ).pipe(
    Schema.filter(
      (handoff) =>
        appGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffHasNoRuntimeClaims(handoff) ||
        'Expected read API response handoff to avoid service, protocol, UI, timer, audit, rollback, adapter, and raw-source claims'
    )
  )
);

export type AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffOptions = Infer<
  typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffOptionsSchema
>;
export type AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffRow = Infer<
  typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffRowSchema
>;
export type AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoff = Infer<
  typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffSchema
>;

export function buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoff(
  optionsInput: unknown,
  readApiHandoffInput: unknown
): AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoff {
  const options =
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffOptionsSchema.parse(optionsInput);
  const readApiHandoff =
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffSchema.parse(readApiHandoffInput);
  const rows = readApiHandoff.rows.map((row) => buildReadApiResponseHandoffRow(options, row));

  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffSchema.parse({
    schemaVersion: options.schemaVersion,
    readApiResponseHandoffId: options.readApiResponseHandoffId,
    sourceReadApiHandoffId: readApiHandoff.serviceReadApiHandoffId,
    generatedAt: options.generatedAt,
    sourceContractRefs: options.sourceContractRefs,
    readApiResponseSummaryRef: options.readApiResponseSummaryRef,
    rows,
    nativeAppRowCount: readApiHandoff.nativeAppRowCount,
    nativeGameRowCount: readApiHandoff.nativeGameRowCount,
    readApiResponseProofRequiredCount: rows.filter(
      (row) =>
        row.readApiResponseHandoffState ===
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffState.ReadApiResponseProofRequired
    ).length,
    blockedBySourceFreshnessCount: rows.filter(
      (row) =>
        row.readApiResponseHandoffState ===
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffState.BlockedBySourceFreshness
    ).length,
    blockedByCompilerDecisionCount: rows.filter(
      (row) =>
        row.readApiResponseHandoffState ===
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffState.BlockedByCompilerDecision
    ).length,
    readApiResponseHandoffNonClaims:
      RequiredAppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffNonClaims,
    ...AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffNoClaimFlags,
  });
}

function buildReadApiResponseHandoffRow(
  options: AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffOptions,
  readApiRow: AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffRow
): AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffRow {
  const readApiResponseHandoffState = readApiResponseStateForReadApiHandoff(readApiRow);
  const responseRequired =
    readApiResponseHandoffState ===
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffState.ReadApiResponseProofRequired;

  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffRowSchema.parse({
    schemaVersion: options.schemaVersion,
    rowId: `${readApiRow.rowId}:read-api-response-handoff`,
    sourceReadApiHandoffRowId: readApiRow.rowId,
    targetDomain: readApiRow.targetDomain,
    readApiResponseHandoffState,
    inheritedProtocolProofRefs: readApiRow.inheritedProtocolProofRefs,
    inheritedAgentProtocolCommandRefs: readApiRow.inheritedAgentProtocolCommandRefs,
    inheritedAgentProtocolEventRefs: readApiRow.inheritedAgentProtocolEventRefs,
    inheritedServiceHandlerRefs: readApiRow.inheritedServiceHandlerRefs,
    inheritedServiceReadApiProofRefs: readApiRow.requiredServiceReadApiProofRefs,
    requiredReadApiResponseProofRefs: responseRequired ? options.readApiResponseProofRefs : [],
    inheritedServiceReadinessProofRefs: readApiRow.inheritedServiceReadinessProofRefs,
    sourceEvidenceRefs: readApiRow.sourceEvidenceRefs,
    serviceReadApiRef: readApiRow.serviceReadApiRef,
    readApiResponseSummaryRef: options.readApiResponseSummaryRef,
    ...AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffNoClaimFlags,
    generatedAt: options.generatedAt,
  });
}

function readApiResponseStateForReadApiHandoff(
  readApiRow: AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffRow
): AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffStateValue {
  for (const state of Object.values(AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffState)) {
    if (
      appGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffMatchesReadApiHandoff(
        readApiRow.serviceReadApiHandoffState,
        state
      )
    ) {
      return state;
    }
  }
  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffState.BlockedByCompilerDecision;
}

export const decodeAppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoff =
  Schema.decodeUnknownSync(AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffSchema);

export { AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffState };

