import { describe, expect, it } from 'vitest';
import {
  planScreenFamilyAiHubRoute,
} from '../../src/screen-evidence-family-hub-routing';
import {
  ScreenFamilyAiHubCapabilitySchema,
  ScreenFamilyAiHubRouteSchema,
} from '@ocentra-parent/schema-domain/screen-evidence-family-hub-routing';
import {
  ScreenFamilyAiHubRouteSchemaVersion,
} from '@ocentra-parent/schema-domain/screen-evidence-family-hub-routing-values';

const AvailableHubCapability = {
  schemaVersion: ScreenFamilyAiHubRouteSchemaVersion,
  hubId: 'screen-family-hub-windows-gpu',
  checkedAt: '2026-06-05T02:25:00.000Z',
  capabilityState: 'available',
  supportedTasks: ['guidedVisionClassification', 'guidedMultimodalClassification'],
  modelRuntimeRef: 'screen-family-hub-qwen2-vl-runtime',
  householdRouteRef: 'household-lan-family-hub-route',
  custodyState: 'live-lan-child-agent',
  noRetention: true,
  localHouseholdOnly: true,
  parentApprovalRequired: true,
  ocentraHostedProcessingAllowed: false,
  rawImageRetentionAllowed: false,
  degradedStates: [],
  unavailableReason: null,
} as const;

const DegradedChildLocalAttempt = {
  attempted: true,
  providerKind: 'localVision',
  executionState: 'degraded',
  modelRuntimeRef: null,
  degradedStates: ['resourceExhausted'],
} as const;

const BaseRouteRequest = {
  routeId: 'screen-family-hub-route-hard-visual',
  queueJobId: 'screen-queue-hard-visual-1',
  routedAt: '2026-06-05T02:26:00.000Z',
  requestedTask: 'guidedVisionClassification',
  sourceChildLocalAttempt: DegradedChildLocalAttempt,
  capability: AvailableHubCapability,
  parentApprovedFamilyHub: true,
  transferMode: 'redactedCrop',
  sourceCustodyState: 'child-device-temp-queue',
  auditEvidenceIds: ['screen-family-hub-audit-evidence-1'],
} as const;

describe('screen evidence family AI hub routing contracts', () => {
  specifyFamilyHubSelection();
  specifyParentDisabledFallback();
  specifyChildLocalSelectionGuard();
  specifyHubUnavailableFallback();
  specifyUnsafeRouteRejections();
});

function specifyFamilyHubSelection() {
  it('selects a local household family hub only after child-local analysis degrades', () => {
    const route = planScreenFamilyAiHubRoute(BaseRouteRequest);

    expect(route.executionState).toBe('selected');
    expect(route.selectedRuntimeRef).toBe('screen-family-hub-qwen2-vl-runtime');
    expect(route.sourceChildLocalAttempt.executionState).toBe('degraded');
    expect(route.transferMode).toBe('redactedCrop');
    expect(route.destinationCustodyState).toBe('live-lan-child-agent');
    expect(route.rawFullScreenshotTransferAllowed).toBe(false);
    expect(route.rawImageRetentionAllowed).toBe(false);
    expect(route.remoteProviderSelected).toBe(false);
    expect(route.remoteApiFallbackAllowed).toBe(false);
    expect(route.remoteDefaultForBlocking).toBe(false);
  });
}

function specifyParentDisabledFallback() {
  it('keeps routing manual-required when the parent has not approved the family hub', () => {
    const route = planScreenFamilyAiHubRoute({
      ...BaseRouteRequest,
      routeId: 'screen-family-hub-route-parent-disabled',
      parentApprovedFamilyHub: false,
    });

    expect(route.executionState).toBe('manualRequired');
    expect(route.selectedRuntimeRef).toBe(null);
    expect(route.transferMode).toBe('noTransfer');
    expect(route.destinationCustodyState).toBe('unavailable');
    expect(route.degradedStates).toEqual(['parentDisabled']);
  });
}

function specifyChildLocalSelectionGuard() {
  it('does not use the family hub when child-local analysis already selected a runtime', () => {
    const route = planScreenFamilyAiHubRoute({
      ...BaseRouteRequest,
      routeId: 'screen-family-hub-route-local-selected',
      sourceChildLocalAttempt: {
        attempted: true,
        providerKind: 'localVision',
        executionState: 'selected',
        modelRuntimeRef: 'child-device-local-vision-runtime',
        degradedStates: [],
      },
    });

    expect(route.executionState).toBe('manualRequired');
    expect(route.selectedRuntimeRef).toBe(null);
    expect(route.transferMode).toBe('noTransfer');
    expect(route.degradedStates).toEqual(['childLocalAlreadySelected']);
  });
}

function specifyHubUnavailableFallback() {
  it('surfaces unavailable when the household hub cannot be reached', () => {
    const unavailableCapability = ScreenFamilyAiHubCapabilitySchema.parse({
      ...AvailableHubCapability,
      capabilityState: 'hubUnavailable',
      modelRuntimeRef: null,
      householdRouteRef: null,
      degradedStates: ['hubUnavailable'],
      unavailableReason: 'household LAN family hub is offline',
    });
    const route = planScreenFamilyAiHubRoute({
      ...BaseRouteRequest,
      routeId: 'screen-family-hub-route-unavailable',
      capability: unavailableCapability,
    });

    expect(route.executionState).toBe('unavailable');
    expect(route.selectedRuntimeRef).toBe(null);
    expect(route.degradedStates).toEqual(['hubUnavailable']);
  });
}

function specifyUnsafeRouteRejections() {
  it('rejects raw screenshot transfer, retention, remote fallback, and Ocentra-hosted processing claims', () => {
    const route = planScreenFamilyAiHubRoute(BaseRouteRequest);
    const rawTransfer = ScreenFamilyAiHubRouteSchema.safeParse({
      ...route,
      rawFullScreenshotTransferAllowed: true,
    });
    const rawRetention = ScreenFamilyAiHubRouteSchema.safeParse({
      ...route,
      rawImageRetentionAllowed: true,
    });
    const remoteFallback = ScreenFamilyAiHubRouteSchema.safeParse({
      ...route,
      remoteApiFallbackAllowed: true,
    });
    const hostedProcessing = ScreenFamilyAiHubRouteSchema.safeParse({
      ...route,
      ocentraHostedProcessingAllowed: true,
    });

    expect(rawTransfer.success).toBe(false);
    expect(rawRetention.success).toBe(false);
    expect(remoteFallback.success).toBe(false);
    expect(hostedProcessing.success).toBe(false);
  });
}
