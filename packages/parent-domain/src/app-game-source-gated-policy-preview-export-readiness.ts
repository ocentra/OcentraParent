import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  AppGameSourceGatedPolicyPreviewReadModelIdSchema,
  AppGameSourceGatedPolicyPreviewReadModelSchema,
  type AppGameSourceGatedPolicyPreviewReadModel,
} from './app-game-source-gated-policy-preview-read-model';
import {
  AppGameSourceGatedPolicyPreviewExportManifestState,
  AppGameSourceGatedPolicyPreviewExportNoClaimFlags,
  AppGameSourceGatedPolicyPreviewExportReadinessState,
  AppGameSourceGatedPolicyPreviewExportSubpath,
  RequiredAppGameSourceGatedPolicyPreviewExportNonClaims,
  RequiredAppGameSourceGatedPolicyPreviewExportSymbols,
  appGameSourceGatedPolicyPreviewExportReadinessCountsMatch,
  appGameSourceGatedPolicyPreviewExportReadinessHasNoRuntimeClaims,
  appGameSourceGatedPolicyPreviewExportReadinessHasRequiredSurface,
  appGameSourceGatedPolicyPreviewProjectionStatesReadyForExport,
} from './app-game-source-gated-policy-preview-export-readiness-rules';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from './reference-primitives';

const SourceGatedPolicyPreviewExportText = Schema.String.pipe(Schema.minLength(1));

export const AppGameSourceGatedPolicyPreviewExportReadinessIdSchema = SourceGatedPolicyPreviewExportText.pipe(
  Schema.brand('AppGameSourceGatedPolicyPreviewExportReadinessId')
);
export const AppGameSourceGatedPolicyPreviewExportContractRefSchema = SourceGatedPolicyPreviewExportText.pipe(
  Schema.brand('AppGameSourceGatedPolicyPreviewExportContractRef')
);
export const AppGameSourceGatedPolicyPreviewExportSubpathSchema = SourceGatedPolicyPreviewExportText.pipe(
  Schema.brand('AppGameSourceGatedPolicyPreviewExportSubpath')
);
export const AppGameSourceGatedPolicyPreviewExportSymbolRefSchema = SourceGatedPolicyPreviewExportText.pipe(
  Schema.brand('AppGameSourceGatedPolicyPreviewExportSymbolRef')
);

export const AppGameSourceGatedPolicyPreviewExportReadinessStateSchema = withParser(
  Schema.Literal(...Object.values(AppGameSourceGatedPolicyPreviewExportReadinessState))
);
export const AppGameSourceGatedPolicyPreviewExportManifestStateSchema = withParser(
  Schema.Literal(...Object.values(AppGameSourceGatedPolicyPreviewExportManifestState))
);
export const AppGameSourceGatedPolicyPreviewExportNonClaimSchema = withParser(
  Schema.Literal(...RequiredAppGameSourceGatedPolicyPreviewExportNonClaims)
);

export const AppGameSourceGatedPolicyPreviewExportReadinessOptionsSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    readinessId: AppGameSourceGatedPolicyPreviewExportReadinessIdSchema,
    generatedAt: ParentTimestampSchema,
    sourceContractRefs: Schema.Array(AppGameSourceGatedPolicyPreviewExportContractRefSchema),
  }).pipe(
    Schema.filter(
      (options) =>
        options.sourceContractRefs.length > 0 ||
        'Expected source-gated policy preview export readiness options to cite source contracts'
    )
  )
);

const AppGameSourceGatedPolicyPreviewExportReadinessBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  readinessId: AppGameSourceGatedPolicyPreviewExportReadinessIdSchema,
  sourceReadModelId: AppGameSourceGatedPolicyPreviewReadModelIdSchema,
  generatedAt: ParentTimestampSchema,
  sourceContractRefs: Schema.Array(AppGameSourceGatedPolicyPreviewExportContractRefSchema),
  requiredExportSubpath: AppGameSourceGatedPolicyPreviewExportSubpathSchema,
  requiredExportSymbols: Schema.Array(AppGameSourceGatedPolicyPreviewExportSymbolRefSchema),
  readinessState: AppGameSourceGatedPolicyPreviewExportReadinessStateSchema,
  manifestState: AppGameSourceGatedPolicyPreviewExportManifestStateSchema,
  nativeAppRowCount: Schema.Number,
  nativeGameRowCount: Schema.Number,
  previewReadyVisibleCount: Schema.Number,
  sourceManualRequiredVisibleCount: Schema.Number,
  compilerManualRequiredVisibleCount: Schema.Number,
  projectionStates: Schema.Array(SourceGatedPolicyPreviewExportText),
  exportNonClaims: Schema.Array(AppGameSourceGatedPolicyPreviewExportNonClaimSchema),
  packageManifestUpdated: Schema.Literal(false),
  serviceRuntimeEventClaimed: Schema.Literal(false),
  portalUiRendered: Schema.Literal(false),
  policyEvaluatorRuntimeClaimed: Schema.Literal(false),
  timerRuntimeClaimed: Schema.Literal(false),
  adapterDispatchClaimed: Schema.Literal(false),
  childDeliveryClaimed: Schema.Literal(false),
  platformEnforcementClaimed: Schema.Literal(false),
  rawPrivateSourceRowsIncluded: Schema.Literal(false),
});

export const AppGameSourceGatedPolicyPreviewExportReadinessSchema = withParser(
  AppGameSourceGatedPolicyPreviewExportReadinessBaseSchema.pipe(
    Schema.filter(
      (readiness) =>
        appGameSourceGatedPolicyPreviewExportReadinessCountsMatch(readiness) ||
        'Expected source-gated policy preview export readiness counts to include app, game, preview-ready, source-manual, and compiler-manual rows'
    )
  )
    .pipe(
      Schema.filter(
        (readiness) =>
          appGameSourceGatedPolicyPreviewExportReadinessHasRequiredSurface(readiness) ||
          'Expected source-gated policy preview export readiness to name the required package subpath and public symbols'
      )
    )
    .pipe(
      Schema.filter(
        (readiness) =>
          appGameSourceGatedPolicyPreviewProjectionStatesReadyForExport(readiness.projectionStates) ||
          'Expected source-gated policy preview export readiness to preserve all WP76 projection states'
      )
    )
    .pipe(
      Schema.filter(
        (readiness) =>
          appGameSourceGatedPolicyPreviewExportReadinessHasNoRuntimeClaims(readiness) ||
          'Expected source-gated policy preview export readiness to avoid manifest, runtime, UI, adapter, and raw-source claims'
      )
    )
);

export type AppGameSourceGatedPolicyPreviewExportReadinessOptions = Infer<
  typeof AppGameSourceGatedPolicyPreviewExportReadinessOptionsSchema
>;
export type AppGameSourceGatedPolicyPreviewExportReadiness = Infer<
  typeof AppGameSourceGatedPolicyPreviewExportReadinessSchema
>;

export function buildAppGameSourceGatedPolicyPreviewExportReadiness(
  optionsInput: unknown,
  readModelInput: unknown
): AppGameSourceGatedPolicyPreviewExportReadiness {
  const options = AppGameSourceGatedPolicyPreviewExportReadinessOptionsSchema.parse(optionsInput);
  const readModel = AppGameSourceGatedPolicyPreviewReadModelSchema.parse(readModelInput);

  return AppGameSourceGatedPolicyPreviewExportReadinessSchema.parse({
    schemaVersion: options.schemaVersion,
    readinessId: options.readinessId,
    sourceReadModelId: readModel.readModelId,
    generatedAt: options.generatedAt,
    sourceContractRefs: options.sourceContractRefs,
    requiredExportSubpath: AppGameSourceGatedPolicyPreviewExportSubpath,
    requiredExportSymbols: RequiredAppGameSourceGatedPolicyPreviewExportSymbols,
    readinessState: AppGameSourceGatedPolicyPreviewExportReadinessState.ReadyForManifestSequencing,
    manifestState: AppGameSourceGatedPolicyPreviewExportManifestState.DeferredByPackageManifestLock,
    nativeAppRowCount: readModel.nativeAppRowCount,
    nativeGameRowCount: readModel.nativeGameRowCount,
    previewReadyVisibleCount: readModel.previewReadyVisibleCount,
    sourceManualRequiredVisibleCount: readModel.sourceManualRequiredVisibleCount,
    compilerManualRequiredVisibleCount: readModel.compilerManualRequiredVisibleCount,
    projectionStates: uniqueProjectionStates(readModel),
    exportNonClaims: RequiredAppGameSourceGatedPolicyPreviewExportNonClaims,
    ...AppGameSourceGatedPolicyPreviewExportNoClaimFlags,
  });
}

export const decodeAppGameSourceGatedPolicyPreviewExportReadiness = Schema.decodeUnknownSync(
  AppGameSourceGatedPolicyPreviewExportReadinessSchema
);

function uniqueProjectionStates(readModel: AppGameSourceGatedPolicyPreviewReadModel) {
  return [...new Set(readModel.rows.map((row) => row.projectionState))];
}

export { AppGameSourceGatedPolicyPreviewExportManifestState, AppGameSourceGatedPolicyPreviewExportReadinessState };
