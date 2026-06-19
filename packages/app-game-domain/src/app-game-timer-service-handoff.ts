import {
  type Infer,
  Schema,
  withParser,
  NonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { AppGamePolicyPreviewTargetDomainSchema } from './app-game-policy-preview-handoff';
import { AppGameSourceFreshnessEvidenceRefSchema } from './app-game-source-freshness-policy-consumption';
import {
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelIdSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelRowIdSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelSummarySchema,
  type AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelRowSchema as ParentSurfaceReadModelRowSchema,
} from './app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model';
import {
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceProofRefSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceRefSchema,
} from './app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-handoff';
import {
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffNoClaimFlags,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffState,
  RequiredAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffNonClaims,
  appGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffCountsMatch,
  appGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffHasNoRuntimeClaims,
  appGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffMatchesReadModel,
  type AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffStateValue,
} from './app-game-timer-service-handoff-rules';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';

type ParentSurfaceReadModelRow = Infer<typeof ParentSurfaceReadModelRowSchema>;

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffIdSchema =
  NonEmptyStringSchema.pipe(
    Schema.brand(
      'AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffId'
    )
  );
export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffRowIdSchema =
  NonEmptyStringSchema.pipe(
    Schema.brand(
      'AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffRowId'
    )
  );
export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffContractRefSchema =
  NonEmptyStringSchema.pipe(
    Schema.brand(
      'AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffContractRef'
    )
  );
export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceProofRefSchema =
  NonEmptyStringSchema.pipe(
    Schema.brand(
      'AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceProofRef'
    )
  );

const ServiceHandoffStateSchema = withParser(
  Schema.Literal(
    ...Object.values(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffState
    )
  )
);
const ServiceHandoffNonClaimSchema = withParser(
  Schema.Literal(
    ...RequiredAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffNonClaims
  )
);

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffOptionsSchema =
  withParser(
    Schema.Struct({
      schemaVersion: ParentContractSchemaVersionSchema,
      parentSurfaceReadModelServiceHandoffId:
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffIdSchema,
      generatedAt: ParentTimestampSchema,
      sourceContractRefs: Schema.Array(
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffContractRefSchema
      ),
      serviceProofRefs: Schema.Array(
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceProofRefSchema
      ),
    }).pipe(
      Schema.filter(
        (options) =>
          (options.sourceContractRefs.length > 0 && options.serviceProofRefs.length > 0) ||
          'Expected service handoff options to cite source contracts and future service proof refs'
      )
    )
  );

const ServiceHandoffRowBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  rowId:
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffRowIdSchema,
  sourceParentSurfaceReadModelRowId:
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelRowIdSchema,
  targetDomain: AppGamePolicyPreviewTargetDomainSchema,
  parentSurfaceReadModelServiceHandoffState: ServiceHandoffStateSchema,
  parentSafeSummary:
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelSummarySchema,
  inheritedParentSurfaceStatusReadModelProofRefs: Schema.Array(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceProofRefSchema
  ),
  requiredParentSurfaceReadModelProofRefs: Schema.Array(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceProofRefSchema
  ),
  requiredServiceProofRefs: Schema.Array(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceProofRefSchema
  ),
  sourceEvidenceRefs: Schema.Array(AppGameSourceFreshnessEvidenceRefSchema),
  parentSurfaceStatusReadModelRef:
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceRefSchema,
  parentSurfaceReadModelRef:
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceRefSchema,
  ...Object.fromEntries(
    Object.keys(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffNoClaimFlags
    ).map((key) => [key, Schema.Literal(false)])
  ),
  generatedAt: ParentTimestampSchema,
});

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffRowSchema =
  withParser(
    ServiceHandoffRowBaseSchema.pipe(
      Schema.filter(
        (row) =>
          row.parentSurfaceReadModelServiceHandoffState !==
            AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffState.ServiceProofRequired ||
          row.requiredServiceProofRefs.length > 0 ||
          'Expected service-proof rows to cite required service proof refs'
      )
    )
  );

const ServiceHandoffBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  parentSurfaceReadModelServiceHandoffId:
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffIdSchema,
  sourceParentSurfaceReadModelId:
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelIdSchema,
  generatedAt: ParentTimestampSchema,
  sourceContractRefs: Schema.Array(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffContractRefSchema
  ),
  rows: Schema.Array(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffRowSchema
  ),
  nativeAppRowCount: Schema.Number,
  nativeGameRowCount: Schema.Number,
  serviceProofRequiredCount: Schema.Number,
  blockedBySourceFreshnessCount: Schema.Number,
  blockedByCompilerDecisionCount: Schema.Number,
  parentSurfaceReadModelServiceHandoffNonClaims: Schema.Array(ServiceHandoffNonClaimSchema),
  ...Object.fromEntries(
    Object.keys(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffNoClaimFlags
    ).map((key) => [key, Schema.Literal(false)])
  ),
});

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffSchema =
  withParser(
    ServiceHandoffBaseSchema.pipe(
      Schema.filter(
        (handoff) =>
          appGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffCountsMatch(
            handoff
          ) || 'Expected service handoff counts to match row states'
      )
    ).pipe(
      Schema.filter(
        (handoff) =>
          appGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffHasNoRuntimeClaims(
            handoff
          ) || 'Expected service handoff to keep runtime and rendering claims false'
      )
    )
  );

export type AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoff =
  Infer<
    typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffSchema
  >;

export function buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoff(
  options: Infer<
    typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffOptionsSchema
  >,
  readModel: Infer<
    typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelSchema
  >
): AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoff {
  const parsedOptions =
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffOptionsSchema.parse(
      options
    );
  const parsedReadModel =
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelSchema.parse(
      readModel
    );
  const rows = parsedReadModel.rows.map((row, index) => buildServiceHandoffRow(parsedOptions, row, index));
  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffSchema.parse(
    {
      schemaVersion: parsedOptions.schemaVersion,
      parentSurfaceReadModelServiceHandoffId: parsedOptions.parentSurfaceReadModelServiceHandoffId,
      sourceParentSurfaceReadModelId: parsedReadModel.parentSurfaceReadModelId,
      generatedAt: parsedOptions.generatedAt,
      sourceContractRefs: parsedOptions.sourceContractRefs,
      rows,
      nativeAppRowCount: rows.filter((row) => row.targetDomain === 'native-app').length,
      nativeGameRowCount: rows.filter((row) => row.targetDomain === 'native-game').length,
      serviceProofRequiredCount: rows.filter(
        (row) =>
          row.parentSurfaceReadModelServiceHandoffState ===
          AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffState.ServiceProofRequired
      ).length,
      blockedBySourceFreshnessCount: rows.filter(
        (row) =>
          row.parentSurfaceReadModelServiceHandoffState ===
          AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffState.BlockedBySourceFreshness
      ).length,
      blockedByCompilerDecisionCount: rows.filter(
        (row) =>
          row.parentSurfaceReadModelServiceHandoffState ===
          AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffState.BlockedByCompilerDecision
      ).length,
      parentSurfaceReadModelServiceHandoffNonClaims:
        RequiredAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffNonClaims,
      ...AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffNoClaimFlags,
    }
  );
}

function buildServiceHandoffRow(
  options: Infer<
    typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffOptionsSchema
  >,
  row: ParentSurfaceReadModelRow,
  index: number
): Infer<
  typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffRowSchema
> {
  const serviceHandoffState = mapServiceHandoffState(row.parentSurfaceReadModelState);
  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffRowSchema.parse(
    {
      schemaVersion: options.schemaVersion,
      rowId: `${options.parentSurfaceReadModelServiceHandoffId}-row-${index + 1}`,
      sourceParentSurfaceReadModelRowId: row.rowId,
      targetDomain: row.targetDomain,
      parentSurfaceReadModelServiceHandoffState: serviceHandoffState,
      parentSafeSummary: row.parentSafeSummary,
      inheritedParentSurfaceStatusReadModelProofRefs: row.inheritedParentSurfaceStatusReadModelProofRefs,
      requiredParentSurfaceReadModelProofRefs: row.requiredParentSurfaceReadModelProofRefs,
      requiredServiceProofRefs:
        serviceHandoffState ===
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffState.ServiceProofRequired
          ? options.serviceProofRefs
          : [],
      sourceEvidenceRefs: row.sourceEvidenceRefs,
      parentSurfaceStatusReadModelRef: row.parentSurfaceStatusReadModelRef,
      parentSurfaceReadModelRef: row.parentSurfaceReadModelRef,
      ...AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffNoClaimFlags,
      generatedAt: options.generatedAt,
    }
  );
}

function mapServiceHandoffState(
  state: ParentSurfaceReadModelRow['parentSurfaceReadModelState']
): AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffStateValue {
  if (
    appGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffMatchesReadModel(
      state,
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffState.ServiceProofRequired
    )
  ) {
    return AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffState.ServiceProofRequired;
  }
  if (
    appGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffMatchesReadModel(
      state,
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffState.BlockedBySourceFreshness
    )
  ) {
    return AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffState.BlockedBySourceFreshness;
  }
  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffState.BlockedByCompilerDecision;
}

