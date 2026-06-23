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
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffIdSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffRowIdSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceProofRefSchema,
} from './app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-handoff';
import type { AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffRowSchema } from './app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-handoff';
import { AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseProofRefSchema } from './app-game-source-gated-policy-preview-timer-service-readiness-read-api-response-handoff';
import { AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiProofRefSchema } from './app-game-source-gated-policy-preview-timer-service-readiness-service-handler-handoff';
import {
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffNoClaimFlags,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffState,
  RequiredAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffNonClaims,
  appGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffCountsMatch,
  appGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffHasNoRuntimeClaims,
  appGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffMatchesParentSurfaceHandoff,
  type AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffStateValue,
} from './app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-read-model-handoff-rules';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from './family-reference-primitives';

type ParentSurfaceHandoffRow = Infer<
  typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffRowSchema
>;

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffIdSchema =
  brandedNonEmptyStringSchema(
    'AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffId'
  );
export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffRowIdSchema =
  NonEmptyStringSchema.pipe(
    Schema.brand(
      'AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffRowId'
    )
  );
export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffContractRefSchema =
  NonEmptyStringSchema.pipe(
    Schema.brand(
      'AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffContractRef'
    )
  );
export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelProofRefSchema =
  brandedNonEmptyStringSchema(
    'AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelProofRef'
  );
export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelRefSchema =
  brandedNonEmptyStringSchema(
    'AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelRef'
  );

const ReadModelStateSchema = withParser(
  Schema.Literal(
    ...Object.values(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffState
    )
  )
);
const ReadModelNonClaimSchema = withParser(
  Schema.Literal(
    ...RequiredAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffNonClaims
  )
);

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffOptionsSchema =
  withParser(
    Schema.Struct({
      schemaVersion: ParentContractSchemaVersionSchema,
      responseConsumerParentSurfaceReadModelHandoffId:
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffIdSchema,
      generatedAt: ParentTimestampSchema,
      sourceContractRefs: Schema.Array(
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffContractRefSchema
      ),
      parentSurfaceReadModelProofRefs: Schema.Array(
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelProofRefSchema
      ),
      parentSurfaceReadModelRef:
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelRefSchema,
    }).pipe(
      Schema.filter(
        (options) =>
          (options.sourceContractRefs.length > 0 && options.parentSurfaceReadModelProofRefs.length > 0) ||
          'Expected response consumer parent-surface read-model handoff options to cite source contracts and future read-model proof refs'
      )
    )
  );

const ReadModelRowBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  rowId: AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffRowIdSchema,
  sourceResponseConsumerParentSurfaceHandoffRowId:
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffRowIdSchema,
  targetDomain: AppGamePolicyPreviewTargetDomainSchema,
  responseConsumerParentSurfaceReadModelHandoffState: ReadModelStateSchema,
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
  requiredParentSurfaceReadModelProofRefs: Schema.Array(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelProofRefSchema
  ),
  inheritedServiceReadinessProofRefs: Schema.Array(AppGameSourceGatedPolicyPreviewTimerProofRefSchema),
  sourceEvidenceRefs: Schema.Array(AppGameSourceFreshnessEvidenceRefSchema),
  serviceReadApiRef: AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffReadApiRefSchema,
  parentSurfaceReadModelRef:
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelRefSchema,
  ...Object.fromEntries(
    Object.keys(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffNoClaimFlags
    ).map((key) => [key, Schema.Literal(false)])
  ),
  generatedAt: ParentTimestampSchema,
});

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffRowSchema =
  withParser(
    ReadModelRowBaseSchema.pipe(
      Schema.filter(
        (row) =>
          row.responseConsumerParentSurfaceReadModelHandoffState !==
            AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffState.ParentSurfaceReadModelProofRequired ||
          (row.inheritedParentSurfaceProofRefs.length > 0 && row.requiredParentSurfaceReadModelProofRefs.length > 0) ||
          'Expected parent-surface read-model proof-required rows to preserve parent-surface proof refs and name read-model proof refs'
      )
    )
  );

const ReadModelHandoffBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  responseConsumerParentSurfaceReadModelHandoffId:
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffIdSchema,
  sourceResponseConsumerParentSurfaceHandoffId:
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffIdSchema,
  generatedAt: ParentTimestampSchema,
  sourceContractRefs: Schema.Array(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffContractRefSchema
  ),
  parentSurfaceReadModelRef:
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelRefSchema,
  rows: Schema.Array(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffRowSchema
  ),
  nativeAppRowCount: Schema.Number,
  nativeGameRowCount: Schema.Number,
  parentSurfaceReadModelProofRequiredCount: Schema.Number,
  blockedBySourceFreshnessCount: Schema.Number,
  blockedByCompilerDecisionCount: Schema.Number,
  responseConsumerParentSurfaceReadModelHandoffNonClaims: Schema.Array(ReadModelNonClaimSchema),
  ...Object.fromEntries(
    Object.keys(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffNoClaimFlags
    ).map((key) => [key, Schema.Literal(false)])
  ),
});

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffSchema =
  withParser(
    ReadModelHandoffBaseSchema.pipe(
      Schema.filter(
        (handoff) =>
          appGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffCountsMatch(
            handoff
          ) || 'Expected response consumer parent-surface read-model handoff counts to match row states'
      )
    ).pipe(
      Schema.filter(
        (handoff) =>
          appGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffHasNoRuntimeClaims(
            handoff
          ) ||
          'Expected response consumer parent-surface read-model handoff to avoid service, protocol, UI, timer, audit, rollback, adapter, and raw-source claims'
      )
    )
  );

export type AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffOptions =
  Infer<
    typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffOptionsSchema
  >;
export type AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoff = Infer<
  typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffSchema
>;

export function buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoff(
  optionsInput: unknown,
  parentSurfaceHandoffInput: unknown
): AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoff {
  const options =
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffOptionsSchema.parse(
      optionsInput
    );
  const parentSurfaceHandoff =
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffSchema.parse(
      parentSurfaceHandoffInput
    );
  const rows = parentSurfaceHandoff.rows.map((row) => buildReadModelRow(options, row));

  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffSchema.parse({
    schemaVersion: options.schemaVersion,
    responseConsumerParentSurfaceReadModelHandoffId: options.responseConsumerParentSurfaceReadModelHandoffId,
    sourceResponseConsumerParentSurfaceHandoffId: parentSurfaceHandoff.responseConsumerParentSurfaceHandoffId,
    generatedAt: options.generatedAt,
    sourceContractRefs: options.sourceContractRefs,
    parentSurfaceReadModelRef: options.parentSurfaceReadModelRef,
    rows,
    nativeAppRowCount: parentSurfaceHandoff.nativeAppRowCount,
    nativeGameRowCount: parentSurfaceHandoff.nativeGameRowCount,
    parentSurfaceReadModelProofRequiredCount: rows.filter(
      (row) =>
        row.responseConsumerParentSurfaceReadModelHandoffState ===
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffState.ParentSurfaceReadModelProofRequired
    ).length,
    blockedBySourceFreshnessCount: rows.filter(
      (row) =>
        row.responseConsumerParentSurfaceReadModelHandoffState ===
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffState.BlockedBySourceFreshness
    ).length,
    blockedByCompilerDecisionCount: rows.filter(
      (row) =>
        row.responseConsumerParentSurfaceReadModelHandoffState ===
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffState.BlockedByCompilerDecision
    ).length,
    responseConsumerParentSurfaceReadModelHandoffNonClaims:
      RequiredAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffNonClaims,
    ...AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffNoClaimFlags,
  });
}

function buildReadModelRow(
  options: AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffOptions,
  parentSurfaceRow: ParentSurfaceHandoffRow
) {
  const state = readModelStateForParentSurfaceHandoff(parentSurfaceRow);
  const readModelRequired =
    state ===
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffState.ParentSurfaceReadModelProofRequired;

  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffRowSchema.parse(
    {
      schemaVersion: options.schemaVersion,
      rowId: `${parentSurfaceRow.rowId}:parent-surface-read-model-handoff`,
      sourceResponseConsumerParentSurfaceHandoffRowId: parentSurfaceRow.rowId,
      targetDomain: parentSurfaceRow.targetDomain,
      responseConsumerParentSurfaceReadModelHandoffState: state,
      inheritedProtocolProofRefs: parentSurfaceRow.inheritedProtocolProofRefs,
      inheritedAgentProtocolCommandRefs: parentSurfaceRow.inheritedAgentProtocolCommandRefs,
      inheritedAgentProtocolEventRefs: parentSurfaceRow.inheritedAgentProtocolEventRefs,
      inheritedServiceHandlerRefs: parentSurfaceRow.inheritedServiceHandlerRefs,
      inheritedServiceReadApiProofRefs: parentSurfaceRow.inheritedServiceReadApiProofRefs,
      inheritedReadApiResponseProofRefs: parentSurfaceRow.inheritedReadApiResponseProofRefs,
      inheritedReadApiResponseConsumerProofRefs: parentSurfaceRow.inheritedReadApiResponseConsumerProofRefs,
      inheritedParentSurfaceProofRefs: parentSurfaceRow.requiredParentSurfaceProofRefs,
      requiredParentSurfaceReadModelProofRefs: readModelRequired ? options.parentSurfaceReadModelProofRefs : [],
      inheritedServiceReadinessProofRefs: parentSurfaceRow.inheritedServiceReadinessProofRefs,
      sourceEvidenceRefs: parentSurfaceRow.sourceEvidenceRefs,
      serviceReadApiRef: parentSurfaceRow.serviceReadApiRef,
      parentSurfaceReadModelRef: options.parentSurfaceReadModelRef,
      ...AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffNoClaimFlags,
      generatedAt: options.generatedAt,
    }
  );
}

function readModelStateForParentSurfaceHandoff(
  parentSurfaceRow: ParentSurfaceHandoffRow
): AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffStateValue {
  for (const state of Object.values(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffState
  )) {
    if (
      appGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffMatchesParentSurfaceHandoff(
        parentSurfaceRow.responseConsumerParentSurfaceHandoffState,
        state
      )
    ) {
      return state;
    }
  }
  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffState.BlockedByCompilerDecision;
}

export const decodeAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoff =
  Schema.decodeUnknownSync(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffSchema
  );

export { AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffState };
