import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { PolicyActionSchema, PolicyDecisionIdSchema } from '@ocentra-parent/policy-domain/policy';
import { ParentContractSchemaVersionSchema, ParentPlatformSchema, ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';
import { ParentEvidenceReferenceSchema } from '@ocentra-parent/family-domain/references';
import {
  V08SupportedAdapterAuditReferenceStateSchema,
  V08SupportedAdapterCapabilitySchema,
  V08SupportedAdapterPlatformSupportStateSchema,
  V08SupportedAdapterRefusalReasonSchema,
  V08SupportedAdapterResultSchema,
  V08SupportedAdapterRollbackReferenceStateSchema,
  V08SupportedAdapterRuntimeBoundarySchema,
  V08SupportedAdapterRuntimeStateSchema,
  V08SupportedAdapterTargetIdentityStateSchema,
} from '@ocentra-parent/enforcement-domain/v0-8-supported-adapter-runtime-proof';

export const ScreenAiAdapterReadinessReadModelIdSchema = brandedNonEmptyStringSchema('ScreenAiAdapterReadinessReadModelId');
export const ScreenAiAdapterReadinessRowIdSchema = brandedNonEmptyStringSchema('ScreenAiAdapterReadinessRowId');
export const ScreenAiAdapterReadinessArtifactRefSchema = brandedNonEmptyStringSchema('ScreenAiAdapterReadinessArtifactRef');
export const ScreenAiAdapterReadinessRequirementSchema = brandedNonEmptyStringSchema('ScreenAiAdapterReadinessRequirement');
export const ScreenAiAdapterReadinessBoundarySchema = brandedNonEmptyStringSchema('ScreenAiAdapterReadinessBoundary');
export const ScreenAiAdapterCompletionResultRefSchema = brandedNonEmptyStringSchema('ScreenAiAdapterCompletionResultRef');

export const ScreenAiAdapterReadinessRuntimeBoundarySchema = withParser(
  Schema.Union(V08SupportedAdapterRuntimeBoundarySchema, Schema.Literal('windows-screen-owned-process-block'))
);

export const ScreenAiAdapterReadinessCapabilitySchema = withParser(
  Schema.Union(V08SupportedAdapterCapabilitySchema, Schema.Literal('screen-owned-process-block'))
);

export const ScreenAiAdapterReadinessRollbackStateSchema = withParser(
  Schema.Union(V08SupportedAdapterRollbackReferenceStateSchema, Schema.Literal('not-required'))
);

export const ScreenAiAdapterReadinessStateSchema = withParser(
  Schema.Literal(
    'real-owned-process-action-proved',
    'manual-required',
    'not-claimed',
    'unavailable',
    'unsupported',
    'degraded'
  )
);

export const ScreenAiAdapterReadinessActionExecutionStateSchema = withParser(Schema.Literal('executed', 'skipped'));

export const ScreenAiAdapterReadinessClaimFlagsSchema = withParser(
  Schema.Struct({
    broadInstalledAppBlockingClaimed: Schema.Boolean,
    networkDomainBlockingClaimed: Schema.Boolean,
    exactActiveTabEnforcementClaimed: Schema.Boolean,
    notificationDeliveryClaimed: Schema.Boolean,
    tamperHardeningClaimed: Schema.Boolean,
    mobileControlClaimed: Schema.Boolean,
    unsupportedPlatformBehaviorClaimed: Schema.Boolean,
  }).pipe(
    Schema.filter(
      (flags) =>
        Object.values(flags).every((flag) => flag === false) ||
        'Expected screen AI adapter readiness proof to reject broad/browser/network/mobile claim upgrades'
    )
  )
);

const ScreenAiAdapterReadinessRowBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  rowId: ScreenAiAdapterReadinessRowIdSchema,
  sourcePolicyDecisionId: PolicyDecisionIdSchema,
  sourcePolicyAction: PolicyActionSchema,
  sourcePolicyDryRun: Schema.Boolean,
  sourceProofArtifact: ScreenAiAdapterReadinessArtifactRefSchema,
  sourceEvidenceReferences: Schema.Array(ParentEvidenceReferenceSchema),
  sourceImageDeletionState: Schema.Literal('deleted'),
  rawImageRetained: Schema.Boolean,
  rawImageDeletedBeforeAdapter: Schema.Boolean,
  readinessState: ScreenAiAdapterReadinessStateSchema,
  actionExecutionState: ScreenAiAdapterReadinessActionExecutionStateSchema,
  adapterRuntimeBoundary: ScreenAiAdapterReadinessRuntimeBoundarySchema,
  adapterCapability: ScreenAiAdapterReadinessCapabilitySchema,
  adapterRuntimeState: V08SupportedAdapterRuntimeStateSchema,
  adapterResult: V08SupportedAdapterResultSchema,
  platform: ParentPlatformSchema,
  platformSupportState: V08SupportedAdapterPlatformSupportStateSchema,
  targetIdentityState: V08SupportedAdapterTargetIdentityStateSchema,
  rollbackReferenceState: ScreenAiAdapterReadinessRollbackStateSchema,
  auditReferenceState: V08SupportedAdapterAuditReferenceStateSchema,
  refusalReason: V08SupportedAdapterRefusalReasonSchema,
  adapterExecutionProofArtifact: Schema.Union(ScreenAiAdapterReadinessArtifactRefSchema, Schema.Null),
  linkedProofArtifacts: Schema.Array(ScreenAiAdapterReadinessArtifactRefSchema),
  manualProofRequirements: Schema.Array(ScreenAiAdapterReadinessRequirementSchema),
  claimFlags: ScreenAiAdapterReadinessClaimFlagsSchema,
  claimBoundary: ScreenAiAdapterReadinessBoundarySchema,
  fallbackBehavior: ScreenAiAdapterReadinessBoundarySchema,
});

type ScreenAiAdapterReadinessRowCandidate = Infer<typeof ScreenAiAdapterReadinessRowBaseSchema>;

export const ScreenAiAdapterReadinessRowSchema = withParser(
  ScreenAiAdapterReadinessRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        screenAiAdapterReadinessRowIsHonest(row) ||
        'Expected screen AI adapter readiness rows to preserve deleted-image custody and adapter claim boundaries'
    )
  )
);

export const ScreenAiAdapterReadinessReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    readModelId: ScreenAiAdapterReadinessReadModelIdSchema,
    generatedAt: ParentTimestampSchema,
    sourceArtifacts: Schema.Array(ScreenAiAdapterReadinessArtifactRefSchema),
    rows: Schema.Array(ScreenAiAdapterReadinessRowSchema),
  }).pipe(
    Schema.filter(
      (readModel) =>
        new Set(readModel.rows.map((row) => row.rowId)).size === readModel.rows.length ||
        'Expected screen AI adapter readiness row ids to be unique'
    )
  )
);

const ScreenAiAdapterCompletionArtifactBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  rowId: ScreenAiAdapterReadinessRowIdSchema,
  sourcePolicyDecisionRef: PolicyDecisionIdSchema,
  sourceEvidenceRefs: Schema.Array(ParentEvidenceReferenceSchema),
  applyResultRef: ScreenAiAdapterCompletionResultRefSchema,
  rollbackOrExpiryRef: ScreenAiAdapterCompletionResultRefSchema,
  auditRef: ScreenAiAdapterCompletionResultRefSchema,
  rawImageRetained: Schema.Boolean,
  rawImageDeletedBeforeAdapter: Schema.Boolean,
  screenDerivedPolicyDecision: Schema.Boolean,
  finalAdapterCompletionClaimed: Schema.Boolean,
});

type ScreenAiAdapterCompletionArtifactCandidate = Infer<typeof ScreenAiAdapterCompletionArtifactBaseSchema>;

export const ScreenAiAdapterCompletionArtifactSchema = withParser(
  ScreenAiAdapterCompletionArtifactBaseSchema.pipe(
    Schema.filter(
      (artifact) =>
        screenAiAdapterCompletionArtifactIsHonest(artifact) ||
        'Expected screen AI adapter completion artifacts to preserve screen-derived custody, apply, rollback, and audit refs'
    )
  )
);

export function screenAiAdapterReadinessCoversRequiredBoundaries(
  readModel: ScreenAiAdapterReadinessReadModel
): boolean {
  const rowIds = new Set<string>(readModel.rows.map((row) => String(row.rowId)));
  return RequiredScreenAiAdapterReadinessRowIds.every((rowId) => rowIds.has(rowId));
}

export function screenAiFinalAdapterCompletionGate(
  readModel: ScreenAiAdapterReadinessReadModel,
  artifacts: readonly ScreenAiAdapterCompletionArtifact[]
) {
  const readinessRows = new Map<string, ScreenAiAdapterReadinessRow>(
    readModel.rows.map((row) => [String(row.rowId), row])
  );
  const artifactsByRowId = new Map<string, ScreenAiAdapterCompletionArtifact>(
    artifacts.map((artifact) => [String(artifact.rowId), artifact])
  );
  const missingRows: string[] = [];
  const invalidRows: string[] = [];
  const completedRows: string[] = [];

  for (const rowId of RequiredScreenAiFinalAdapterCompletionRowIds) {
    const readinessRow = readinessRows.get(rowId);
    const artifact = artifactsByRowId.get(rowId);

    if (!readinessRow || !artifact) {
      missingRows.push(rowId);
      continue;
    }

    if (screenAiCompletionArtifactMatchesReadinessRow(readinessRow, artifact)) {
      completedRows.push(rowId);
    } else {
      invalidRows.push(rowId);
    }
  }

  return {
    completed: missingRows.length === 0 && invalidRows.length === 0,
    requiredRows: RequiredScreenAiFinalAdapterCompletionRowIds.length,
    completedRows: completedRows.length,
    missingRows,
    invalidRows,
    rawImageRetainedRows: artifacts.filter((artifact) => artifact.rawImageRetained).length,
  };
}

export function screenAiFinalAdapterCompletionGateIsSatisfied(
  readModel: ScreenAiAdapterReadinessReadModel,
  artifacts: readonly ScreenAiAdapterCompletionArtifact[]
): boolean {
  return screenAiFinalAdapterCompletionGate(readModel, artifacts).completed;
}

const RequiredScreenAiAdapterReadinessRowIds = [
  'screen-ai-owned-process-time-limit-real-adapter',
  'screen-ai-owned-process-block-real-adapter',
  'screen-ai-broad-installed-app-manual-required',
  'screen-ai-host-network-domain-manual-required',
  'screen-ai-managed-active-tab-not-claimed',
  'screen-ai-android-mobile-control-manual-required',
  'screen-ai-ios-mobile-control-manual-required',
  'screen-ai-linux-host-adapter-unavailable',
] as const;

const RequiredScreenAiFinalAdapterCompletionRowIds = [
  'screen-ai-broad-installed-app-manual-required',
  'screen-ai-host-network-domain-manual-required',
  'screen-ai-managed-active-tab-not-claimed',
  'screen-ai-android-mobile-control-manual-required',
  'screen-ai-ios-mobile-control-manual-required',
] as const;

export function summarizeScreenAiAdapterReadiness(readModel: ScreenAiAdapterReadinessReadModel) {
  return {
    rowCount: readModel.rows.length,
    byReadinessState: countBy(readModel.rows.map((row) => row.readinessState)),
    byPlatform: countBy(readModel.rows.map((row) => row.platform)),
    executedRows: readModel.rows.filter((row) => row.actionExecutionState === 'executed').length,
    skippedRows: readModel.rows.filter((row) => row.actionExecutionState === 'skipped').length,
    rawImageRetainedRows: readModel.rows.filter((row) => row.rawImageRetained).length,
    claimUpgradeRows: readModel.rows.filter((row) => Object.values(row.claimFlags).some(Boolean)).length,
  };
}

function screenAiAdapterCompletionArtifactIsHonest(artifact: ScreenAiAdapterCompletionArtifactCandidate): boolean {
  return (
    artifact.sourceEvidenceRefs.length > 0 &&
    artifact.applyResultRef.length > 0 &&
    artifact.rollbackOrExpiryRef.length > 0 &&
    artifact.auditRef.length > 0 &&
    artifact.rawImageRetained === false &&
    artifact.rawImageDeletedBeforeAdapter === true &&
    artifact.screenDerivedPolicyDecision === true &&
    artifact.finalAdapterCompletionClaimed === true
  );
}

function screenAiCompletionArtifactMatchesReadinessRow(
  row: ScreenAiAdapterReadinessRow,
  artifact: ScreenAiAdapterCompletionArtifact
): boolean {
  return (
    row.sourcePolicyDecisionId === artifact.sourcePolicyDecisionRef &&
    row.sourceImageDeletionState === 'deleted' &&
    row.rawImageRetained === false &&
    row.rawImageDeletedBeforeAdapter === true &&
    row.actionExecutionState === 'skipped' &&
    row.adapterExecutionProofArtifact === null &&
    artifact.sourceEvidenceRefs.length > 0 &&
    artifact.rawImageRetained === false &&
    artifact.rawImageDeletedBeforeAdapter === true &&
    artifact.screenDerivedPolicyDecision === true &&
    artifact.finalAdapterCompletionClaimed === true
  );
}

function screenAiAdapterReadinessRowIsHonest(row: ScreenAiAdapterReadinessRowCandidate): boolean {
  if (!screenAiAdapterReadinessRowHasSourceCustody(row)) {
    return false;
  }

  switch (row.readinessState) {
    case 'real-owned-process-action-proved':
      return realOwnedProcessActionRowIsHonest(row);
    case 'manual-required':
      return manualRequiredRowIsHonest(row);
    case 'not-claimed':
      return notClaimedRowIsHonest(row);
    case 'unavailable':
      return unavailableRowIsHonest(row);
    case 'unsupported':
      return unsupportedRowIsHonest(row);
    case 'degraded':
      return degradedRowIsHonest(row);
  }
}

function screenAiAdapterReadinessRowHasSourceCustody(row: ScreenAiAdapterReadinessRowCandidate): boolean {
  return (
    row.sourcePolicyDryRun === true &&
    row.sourceEvidenceReferences.length > 0 &&
    row.sourceImageDeletionState === 'deleted' &&
    row.rawImageRetained === false &&
    row.rawImageDeletedBeforeAdapter === true
  );
}

function realOwnedProcessActionRowIsHonest(row: ScreenAiAdapterReadinessRowCandidate): boolean {
  return (
    row.platform === 'windows' &&
    row.actionExecutionState === 'executed' &&
    row.adapterRuntimeState === 'implemented-boundary' &&
    row.adapterResult === 'supported-boundary-proved' &&
    row.platformSupportState === 'supported-on-windows' &&
    row.refusalReason === 'none' &&
    row.adapterExecutionProofArtifact !== null &&
    row.manualProofRequirements.length === 0 &&
    row.auditReferenceState === 'audit-reference-backed'
  );
}

function manualRequiredRowIsHonest(row: ScreenAiAdapterReadinessRowCandidate): boolean {
  return (
    row.actionExecutionState === 'skipped' &&
    row.adapterRuntimeState === 'manual-required' &&
    row.adapterResult === 'manual-proof-required' &&
    row.platformSupportState === 'manual-required' &&
    row.refusalReason === 'manual-artifact-required' &&
    row.adapterExecutionProofArtifact === null &&
    row.manualProofRequirements.length > 0
  );
}

function notClaimedRowIsHonest(row: ScreenAiAdapterReadinessRowCandidate): boolean {
  return (
    row.actionExecutionState === 'skipped' &&
    row.adapterRuntimeState === 'not-claimed' &&
    row.adapterResult === 'not-claimed' &&
    row.refusalReason === 'not-claimed-boundary' &&
    row.adapterExecutionProofArtifact === null &&
    row.manualProofRequirements.length > 0
  );
}

function unavailableRowIsHonest(row: ScreenAiAdapterReadinessRowCandidate): boolean {
  return (
    row.actionExecutionState === 'skipped' &&
    row.adapterRuntimeState === 'unavailable' &&
    row.adapterResult === 'target-unavailable' &&
    row.platformSupportState === 'unavailable-on-target' &&
    row.refusalReason === 'target-unavailable' &&
    row.adapterExecutionProofArtifact === null &&
    row.manualProofRequirements.length > 0
  );
}

function unsupportedRowIsHonest(row: ScreenAiAdapterReadinessRowCandidate): boolean {
  return (
    row.actionExecutionState === 'skipped' &&
    row.adapterRuntimeState === 'unsupported' &&
    row.adapterResult === 'unsupported-platform' &&
    row.platformSupportState === 'unsupported-platform' &&
    row.refusalReason === 'unsupported-platform' &&
    row.adapterExecutionProofArtifact === null &&
    row.manualProofRequirements.length > 0
  );
}

function degradedRowIsHonest(row: ScreenAiAdapterReadinessRowCandidate): boolean {
  return (
    row.actionExecutionState === 'skipped' &&
    row.adapterRuntimeState === 'degraded' &&
    row.adapterResult === 'degraded-permission-or-dependency' &&
    row.platformSupportState === 'degraded' &&
    row.refusalReason === 'permission-or-dependency-degraded' &&
    row.adapterExecutionProofArtifact === null &&
    row.manualProofRequirements.length > 0
  );
}

function countBy(values: readonly string[]): Record<string, number> {
  return values.reduce<Record<string, number>>((counts, value) => {
    counts[value] = (counts[value] ?? 0) + 1;
    return counts;
  }, {});
}

export type ScreenAiAdapterReadinessReadModelId = typeof ScreenAiAdapterReadinessReadModelIdSchema.Type;
export type ScreenAiAdapterReadinessRowId = typeof ScreenAiAdapterReadinessRowIdSchema.Type;
export type ScreenAiAdapterReadinessArtifactRef = typeof ScreenAiAdapterReadinessArtifactRefSchema.Type;
export type ScreenAiAdapterReadinessRequirement = typeof ScreenAiAdapterReadinessRequirementSchema.Type;
export type ScreenAiAdapterReadinessBoundary = typeof ScreenAiAdapterReadinessBoundarySchema.Type;
export type ScreenAiAdapterCompletionResultRef = typeof ScreenAiAdapterCompletionResultRefSchema.Type;
export type ScreenAiAdapterReadinessRuntimeBoundary = Infer<typeof ScreenAiAdapterReadinessRuntimeBoundarySchema>;
export type ScreenAiAdapterReadinessCapability = Infer<typeof ScreenAiAdapterReadinessCapabilitySchema>;
export type ScreenAiAdapterReadinessRollbackState = Infer<typeof ScreenAiAdapterReadinessRollbackStateSchema>;
export type ScreenAiAdapterReadinessState = Infer<typeof ScreenAiAdapterReadinessStateSchema>;
export type ScreenAiAdapterReadinessActionExecutionState = Infer<
  typeof ScreenAiAdapterReadinessActionExecutionStateSchema
>;
export type ScreenAiAdapterReadinessClaimFlags = Infer<typeof ScreenAiAdapterReadinessClaimFlagsSchema>;
export type ScreenAiAdapterReadinessRow = Infer<typeof ScreenAiAdapterReadinessRowSchema>;
export type ScreenAiAdapterReadinessReadModel = Infer<typeof ScreenAiAdapterReadinessReadModelSchema>;
export type ScreenAiAdapterCompletionArtifact = Infer<typeof ScreenAiAdapterCompletionArtifactSchema>;

