import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AppGamePolicyPreviewTargetDomainSchema } from './app-game-policy-preview-handoff';
import { AppGameSourceFreshnessEvidenceRefSchema } from './app-game-source-freshness-policy-consumption';
import {
  AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffIdSchema,
  AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffRowIdSchema,
  AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffSchema,
  AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelProofRefSchema,
  type AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffRowSchema as StatusReadModelHandoffRowSchema,
} from './app-game-timer-service-response-consumer-parent-surface-status-read-model-handoff';
import {
  AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffNoClaimFlags,
  AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffState,
  RequiredAppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffNonClaims,
  appGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffCountsMatch,
  appGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffHasNoRuntimeClaims,
  appGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffMatchesStatusReadModelHandoff,
  type AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffStateValue,
} from './app-game-timer-service-response-consumer-parent-surface-status-read-model-parent-surface-handoff-rules';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from './reference-primitives';

const ParentSurfaceText = Schema.String.pipe(Schema.minLength(1));

type StatusReadModelHandoffRow = Infer<typeof StatusReadModelHandoffRowSchema>;

export const AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffIdSchema =
  ParentSurfaceText.pipe(
    Schema.brand('AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffId')
  );
export const AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffRowIdSchema =
  ParentSurfaceText.pipe(
    Schema.brand('AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffRowId')
  );
export const AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceProofRefSchema =
  ParentSurfaceText.pipe(
    Schema.brand('AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceProofRef')
  );
export const AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceRefSchema =
  ParentSurfaceText.pipe(
    Schema.brand('AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceRef')
  );
export const AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceContractRefSchema =
  ParentSurfaceText.pipe(
    Schema.brand('AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceContractRef')
  );

const ParentSurfaceHandoffStateSchema = withParser(
  Schema.Literal(
    ...Object.values(AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffState)
  )
);
const ParentSurfaceHandoffNonClaimSchema = withParser(
  Schema.Literal(
    ...RequiredAppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffNonClaims
  )
);

export const AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffOptionsSchema =
  withParser(
    Schema.Struct({
      schemaVersion: ParentContractSchemaVersionSchema,
      serviceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffId:
        AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffIdSchema,
      generatedAt: ParentTimestampSchema,
      sourceContractRefs: Schema.Array(
        AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceContractRefSchema
      ),
      parentSurfaceProofRefs: Schema.Array(
        AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceProofRefSchema
      ),
      parentSurfaceRef: AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceRefSchema,
    }).pipe(
      Schema.filter(
        (options) =>
          (options.sourceContractRefs.length > 0 && options.parentSurfaceProofRefs.length > 0) ||
          'Expected parent-surface handoff options to cite source contracts and parent-surface proof refs'
      )
    )
  );

const ParentSurfaceHandoffRowBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  rowId: AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffRowIdSchema,
  sourceServiceResponseConsumerParentSurfaceStatusReadModelHandoffRowId:
    AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffRowIdSchema,
  targetDomain: AppGamePolicyPreviewTargetDomainSchema,
  serviceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffState: ParentSurfaceHandoffStateSchema,
  inheritedParentSurfaceStatusReadModelProofRefs: Schema.Array(
    AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelProofRefSchema
  ),
  requiredParentSurfaceProofRefs: Schema.Array(
    AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceProofRefSchema
  ),
  sourceEvidenceRefs: Schema.Array(AppGameSourceFreshnessEvidenceRefSchema),
  parentSurfaceRef: AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceRefSchema,
  ...Object.fromEntries(
    Object.keys(AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffNoClaimFlags).map(
      (key) => [key, Schema.Literal(false)]
    )
  ),
  generatedAt: ParentTimestampSchema,
});

export const AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffRowSchema = withParser(
  ParentSurfaceHandoffRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        row.serviceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffState !==
          AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffState.ParentSurfaceProofRequired ||
        (row.inheritedParentSurfaceStatusReadModelProofRefs.length > 0 &&
          row.requiredParentSurfaceProofRefs.length > 0) ||
        'Expected parent-surface rows to preserve status read-model proof refs and cite parent-surface proof refs'
    )
  )
);

const ParentSurfaceHandoffBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  serviceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffId:
    AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffIdSchema,
  sourceServiceResponseConsumerParentSurfaceStatusReadModelHandoffId:
    AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffIdSchema,
  generatedAt: ParentTimestampSchema,
  sourceContractRefs: Schema.Array(
    AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceContractRefSchema
  ),
  parentSurfaceRef: AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceRefSchema,
  rows: Schema.Array(AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffRowSchema),
  nativeAppRowCount: Schema.Number,
  nativeGameRowCount: Schema.Number,
  parentSurfaceProofRequiredCount: Schema.Number,
  blockedBySourceFreshnessCount: Schema.Number,
  blockedByCompilerDecisionCount: Schema.Number,
  serviceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffNonClaims: Schema.Array(
    ParentSurfaceHandoffNonClaimSchema
  ),
  ...Object.fromEntries(
    Object.keys(AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffNoClaimFlags).map(
      (key) => [key, Schema.Literal(false)]
    )
  ),
});

export const AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffSchema = withParser(
  ParentSurfaceHandoffBaseSchema.pipe(
    Schema.filter(
      (handoff) =>
        appGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffCountsMatch(handoff) ||
        'Expected parent-surface handoff counts to match row states'
    )
  ).pipe(
    Schema.filter(
      (handoff) =>
        appGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffHasNoRuntimeClaims(
          handoff
        ) ||
        'Expected parent-surface handoff to keep runtime, persistence, rendering, adapter, child, platform, and raw-source claims false'
    )
  )
);

export function buildAppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoff(
  options: Infer<
    typeof AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffOptionsSchema
  >,
  statusReadModelHandoff: Infer<typeof AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffSchema>
): Infer<typeof AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffSchema> {
  const parsedOptions =
    AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffOptionsSchema.parse(options);
  const parsedStatusReadModelHandoff =
    AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelHandoffSchema.parse(statusReadModelHandoff);
  const rows = parsedStatusReadModelHandoff.rows.map((row, index) =>
    buildParentSurfaceHandoffRow(parsedOptions, row, index)
  );

  return AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffSchema.parse({
    schemaVersion: parsedOptions.schemaVersion,
    serviceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffId:
      parsedOptions.serviceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffId,
    sourceServiceResponseConsumerParentSurfaceStatusReadModelHandoffId:
      parsedStatusReadModelHandoff.serviceResponseConsumerParentSurfaceStatusReadModelHandoffId,
    generatedAt: parsedOptions.generatedAt,
    sourceContractRefs: parsedOptions.sourceContractRefs,
    parentSurfaceRef: parsedOptions.parentSurfaceRef,
    rows,
    nativeAppRowCount: rows.filter((row) => row.targetDomain === 'native-app').length,
    nativeGameRowCount: rows.filter((row) => row.targetDomain === 'native-game').length,
    parentSurfaceProofRequiredCount: rows.filter(
      (row) =>
        row.serviceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffState ===
        AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffState.ParentSurfaceProofRequired
    ).length,
    blockedBySourceFreshnessCount: rows.filter(
      (row) =>
        row.serviceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffState ===
        AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffState.BlockedBySourceFreshness
    ).length,
    blockedByCompilerDecisionCount: rows.filter(
      (row) =>
        row.serviceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffState ===
        AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffState.BlockedByCompilerDecision
    ).length,
    serviceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffNonClaims:
      RequiredAppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffNonClaims,
    ...AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffNoClaimFlags,
  });
}

function buildParentSurfaceHandoffRow(
  options: Infer<
    typeof AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffOptionsSchema
  >,
  row: StatusReadModelHandoffRow,
  index: number
): Infer<typeof AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffRowSchema> {
  const parentSurfaceHandoffState = mapParentSurfaceHandoffState(
    row.serviceResponseConsumerParentSurfaceStatusReadModelHandoffState
  );
  return AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffRowSchema.parse({
    schemaVersion: options.schemaVersion,
    rowId: `${options.serviceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffId}-row-${index + 1}`,
    sourceServiceResponseConsumerParentSurfaceStatusReadModelHandoffRowId: row.rowId,
    targetDomain: row.targetDomain,
    serviceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffState: parentSurfaceHandoffState,
    inheritedParentSurfaceStatusReadModelProofRefs: row.requiredParentSurfaceStatusReadModelProofRefs,
    requiredParentSurfaceProofRefs:
      parentSurfaceHandoffState ===
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffState.ParentSurfaceProofRequired
        ? options.parentSurfaceProofRefs
        : [],
    sourceEvidenceRefs: row.sourceEvidenceRefs,
    parentSurfaceRef: options.parentSurfaceRef,
    ...AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffNoClaimFlags,
    generatedAt: options.generatedAt,
  });
}

function mapParentSurfaceHandoffState(
  state: StatusReadModelHandoffRow['serviceResponseConsumerParentSurfaceStatusReadModelHandoffState']
): AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffStateValue {
  if (
    appGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffMatchesStatusReadModelHandoff(
      state,
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffState.ParentSurfaceProofRequired
    )
  ) {
    return AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffState.ParentSurfaceProofRequired;
  }
  if (
    appGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffMatchesStatusReadModelHandoff(
      state,
      AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffState.BlockedBySourceFreshness
    )
  ) {
    return AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffState.BlockedBySourceFreshness;
  }
  return AppGameTimerServiceResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffState.BlockedByCompilerDecision;
}
