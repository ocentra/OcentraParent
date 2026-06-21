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
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffIdSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffRowIdSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiProofRefSchema,
  type AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffRow,
} from './app-game-source-gated-policy-preview-timer-service-readiness-service-handler-handoff';
import {
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffNoClaimFlags,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffState,
  RequiredAppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffNonClaims,
  appGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffCountsMatch,
  appGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffHasNoRuntimeClaims,
  appGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffMatchesServiceHandlerHandoff,
  type AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffStateValue,
} from './app-game-source-gated-policy-preview-timer-service-readiness-read-api-handoff-rules';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from './family-reference-primitives';

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffIdSchema =
  brandedNonEmptyStringSchema('AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffId');
export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffRowIdSchema =
  brandedNonEmptyStringSchema('AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffRowId');
export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffContractRefSchema =
  brandedNonEmptyStringSchema('AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffContractRef');
export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiSummaryRefSchema =
  brandedNonEmptyStringSchema('AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiSummaryRef');

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffStateSchema = withParser(
  Schema.Literal(...Object.values(AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffState))
);
export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffNonClaimSchema = withParser(
  Schema.Literal(...RequiredAppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffNonClaims)
);

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffOptionsSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    serviceReadApiHandoffId: AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffIdSchema,
    generatedAt: ParentTimestampSchema,
    sourceContractRefs: Schema.Array(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffContractRefSchema
    ),
    serviceReadApiProofRefs: Schema.Array(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiProofRefSchema
    ),
    serviceReadApiSummaryRef: AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiSummaryRefSchema,
  }).pipe(
    Schema.filter(
      (options) =>
        (options.sourceContractRefs.length > 0 && options.serviceReadApiProofRefs.length > 0) ||
        'Expected source-gated policy preview timer service-readiness service read API handoff options to cite source contracts and future read API proof refs'
    )
  )
);

const AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffRowBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  rowId: AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffRowIdSchema,
  sourceServiceHandlerHandoffRowId:
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffRowIdSchema,
  targetDomain: AppGamePolicyPreviewTargetDomainSchema,
  serviceReadApiHandoffState: AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffStateSchema,
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
  requiredServiceReadApiProofRefs: Schema.Array(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiProofRefSchema
  ),
  inheritedServiceReadinessProofRefs: Schema.Array(AppGameSourceGatedPolicyPreviewTimerProofRefSchema),
  sourceEvidenceRefs: Schema.Array(AppGameSourceFreshnessEvidenceRefSchema),
  serviceReadApiRef: AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffReadApiRefSchema,
  serviceReadApiSummaryRef: AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiSummaryRefSchema,
  serviceCommandRegistered: Schema.Literal(false),
  serviceHandlerImplemented: Schema.Literal(false),
  serviceReadApiImplemented: Schema.Literal(false),
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

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffRowSchema = withParser(
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        row.serviceReadApiHandoffState !==
          AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffState.ServiceReadApiProofRequired ||
        (row.inheritedServiceHandlerRefs.length > 0 && row.requiredServiceReadApiProofRefs.length > 0) ||
        'Expected service read API proof-required rows to preserve future service handler refs and name read API proof refs'
    )
  )
);

const AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  serviceReadApiHandoffId: AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffIdSchema,
  sourceServiceHandlerHandoffId: AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffIdSchema,
  generatedAt: ParentTimestampSchema,
  sourceContractRefs: Schema.Array(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffContractRefSchema
  ),
  serviceReadApiSummaryRef: AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiSummaryRefSchema,
  rows: Schema.Array(AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffRowSchema),
  nativeAppRowCount: Schema.Number,
  nativeGameRowCount: Schema.Number,
  serviceReadApiProofRequiredCount: Schema.Number,
  blockedBySourceFreshnessCount: Schema.Number,
  blockedByCompilerDecisionCount: Schema.Number,
  serviceReadApiHandoffNonClaims: Schema.Array(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffNonClaimSchema
  ),
  serviceCommandRegistered: Schema.Literal(false),
  serviceHandlerImplemented: Schema.Literal(false),
  serviceReadApiImplemented: Schema.Literal(false),
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

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffSchema = withParser(
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffBaseSchema.pipe(
    Schema.filter(
      (handoff) =>
        appGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffCountsMatch(handoff) ||
        'Expected source-gated policy preview timer service-readiness service read API handoff counts to match row states'
    )
  ).pipe(
    Schema.filter(
      (handoff) =>
        appGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffHasNoRuntimeClaims(handoff) ||
        'Expected source-gated policy preview timer service-readiness service read API handoff to avoid service, protocol, UI, timer, audit, rollback, adapter, and raw-source claims'
    )
  )
);

export type AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffOptions = Infer<
  typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffOptionsSchema
>;
export type AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffRow = Infer<
  typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffRowSchema
>;
export type AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoff = Infer<
  typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffSchema
>;

export function buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoff(
  optionsInput: unknown,
  serviceHandlerHandoffInput: unknown
): AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoff {
  const options =
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffOptionsSchema.parse(optionsInput);
  const serviceHandlerHandoff =
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffSchema.parse(serviceHandlerHandoffInput);
  const rows = serviceHandlerHandoff.rows.map((row) => buildServiceReadApiHandoffRow(options, row));

  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffSchema.parse({
    schemaVersion: options.schemaVersion,
    serviceReadApiHandoffId: options.serviceReadApiHandoffId,
    sourceServiceHandlerHandoffId: serviceHandlerHandoff.serviceHandlerHandoffId,
    generatedAt: options.generatedAt,
    sourceContractRefs: options.sourceContractRefs,
    serviceReadApiSummaryRef: options.serviceReadApiSummaryRef,
    rows,
    nativeAppRowCount: serviceHandlerHandoff.nativeAppRowCount,
    nativeGameRowCount: serviceHandlerHandoff.nativeGameRowCount,
    serviceReadApiProofRequiredCount: rows.filter(
      (row) =>
        row.serviceReadApiHandoffState ===
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffState.ServiceReadApiProofRequired
    ).length,
    blockedBySourceFreshnessCount: rows.filter(
      (row) =>
        row.serviceReadApiHandoffState ===
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffState.BlockedBySourceFreshness
    ).length,
    blockedByCompilerDecisionCount: rows.filter(
      (row) =>
        row.serviceReadApiHandoffState ===
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffState.BlockedByCompilerDecision
    ).length,
    serviceReadApiHandoffNonClaims:
      RequiredAppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffNonClaims,
    ...AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffNoClaimFlags,
  });
}

function buildServiceReadApiHandoffRow(
  options: AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffOptions,
  serviceHandlerRow: AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffRow
): AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffRow {
  const serviceReadApiHandoffState = serviceReadApiStateForServiceHandlerHandoff(serviceHandlerRow);
  const readApiRequired =
    serviceReadApiHandoffState ===
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffState.ServiceReadApiProofRequired;

  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffRowSchema.parse({
    schemaVersion: options.schemaVersion,
    rowId: `${serviceHandlerRow.rowId}:read-api-handoff`,
    sourceServiceHandlerHandoffRowId: serviceHandlerRow.rowId,
    targetDomain: serviceHandlerRow.targetDomain,
    serviceReadApiHandoffState,
    inheritedProtocolProofRefs: serviceHandlerRow.inheritedProtocolProofRefs,
    inheritedAgentProtocolCommandRefs: serviceHandlerRow.inheritedAgentProtocolCommandRefs,
    inheritedAgentProtocolEventRefs: serviceHandlerRow.inheritedAgentProtocolEventRefs,
    inheritedServiceHandlerRefs: serviceHandlerRow.requiredServiceHandlerRefs,
    requiredServiceReadApiProofRefs: readApiRequired ? serviceHandlerRow.requiredServiceReadApiProofRefs : [],
    inheritedServiceReadinessProofRefs: serviceHandlerRow.inheritedServiceReadinessProofRefs,
    sourceEvidenceRefs: serviceHandlerRow.sourceEvidenceRefs,
    serviceReadApiRef: serviceHandlerRow.serviceReadApiRef,
    serviceReadApiSummaryRef: options.serviceReadApiSummaryRef,
    ...AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffNoClaimFlags,
    generatedAt: options.generatedAt,
  });
}

function serviceReadApiStateForServiceHandlerHandoff(
  serviceHandlerRow: AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffRow
): AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffStateValue {
  for (const state of Object.values(AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffState)) {
    if (
      appGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffMatchesServiceHandlerHandoff(
        serviceHandlerRow.serviceHandlerHandoffState,
        state
      )
    ) {
      return state;
    }
  }
  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffState.BlockedByCompilerDecision;
}

export const decodeAppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoff = Schema.decodeUnknownSync(
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffSchema
);

export { AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffState };

