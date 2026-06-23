import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from './effect';
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
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffIdSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffRowIdSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerProofRefSchema,
} from './app-game-source-gated-policy-preview-timer-service-readiness-read-api-response-consumer-handoff';
import type { AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffRowSchema } from './app-game-source-gated-policy-preview-timer-service-readiness-read-api-response-consumer-handoff';
import { AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseProofRefSchema } from './app-game-source-gated-policy-preview-timer-service-readiness-read-api-response-handoff';
import { AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiProofRefSchema } from './app-game-source-gated-policy-preview-timer-service-readiness-service-handler-handoff';
import {
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffNoClaimFlags,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffState,
  RequiredAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffNonClaims,
  appGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffCountsMatch,
  appGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffHasNoRuntimeClaims,
  appGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffMatchesResponseHandoff,
  type AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffStateValue,
} from './app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-handoff-rules';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from './family-reference-primitives';

type AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffRow = Infer<
  typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffRowSchema
>;

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffIdSchema =
  brandedNonEmptyStringSchema(
    'AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffId'
  );
export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffRowIdSchema =
  brandedNonEmptyStringSchema(
    'AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffRowId'
  );
export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffContractRefSchema =
  brandedNonEmptyStringSchema(
    'AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffContractRef'
  );
export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceProofRefSchema =
  brandedNonEmptyStringSchema(
    'AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceProofRef'
  );
export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceSummaryRefSchema =
  brandedNonEmptyStringSchema(
    'AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceSummaryRef'
  );

const ConsumerStateSchema = withParser(
  Schema.Literal(
    ...Object.values(AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffState)
  )
);
const ConsumerNonClaimSchema = withParser(
  Schema.Literal(
    ...RequiredAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffNonClaims
  )
);

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffOptionsSchema =
  withParser(
    Schema.Struct({
      schemaVersion: ParentContractSchemaVersionSchema,
      responseConsumerParentSurfaceHandoffId:
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffIdSchema,
      generatedAt: ParentTimestampSchema,
      sourceContractRefs: Schema.Array(
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffContractRefSchema
      ),
      parentSurfaceProofRefs: Schema.Array(
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceProofRefSchema
      ),
      parentSurfaceSummaryRef:
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceSummaryRefSchema,
    }).pipe(
      Schema.filter(
        (options) =>
          (options.sourceContractRefs.length > 0 && options.parentSurfaceProofRefs.length > 0) ||
          'Expected response consumer parent-surface handoff options to cite source contracts and future consumer proof refs'
      )
    )
  );

const ConsumerRowBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  rowId: AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffRowIdSchema,
  sourceReadApiResponseConsumerHandoffRowId:
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffRowIdSchema,
  targetDomain: AppGamePolicyPreviewTargetDomainSchema,
  responseConsumerParentSurfaceHandoffState: ConsumerStateSchema,
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
  inheritedReadApiResponseConsumerProofRefs: Schema.Array(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerProofRefSchema
  ),
  requiredParentSurfaceProofRefs: Schema.Array(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceProofRefSchema
  ),
  inheritedServiceReadinessProofRefs: Schema.Array(AppGameSourceGatedPolicyPreviewTimerProofRefSchema),
  sourceEvidenceRefs: Schema.Array(AppGameSourceFreshnessEvidenceRefSchema),
  serviceReadApiRef: AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffReadApiRefSchema,
  parentSurfaceSummaryRef:
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceSummaryRefSchema,
  ...Object.fromEntries(
    Object.keys(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffNoClaimFlags
    ).map((key) => [key, Schema.Literal(false)])
  ),
  generatedAt: ParentTimestampSchema,
});

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffRowSchema =
  withParser(
    ConsumerRowBaseSchema.pipe(
      Schema.filter(
        (row) =>
          row.responseConsumerParentSurfaceHandoffState !==
            AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffState.ParentSurfaceProofRequired ||
          (row.inheritedReadApiResponseConsumerProofRefs.length > 0 && row.requiredParentSurfaceProofRefs.length > 0) ||
          'Expected parent-surface proof-required rows to preserve consumer proof refs and name parent-surface proof refs'
      )
    )
  );

const ConsumerHandoffBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  responseConsumerParentSurfaceHandoffId:
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffIdSchema,
  sourceReadApiResponseConsumerHandoffId:
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffIdSchema,
  generatedAt: ParentTimestampSchema,
  sourceContractRefs: Schema.Array(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffContractRefSchema
  ),
  parentSurfaceSummaryRef:
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceSummaryRefSchema,
  rows: Schema.Array(AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffRowSchema),
  nativeAppRowCount: Schema.Number,
  nativeGameRowCount: Schema.Number,
  parentSurfaceProofRequiredCount: Schema.Number,
  blockedBySourceFreshnessCount: Schema.Number,
  blockedByCompilerDecisionCount: Schema.Number,
  responseConsumerParentSurfaceHandoffNonClaims: Schema.Array(ConsumerNonClaimSchema),
  ...Object.fromEntries(
    Object.keys(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffNoClaimFlags
    ).map((key) => [key, Schema.Literal(false)])
  ),
});

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffSchema =
  withParser(
    ConsumerHandoffBaseSchema.pipe(
      Schema.filter(
        (handoff) =>
          appGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffCountsMatch(
            handoff
          ) || 'Expected response consumer parent-surface handoff counts to match row states'
      )
    ).pipe(
      Schema.filter(
        (handoff) =>
          appGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffHasNoRuntimeClaims(
            handoff
          ) ||
          'Expected response consumer parent-surface handoff to avoid service, protocol, UI, timer, audit, rollback, adapter, and raw-source claims'
      )
    )
  );

export type AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffOptions = Infer<
  typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffOptionsSchema
>;
export type AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoff = Infer<
  typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffSchema
>;

export function buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoff(
  optionsInput: unknown,
  responseConsumerHandoffInput: unknown
): AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoff {
  const options =
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffOptionsSchema.parse(
      optionsInput
    );
  const responseConsumerHandoff =
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffSchema.parse(
      responseConsumerHandoffInput
    );
  const rows = responseConsumerHandoff.rows.map((row) => buildParentSurfaceRow(options, row));

  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffSchema.parse({
    schemaVersion: options.schemaVersion,
    responseConsumerParentSurfaceHandoffId: options.responseConsumerParentSurfaceHandoffId,
    sourceReadApiResponseConsumerHandoffId: responseConsumerHandoff.readApiResponseConsumerHandoffId,
    generatedAt: options.generatedAt,
    sourceContractRefs: options.sourceContractRefs,
    parentSurfaceSummaryRef: options.parentSurfaceSummaryRef,
    rows,
    nativeAppRowCount: responseConsumerHandoff.nativeAppRowCount,
    nativeGameRowCount: responseConsumerHandoff.nativeGameRowCount,
    parentSurfaceProofRequiredCount: rows.filter(
      (row) =>
        row.responseConsumerParentSurfaceHandoffState ===
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffState.ParentSurfaceProofRequired
    ).length,
    blockedBySourceFreshnessCount: rows.filter(
      (row) =>
        row.responseConsumerParentSurfaceHandoffState ===
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffState.BlockedBySourceFreshness
    ).length,
    blockedByCompilerDecisionCount: rows.filter(
      (row) =>
        row.responseConsumerParentSurfaceHandoffState ===
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffState.BlockedByCompilerDecision
    ).length,
    responseConsumerParentSurfaceHandoffNonClaims:
      RequiredAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffNonClaims,
    ...AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffNoClaimFlags,
  });
}

function buildParentSurfaceRow(
  options: AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffOptions,
  responseConsumerRow: AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffRow
) {
  const state = parentSurfaceStateForResponseConsumerHandoff(responseConsumerRow);
  const consumerRequired =
    state ===
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffState.ParentSurfaceProofRequired;

  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffRowSchema.parse({
    schemaVersion: options.schemaVersion,
    rowId: `${responseConsumerRow.rowId}:response-consumer-parent-surface-handoff`,
    sourceReadApiResponseConsumerHandoffRowId: responseConsumerRow.rowId,
    targetDomain: responseConsumerRow.targetDomain,
    responseConsumerParentSurfaceHandoffState: state,
    inheritedProtocolProofRefs: responseConsumerRow.inheritedProtocolProofRefs,
    inheritedAgentProtocolCommandRefs: responseConsumerRow.inheritedAgentProtocolCommandRefs,
    inheritedAgentProtocolEventRefs: responseConsumerRow.inheritedAgentProtocolEventRefs,
    inheritedServiceHandlerRefs: responseConsumerRow.inheritedServiceHandlerRefs,
    inheritedServiceReadApiProofRefs: responseConsumerRow.inheritedServiceReadApiProofRefs,
    inheritedReadApiResponseProofRefs: responseConsumerRow.inheritedReadApiResponseProofRefs,
    inheritedReadApiResponseConsumerProofRefs: responseConsumerRow.requiredReadApiResponseConsumerProofRefs,
    requiredParentSurfaceProofRefs: consumerRequired ? options.parentSurfaceProofRefs : [],
    inheritedServiceReadinessProofRefs: responseConsumerRow.inheritedServiceReadinessProofRefs,
    sourceEvidenceRefs: responseConsumerRow.sourceEvidenceRefs,
    serviceReadApiRef: responseConsumerRow.serviceReadApiRef,
    parentSurfaceSummaryRef: options.parentSurfaceSummaryRef,
    ...AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffNoClaimFlags,
    generatedAt: options.generatedAt,
  });
}

function parentSurfaceStateForResponseConsumerHandoff(
  responseConsumerRow: AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffRow
): AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffStateValue {
  for (const state of Object.values(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffState
  )) {
    if (
      appGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffMatchesResponseHandoff(
        responseConsumerRow.readApiResponseConsumerHandoffState,
        state
      )
    ) {
      return state;
    }
  }
  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffState.BlockedByCompilerDecision;
}

export const decodeAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoff =
  Schema.decodeUnknownSync(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffSchema
  );

export { AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffState };
