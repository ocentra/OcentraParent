import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { AppGamePolicyPreviewTargetDomainSchema } from './app-game-policy-preview-handoff';
import { AppGameSourceFreshnessEvidenceRefSchema } from '@ocentra-parent/schema-domain/app-game-source-freshness-policy-consumption';
import { AppGameSourceGatedPolicyPreviewTimerProofRefSchema } from './app-game-source-gated-policy-preview-timer-status';
import { AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffReadApiRefSchema } from './app-game-source-gated-policy-preview-timer-service-readiness-handoff';
import { AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolProofRefSchema } from './app-game-source-gated-policy-preview-timer-service-readiness-protocol-handoff';
import {
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelIdSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelRowIdSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelSchema,
  type AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelRow,
} from './app-game-source-gated-policy-preview-timer-service-readiness-protocol-read-model';
import {
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffNoClaimFlags,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffState,
  RequiredAppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffNonClaims,
  appGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffCountsMatch,
  appGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffHasNoRuntimeClaims,
  appGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffMatchesReadModel,
  type AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffStateValue,
} from '@ocentra-parent/schema-domain/app-game-source-gated-policy-preview-timer-service-readiness-protocol-command-handoff-rules';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffIdSchema =
  brandedNonEmptyStringSchema('AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffId');
export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffRowIdSchema =
  brandedNonEmptyStringSchema('AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffRowId');
export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffContractRefSchema =
  brandedNonEmptyStringSchema('AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffContractRef');
export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandRefSchema =
  brandedNonEmptyStringSchema('AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandRef');
export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolEventRefSchema =
  brandedNonEmptyStringSchema('AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolEventRef');
export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolServiceHandlerRefSchema =
  brandedNonEmptyStringSchema('AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolServiceHandlerRef');
export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandSummaryRefSchema =
  brandedNonEmptyStringSchema('AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandSummaryRef');

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffStateSchema = withParser(
  Schema.Literal(...Object.values(AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffState))
);
export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffNonClaimSchema = withParser(
  Schema.Literal(...RequiredAppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffNonClaims)
);

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffOptionsSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    commandHandoffId: AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffIdSchema,
    generatedAt: ParentTimestampSchema,
    sourceContractRefs: Schema.Array(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffContractRefSchema
    ),
    protocolCommandRefs: Schema.Array(AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandRefSchema),
    protocolEventRefs: Schema.Array(AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolEventRefSchema),
    serviceHandlerRefs: Schema.Array(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolServiceHandlerRefSchema
    ),
    commandSummaryRef: AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandSummaryRefSchema,
  })
    .pipe(
      Schema.filter(
        (options) =>
          options.sourceContractRefs.length > 0 ||
          'Expected source-gated policy preview timer service-readiness protocol command handoff options to cite source contracts'
      )
    )
    .pipe(
      Schema.filter(
        (options) =>
          (options.protocolCommandRefs.length > 0 &&
            options.protocolEventRefs.length > 0 &&
            options.serviceHandlerRefs.length > 0) ||
          'Expected source-gated policy preview timer service-readiness protocol command handoff options to name future command, event, and handler refs'
      )
    )
);

const AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffRowBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  rowId: AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffRowIdSchema,
  sourceProtocolReadModelRowId: AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelRowIdSchema,
  targetDomain: AppGamePolicyPreviewTargetDomainSchema,
  protocolCommandHandoffState: AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffStateSchema,
  requiredProtocolProofRefs: Schema.Array(AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolProofRefSchema),
  requiredAgentProtocolCommandRefs: Schema.Array(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandRefSchema
  ),
  requiredAgentProtocolEventRefs: Schema.Array(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolEventRefSchema
  ),
  requiredServiceHandlerRefs: Schema.Array(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolServiceHandlerRefSchema
  ),
  inheritedServiceReadinessProofRefs: Schema.Array(AppGameSourceGatedPolicyPreviewTimerProofRefSchema),
  sourceEvidenceRefs: Schema.Array(AppGameSourceFreshnessEvidenceRefSchema),
  serviceReadApiRef: AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffReadApiRefSchema,
  commandSummaryRef: AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandSummaryRefSchema,
  agentProtocolCommandImplemented: Schema.Literal(false),
  agentProtocolEventImplemented: Schema.Literal(false),
  rustProtocolMirrored: Schema.Literal(false),
  serviceCommandRegistered: Schema.Literal(false),
  serviceHandlerImplemented: Schema.Literal(false),
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

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffRowSchema = withParser(
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        row.protocolCommandHandoffState !==
          AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffState.ProtocolCommandHandoffProofRequired ||
        row.requiredAgentProtocolCommandRefs.length > 0 ||
        'Expected protocol command handoff proof-required rows to name future command refs'
    )
  )
    .pipe(
      Schema.filter(
        (row) =>
          row.protocolCommandHandoffState !==
            AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffState.ProtocolCommandHandoffProofRequired ||
          row.requiredAgentProtocolEventRefs.length > 0 ||
          'Expected protocol command handoff proof-required rows to name future event refs'
      )
    )
    .pipe(
      Schema.filter(
        (row) =>
          row.protocolCommandHandoffState !==
            AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffState.ProtocolCommandHandoffProofRequired ||
          row.requiredServiceHandlerRefs.length > 0 ||
          'Expected protocol command handoff proof-required rows to name future service handler refs'
      )
    )
);

const AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  commandHandoffId: AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffIdSchema,
  sourceProtocolReadModelId: AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelIdSchema,
  generatedAt: ParentTimestampSchema,
  sourceContractRefs: Schema.Array(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffContractRefSchema
  ),
  commandSummaryRef: AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandSummaryRefSchema,
  rows: Schema.Array(AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffRowSchema),
  nativeAppRowCount: Schema.Number,
  nativeGameRowCount: Schema.Number,
  protocolCommandHandoffProofRequiredCount: Schema.Number,
  blockedBySourceFreshnessCount: Schema.Number,
  blockedByCompilerDecisionCount: Schema.Number,
  protocolCommandHandoffNonClaims: Schema.Array(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffNonClaimSchema
  ),
  agentProtocolCommandImplemented: Schema.Literal(false),
  agentProtocolEventImplemented: Schema.Literal(false),
  rustProtocolMirrored: Schema.Literal(false),
  serviceCommandRegistered: Schema.Literal(false),
  serviceHandlerImplemented: Schema.Literal(false),
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

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffSchema = withParser(
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffBaseSchema.pipe(
    Schema.filter(
      (handoff) =>
        appGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffCountsMatch(handoff) ||
        'Expected source-gated policy preview timer service-readiness protocol command handoff counts to match row states'
    )
  ).pipe(
    Schema.filter(
      (handoff) =>
        appGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffHasNoRuntimeClaims(handoff) ||
        'Expected source-gated policy preview timer service-readiness protocol command handoff to avoid protocol, service, UI, timer, audit, rollback, adapter, and raw-source claims'
    )
  )
);

export type AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffOptions = Infer<
  typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffOptionsSchema
>;
export type AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffRow = Infer<
  typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffRowSchema
>;
export type AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoff = Infer<
  typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffSchema
>;

export function buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoff(
  optionsInput: unknown,
  readModelInput: unknown
): AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoff {
  const options =
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffOptionsSchema.parse(optionsInput);
  const readModel = AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelSchema.parse(readModelInput);
  const rows = readModel.rows.map((row) => buildProtocolCommandHandoffRow(options, row));

  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffSchema.parse({
    schemaVersion: options.schemaVersion,
    commandHandoffId: options.commandHandoffId,
    sourceProtocolReadModelId: readModel.readModelId,
    generatedAt: options.generatedAt,
    sourceContractRefs: options.sourceContractRefs,
    commandSummaryRef: options.commandSummaryRef,
    rows,
    nativeAppRowCount: readModel.nativeAppRowCount,
    nativeGameRowCount: readModel.nativeGameRowCount,
    protocolCommandHandoffProofRequiredCount: rows.filter(
      (row) =>
        row.protocolCommandHandoffState ===
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffState.ProtocolCommandHandoffProofRequired
    ).length,
    blockedBySourceFreshnessCount: rows.filter(
      (row) =>
        row.protocolCommandHandoffState ===
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffState.BlockedBySourceFreshness
    ).length,
    blockedByCompilerDecisionCount: rows.filter(
      (row) =>
        row.protocolCommandHandoffState ===
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffState.BlockedByCompilerDecision
    ).length,
    protocolCommandHandoffNonClaims:
      RequiredAppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffNonClaims,
    ...AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffNoClaimFlags,
  });
}

function buildProtocolCommandHandoffRow(
  options: AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffOptions,
  readModelRow: AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelRow
): AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffRow {
  const protocolCommandHandoffState = commandHandoffStateForReadModel(readModelRow);
  const commandRequired =
    protocolCommandHandoffState ===
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffState.ProtocolCommandHandoffProofRequired;

  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffRowSchema.parse({
    schemaVersion: options.schemaVersion,
    rowId: `${readModelRow.rowId}:protocol-command-handoff`,
    sourceProtocolReadModelRowId: readModelRow.rowId,
    targetDomain: readModelRow.targetDomain,
    protocolCommandHandoffState,
    requiredProtocolProofRefs: readModelRow.requiredProtocolProofRefs,
    requiredAgentProtocolCommandRefs: commandRequired ? options.protocolCommandRefs : [],
    requiredAgentProtocolEventRefs: commandRequired ? options.protocolEventRefs : [],
    requiredServiceHandlerRefs: commandRequired ? options.serviceHandlerRefs : [],
    inheritedServiceReadinessProofRefs: readModelRow.inheritedServiceReadinessProofRefs,
    sourceEvidenceRefs: readModelRow.sourceEvidenceRefs,
    serviceReadApiRef: readModelRow.serviceReadApiRef,
    commandSummaryRef: options.commandSummaryRef,
    ...AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffNoClaimFlags,
    generatedAt: options.generatedAt,
  });
}

function commandHandoffStateForReadModel(
  readModelRow: AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelRow
): AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffStateValue {
  for (const state of Object.values(AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffState)) {
    if (
      appGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffMatchesReadModel(
        readModelRow.protocolReadModelState,
        state
      )
    ) {
      return state;
    }
  }
  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffState.BlockedByCompilerDecision;
}

export const decodeAppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoff =
  Schema.decodeUnknownSync(AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffSchema);

export { AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffState };

