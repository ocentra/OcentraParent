import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AppGamePolicyPreviewTargetDomainSchema } from './app-game-policy-preview-handoff';
import { AppGameSourceFreshnessEvidenceRefSchema } from './app-game-source-freshness-policy-consumption';
import {
  AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelIdSchema,
  AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelRowIdSchema,
  AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelSchema,
  AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelSummarySchema,
  type AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelRowSchema as ParentSurfaceReadModelRowSchema,
} from './app-game-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-read-model';
import {
  AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelProofRefSchema,
  AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelRefSchema,
} from './app-game-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-read-model-handoff';
import {
  AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffNoClaimFlags,
  AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffState,
  RequiredAppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffNonClaims,
  appGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffCountsMatch,
  appGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffHasNoRuntimeClaims,
  appGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffMatchesReadModel,
  type AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffStateValue,
} from './app-game-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-read-model-service-handoff-rules';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from './reference-primitives';

const ServiceHandoffText = Schema.String.pipe(Schema.minLength(1));

type ParentSurfaceReadModelRow = Infer<typeof ParentSurfaceReadModelRowSchema>;

export const AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffIdSchema =
  ServiceHandoffText.pipe(
    Schema.brand(
      'AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffId'
    )
  );
export const AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffRowIdSchema =
  ServiceHandoffText.pipe(
    Schema.brand(
      'AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffRowId'
    )
  );
export const AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffContractRefSchema =
  ServiceHandoffText.pipe(
    Schema.brand(
      'AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffContractRef'
    )
  );
export const AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceProofRefSchema =
  ServiceHandoffText.pipe(
    Schema.brand('AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceProofRef')
  );

const ServiceHandoffStateSchema = withParser(
  Schema.Literal(
    ...Object.values(
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffState
    )
  )
);
const ServiceHandoffNonClaimSchema = withParser(
  Schema.Literal(
    ...RequiredAppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffNonClaims
  )
);

export const AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffOptionsSchema =
  withParser(
    Schema.Struct({
      schemaVersion: ParentContractSchemaVersionSchema,
      parentSurfaceReadModelServiceHandoffId:
        AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffIdSchema,
      generatedAt: ParentTimestampSchema,
      sourceContractRefs: Schema.Array(
        AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffContractRefSchema
      ),
      serviceProofRefs: Schema.Array(
        AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceProofRefSchema
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
  rowId: AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffRowIdSchema,
  sourceParentSurfaceReadModelRowId:
    AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelRowIdSchema,
  targetDomain: AppGamePolicyPreviewTargetDomainSchema,
  parentSurfaceReadModelServiceHandoffState: ServiceHandoffStateSchema,
  parentSafeSummary: AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelSummarySchema,
  requiredParentSurfaceReadModelProofRefs: Schema.Array(
    AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelProofRefSchema
  ),
  requiredServiceProofRefs: Schema.Array(
    AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceProofRefSchema
  ),
  sourceEvidenceRefs: Schema.Array(AppGameSourceFreshnessEvidenceRefSchema),
  parentSurfaceReadModelRef:
    AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelRefSchema,
  ...Object.fromEntries(
    Object.keys(
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffNoClaimFlags
    ).map((key) => [key, Schema.Literal(false)])
  ),
  generatedAt: ParentTimestampSchema,
});

export const AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffRowSchema =
  withParser(
    ServiceHandoffRowBaseSchema.pipe(
      Schema.filter(
        (row) =>
          row.parentSurfaceReadModelServiceHandoffState !==
            AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffState.ServiceProofRequired ||
          row.requiredServiceProofRefs.length > 0 ||
          'Expected service-proof rows to cite required service proof refs'
      )
    )
  );

const ServiceHandoffBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  parentSurfaceReadModelServiceHandoffId:
    AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffIdSchema,
  sourceParentSurfaceReadModelId:
    AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelIdSchema,
  generatedAt: ParentTimestampSchema,
  sourceContractRefs: Schema.Array(
    AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffContractRefSchema
  ),
  rows: Schema.Array(
    AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffRowSchema
  ),
  nativeAppRowCount: Schema.Number,
  nativeGameRowCount: Schema.Number,
  serviceProofRequiredCount: Schema.Number,
  blockedBySourceFreshnessCount: Schema.Number,
  blockedByCompilerDecisionCount: Schema.Number,
  parentSurfaceReadModelServiceHandoffNonClaims: Schema.Array(ServiceHandoffNonClaimSchema),
  ...Object.fromEntries(
    Object.keys(
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffNoClaimFlags
    ).map((key) => [key, Schema.Literal(false)])
  ),
});

export const AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffSchema =
  withParser(
    ServiceHandoffBaseSchema.pipe(
      Schema.filter(
        (handoff) =>
          appGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffCountsMatch(
            handoff
          ) || 'Expected service handoff counts to match row states'
      )
    ).pipe(
      Schema.filter(
        (handoff) =>
          appGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffHasNoRuntimeClaims(
            handoff
          ) || 'Expected service handoff to keep runtime and rendering claims false'
      )
    )
  );

export type AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoff = Infer<
  typeof AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffSchema
>;

export function buildAppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoff(
  options: Infer<
    typeof AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffOptionsSchema
  >,
  readModel: Infer<typeof AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelSchema>
): AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoff {
  const parsedOptions =
    AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffOptionsSchema.parse(
      options
    );
  const parsedReadModel =
    AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelSchema.parse(readModel);
  const rows = parsedReadModel.rows.map((row, index) => buildServiceHandoffRow(parsedOptions, row, index));
  return AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffSchema.parse(
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
          AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffState.ServiceProofRequired
      ).length,
      blockedBySourceFreshnessCount: rows.filter(
        (row) =>
          row.parentSurfaceReadModelServiceHandoffState ===
          AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffState.BlockedBySourceFreshness
      ).length,
      blockedByCompilerDecisionCount: rows.filter(
        (row) =>
          row.parentSurfaceReadModelServiceHandoffState ===
          AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffState.BlockedByCompilerDecision
      ).length,
      parentSurfaceReadModelServiceHandoffNonClaims:
        RequiredAppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffNonClaims,
      ...AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffNoClaimFlags,
    }
  );
}

function buildServiceHandoffRow(
  options: Infer<
    typeof AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffOptionsSchema
  >,
  row: ParentSurfaceReadModelRow,
  index: number
): Infer<
  typeof AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffRowSchema
> {
  const serviceHandoffState = mapServiceHandoffState(row.parentSurfaceReadModelState);
  return AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffRowSchema.parse(
    {
      schemaVersion: options.schemaVersion,
      rowId: `${options.parentSurfaceReadModelServiceHandoffId}-row-${index + 1}`,
      sourceParentSurfaceReadModelRowId: row.rowId,
      targetDomain: row.targetDomain,
      parentSurfaceReadModelServiceHandoffState: serviceHandoffState,
      parentSafeSummary: row.parentSafeSummary,
      requiredParentSurfaceReadModelProofRefs: row.requiredParentSurfaceReadModelProofRefs,
      requiredServiceProofRefs:
        serviceHandoffState ===
        AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffState.ServiceProofRequired
          ? options.serviceProofRefs
          : [],
      sourceEvidenceRefs: row.sourceEvidenceRefs,
      parentSurfaceReadModelRef: row.parentSurfaceReadModelRef,
      ...AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffNoClaimFlags,
      generatedAt: options.generatedAt,
    }
  );
}

function mapServiceHandoffState(
  state: ParentSurfaceReadModelRow['parentSurfaceReadModelState']
): AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffStateValue {
  if (
    appGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffMatchesReadModel(
      state,
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffState.ServiceProofRequired
    )
  ) {
    return AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffState.ServiceProofRequired;
  }
  if (
    appGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffMatchesReadModel(
      state,
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffState.BlockedBySourceFreshness
    )
  ) {
    return AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffState.BlockedBySourceFreshness;
  }
  return AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffState.BlockedByCompilerDecision;
}
