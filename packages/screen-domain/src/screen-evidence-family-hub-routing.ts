import { type Infer } from '@ocentra-parent/schema-domain/effect';
import {
  ScreenFamilyAiHubRouteRequestSchema,
  ScreenFamilyAiHubRouteSchema,
  type ScreenFamilyAiHubCapability,
  type ScreenFamilyAiHubRoute,
} from '@ocentra-parent/schema-domain/screen-evidence-family-hub-routing';
import { ScreenFamilyAiHubRouteSchemaVersion } from '@ocentra-parent/schema-domain/screen-evidence-family-hub-routing-values';

type ScreenFamilyAiHubRouteRequest = Infer<typeof ScreenFamilyAiHubRouteRequestSchema>;

export function planScreenFamilyAiHubRoute(request: ScreenFamilyAiHubRouteRequest): ScreenFamilyAiHubRoute {
  const parsed = ScreenFamilyAiHubRouteRequestSchema.parse(request);
  const selected = screenFamilyAiHubCanServe(parsed);
  const degradedStates = selected ? [] : screenFamilyAiHubDegradedStatesFor(parsed);

  return ScreenFamilyAiHubRouteSchema.parse({
    schemaVersion: ScreenFamilyAiHubRouteSchemaVersion,
    routeId: parsed.routeId,
    queueJobId: parsed.queueJobId,
    routedAt: parsed.routedAt,
    requestedTask: parsed.requestedTask,
    sourceChildLocalAttempt: parsed.sourceChildLocalAttempt,
    capability: parsed.capability,
    executionState: selected ? 'selected' : screenFamilyAiHubExecutionStateFor(parsed.capability),
    selectedRuntimeRef: selected ? parsed.capability.modelRuntimeRef : null,
    transferMode: selected ? parsed.transferMode : 'noTransfer',
    sourceCustodyState: parsed.sourceCustodyState,
    destinationCustodyState: selected ? 'live-lan-child-agent' : 'unavailable',
    degradedStates,
    auditEvidenceIds: parsed.auditEvidenceIds,
    parentApprovedFamilyHub: parsed.parentApprovedFamilyHub,
    localProviderAttempted: true,
    childSafetyLocalFallbackPreserved: true,
    summaryFirst: true,
    redactedOrCroppedInputRequired: true,
    rawFullScreenshotTransferAllowed: false,
    rawImageRetentionAllowed: false,
    remoteProviderSelected: false,
    remoteApiFallbackAllowed: false,
    ocentraHostedProcessingAllowed: false,
    remoteDefaultForBlocking: false,
  });
}

function screenFamilyAiHubCanServe(request: ScreenFamilyAiHubRouteRequest) {
  return (
    request.parentApprovedFamilyHub &&
    request.sourceChildLocalAttempt.executionState !== 'selected' &&
    request.capability.capabilityState === 'available' &&
    request.capability.supportedTasks.includes(request.requestedTask) &&
    request.transferMode !== 'noTransfer' &&
    (request.sourceCustodyState === 'child-device-temp-queue' || request.sourceCustodyState === 'child-device-journal')
  );
}

function screenFamilyAiHubDegradedStatesFor(request: ScreenFamilyAiHubRouteRequest) {
  if (request.sourceChildLocalAttempt.executionState === 'selected') {
    return ['childLocalAlreadySelected'] as const;
  }
  if (!request.parentApprovedFamilyHub) {
    return ['parentDisabled'] as const;
  }
  if (!request.capability.supportedTasks.includes(request.requestedTask)) {
    return ['unsupportedTask'] as const;
  }
  if (
    request.sourceCustodyState !== 'child-device-temp-queue' &&
    request.sourceCustodyState !== 'child-device-journal'
  ) {
    return ['custodyUnsafe'] as const;
  }
  return request.capability.degradedStates.length > 0
    ? request.capability.degradedStates
    : (['manualRequired'] as const);
}

function screenFamilyAiHubExecutionStateFor(capability: ScreenFamilyAiHubCapability) {
  return capability.capabilityState === 'hubUnavailable' ? 'unavailable' : 'manualRequired';
}
