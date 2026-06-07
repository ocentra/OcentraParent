import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AppGamePolicyPreviewTargetDomainSchema } from './app-game-policy-preview-handoff';
import { AppGameSourceFreshnessEvidenceRefSchema } from './app-game-source-freshness-policy-consumption';
import {
  AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffIdSchema,
  AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffRowIdSchema,
  AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffSchema,
  AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceProofRefSchema,
  type AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffRowSchema as ServiceHandoffRowSchema,
} from './app-game-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-read-model-service-handoff';
import {
  AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffNoClaimFlags,
  AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffState,
  RequiredAppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffNonClaims,
  appGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffCountsMatch,
  appGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffHasNoRuntimeClaims,
  appGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffMatchesServiceHandoff,
  type AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffStateValue,
} from './app-game-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-read-model-service-read-model-handoff-rules';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from './reference-primitives';

const ServiceReadModelHandoffText = Schema.String.pipe(Schema.minLength(1));

type ServiceHandoffRow = Infer<typeof ServiceHandoffRowSchema>;

export const AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffIdSchema =
  ServiceReadModelHandoffText.pipe(
    Schema.brand(
      'AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffId'
    )
  );
export const AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffRowIdSchema =
  ServiceReadModelHandoffText.pipe(
    Schema.brand(
      'AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffRowId'
    )
  );
export const AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelProofRefSchema =
  ServiceReadModelHandoffText.pipe(
    Schema.brand(
      'AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelProofRef'
    )
  );
export const AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffContractRefSchema =
  ServiceReadModelHandoffText.pipe(
    Schema.brand(
      'AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffContractRef'
    )
  );

const ServiceReadModelHandoffStateSchema = withParser(
  Schema.Literal(
    ...Object.values(
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffState
    )
  )
);
const ServiceReadModelHandoffNonClaimSchema = withParser(
  Schema.Literal(
    ...RequiredAppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffNonClaims
  )
);

export const AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffOptionsSchema =
  withParser(
    Schema.Struct({
      schemaVersion: ParentContractSchemaVersionSchema,
      parentSurfaceReadModelServiceReadModelHandoffId:
        AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffIdSchema,
      generatedAt: ParentTimestampSchema,
      sourceContractRefs: Schema.Array(
        AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffContractRefSchema
      ),
      serviceReadModelProofRefs: Schema.Array(
        AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelProofRefSchema
      ),
    }).pipe(
      Schema.filter(
        (options) =>
          (options.sourceContractRefs.length > 0 && options.serviceReadModelProofRefs.length > 0) ||
          'Expected service read-model handoff options to cite source contracts and future read-model proof refs'
      )
    )
  );

const ServiceReadModelHandoffRowBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  rowId:
    AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffRowIdSchema,
  sourceParentSurfaceReadModelServiceHandoffRowId:
    AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffRowIdSchema,
  targetDomain: AppGamePolicyPreviewTargetDomainSchema,
  parentSurfaceReadModelServiceReadModelHandoffState: ServiceReadModelHandoffStateSchema,
  inheritedServiceProofRefs: Schema.Array(
    AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceProofRefSchema
  ),
  requiredServiceReadModelProofRefs: Schema.Array(
    AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelProofRefSchema
  ),
  sourceEvidenceRefs: Schema.Array(AppGameSourceFreshnessEvidenceRefSchema),
  ...Object.fromEntries(
    Object.keys(
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffNoClaimFlags
    ).map((key) => [key, Schema.Literal(false)])
  ),
  generatedAt: ParentTimestampSchema,
});

export const AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffRowSchema =
  withParser(
    ServiceReadModelHandoffRowBaseSchema.pipe(
      Schema.filter(
        (row) =>
          row.parentSurfaceReadModelServiceReadModelHandoffState !==
            AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffState.ServiceReadModelProofRequired ||
          row.requiredServiceReadModelProofRefs.length > 0 ||
          'Expected service read-model proof rows to cite required proof refs'
      )
    )
  );

const ServiceReadModelHandoffBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  parentSurfaceReadModelServiceReadModelHandoffId:
    AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffIdSchema,
  sourceParentSurfaceReadModelServiceHandoffId:
    AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffIdSchema,
  generatedAt: ParentTimestampSchema,
  sourceContractRefs: Schema.Array(
    AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffContractRefSchema
  ),
  rows: Schema.Array(
    AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffRowSchema
  ),
  nativeAppRowCount: Schema.Number,
  nativeGameRowCount: Schema.Number,
  serviceReadModelProofRequiredCount: Schema.Number,
  blockedBySourceFreshnessCount: Schema.Number,
  blockedByCompilerDecisionCount: Schema.Number,
  parentSurfaceReadModelServiceReadModelHandoffNonClaims: Schema.Array(ServiceReadModelHandoffNonClaimSchema),
  ...Object.fromEntries(
    Object.keys(
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffNoClaimFlags
    ).map((key) => [key, Schema.Literal(false)])
  ),
});

export const AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffSchema =
  withParser(
    ServiceReadModelHandoffBaseSchema.pipe(
      Schema.filter(
        (handoff) =>
          appGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffCountsMatch(
            handoff
          ) || 'Expected service read-model handoff counts to match row states'
      )
    ).pipe(
      Schema.filter(
        (handoff) =>
          appGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffHasNoRuntimeClaims(
            handoff
          ) || 'Expected service read-model handoff to keep runtime and rendering claims false'
      )
    )
  );

export function buildAppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoff(
  options: Infer<
    typeof AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffOptionsSchema
  >,
  serviceHandoff: Infer<
    typeof AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffSchema
  >
): Infer<
  typeof AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffSchema
> {
  const parsedOptions =
    AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffOptionsSchema.parse(
      options
    );
  const parsedServiceHandoff =
    AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoffSchema.parse(
      serviceHandoff
    );
  const rows = parsedServiceHandoff.rows.map((row, index) =>
    buildServiceReadModelHandoffRow(parsedOptions, row, index)
  );
  return AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffSchema.parse(
    {
      schemaVersion: parsedOptions.schemaVersion,
      parentSurfaceReadModelServiceReadModelHandoffId: parsedOptions.parentSurfaceReadModelServiceReadModelHandoffId,
      sourceParentSurfaceReadModelServiceHandoffId: parsedServiceHandoff.parentSurfaceReadModelServiceHandoffId,
      generatedAt: parsedOptions.generatedAt,
      sourceContractRefs: parsedOptions.sourceContractRefs,
      rows,
      nativeAppRowCount: rows.filter((row) => row.targetDomain === 'native-app').length,
      nativeGameRowCount: rows.filter((row) => row.targetDomain === 'native-game').length,
      serviceReadModelProofRequiredCount: rows.filter(
        (row) =>
          row.parentSurfaceReadModelServiceReadModelHandoffState ===
          AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffState.ServiceReadModelProofRequired
      ).length,
      blockedBySourceFreshnessCount: rows.filter(
        (row) =>
          row.parentSurfaceReadModelServiceReadModelHandoffState ===
          AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffState.BlockedBySourceFreshness
      ).length,
      blockedByCompilerDecisionCount: rows.filter(
        (row) =>
          row.parentSurfaceReadModelServiceReadModelHandoffState ===
          AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffState.BlockedByCompilerDecision
      ).length,
      parentSurfaceReadModelServiceReadModelHandoffNonClaims:
        RequiredAppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffNonClaims,
      ...AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffNoClaimFlags,
    }
  );
}

function buildServiceReadModelHandoffRow(
  options: Infer<
    typeof AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffOptionsSchema
  >,
  row: ServiceHandoffRow,
  index: number
): Infer<
  typeof AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffRowSchema
> {
  const serviceReadModelHandoffState = mapServiceReadModelHandoffState(row.parentSurfaceReadModelServiceHandoffState);
  return AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffRowSchema.parse(
    {
      schemaVersion: options.schemaVersion,
      rowId: `${options.parentSurfaceReadModelServiceReadModelHandoffId}-row-${index + 1}`,
      sourceParentSurfaceReadModelServiceHandoffRowId: row.rowId,
      targetDomain: row.targetDomain,
      parentSurfaceReadModelServiceReadModelHandoffState: serviceReadModelHandoffState,
      inheritedServiceProofRefs: row.requiredServiceProofRefs,
      requiredServiceReadModelProofRefs:
        serviceReadModelHandoffState ===
        AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffState.ServiceReadModelProofRequired
          ? options.serviceReadModelProofRefs
          : [],
      sourceEvidenceRefs: row.sourceEvidenceRefs,
      ...AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffNoClaimFlags,
      generatedAt: options.generatedAt,
    }
  );
}

function mapServiceReadModelHandoffState(
  state: ServiceHandoffRow['parentSurfaceReadModelServiceHandoffState']
): AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffStateValue {
  if (
    appGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffMatchesServiceHandoff(
      state,
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffState.ServiceReadModelProofRequired
    )
  ) {
    return AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffState.ServiceReadModelProofRequired;
  }
  if (
    appGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffMatchesServiceHandoff(
      state,
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffState.BlockedBySourceFreshness
    )
  ) {
    return AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffState.BlockedBySourceFreshness;
  }
  return AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoffState.BlockedByCompilerDecision;
}
