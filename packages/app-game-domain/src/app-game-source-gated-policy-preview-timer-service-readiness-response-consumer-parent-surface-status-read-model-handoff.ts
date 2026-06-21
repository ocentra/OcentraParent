import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema,
  NonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { AppGamePolicyPreviewTargetDomainSchema } from './app-game-policy-preview-handoff';
import { AppGameSourceFreshnessEvidenceRefSchema } from '@ocentra-parent/schema-domain/app-game-source-freshness-policy-consumption';
import { AppGameSourceGatedPolicyPreviewTimerProofRefSchema } from './app-game-source-gated-policy-preview-timer-status';
import { AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffReadApiRefSchema } from './app-game-source-gated-policy-preview-timer-service-readiness-handoff';
import { AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolProofRefSchema } from './app-game-source-gated-policy-preview-timer-service-readiness-protocol-handoff';
import {
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandRefSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolEventRefSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolServiceHandlerRefSchema,
} from './app-game-source-gated-policy-preview-timer-service-readiness-protocol-command-handoff';
import { AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerProofRefSchema } from './app-game-source-gated-policy-preview-timer-service-readiness-read-api-response-consumer-handoff';
import { AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseProofRefSchema } from './app-game-source-gated-policy-preview-timer-service-readiness-read-api-response-handoff';
import { AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiProofRefSchema } from './app-game-source-gated-policy-preview-timer-service-readiness-service-handler-handoff';
import { AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceProofRefSchema } from './app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-handoff';
import {
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelProofRefSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelRefSchema,
} from './app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-read-model-handoff';
import {
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffIdSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffRowIdSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusProofRefSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusRefSchema,
  type AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffRowSchema,
} from './app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-handoff';
import {
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoffNoClaimFlags,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoffState,
  RequiredAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoffNonClaims,
  appGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoffCountsMatch,
  appGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoffHasNoRuntimeClaims,
  appGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoffMatchesStatusHandoff,
  type AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoffStateValue,
} from '@ocentra-parent/schema-domain/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-handoff-rules';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';

type ParentSurfaceStatusHandoffRow = Infer<
  typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffRowSchema
>;

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoffIdSchema =
  NonEmptyStringSchema.pipe(
    Schema.brand(
      'AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoffId'
    )
  );
export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoffRowIdSchema =
  NonEmptyStringSchema.pipe(
    Schema.brand(
      'AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoffRowId'
    )
  );
export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoffContractRefSchema =
  NonEmptyStringSchema.pipe(
    Schema.brand(
      'AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoffContractRef'
    )
  );
export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelProofRefSchema =
  NonEmptyStringSchema.pipe(
    Schema.brand(
      'AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelProofRef'
    )
  );
export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelRefSchema =
  brandedNonEmptyStringSchema('AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelRef');

const StatusReadModelStateSchema = withParser(
  Schema.Literal(
    ...Object.values(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoffState
    )
  )
);
const StatusReadModelNonClaimSchema = withParser(
  Schema.Literal(
    ...RequiredAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoffNonClaims
  )
);

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoffOptionsSchema =
  withParser(
    Schema.Struct({
      schemaVersion: ParentContractSchemaVersionSchema,
      responseConsumerParentSurfaceStatusReadModelHandoffId:
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoffIdSchema,
      generatedAt: ParentTimestampSchema,
      sourceContractRefs: Schema.Array(
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoffContractRefSchema
      ),
      parentSurfaceStatusReadModelProofRefs: Schema.Array(
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelProofRefSchema
      ),
      parentSurfaceStatusReadModelRef:
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelRefSchema,
    }).pipe(
      Schema.filter(
        (options) =>
          (options.sourceContractRefs.length > 0 && options.parentSurfaceStatusReadModelProofRefs.length > 0) ||
          'Expected response consumer parent-surface status read-model handoff options to cite source contracts and future status read-model proof refs'
      )
    )
  );

const StatusReadModelRowBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  rowId:
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoffRowIdSchema,
  sourceResponseConsumerParentSurfaceStatusHandoffRowId:
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffRowIdSchema,
  targetDomain: AppGamePolicyPreviewTargetDomainSchema,
  responseConsumerParentSurfaceStatusReadModelHandoffState: StatusReadModelStateSchema,
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
  inheritedParentSurfaceStatusProofRefs: Schema.Array(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusProofRefSchema
  ),
  requiredParentSurfaceStatusReadModelProofRefs: Schema.Array(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelProofRefSchema
  ),
  inheritedServiceReadinessProofRefs: Schema.Array(AppGameSourceGatedPolicyPreviewTimerProofRefSchema),
  sourceEvidenceRefs: Schema.Array(AppGameSourceFreshnessEvidenceRefSchema),
  serviceReadApiRef: AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffReadApiRefSchema,
  parentSurfaceReadModelRef:
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelRefSchema,
  parentSurfaceStatusRef:
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusRefSchema,
  parentSurfaceStatusReadModelRef:
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelRefSchema,
  ...Object.fromEntries(
    Object.keys(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoffNoClaimFlags
    ).map((key) => [key, Schema.Literal(false)])
  ),
  generatedAt: ParentTimestampSchema,
});

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoffRowSchema =
  withParser(
    StatusReadModelRowBaseSchema.pipe(
      Schema.filter(
        (row) =>
          row.responseConsumerParentSurfaceStatusReadModelHandoffState !==
            AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoffState.ParentSurfaceStatusReadModelProofRequired ||
          (row.inheritedParentSurfaceStatusProofRefs.length > 0 &&
            row.requiredParentSurfaceStatusReadModelProofRefs.length > 0) ||
          'Expected parent-surface status read-model proof-required rows to preserve status proof refs and name status read-model proof refs'
      )
    )
  );

const StatusReadModelHandoffBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  responseConsumerParentSurfaceStatusReadModelHandoffId:
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoffIdSchema,
  sourceResponseConsumerParentSurfaceStatusHandoffId:
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffIdSchema,
  generatedAt: ParentTimestampSchema,
  sourceContractRefs: Schema.Array(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoffContractRefSchema
  ),
  parentSurfaceStatusReadModelRef:
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelRefSchema,
  rows: Schema.Array(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoffRowSchema
  ),
  nativeAppRowCount: Schema.Number,
  nativeGameRowCount: Schema.Number,
  parentSurfaceStatusReadModelProofRequiredCount: Schema.Number,
  blockedBySourceFreshnessCount: Schema.Number,
  blockedByCompilerDecisionCount: Schema.Number,
  responseConsumerParentSurfaceStatusReadModelHandoffNonClaims: Schema.Array(StatusReadModelNonClaimSchema),
  ...Object.fromEntries(
    Object.keys(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoffNoClaimFlags
    ).map((key) => [key, Schema.Literal(false)])
  ),
});

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoffSchema =
  withParser(
    StatusReadModelHandoffBaseSchema.pipe(
      Schema.filter(
        (handoff) =>
          appGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoffCountsMatch(
            handoff
          ) || 'Expected response consumer parent-surface status read-model handoff counts to match row states'
      )
    ).pipe(
      Schema.filter(
        (handoff) =>
          appGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoffHasNoRuntimeClaims(
            handoff
          ) ||
          'Expected response consumer parent-surface status read-model handoff to avoid service, protocol, UI, timer, audit, rollback, adapter, and raw-source claims'
      )
    )
  );

export type AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoffOptions =
  Infer<
    typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoffOptionsSchema
  >;
export type AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoff =
  Infer<
    typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoffSchema
  >;

export function buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoff(
  optionsInput: unknown,
  parentSurfaceStatusHandoffInput: unknown
): AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoff {
  const options =
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoffOptionsSchema.parse(
      optionsInput
    );
  const parentSurfaceStatusHandoff =
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffSchema.parse(
      parentSurfaceStatusHandoffInput
    );
  const rows = parentSurfaceStatusHandoff.rows.map((row) => buildStatusReadModelRow(options, row));

  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoffSchema.parse(
    {
      schemaVersion: options.schemaVersion,
      responseConsumerParentSurfaceStatusReadModelHandoffId:
        options.responseConsumerParentSurfaceStatusReadModelHandoffId,
      sourceResponseConsumerParentSurfaceStatusHandoffId:
        parentSurfaceStatusHandoff.responseConsumerParentSurfaceStatusHandoffId,
      generatedAt: options.generatedAt,
      sourceContractRefs: options.sourceContractRefs,
      parentSurfaceStatusReadModelRef: options.parentSurfaceStatusReadModelRef,
      rows,
      nativeAppRowCount: parentSurfaceStatusHandoff.nativeAppRowCount,
      nativeGameRowCount: parentSurfaceStatusHandoff.nativeGameRowCount,
      parentSurfaceStatusReadModelProofRequiredCount: rows.filter(
        (row) =>
          row.responseConsumerParentSurfaceStatusReadModelHandoffState ===
          AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoffState.ParentSurfaceStatusReadModelProofRequired
      ).length,
      blockedBySourceFreshnessCount: rows.filter(
        (row) =>
          row.responseConsumerParentSurfaceStatusReadModelHandoffState ===
          AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoffState.BlockedBySourceFreshness
      ).length,
      blockedByCompilerDecisionCount: rows.filter(
        (row) =>
          row.responseConsumerParentSurfaceStatusReadModelHandoffState ===
          AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoffState.BlockedByCompilerDecision
      ).length,
      responseConsumerParentSurfaceStatusReadModelHandoffNonClaims:
        RequiredAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoffNonClaims,
      ...AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoffNoClaimFlags,
    }
  );
}

function buildStatusReadModelRow(
  options: AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoffOptions,
  statusRow: ParentSurfaceStatusHandoffRow
) {
  const state = statusReadModelStateForStatusHandoff(statusRow);
  const statusReadModelRequired =
    state ===
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoffState.ParentSurfaceStatusReadModelProofRequired;

  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoffRowSchema.parse(
    {
      schemaVersion: options.schemaVersion,
      rowId: `${statusRow.rowId}:parent-surface-status-read-model-handoff`,
      sourceResponseConsumerParentSurfaceStatusHandoffRowId: statusRow.rowId,
      targetDomain: statusRow.targetDomain,
      responseConsumerParentSurfaceStatusReadModelHandoffState: state,
      inheritedProtocolProofRefs: statusRow.inheritedProtocolProofRefs,
      inheritedAgentProtocolCommandRefs: statusRow.inheritedAgentProtocolCommandRefs,
      inheritedAgentProtocolEventRefs: statusRow.inheritedAgentProtocolEventRefs,
      inheritedServiceHandlerRefs: statusRow.inheritedServiceHandlerRefs,
      inheritedServiceReadApiProofRefs: statusRow.inheritedServiceReadApiProofRefs,
      inheritedReadApiResponseProofRefs: statusRow.inheritedReadApiResponseProofRefs,
      inheritedReadApiResponseConsumerProofRefs: statusRow.inheritedReadApiResponseConsumerProofRefs,
      inheritedParentSurfaceProofRefs: statusRow.inheritedParentSurfaceProofRefs,
      inheritedParentSurfaceReadModelProofRefs: statusRow.inheritedParentSurfaceReadModelProofRefs,
      inheritedParentSurfaceStatusProofRefs: statusRow.requiredParentSurfaceStatusProofRefs,
      requiredParentSurfaceStatusReadModelProofRefs: statusReadModelRequired
        ? options.parentSurfaceStatusReadModelProofRefs
        : [],
      inheritedServiceReadinessProofRefs: statusRow.inheritedServiceReadinessProofRefs,
      sourceEvidenceRefs: statusRow.sourceEvidenceRefs,
      serviceReadApiRef: statusRow.serviceReadApiRef,
      parentSurfaceReadModelRef: statusRow.parentSurfaceReadModelRef,
      parentSurfaceStatusRef: statusRow.parentSurfaceStatusRef,
      parentSurfaceStatusReadModelRef: options.parentSurfaceStatusReadModelRef,
      ...AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoffNoClaimFlags,
      generatedAt: options.generatedAt,
    }
  );
}

function statusReadModelStateForStatusHandoff(
  statusRow: ParentSurfaceStatusHandoffRow
): AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoffStateValue {
  for (const state of Object.values(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoffState
  )) {
    if (
      appGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoffMatchesStatusHandoff(
        statusRow.responseConsumerParentSurfaceStatusHandoffState,
        state
      )
    ) {
      return state;
    }
  }
  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoffState.BlockedByCompilerDecision;
}

export const decodeAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoff =
  Schema.decodeUnknownSync(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoffSchema
  );

export { AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoffState };

