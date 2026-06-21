import { Schema } from '@ocentra-parent/schema-domain/effect';
import {
  AppGameAdapterExecutionReadinessReadModelSchema,
  AppGameAdapterExecutionReadinessReferenceSchema,
  AppGameAdapterExecutionReadinessRowSchema,
  type AppGameAdapterExecutionReadinessReadModel as AppGameAdapterExecutionReadinessReadModelShape,
  type AppGameAdapterExecutionReadinessReference,
  type AppGameAdapterExecutionReadinessRow,
  type AppGameAdapterExecutionState,
  type AppGameAdapterHostCapabilityState,
} from '@ocentra-parent/schema-domain/app-game-adapter-execution-readiness';
import { ParentContractSchemaVersion } from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  V08SupportedAdapterRuntimeProofReadModel,
  type V08SupportedAdapterRuntimeProofEntry,
  type V08SupportedAdapterRuntimeProofReadModel as V08SupportedAdapterRuntimeProofReadModelType,
} from '@ocentra-parent/schema-domain/v0-8-supported-adapter-runtime-proof';

const generatedAt = '2026-06-08T09:17:00.000Z';

const AppGameAdapterRuntimeProofEntryIds = new Set([
  'windows-app-game-owned-process-time-limit',
  'windows-broad-installed-app-blocking-manual-gate',
  'windows-broad-installed-app-artifact-status',
  'windows-adapter-permission-dependency-degraded',
  'linux-host-adapter-unavailable',
  'macos-host-adapter-unsupported',
  'android-mobile-control-manual-gate',
  'ios-mobile-control-manual-gate',
]);
const decodeAppGameAdapterExecutionReadinessReference = Schema.decodeUnknownSync(
  AppGameAdapterExecutionReadinessReferenceSchema
);

export const AppGameAdapterExecutionReadinessReadModel = buildAppGameAdapterExecutionReadinessReadModel(
  V08SupportedAdapterRuntimeProofReadModel
);

export function buildAppGameAdapterExecutionReadinessReadModel(
  sourceReadModel: V08SupportedAdapterRuntimeProofReadModelType
): AppGameAdapterExecutionReadinessReadModelShape {
  return AppGameAdapterExecutionReadinessReadModelSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    readModelId: 'app-game-adapter-execution-readiness',
    generatedAt,
    sourceReadModelIds: [String(sourceReadModel.readModelId)],
    rows: sourceReadModel.entries
      .filter((entry: V08SupportedAdapterRuntimeProofEntry) =>
        AppGameAdapterRuntimeProofEntryIds.has(String(entry.proofEntryId))
      )
      .map(appGameAdapterExecutionReadinessRowFromEntry),
  });
}

function appGameAdapterExecutionReadinessRowFromEntry(
  entry: V08SupportedAdapterRuntimeProofEntry
): AppGameAdapterExecutionReadinessRow {
  const executionState = appGameAdapterExecutionState(entry);
  const executionAllowed = executionState === 'proved-scoped-execution';
  return AppGameAdapterExecutionReadinessRowSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    rowId: `app-game-adapter-execution-${entry.proofEntryId}`,
    sourceProofEntryId: entry.proofEntryId,
    platform: entry.platform,
    productMeanings: ['native-app', 'native-game'],
    adapterCapability: entry.adapterCapability,
    adapterExecutionState: executionState,
    executionDecision: executionAllowed ? 'execution-allowed' : 'blocked-before-execution',
    runtimeBoundary: entry.runtimeBoundary,
    targetIdentityState: entry.targetIdentityState,
    rollbackReferenceState: entry.rollbackReferenceState,
    auditReferenceState: entry.auditReferenceState,
    evidenceRefs: entry.evidenceRefs,
    hostCapabilityState: appGameHostCapabilityState(entry),
    hostCapabilityEvidenceRefs: appGameHostCapabilityEvidenceRefs(entry),
    hostCapabilityProbeRefs: appGameHostCapabilityProbeRefs(entry),
    linkedProofArtifacts: entry.linkedProofArtifacts,
    manualProofRequirements: executionAllowed ? [] : appGameManualProofRequirements(entry),
    claimBoundary: appGameClaimBoundary(entry),
    fallbackBehavior: entry.fallbackBehavior,
    adapterExecutionClaimed: executionAllowed,
    broadInstalledAppBlockingClaimed: false,
    childDeviceDeliveryClaimed: false,
    platformEnforcementClaimed: false,
    providerDeliveryClaimed: false,
    privateDiagnosticsClaimed: false,
    lastCheckedAt: generatedAt,
  });
}

function appGameHostCapabilityProbeRefs(
  entry: V08SupportedAdapterRuntimeProofEntry
): readonly AppGameAdapterExecutionReadinessReference[] {
  if (entry.platform === 'macos' || entry.platform === 'ios') {
    return appGameReadinessReferences([]);
  }
  if (entry.platform === 'windows') {
    return appGameReadinessReferences(['windows-host-local-probe-ref']);
  }
  return appGameReadinessReferences([]);
}

function appGameHostCapabilityState(entry: V08SupportedAdapterRuntimeProofEntry): AppGameAdapterHostCapabilityState {
  if (entry.platform === 'windows') {
    return 'available';
  }
  if (entry.platform === 'macos' || entry.platform === 'ios') {
    return 'not-applicable';
  }
  return 'not-detected';
}

function appGameHostCapabilityEvidenceRefs(
  entry: V08SupportedAdapterRuntimeProofEntry
): readonly AppGameAdapterExecutionReadinessReference[] {
  if (entry.platform === 'windows') {
    return appGameReadinessReferences(['adapter-capability-state-ref']);
  }
  return appGameReadinessReferences([]);
}

function appGameAdapterExecutionState(entry: V08SupportedAdapterRuntimeProofEntry): AppGameAdapterExecutionState {
  if (entry.proofEntryId === 'windows-app-game-owned-process-time-limit') {
    return 'proved-scoped-execution';
  }
  if (entry.runtimeState === 'manual-required') return 'manual-required';
  if (entry.runtimeState === 'unavailable') return 'unavailable';
  if (entry.runtimeState === 'unsupported') return 'unsupported';
  if (entry.runtimeState === 'degraded') return 'degraded';
  return 'not-claimed';
}

function appGameManualProofRequirements(
  entry: V08SupportedAdapterRuntimeProofEntry
): readonly AppGameAdapterExecutionReadinessReference[] {
  if (entry.manualProofRequirements.length > 0) {
    return appGameReadinessReferences(entry.manualProofRequirements);
  }
  return appGameReadinessReferences(['app-game adapter execution proof requirement']);
}

function appGameClaimBoundary(entry: V08SupportedAdapterRuntimeProofEntry): string {
  if (entry.proofEntryId === 'windows-app-game-owned-process-time-limit') {
    return `${entry.claimBoundary} Adapter execution is claimable only for scoped owned-process time-limit rows.`;
  }
  return `${entry.claimBoundary} Adapter execution is blocked before runtime for this app/game row.`;
}

function appGameReadinessReferences(values: readonly string[]): readonly AppGameAdapterExecutionReadinessReference[] {
  return values.map((value) => decodeAppGameAdapterExecutionReadinessReference(value));
}

export function summarizeAppGameAdapterExecutionReadiness(
  readModel: AppGameAdapterExecutionReadinessReadModelShape
): Record<string, number> {
  return {
    rows: readModel.rows.length,
    executionAllowed: readModel.rows.filter((row) => row.executionDecision === 'execution-allowed').length,
    blockedBeforeExecution: readModel.rows.filter((row) => row.executionDecision === 'blocked-before-execution').length,
    adapterExecutionClaimed: readModel.rows.filter((row) => row.adapterExecutionClaimed).length,
    broadInstalledAppBlockingClaimed: readModel.rows.filter((row) => row.broadInstalledAppBlockingClaimed).length,
    childDeviceDeliveryClaimed: readModel.rows.filter((row) => row.childDeviceDeliveryClaimed).length,
    platformEnforcementClaimed: readModel.rows.filter((row) => row.platformEnforcementClaimed).length,
    providerDeliveryClaimed: readModel.rows.filter((row) => row.providerDeliveryClaimed).length,
    privateDiagnosticsClaimed: readModel.rows.filter((row) => row.privateDiagnosticsClaimed).length,
  };
}

export const decodeAppGameAdapterExecutionReadinessRow = Schema.decodeUnknownSync(
  AppGameAdapterExecutionReadinessRowSchema
);
export const decodeAppGameAdapterExecutionReadinessReadModel = Schema.decodeUnknownSync(
  AppGameAdapterExecutionReadinessReadModelSchema
);
