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
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffIdSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffRowIdSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandRefSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolEventRefSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolServiceHandlerRefSchema,
  type AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffRow,
} from './app-game-source-gated-policy-preview-timer-service-readiness-protocol-command-handoff';
import {
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffNoClaimFlags,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffState,
  RequiredAppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffNonClaims,
  appGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffCountsMatch,
  appGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffHasNoRuntimeClaims,
  appGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffMatchesCommandHandoff,
  type AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffStateValue,
} from './app-game-source-gated-policy-preview-timer-service-readiness-service-handler-handoff-rules';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from './family-reference-primitives';

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffIdSchema =
  brandedNonEmptyStringSchema('AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffId');
export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffRowIdSchema =
  brandedNonEmptyStringSchema('AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffRowId');
export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffContractRefSchema =
  brandedNonEmptyStringSchema('AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffContractRef');
export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiProofRefSchema =
  brandedNonEmptyStringSchema('AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiProofRef');
export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerSummaryRefSchema =
  brandedNonEmptyStringSchema('AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerSummaryRef');

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffStateSchema = withParser(
  Schema.Literal(...Object.values(AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffState))
);
export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffNonClaimSchema = withParser(
  Schema.Literal(...RequiredAppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffNonClaims)
);

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffOptionsSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    serviceHandlerHandoffId: AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffIdSchema,
    generatedAt: ParentTimestampSchema,
    sourceContractRefs: Schema.Array(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffContractRefSchema
    ),
    serviceReadApiProofRefs: Schema.Array(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiProofRefSchema
    ),
    serviceHandlerSummaryRef: AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerSummaryRefSchema,
  }).pipe(
    Schema.filter(
      (options) =>
        (options.sourceContractRefs.length > 0 && options.serviceReadApiProofRefs.length > 0) ||
        'Expected source-gated policy preview timer service-readiness service handler handoff options to cite source contracts and future read API proof refs'
    )
  )
);

const AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffRowBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  rowId: AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffRowIdSchema,
  sourceProtocolCommandHandoffRowId:
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffRowIdSchema,
  targetDomain: AppGamePolicyPreviewTargetDomainSchema,
  serviceHandlerHandoffState: AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffStateSchema,
  inheritedProtocolProofRefs: Schema.Array(AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolProofRefSchema),
  inheritedAgentProtocolCommandRefs: Schema.Array(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandRefSchema
  ),
  inheritedAgentProtocolEventRefs: Schema.Array(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolEventRefSchema
  ),
  requiredServiceHandlerRefs: Schema.Array(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolServiceHandlerRefSchema
  ),
  requiredServiceReadApiProofRefs: Schema.Array(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiProofRefSchema
  ),
  inheritedServiceReadinessProofRefs: Schema.Array(AppGameSourceGatedPolicyPreviewTimerProofRefSchema),
  sourceEvidenceRefs: Schema.Array(AppGameSourceFreshnessEvidenceRefSchema),
  serviceReadApiRef: AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffReadApiRefSchema,
  serviceHandlerSummaryRef: AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerSummaryRefSchema,
  serviceCommandRegistered: Schema.Literal(false),
  serviceHandlerImplemented: Schema.Literal(false),
  serviceEventEmitted: Schema.Literal(false),
  serviceReadApiImplemented: Schema.Literal(false),
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

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffRowSchema = withParser(
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        row.serviceHandlerHandoffState !==
          AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffState.ServiceHandlerProofRequired ||
        (row.requiredServiceHandlerRefs.length > 0 && row.requiredServiceReadApiProofRefs.length > 0) ||
        'Expected service handler proof-required rows to name future service handler and read API proof refs'
    )
  )
);

const AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  serviceHandlerHandoffId: AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffIdSchema,
  sourceProtocolCommandHandoffId: AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffIdSchema,
  generatedAt: ParentTimestampSchema,
  sourceContractRefs: Schema.Array(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffContractRefSchema
  ),
  serviceHandlerSummaryRef: AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerSummaryRefSchema,
  rows: Schema.Array(AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffRowSchema),
  nativeAppRowCount: Schema.Number,
  nativeGameRowCount: Schema.Number,
  serviceHandlerProofRequiredCount: Schema.Number,
  blockedBySourceFreshnessCount: Schema.Number,
  blockedByCompilerDecisionCount: Schema.Number,
  serviceHandlerHandoffNonClaims: Schema.Array(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffNonClaimSchema
  ),
  serviceCommandRegistered: Schema.Literal(false),
  serviceHandlerImplemented: Schema.Literal(false),
  serviceEventEmitted: Schema.Literal(false),
  serviceReadApiImplemented: Schema.Literal(false),
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

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffSchema = withParser(
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffBaseSchema.pipe(
    Schema.filter(
      (handoff) =>
        appGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffCountsMatch(handoff) ||
        'Expected source-gated policy preview timer service-readiness service handler handoff counts to match row states'
    )
  ).pipe(
    Schema.filter(
      (handoff) =>
        appGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffHasNoRuntimeClaims(handoff) ||
        'Expected source-gated policy preview timer service-readiness service handler handoff to avoid service, protocol, UI, timer, audit, rollback, adapter, and raw-source claims'
    )
  )
);

export type AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffOptions = Infer<
  typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffOptionsSchema
>;
export type AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffRow = Infer<
  typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffRowSchema
>;
export type AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoff = Infer<
  typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffSchema
>;

export function buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoff(
  optionsInput: unknown,
  commandHandoffInput: unknown
): AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoff {
  const options =
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffOptionsSchema.parse(optionsInput);
  const commandHandoff =
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffSchema.parse(commandHandoffInput);
  const rows = commandHandoff.rows.map((row) => buildServiceHandlerHandoffRow(options, row));

  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffSchema.parse({
    schemaVersion: options.schemaVersion,
    serviceHandlerHandoffId: options.serviceHandlerHandoffId,
    sourceProtocolCommandHandoffId: commandHandoff.commandHandoffId,
    generatedAt: options.generatedAt,
    sourceContractRefs: options.sourceContractRefs,
    serviceHandlerSummaryRef: options.serviceHandlerSummaryRef,
    rows,
    nativeAppRowCount: commandHandoff.nativeAppRowCount,
    nativeGameRowCount: commandHandoff.nativeGameRowCount,
    serviceHandlerProofRequiredCount: rows.filter(
      (row) =>
        row.serviceHandlerHandoffState ===
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffState.ServiceHandlerProofRequired
    ).length,
    blockedBySourceFreshnessCount: rows.filter(
      (row) =>
        row.serviceHandlerHandoffState ===
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffState.BlockedBySourceFreshness
    ).length,
    blockedByCompilerDecisionCount: rows.filter(
      (row) =>
        row.serviceHandlerHandoffState ===
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffState.BlockedByCompilerDecision
    ).length,
    serviceHandlerHandoffNonClaims:
      RequiredAppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffNonClaims,
    ...AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffNoClaimFlags,
  });
}

function buildServiceHandlerHandoffRow(
  options: AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffOptions,
  commandRow: AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffRow
): AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffRow {
  const serviceHandlerHandoffState = serviceHandlerStateForCommandHandoff(commandRow);
  const handlerRequired =
    serviceHandlerHandoffState ===
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffState.ServiceHandlerProofRequired;

  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffRowSchema.parse({
    schemaVersion: options.schemaVersion,
    rowId: `${commandRow.rowId}:service-handler-handoff`,
    sourceProtocolCommandHandoffRowId: commandRow.rowId,
    targetDomain: commandRow.targetDomain,
    serviceHandlerHandoffState,
    inheritedProtocolProofRefs: commandRow.requiredProtocolProofRefs,
    inheritedAgentProtocolCommandRefs: commandRow.requiredAgentProtocolCommandRefs,
    inheritedAgentProtocolEventRefs: commandRow.requiredAgentProtocolEventRefs,
    requiredServiceHandlerRefs: handlerRequired ? commandRow.requiredServiceHandlerRefs : [],
    requiredServiceReadApiProofRefs: handlerRequired ? options.serviceReadApiProofRefs : [],
    inheritedServiceReadinessProofRefs: commandRow.inheritedServiceReadinessProofRefs,
    sourceEvidenceRefs: commandRow.sourceEvidenceRefs,
    serviceReadApiRef: commandRow.serviceReadApiRef,
    serviceHandlerSummaryRef: options.serviceHandlerSummaryRef,
    ...AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffNoClaimFlags,
    generatedAt: options.generatedAt,
  });
}

function serviceHandlerStateForCommandHandoff(
  commandRow: AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffRow
): AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffStateValue {
  for (const state of Object.values(AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffState)) {
    if (
      appGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffMatchesCommandHandoff(
        commandRow.protocolCommandHandoffState,
        state
      )
    ) {
      return state;
    }
  }
  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffState.BlockedByCompilerDecision;
}

export const decodeAppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoff = Schema.decodeUnknownSync(
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffSchema
);

export { AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffState };

