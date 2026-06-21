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
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffIdSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffRowIdSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseProofRefSchema,
  type AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffRow,
} from './app-game-source-gated-policy-preview-timer-service-readiness-read-api-response-handoff';
import { AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiProofRefSchema } from './app-game-source-gated-policy-preview-timer-service-readiness-service-handler-handoff';
import {
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffNoClaimFlags,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffState,
  RequiredAppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffNonClaims,
  appGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffCountsMatch,
  appGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffHasNoRuntimeClaims,
  appGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffMatchesResponseHandoff,
  type AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffStateValue,
} from './app-game-source-gated-policy-preview-timer-service-readiness-read-api-response-consumer-handoff-rules';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from './family-reference-primitives';

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffIdSchema =
  brandedNonEmptyStringSchema('AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffId');
export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffRowIdSchema =
  brandedNonEmptyStringSchema('AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffRowId');
export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffContractRefSchema =
  brandedNonEmptyStringSchema('AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffContractRef');
export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerProofRefSchema =
  brandedNonEmptyStringSchema('AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerProofRef');
export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerSummaryRefSchema =
  brandedNonEmptyStringSchema('AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerSummaryRef');

const ConsumerStateSchema = withParser(
  Schema.Literal(
    ...Object.values(AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffState)
  )
);
const ConsumerNonClaimSchema = withParser(
  Schema.Literal(...RequiredAppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffNonClaims)
);

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffOptionsSchema =
  withParser(
    Schema.Struct({
      schemaVersion: ParentContractSchemaVersionSchema,
      readApiResponseConsumerHandoffId:
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffIdSchema,
      generatedAt: ParentTimestampSchema,
      sourceContractRefs: Schema.Array(
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffContractRefSchema
      ),
      readApiResponseConsumerProofRefs: Schema.Array(
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerProofRefSchema
      ),
      readApiResponseConsumerSummaryRef:
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerSummaryRefSchema,
    }).pipe(
      Schema.filter(
        (options) =>
          (options.sourceContractRefs.length > 0 && options.readApiResponseConsumerProofRefs.length > 0) ||
          'Expected read API response consumer handoff options to cite source contracts and future consumer proof refs'
      )
    )
  );

const ConsumerRowBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  rowId: AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffRowIdSchema,
  sourceReadApiResponseHandoffRowId:
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffRowIdSchema,
  targetDomain: AppGamePolicyPreviewTargetDomainSchema,
  readApiResponseConsumerHandoffState: ConsumerStateSchema,
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
  inheritedReadApiResponseProofRefs: Schema.Array(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseProofRefSchema
  ),
  requiredReadApiResponseConsumerProofRefs: Schema.Array(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerProofRefSchema
  ),
  inheritedServiceReadinessProofRefs: Schema.Array(AppGameSourceGatedPolicyPreviewTimerProofRefSchema),
  sourceEvidenceRefs: Schema.Array(AppGameSourceFreshnessEvidenceRefSchema),
  serviceReadApiRef: AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffReadApiRefSchema,
  readApiResponseConsumerSummaryRef:
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerSummaryRefSchema,
  ...Object.fromEntries(
    Object.keys(AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffNoClaimFlags).map(
      (key) => [key, Schema.Literal(false)]
    )
  ),
  generatedAt: ParentTimestampSchema,
});

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffRowSchema = withParser(
  ConsumerRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        row.readApiResponseConsumerHandoffState !==
          AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffState.ReadApiResponseConsumerProofRequired ||
        (row.inheritedReadApiResponseProofRefs.length > 0 && row.requiredReadApiResponseConsumerProofRefs.length > 0) ||
        'Expected consumer proof-required rows to preserve response proof refs and name consumer proof refs'
    )
  )
);

const ConsumerHandoffBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  readApiResponseConsumerHandoffId:
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffIdSchema,
  sourceReadApiResponseHandoffId: AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffIdSchema,
  generatedAt: ParentTimestampSchema,
  sourceContractRefs: Schema.Array(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffContractRefSchema
  ),
  readApiResponseConsumerSummaryRef:
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerSummaryRefSchema,
  rows: Schema.Array(AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffRowSchema),
  nativeAppRowCount: Schema.Number,
  nativeGameRowCount: Schema.Number,
  readApiResponseConsumerProofRequiredCount: Schema.Number,
  blockedBySourceFreshnessCount: Schema.Number,
  blockedByCompilerDecisionCount: Schema.Number,
  readApiResponseConsumerHandoffNonClaims: Schema.Array(ConsumerNonClaimSchema),
  ...Object.fromEntries(
    Object.keys(AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffNoClaimFlags).map(
      (key) => [key, Schema.Literal(false)]
    )
  ),
});

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffSchema = withParser(
  ConsumerHandoffBaseSchema.pipe(
    Schema.filter(
      (handoff) =>
        appGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffCountsMatch(handoff) ||
        'Expected read API response consumer handoff counts to match row states'
    )
  ).pipe(
    Schema.filter(
      (handoff) =>
        appGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffHasNoRuntimeClaims(handoff) ||
        'Expected read API response consumer handoff to avoid service, protocol, UI, timer, audit, rollback, adapter, and raw-source claims'
    )
  )
);

export type AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffOptions = Infer<
  typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffOptionsSchema
>;
export type AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoff = Infer<
  typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffSchema
>;

export function buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoff(
  optionsInput: unknown,
  responseHandoffInput: unknown
): AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoff {
  const options =
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffOptionsSchema.parse(optionsInput);
  const responseHandoff =
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffSchema.parse(responseHandoffInput);
  const rows = responseHandoff.rows.map((row) => buildConsumerRow(options, row));

  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffSchema.parse({
    schemaVersion: options.schemaVersion,
    readApiResponseConsumerHandoffId: options.readApiResponseConsumerHandoffId,
    sourceReadApiResponseHandoffId: responseHandoff.readApiResponseHandoffId,
    generatedAt: options.generatedAt,
    sourceContractRefs: options.sourceContractRefs,
    readApiResponseConsumerSummaryRef: options.readApiResponseConsumerSummaryRef,
    rows,
    nativeAppRowCount: responseHandoff.nativeAppRowCount,
    nativeGameRowCount: responseHandoff.nativeGameRowCount,
    readApiResponseConsumerProofRequiredCount: rows.filter(
      (row) =>
        row.readApiResponseConsumerHandoffState ===
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffState.ReadApiResponseConsumerProofRequired
    ).length,
    blockedBySourceFreshnessCount: rows.filter(
      (row) =>
        row.readApiResponseConsumerHandoffState ===
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffState.BlockedBySourceFreshness
    ).length,
    blockedByCompilerDecisionCount: rows.filter(
      (row) =>
        row.readApiResponseConsumerHandoffState ===
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffState.BlockedByCompilerDecision
    ).length,
    readApiResponseConsumerHandoffNonClaims:
      RequiredAppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffNonClaims,
    ...AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffNoClaimFlags,
  });
}

function buildConsumerRow(
  options: AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffOptions,
  responseRow: AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffRow
) {
  const state = consumerStateForResponseHandoff(responseRow);
  const consumerRequired =
    state ===
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffState.ReadApiResponseConsumerProofRequired;

  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffRowSchema.parse({
    schemaVersion: options.schemaVersion,
    rowId: `${responseRow.rowId}:read-api-response-consumer-handoff`,
    sourceReadApiResponseHandoffRowId: responseRow.rowId,
    targetDomain: responseRow.targetDomain,
    readApiResponseConsumerHandoffState: state,
    inheritedProtocolProofRefs: responseRow.inheritedProtocolProofRefs,
    inheritedAgentProtocolCommandRefs: responseRow.inheritedAgentProtocolCommandRefs,
    inheritedAgentProtocolEventRefs: responseRow.inheritedAgentProtocolEventRefs,
    inheritedServiceHandlerRefs: responseRow.inheritedServiceHandlerRefs,
    inheritedServiceReadApiProofRefs: responseRow.inheritedServiceReadApiProofRefs,
    inheritedReadApiResponseProofRefs: responseRow.requiredReadApiResponseProofRefs,
    requiredReadApiResponseConsumerProofRefs: consumerRequired ? options.readApiResponseConsumerProofRefs : [],
    inheritedServiceReadinessProofRefs: responseRow.inheritedServiceReadinessProofRefs,
    sourceEvidenceRefs: responseRow.sourceEvidenceRefs,
    serviceReadApiRef: responseRow.serviceReadApiRef,
    readApiResponseConsumerSummaryRef: options.readApiResponseConsumerSummaryRef,
    ...AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffNoClaimFlags,
    generatedAt: options.generatedAt,
  });
}

function consumerStateForResponseHandoff(
  responseRow: AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffRow
): AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffStateValue {
  for (const state of Object.values(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffState
  )) {
    if (
      appGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffMatchesResponseHandoff(
        responseRow.readApiResponseHandoffState,
        state
      )
    ) {
      return state;
    }
  }
  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffState.BlockedByCompilerDecision;
}

export const decodeAppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoff =
  Schema.decodeUnknownSync(AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffSchema);

export { AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffState };

