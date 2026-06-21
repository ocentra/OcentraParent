import {
  type Infer,
  Schema,
  withParser,
  NonEmptyStringSchema
} from './effect';
import { AppGamePolicyPreviewTargetDomainSchema } from './app-game-policy-preview-handoff';
import { AppGameSourceFreshnessEvidenceRefSchema } from './app-game-source-freshness-policy-consumption';
import {
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffIdSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffRowIdSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffSchema,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventProofRefSchema,
  type AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffRowSchema as ServiceEventHandoffRowSchema,
} from './app-game-timer-service-event-handoff';
import {
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoffNoClaimFlags,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoffState,
  RequiredAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoffNonClaims,
  appGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoffCountsMatch,
  appGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoffHasNoRuntimeClaims,
  appGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoffMatchesServiceEventHandoff,
  type AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoffStateValue,
} from './app-game-timer-service-read-api-handoff-rules';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from './family-reference-primitives';

type ServiceEventHandoffRow = Infer<typeof ServiceEventHandoffRowSchema>;

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoffIdSchema =
  NonEmptyStringSchema.pipe(
    Schema.brand(
      'AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoffId'
    )
  );
export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoffRowIdSchema =
  NonEmptyStringSchema.pipe(
    Schema.brand(
      'AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoffRowId'
    )
  );
export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiProofRefSchema =
  NonEmptyStringSchema.pipe(
    Schema.brand(
      'AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiProofRef'
    )
  );
export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoffContractRefSchema =
  NonEmptyStringSchema.pipe(
    Schema.brand(
      'AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoffContractRef'
    )
  );

const ServiceReadApiHandoffStateSchema = withParser(
  Schema.Literal(
    ...Object.values(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoffState
    )
  )
);
const ServiceReadApiHandoffNonClaimSchema = withParser(
  Schema.Literal(
    ...RequiredAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoffNonClaims
  )
);

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoffOptionsSchema =
  withParser(
    Schema.Struct({
      schemaVersion: ParentContractSchemaVersionSchema,
      parentSurfaceReadModelServiceReadApiHandoffId:
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoffIdSchema,
      generatedAt: ParentTimestampSchema,
      sourceContractRefs: Schema.Array(
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoffContractRefSchema
      ),
      serviceReadApiProofRefs: Schema.Array(
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiProofRefSchema
      ),
    }).pipe(
      Schema.filter(
        (options) =>
          (options.sourceContractRefs.length > 0 && options.serviceReadApiProofRefs.length > 0) ||
          'Expected service read API handoff options to cite source contracts and future read API proof refs'
      )
    )
  );

const ServiceReadApiHandoffRowBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  rowId:
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoffRowIdSchema,
  sourceParentSurfaceReadModelServiceEventHandoffRowId:
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffRowIdSchema,
  targetDomain: AppGamePolicyPreviewTargetDomainSchema,
  parentSurfaceReadModelServiceReadApiHandoffState: ServiceReadApiHandoffStateSchema,
  inheritedServiceEventProofRefs: Schema.Array(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventProofRefSchema
  ),
  requiredServiceReadApiProofRefs: Schema.Array(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiProofRefSchema
  ),
  sourceEvidenceRefs: Schema.Array(AppGameSourceFreshnessEvidenceRefSchema),
  ...Object.fromEntries(
    Object.keys(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoffNoClaimFlags
    ).map((key) => [key, Schema.Literal(false)])
  ),
  generatedAt: ParentTimestampSchema,
});

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoffRowSchema =
  withParser(
    ServiceReadApiHandoffRowBaseSchema.pipe(
      Schema.filter(
        (row) =>
          row.parentSurfaceReadModelServiceReadApiHandoffState !==
            AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoffState.ServiceReadApiProofRequired ||
          row.requiredServiceReadApiProofRefs.length > 0 ||
          'Expected service read API proof rows to cite required proof refs'
      )
    )
  );

const ServiceReadApiHandoffBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  parentSurfaceReadModelServiceReadApiHandoffId:
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoffIdSchema,
  sourceParentSurfaceReadModelServiceEventHandoffId:
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffIdSchema,
  generatedAt: ParentTimestampSchema,
  sourceContractRefs: Schema.Array(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoffContractRefSchema
  ),
  rows: Schema.Array(
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoffRowSchema
  ),
  nativeAppRowCount: Schema.Number,
  nativeGameRowCount: Schema.Number,
  serviceReadApiProofRequiredCount: Schema.Number,
  blockedBySourceFreshnessCount: Schema.Number,
  blockedByCompilerDecisionCount: Schema.Number,
  parentSurfaceReadModelServiceReadApiHandoffNonClaims: Schema.Array(ServiceReadApiHandoffNonClaimSchema),
  ...Object.fromEntries(
    Object.keys(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoffNoClaimFlags
    ).map((key) => [key, Schema.Literal(false)])
  ),
});

export const AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoffSchema =
  withParser(
    ServiceReadApiHandoffBaseSchema.pipe(
      Schema.filter(
        (handoff) =>
          appGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoffCountsMatch(
            handoff
          ) || 'Expected service read API handoff counts to match row states'
      )
    ).pipe(
      Schema.filter(
        (handoff) =>
          appGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoffHasNoRuntimeClaims(
            handoff
          ) || 'Expected service read API handoff to keep runtime and rendering claims false'
      )
    )
  );

export function buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoff(
  options: Infer<
    typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoffOptionsSchema
  >,
  serviceEventHandoff: Infer<
    typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffSchema
  >
): Infer<
  typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoffSchema
> {
  const parsedOptions =
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoffOptionsSchema.parse(
      options
    );
  const parsedServiceEventHandoff =
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoffSchema.parse(
      serviceEventHandoff
    );
  const rows = parsedServiceEventHandoff.rows.map((row, index) =>
    buildServiceReadApiHandoffRow(parsedOptions, row, index)
  );
  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoffSchema.parse(
    {
      schemaVersion: parsedOptions.schemaVersion,
      parentSurfaceReadModelServiceReadApiHandoffId: parsedOptions.parentSurfaceReadModelServiceReadApiHandoffId,
      sourceParentSurfaceReadModelServiceEventHandoffId:
        parsedServiceEventHandoff.parentSurfaceReadModelServiceEventHandoffId,
      generatedAt: parsedOptions.generatedAt,
      sourceContractRefs: parsedOptions.sourceContractRefs,
      rows,
      nativeAppRowCount: rows.filter((row) => row.targetDomain === 'native-app').length,
      nativeGameRowCount: rows.filter((row) => row.targetDomain === 'native-game').length,
      serviceReadApiProofRequiredCount: rows.filter(
        (row) =>
          row.parentSurfaceReadModelServiceReadApiHandoffState ===
          AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoffState.ServiceReadApiProofRequired
      ).length,
      blockedBySourceFreshnessCount: rows.filter(
        (row) =>
          row.parentSurfaceReadModelServiceReadApiHandoffState ===
          AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoffState.BlockedBySourceFreshness
      ).length,
      blockedByCompilerDecisionCount: rows.filter(
        (row) =>
          row.parentSurfaceReadModelServiceReadApiHandoffState ===
          AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoffState.BlockedByCompilerDecision
      ).length,
      parentSurfaceReadModelServiceReadApiHandoffNonClaims:
        RequiredAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoffNonClaims,
      ...AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoffNoClaimFlags,
    }
  );
}

function buildServiceReadApiHandoffRow(
  options: Infer<
    typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoffOptionsSchema
  >,
  row: ServiceEventHandoffRow,
  index: number
): Infer<
  typeof AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoffRowSchema
> {
  const serviceReadApiHandoffState = mapServiceReadApiHandoffState(row.parentSurfaceReadModelServiceEventHandoffState);
  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoffRowSchema.parse(
    {
      schemaVersion: options.schemaVersion,
      rowId: `${options.parentSurfaceReadModelServiceReadApiHandoffId}-row-${index + 1}`,
      sourceParentSurfaceReadModelServiceEventHandoffRowId: row.rowId,
      targetDomain: row.targetDomain,
      parentSurfaceReadModelServiceReadApiHandoffState: serviceReadApiHandoffState,
      inheritedServiceEventProofRefs: row.requiredServiceEventProofRefs,
      requiredServiceReadApiProofRefs:
        serviceReadApiHandoffState ===
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoffState.ServiceReadApiProofRequired
          ? options.serviceReadApiProofRefs
          : [],
      sourceEvidenceRefs: row.sourceEvidenceRefs,
      ...AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoffNoClaimFlags,
      generatedAt: options.generatedAt,
    }
  );
}

function mapServiceReadApiHandoffState(
  state: ServiceEventHandoffRow['parentSurfaceReadModelServiceEventHandoffState']
): AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoffStateValue {
  if (
    appGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoffMatchesServiceEventHandoff(
      state,
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoffState.ServiceReadApiProofRequired
    )
  ) {
    return AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoffState.ServiceReadApiProofRequired;
  }
  if (
    appGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoffMatchesServiceEventHandoff(
      state,
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoffState.BlockedBySourceFreshness
    )
  ) {
    return AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoffState.BlockedBySourceFreshness;
  }
  return AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoffState.BlockedByCompilerDecision;
}

