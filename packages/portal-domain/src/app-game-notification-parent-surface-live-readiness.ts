import {
  AgentAppGameNotificationReadinessReadModelSchema,
  AgentAppGameNotificationReadinessState,
  type AgentAppGameNotificationReadinessReadModel,
  type AgentAppGameNotificationReadinessRow,
} from '@ocentra-parent/agent-protocol-domain/app-game-notification-readiness';
import {
  AppGameNotificationParentSurfaceIntentReadModelSchema,
  RequiredAppGameNotificationParentSurfaceIntentNonClaims,
  type AppGameNotificationParentSurfaceIntentReadModel,
} from '@ocentra-parent/app-game-domain/app-game-notification-parent-surface-intent';
import { ParentContractSchemaVersion } from '@ocentra-parent/family-domain/reference-primitives';

const AppGameNotificationParentSurfaceLiveReadiness = {
  AuditSourceRef: 'app-game-notification-readiness-service-event',
  FamilyId: 'app-game-notification-live-family',
  IntentId: 'app-game-notification-live-parent-surface-read-model',
  MinimalSurfacePayloadBoundary:
    'Live service readiness rows are rendered as parent setup status only; provider delivery, preference mutation, scheduler, and outbox runtime results remain unproved.',
  PreferenceHandoffId: 'app-game-notification-readiness-service-preference-status-source',
  PreferenceManualProof: 'app-game-notification-parent-preference-proof-required',
  ProviderHandoffId: 'app-game-notification-readiness-service-provider-status-source',
  ProviderManualProof: 'app-game-notification-provider-adapter-proof-required',
  ReadinessContractRef: 'app-game-notification-readiness-service-read-model',
  SurfaceContractRef: 'app-game-notification-parent-surface-intent-read-model',
  StateManualProofPrefix: 'app-game-notification-readiness-state-',
  ReasonManualProofPrefix: 'app-game-notification-readiness-reason-',
} as const;

export function createAppGameNotificationParentSurfaceReadModelFromReadiness(
  readModelInput: unknown
): AppGameNotificationParentSurfaceIntentReadModel | null {
  const parsed = AgentAppGameNotificationReadinessReadModelSchema.safeParse(readModelInput);
  if (!parsed.success) {
    return null;
  }

  return parentSurfaceReadModelFromReadiness(parsed.data);
}

function parentSurfaceReadModelFromReadiness(
  readModel: AgentAppGameNotificationReadinessReadModel
): AppGameNotificationParentSurfaceIntentReadModel {
  const rows = readModel.rows.map((row) => parentSurfaceRowFromReadiness(row, readModel));

  return AppGameNotificationParentSurfaceIntentReadModelSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    intentId: AppGameNotificationParentSurfaceLiveReadiness.IntentId,
    generatedAt: readModel.generatedAt,
    family: { familyId: AppGameNotificationParentSurfaceLiveReadiness.FamilyId },
    sourceProviderStatusHandoffId: AppGameNotificationParentSurfaceLiveReadiness.ProviderHandoffId,
    sourcePreferenceStatusHandoffId: AppGameNotificationParentSurfaceLiveReadiness.PreferenceHandoffId,
    sourceContractRefs: [
      AppGameNotificationParentSurfaceLiveReadiness.ReadinessContractRef,
      AppGameNotificationParentSurfaceLiveReadiness.SurfaceContractRef,
    ],
    rows,
    manualActionRequiredCount: rows.filter((row) => row.parentSurfaceStatus === 'manual-action-required').length,
    unavailableVisibleCount: rows.filter((row) => row.parentSurfaceStatus === 'unavailable-visible').length,
    historyVisibleCount: rows.length,
    preferenceSetupRequiredCount: rows.filter((row) => row.preferenceVisibility === 'preference-setup-required').length,
    parentSurfaceNonClaims: RequiredAppGameNotificationParentSurfaceIntentNonClaims,
    parentNotificationUiRendered: false,
    parentPreferenceUiRendered: false,
    parentFrequencyControlUiRendered: false,
    providerDeliveryRuntimeClaimed: false,
    providerReceiptIngestionClaimed: false,
    providerCredentialsClaimed: false,
    cloudRoutingClaimed: false,
    childDeliveryClaimed: false,
    productionRuntimeClaimed: false,
    productionDurableOutboxStorageClaimed: false,
    adapterDispatchClaimed: false,
  });
}

function parentSurfaceRowFromReadiness(
  row: AgentAppGameNotificationReadinessRow,
  readModel: AgentAppGameNotificationReadinessReadModel
) {
  const unavailable = row.readinessState === AgentAppGameNotificationReadinessState.Unavailable;

  return {
    surfaceRowId: `app-game-notification-live-parent-surface-${row.rowId}`,
    sourceProviderHandoffRowId: `app-game-notification-live-provider-readiness-${row.rowId}`,
    sourcePreferenceHandoffRowId: `app-game-notification-live-preference-readiness-${row.rowId}`,
    sourceSchedulerEntryRef: null,
    sourceOutboxRecordRef: null,
    providerStatus: unavailable ? 'unavailable' : 'manual-required',
    deliveryResultState: unavailable ? 'not-sent' : 'manual-required',
    parentPreferenceState: unavailable ? 'channel-disabled' : 'manual-setup-required',
    quietHoursDecision: unavailable ? 'allow' : 'manual-required',
    providerChannel: 'in-app',
    parentSurfaceStatus: unavailable ? 'unavailable-visible' : 'manual-action-required',
    historyVisibility: unavailable ? 'unavailable-row-visible' : 'manual-review-only',
    preferenceVisibility: unavailable ? 'preference-disabled-visible' : 'preference-setup-required',
    drillInRefs: readinessDrillInRefs(row),
    auditRefs: readinessAuditRefs(row, readModel),
    manualProofRequirements: readinessManualProofRequirements(row),
    minimalSurfacePayloadBoundary: AppGameNotificationParentSurfaceLiveReadiness.MinimalSurfacePayloadBoundary,
    sensitiveDetailIncluded: false,
    providerDeliveryClaimed: false,
    providerReceiptClaimed: false,
    parentPreferenceMutationClaimed: false,
    childDeliveryClaimed: false,
  } as const;
}

function readinessDrillInRefs(row: AgentAppGameNotificationReadinessRow): readonly string[] {
  return uniqueRefs([row.minimalPayloadRef, ...row.evidenceReferenceIds, ...row.evidence.map((ref) => ref.evidenceId)]);
}

function readinessAuditRefs(
  row: AgentAppGameNotificationReadinessRow,
  readModel: AgentAppGameNotificationReadinessReadModel
): readonly string[] {
  return uniqueRefs([
    AppGameNotificationParentSurfaceLiveReadiness.AuditSourceRef,
    readModel.custodyLabel,
    readModel.capabilityStatus,
    row.rowId,
    row.reason,
  ]);
}

function readinessManualProofRequirements(row: AgentAppGameNotificationReadinessRow): readonly string[] {
  return uniqueRefs([
    AppGameNotificationParentSurfaceLiveReadiness.ProviderManualProof,
    AppGameNotificationParentSurfaceLiveReadiness.PreferenceManualProof,
    AppGameNotificationParentSurfaceLiveReadiness.StateManualProofPrefix + row.readinessState,
    AppGameNotificationParentSurfaceLiveReadiness.ReasonManualProofPrefix + row.reason,
  ]);
}

function uniqueRefs(refs: readonly string[]): readonly string[] {
  return [...new Set(refs.filter((ref) => ref.trim().length > 0))];
}
