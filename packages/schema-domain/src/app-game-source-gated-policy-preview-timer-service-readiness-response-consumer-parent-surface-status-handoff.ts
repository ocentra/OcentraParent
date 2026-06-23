import { type Infer, Schema, withParser, brandedNonEmptyStringSchema, NonEmptyStringSchema } from './effect';
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
import { AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerProofRefSchema } from './app-game-source-gated-policy-preview-timer-service-readiness-read-api-response-consumer-handoff';
import {
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffIdSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffRowIdSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelProofRefSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelRefSchema,
} from './app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-read-model-handoff';
import type { AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffRowSchema } from './app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-read-model-handoff';
import { AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceProofRefSchema } from './app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-handoff';
import { AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseProofRefSchema } from './app-game-source-gated-policy-preview-timer-service-readiness-read-api-response-handoff';
import { AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiProofRefSchema } from './app-game-source-gated-policy-preview-timer-service-readiness-service-handler-handoff';
import {
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffNoClaimFlags,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffState,
  RequiredAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffNonClaims,
  appGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffCountsMatch,
  appGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffHasNoRuntimeClaims,
  appGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffMatchesReadModelHandoff,
  type AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffStateValue,
} from './app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-handoff-rules';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from './family-reference-primitives';

type ParentSurfaceReadModelHandoffRow = Infer<
  typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffRowSchema
>;

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffIdSchema =
  brandedNonEmptyStringSchema(
    'AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffId'
  );
export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffRowIdSchema =
  brandedNonEmptyStringSchema(
    'AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffRowId'
  );
export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffContractRefSchema =
  NonEmptyStringSchema.pipe(
    Schema.brand(
      'AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffContractRef'
    )
  );
export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusProofRefSchema =
  brandedNonEmptyStringSchema(
    'AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusProofRef'
  );
export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusRefSchema =
  brandedNonEmptyStringSchema(
    'AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusRef'
  );

const StatusStateSchema = withParser(
  Schema.Literal(
    ...Object.values(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffState
    )
  )
);
const StatusNonClaimSchema = withParser(
  Schema.Literal(
    ...RequiredAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffNonClaims
  )
);

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffOptionsSchema =
  withParser(
    Schema.Struct({
      schemaVersion: ParentContractSchemaVersionSchema,
      responseConsumerParentSurfaceStatusHandoffId:
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffIdSchema,
      generatedAt: ParentTimestampSchema,
      sourceContractRefs: Schema.Array(
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffContractRefSchema
      ),
      parentSurfaceStatusProofRefs: Schema.Array(
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusProofRefSchema
      ),
      parentSurfaceStatusRef:
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusRefSchema,
    }).pipe(
      Schema.filter(
        (options) =>
          (options.sourceContractRefs.length > 0 && options.parentSurfaceStatusProofRefs.length > 0) ||
          'Expected response consumer parent-surface status handoff options to cite source contracts and future status proof refs'
      )
    )
  );

const StatusRowBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  rowId: AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffRowIdSchema,
  sourceResponseConsumerParentSurfaceReadModelHandoffRowId:
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffRowIdSchema,
  targetDomain: AppGamePolicyPreviewTargetDomainSchema,
  responseConsumerParentSurfaceStatusHandoffState: StatusStateSchema,
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
  inheritedParentSurfaceProofRefs: Schema.Array(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceProofRefSchema
  ),
  inheritedParentSurfaceReadModelProofRefs: Schema.Array(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelProofRefSchema
  ),
  requiredParentSurfaceStatusProofRefs: Schema.Array(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusProofRefSchema
  ),
  inheritedServiceReadinessProofRefs: Schema.Array(AppGameSourceGatedPolicyPreviewTimerProofRefSchema),
  sourceEvidenceRefs: Schema.Array(AppGameSourceFreshnessEvidenceRefSchema),
  serviceReadApiRef: AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffReadApiRefSchema,
  parentSurfaceReadModelRef:
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelRefSchema,
  parentSurfaceStatusRef:
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusRefSchema,
  ...Object.fromEntries(
    Object.keys(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffNoClaimFlags
    ).map((key) => [key, Schema.Literal(false)])
  ),
  generatedAt: ParentTimestampSchema,
});

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffRowSchema =
  withParser(
    StatusRowBaseSchema.pipe(
      Schema.filter(
        (row) =>
          row.responseConsumerParentSurfaceStatusHandoffState !==
            AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffState.ParentSurfaceStatusProofRequired ||
          (row.inheritedParentSurfaceReadModelProofRefs.length > 0 &&
            row.requiredParentSurfaceStatusProofRefs.length > 0) ||
          'Expected parent-surface status proof-required rows to preserve read-model proof refs and name status proof refs'
      )
    )
  );

const StatusHandoffBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  responseConsumerParentSurfaceStatusHandoffId:
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffIdSchema,
  sourceResponseConsumerParentSurfaceReadModelHandoffId:
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffIdSchema,
  generatedAt: ParentTimestampSchema,
  sourceContractRefs: Schema.Array(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffContractRefSchema
  ),
  parentSurfaceStatusRef:
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusRefSchema,
  rows: Schema.Array(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffRowSchema
  ),
  nativeAppRowCount: Schema.Number,
  nativeGameRowCount: Schema.Number,
  parentSurfaceStatusProofRequiredCount: Schema.Number,
  blockedBySourceFreshnessCount: Schema.Number,
  blockedByCompilerDecisionCount: Schema.Number,
  responseConsumerParentSurfaceStatusHandoffNonClaims: Schema.Array(StatusNonClaimSchema),
  ...Object.fromEntries(
    Object.keys(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffNoClaimFlags
    ).map((key) => [key, Schema.Literal(false)])
  ),
});

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffSchema =
  withParser(
    StatusHandoffBaseSchema.pipe(
      Schema.filter(
        (handoff) =>
          appGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffCountsMatch(
            handoff
          ) || 'Expected response consumer parent-surface status handoff counts to match row states'
      )
    ).pipe(
      Schema.filter(
        (handoff) =>
          appGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffHasNoRuntimeClaims(
            handoff
          ) ||
          'Expected response consumer parent-surface status handoff to avoid service, protocol, UI, timer, audit, rollback, adapter, and raw-source claims'
      )
    )
  );

export type AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffOptions =
  Infer<
    typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffOptionsSchema
  >;
export type AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoff = Infer<
  typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffSchema
>;

export function buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoff(
  optionsInput: unknown,
  parentSurfaceReadModelHandoffInput: unknown
): AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoff {
  const options =
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffOptionsSchema.parse(
      optionsInput
    );
  const parentSurfaceReadModelHandoff =
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffSchema.parse(
      parentSurfaceReadModelHandoffInput
    );
  const rows = parentSurfaceReadModelHandoff.rows.map((row) => buildStatusRow(options, row));

  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffSchema.parse({
    schemaVersion: options.schemaVersion,
    responseConsumerParentSurfaceStatusHandoffId: options.responseConsumerParentSurfaceStatusHandoffId,
    sourceResponseConsumerParentSurfaceReadModelHandoffId:
      parentSurfaceReadModelHandoff.responseConsumerParentSurfaceReadModelHandoffId,
    generatedAt: options.generatedAt,
    sourceContractRefs: options.sourceContractRefs,
    parentSurfaceStatusRef: options.parentSurfaceStatusRef,
    rows,
    nativeAppRowCount: parentSurfaceReadModelHandoff.nativeAppRowCount,
    nativeGameRowCount: parentSurfaceReadModelHandoff.nativeGameRowCount,
    parentSurfaceStatusProofRequiredCount: rows.filter(
      (row) =>
        row.responseConsumerParentSurfaceStatusHandoffState ===
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffState.ParentSurfaceStatusProofRequired
    ).length,
    blockedBySourceFreshnessCount: rows.filter(
      (row) =>
        row.responseConsumerParentSurfaceStatusHandoffState ===
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffState.BlockedBySourceFreshness
    ).length,
    blockedByCompilerDecisionCount: rows.filter(
      (row) =>
        row.responseConsumerParentSurfaceStatusHandoffState ===
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffState.BlockedByCompilerDecision
    ).length,
    responseConsumerParentSurfaceStatusHandoffNonClaims:
      RequiredAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffNonClaims,
    ...AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffNoClaimFlags,
  });
}

function buildStatusRow(
  options: AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffOptions,
  readModelRow: ParentSurfaceReadModelHandoffRow
) {
  const state = statusStateForReadModelHandoff(readModelRow);
  const statusRequired =
    state ===
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffState.ParentSurfaceStatusProofRequired;

  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffRowSchema.parse({
    schemaVersion: options.schemaVersion,
    rowId: `${readModelRow.rowId}:parent-surface-status-handoff`,
    sourceResponseConsumerParentSurfaceReadModelHandoffRowId: readModelRow.rowId,
    targetDomain: readModelRow.targetDomain,
    responseConsumerParentSurfaceStatusHandoffState: state,
    inheritedProtocolProofRefs: readModelRow.inheritedProtocolProofRefs,
    inheritedAgentProtocolCommandRefs: readModelRow.inheritedAgentProtocolCommandRefs,
    inheritedAgentProtocolEventRefs: readModelRow.inheritedAgentProtocolEventRefs,
    inheritedServiceHandlerRefs: readModelRow.inheritedServiceHandlerRefs,
    inheritedServiceReadApiProofRefs: readModelRow.inheritedServiceReadApiProofRefs,
    inheritedReadApiResponseProofRefs: readModelRow.inheritedReadApiResponseProofRefs,
    inheritedReadApiResponseConsumerProofRefs: readModelRow.inheritedReadApiResponseConsumerProofRefs,
    inheritedParentSurfaceProofRefs: readModelRow.inheritedParentSurfaceProofRefs,
    inheritedParentSurfaceReadModelProofRefs: readModelRow.requiredParentSurfaceReadModelProofRefs,
    requiredParentSurfaceStatusProofRefs: statusRequired ? options.parentSurfaceStatusProofRefs : [],
    inheritedServiceReadinessProofRefs: readModelRow.inheritedServiceReadinessProofRefs,
    sourceEvidenceRefs: readModelRow.sourceEvidenceRefs,
    serviceReadApiRef: readModelRow.serviceReadApiRef,
    parentSurfaceReadModelRef: readModelRow.parentSurfaceReadModelRef,
    parentSurfaceStatusRef: options.parentSurfaceStatusRef,
    ...AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffNoClaimFlags,
    generatedAt: options.generatedAt,
  });
}

function statusStateForReadModelHandoff(
  readModelRow: ParentSurfaceReadModelHandoffRow
): AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffStateValue {
  for (const state of Object.values(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffState
  )) {
    if (
      appGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffMatchesReadModelHandoff(
        readModelRow.responseConsumerParentSurfaceReadModelHandoffState,
        state
      )
    ) {
      return state;
    }
  }
  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffState.BlockedByCompilerDecision;
}

export const decodeAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoff =
  Schema.decodeUnknownSync(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffSchema
  );

export { AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffState };
