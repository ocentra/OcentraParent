import {
  type Infer,
  Schema,
  withParser,
  NonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { AppGamePolicyPreviewTargetDomainSchema } from './app-game-policy-preview-handoff';
import { AppGameSourceFreshnessEvidenceRefSchema } from './app-game-source-freshness-policy-consumption';
import {
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffIdSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffRowIdSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelProofRefSchema,
  type AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffRowSchema as ServiceReadModelHandoffRowSchema,
} from './app-game-timer-service-read-model-handoff';
import {
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffNoClaimFlags,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffState,
  RequiredAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffNonClaims,
  appGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffCountsMatch,
  appGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffHasNoRuntimeClaims,
  appGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffMatchesServiceReadModelHandoff,
  type AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffStateValue,
} from './app-game-timer-service-event-handoff-rules';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';

type ServiceReadModelHandoffRow = Infer<typeof ServiceReadModelHandoffRowSchema>;

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffIdSchema =
  NonEmptyStringSchema.pipe(
    Schema.brand(
      'AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffId'
    )
  );
export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffRowIdSchema =
  NonEmptyStringSchema.pipe(
    Schema.brand(
      'AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffRowId'
    )
  );
export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventProofRefSchema =
  NonEmptyStringSchema.pipe(
    Schema.brand(
      'AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventProofRef'
    )
  );
export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffContractRefSchema =
  NonEmptyStringSchema.pipe(
    Schema.brand(
      'AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffContractRef'
    )
  );

const ServiceEventHandoffStateSchema = withParser(
  Schema.Literal(
    ...Object.values(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffState
    )
  )
);
const ServiceEventHandoffNonClaimSchema = withParser(
  Schema.Literal(
    ...RequiredAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffNonClaims
  )
);

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffOptionsSchema =
  withParser(
    Schema.Struct({
      schemaVersion: ParentContractSchemaVersionSchema,
      parentSurfaceReadModelServiceEventHandoffId:
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffIdSchema,
      generatedAt: ParentTimestampSchema,
      sourceContractRefs: Schema.Array(
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffContractRefSchema
      ),
      serviceEventProofRefs: Schema.Array(
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventProofRefSchema
      ),
    }).pipe(
      Schema.filter(
        (options) =>
          (options.sourceContractRefs.length > 0 && options.serviceEventProofRefs.length > 0) ||
          'Expected service event handoff options to cite source contracts and future event proof refs'
      )
    )
  );

const ServiceEventHandoffRowBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  rowId:
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffRowIdSchema,
  sourceParentSurfaceReadModelServiceReadModelHandoffRowId:
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffRowIdSchema,
  targetDomain: AppGamePolicyPreviewTargetDomainSchema,
  parentSurfaceReadModelServiceEventHandoffState: ServiceEventHandoffStateSchema,
  inheritedServiceReadModelProofRefs: Schema.Array(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelProofRefSchema
  ),
  requiredServiceEventProofRefs: Schema.Array(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventProofRefSchema
  ),
  sourceEvidenceRefs: Schema.Array(AppGameSourceFreshnessEvidenceRefSchema),
  ...Object.fromEntries(
    Object.keys(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffNoClaimFlags
    ).map((key) => [key, Schema.Literal(false)])
  ),
  generatedAt: ParentTimestampSchema,
});

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffRowSchema =
  withParser(
    ServiceEventHandoffRowBaseSchema.pipe(
      Schema.filter(
        (row) =>
          row.parentSurfaceReadModelServiceEventHandoffState !==
            AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffState.ServiceEventProofRequired ||
          row.requiredServiceEventProofRefs.length > 0 ||
          'Expected service event proof rows to cite required proof refs'
      )
    )
  );

const ServiceEventHandoffBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  parentSurfaceReadModelServiceEventHandoffId:
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffIdSchema,
  sourceParentSurfaceReadModelServiceReadModelHandoffId:
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffIdSchema,
  generatedAt: ParentTimestampSchema,
  sourceContractRefs: Schema.Array(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffContractRefSchema
  ),
  rows: Schema.Array(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffRowSchema
  ),
  nativeAppRowCount: Schema.Number,
  nativeGameRowCount: Schema.Number,
  serviceEventProofRequiredCount: Schema.Number,
  blockedBySourceFreshnessCount: Schema.Number,
  blockedByCompilerDecisionCount: Schema.Number,
  parentSurfaceReadModelServiceEventHandoffNonClaims: Schema.Array(ServiceEventHandoffNonClaimSchema),
  ...Object.fromEntries(
    Object.keys(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffNoClaimFlags
    ).map((key) => [key, Schema.Literal(false)])
  ),
});

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffSchema =
  withParser(
    ServiceEventHandoffBaseSchema.pipe(
      Schema.filter(
        (handoff) =>
          appGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffCountsMatch(
            handoff
          ) || 'Expected service event handoff counts to match row states'
      )
    ).pipe(
      Schema.filter(
        (handoff) =>
          appGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffHasNoRuntimeClaims(
            handoff
          ) || 'Expected service event handoff to keep runtime and rendering claims false'
      )
    )
  );

export function buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoff(
  options: Infer<
    typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffOptionsSchema
  >,
  serviceReadModelHandoff: Infer<
    typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffSchema
  >
): Infer<
  typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffSchema
> {
  const parsedOptions =
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffOptionsSchema.parse(
      options
    );
  const parsedServiceReadModelHandoff =
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffSchema.parse(
      serviceReadModelHandoff
    );
  const rows = parsedServiceReadModelHandoff.rows.map((row, index) =>
    buildServiceEventHandoffRow(parsedOptions, row, index)
  );
  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffSchema.parse(
    {
      schemaVersion: parsedOptions.schemaVersion,
      parentSurfaceReadModelServiceEventHandoffId: parsedOptions.parentSurfaceReadModelServiceEventHandoffId,
      sourceParentSurfaceReadModelServiceReadModelHandoffId:
        parsedServiceReadModelHandoff.parentSurfaceReadModelServiceReadModelHandoffId,
      generatedAt: parsedOptions.generatedAt,
      sourceContractRefs: parsedOptions.sourceContractRefs,
      rows,
      nativeAppRowCount: rows.filter((row) => row.targetDomain === 'native-app').length,
      nativeGameRowCount: rows.filter((row) => row.targetDomain === 'native-game').length,
      serviceEventProofRequiredCount: rows.filter(
        (row) =>
          row.parentSurfaceReadModelServiceEventHandoffState ===
          AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffState.ServiceEventProofRequired
      ).length,
      blockedBySourceFreshnessCount: rows.filter(
        (row) =>
          row.parentSurfaceReadModelServiceEventHandoffState ===
          AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffState.BlockedBySourceFreshness
      ).length,
      blockedByCompilerDecisionCount: rows.filter(
        (row) =>
          row.parentSurfaceReadModelServiceEventHandoffState ===
          AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffState.BlockedByCompilerDecision
      ).length,
      parentSurfaceReadModelServiceEventHandoffNonClaims:
        RequiredAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffNonClaims,
      ...AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffNoClaimFlags,
    }
  );
}

function buildServiceEventHandoffRow(
  options: Infer<
    typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffOptionsSchema
  >,
  row: ServiceReadModelHandoffRow,
  index: number
): Infer<
  typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffRowSchema
> {
  const serviceEventHandoffState = mapServiceEventHandoffState(row.parentSurfaceReadModelServiceReadModelHandoffState);
  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffRowSchema.parse(
    {
      schemaVersion: options.schemaVersion,
      rowId: `${options.parentSurfaceReadModelServiceEventHandoffId}-row-${index + 1}`,
      sourceParentSurfaceReadModelServiceReadModelHandoffRowId: row.rowId,
      targetDomain: row.targetDomain,
      parentSurfaceReadModelServiceEventHandoffState: serviceEventHandoffState,
      inheritedServiceReadModelProofRefs: row.requiredServiceReadModelProofRefs,
      requiredServiceEventProofRefs:
        serviceEventHandoffState ===
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffState.ServiceEventProofRequired
          ? options.serviceEventProofRefs
          : [],
      sourceEvidenceRefs: row.sourceEvidenceRefs,
      ...AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffNoClaimFlags,
      generatedAt: options.generatedAt,
    }
  );
}

function mapServiceEventHandoffState(
  state: ServiceReadModelHandoffRow['parentSurfaceReadModelServiceReadModelHandoffState']
): AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffStateValue {
  if (
    appGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffMatchesServiceReadModelHandoff(
      state,
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffState.ServiceEventProofRequired
    )
  ) {
    return AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffState.ServiceEventProofRequired;
  }
  if (
    appGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffMatchesServiceReadModelHandoff(
      state,
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffState.BlockedBySourceFreshness
    )
  ) {
    return AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffState.BlockedBySourceFreshness;
  }
  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffState.BlockedByCompilerDecision;
}

