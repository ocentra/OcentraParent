import { describe, expect, it } from 'vitest';
import {
  ParentAgentEvent as AgentEvent,
  type ParentNetworkRuntimeEventResultSnapshot,
  type ParentRouteLiveActivitySnapshot,
} from '../../generated/parent-ui-bridge';
import { EMPTY_ROUTE_LIVE_ACTIVITY_STATE, resolveSnapshotLiveActivityState } from '../../src/route-live-activity-state';
import {
  activityReportDocumentSnapshot,
  activityReportEventSnapshots,
  activityReportHistorySnapshot,
  activityTrackingReadModelResultSnapshot,
  malformedActivityReportEventSnapshot,
  networkFlowReadModelSnapshot,
} from './live-activity-state-test-support';
import { createParentPortalActivityUiIntent } from '../../../../vendor/ocentra-parent-core-ui/AppPages/ParentPortal/activity-ui-intent';

type ResolvedLiveActivityState = ReturnType<typeof resolveSnapshotLiveActivityState>;

const rustOwnedRouteSnapshotValue: ParentRouteLiveActivitySnapshot = {
  recentSummary: {
    schemaVersion: 1,
    limit: 10,
    returned: 1,
    firstObservedAt: '2026-06-23T00:00:00Z',
    lastObservedAt: '2026-06-23T00:01:00Z',
    lastEventId: 'activity-event-1',
    mostRecentKind: 'process-observed',
    mostRecentObserver: 'agent-service',
    mostRecentSubjectKind: 'process',
    mostRecentSubjectId: 'process-subject-1',
    mostRecentSubjectName: 'Child Laptop',
  },
  browserManagedEvent: {
    event: AgentEvent.BrowserManagedStatusReported,
    eventId: 'evt-rust-browser-managed',
    sentAt: '2026-06-23T00:00:00Z',
    sourcePeerId: 'local-dev-agent',
    sourceRole: 'agent-service',
    targetPeerId: 'portal-dev',
    targetRole: 'portal',
    severity: 'info',
    payload: {
      reason: 'Managed browser session is ready.',
    },
  },
  browserManagedStatus: {
    schemaVersion: 1,
    checkedAt: '2026-06-23T00:00:00Z',
    managedBrowserSessionId: 'managed-session-1',
    browserFamily: 'chrome',
    browserChannel: 'stable',
    browserVersion: '128.0.0',
    profileId: 'profile-1',
    profilePathRef: 'profile-path-ref-1',
    profileRootRef: 'profile-root-ref-1',
    profileScopeId: 'profile-scope-1',
    profileLifecycleState: 'ready',
    policyRevision: 'policy-revision-1',
    processId: 4242,
    bridgeKind: 'chromium-devtools-protocol',
    bridgeEndpointRef: 'bridge-endpoint-ref-1',
    unmanagedProcessName: null,
    unmanagedExecutablePathRef: null,
    unmanagedSignatureRef: null,
    unmanagedProcessHashRef: null,
    unmanagedProcessKind: null,
    unmanagedDetectionConfidence: null,
    unmanagedDetectionReason: null,
    managedState: 'bridge-connected',
    capabilityStatus: 'available',
    degradedReason: null,
    startedAt: '2026-06-23T00:00:00Z',
    custodyLabel: 'child-device-local',
    queryVisibility: 'live-local',
  },
  browserEvidenceReadModel: browserEvidenceReadModelSnapshot(),
  localAiRuntimeStatusEvent: {
    event: AgentEvent.LocalAiRuntimeStatusReported,
    eventId: 'evt-rust-local-ai',
    sentAt: '2026-06-23T00:00:00Z',
    sourcePeerId: 'local-dev-agent',
    sourceRole: 'agent-service',
    targetPeerId: 'portal-dev',
    targetRole: 'portal',
    severity: 'info',
    payload: {
      reason: 'Local AI runtime is ready.',
    },
  },
  networkFlowEvent: {
    event: AgentEvent.NetworkFlowReadModelReported,
    eventId: 'evt-rust-network-flow',
    sentAt: '2026-06-23T00:00:00Z',
    sourcePeerId: 'local-dev-agent',
    sourceRole: 'agent-service',
    targetPeerId: 'portal-dev',
    targetRole: 'portal',
    severity: 'info',
    payload: {},
  },
  networkFlowReadModel: networkFlowReadModelSnapshot(),
  appGameNotificationParentSurfacePanel: {
    eyebrow: 'Rust-owned panel',
    title: 'App/game notification parent surface',
    body: 'Rust-owned notification parent-surface rows are rendered directly in the portal.',
    state: 'manual-required',
    summary: '1 manual action',
    productClaim: 'Provider delivery, preference mutation, child delivery, and runtime dispatch remain unclaimed.',
    metrics: [
      { label: 'Rows returned', value: '2' },
      { label: 'Status', value: '1 manual action' },
    ],
    rows: [
      {
        key: 'app-game-notification-parent-surface-time-limit',
        title: 'app-game-notification-parent-surface-time-limit',
        details: [{ label: 'Runtime reference', value: 'scheduler-entry-app-game-time-limit' }],
      },
    ],
    emptyMessage: 'No app/game notification parent-surface panel has been reported yet.',
  },
  appGamePolicyReadinessPanel: {
    eyebrow: 'Rust-owned panel',
    title: 'App/game policy readiness',
    body: 'Rust-owned policy readiness rows are rendered directly in the portal.',
    loadState: 'ready',
    summaryDetails: [{ label: 'Manual review', value: 'Manual required' }],
    rows: [{ title: 'Policy evidence', details: [{ label: 'Reason', value: 'Ready' }] }],
    emptyMessage: 'No app/game policy readiness panel has been reported yet.',
    productClaim: 'Approval workflow, category routing, and adapter dispatch remain unclaimed.',
  },
  appGamePlatformProofStatusPanel: {
    eyebrow: 'Rust-owned panel',
    title: 'App/game platform proof status',
    body: 'Rust-owned platform proof rows are rendered directly in the portal.',
    loadState: 'ready',
    summaryDetails: [{ label: 'Platform proofs', value: '2' }],
    rows: [{ title: 'windows', details: [{ label: 'Host capability', value: 'available' }] }],
    emptyMessage: 'No app/game platform proof-status panel has been reported yet.',
    productClaim: 'Broad blocking, platform enforcement, and child delivery remain unclaimed.',
  },
  appGameChildRuntimeTransportReceiptPanel: {
    eyebrow: 'Rust-owned panel',
    title: 'App/game child runtime transport receipt',
    body: 'Rust-owned child runtime transport rows are rendered directly in the portal.',
    loadState: 'ready',
    summaryDetails: [{ label: 'Transport rows', value: '2' }],
    rows: [
      {
        title: 'app-game-child-runtime-transport-receipt-warning',
        details: [{ label: 'Boundary state', value: 'child-runtime-transport-required' }],
      },
    ],
    emptyMessage: 'No app/game child runtime transport receipt panel has been reported yet.',
    productClaim: 'Runtime transport, runtime receipt, and provider delivery remain unclaimed.',
  },
  activityTrackingReadModelEvent: {
    event: AgentEvent.ActivityTrackingReadModelReported,
    eventId: 'evt-rust-tracking',
    sentAt: '2026-06-23T00:00:00Z',
    sourcePeerId: 'local-dev-agent',
    sourceRole: 'agent-service',
    targetPeerId: 'portal-dev',
    targetRole: 'portal',
    severity: 'info',
    payload: null,
  },
  activityTrackingReadModel: activityTrackingReadModelResultSnapshot(),
};

describe('portal live activity contract projection', () => {
  it('derives the mounted app inventory/session dashboard from service-backed states', () => {
    const intent = createParentPortalActivityUiIntent(
      {
        activityAppUseReadModel: {
          ok: true,
          state: 'ready',
          value: {
            state: 'ready',
            rows: [
              {
                rowId: 'app-1',
                appName: 'Study Timer',
                inventoryState: 'installed',
                runtimeState: 'running',
                foregroundState: 'foreground',
                runningRowCount: 1,
                foregroundRowCount: 1,
                inventoryRowCount: 1,
              },
            ],
          },
        },
        activityGamesReadModel: { ok: true, state: 'ready', value: { state: 'ready', rows: [] } },
      },
      0
    );

    expect(intent.appGameDashboard.rows[0]?.label).toBe('Study Timer');
    expect(intent.appGameDashboard.rows[0]?.inventoryCount).toBe(1);
    expect(intent.appGameDashboard.rows[0]?.runningCount).toBe(1);
    expect(intent.appGameDashboard.rows[0]?.foregroundCount).toBe(1);
    expect(intent.appGameDashboard.emptyMessage).toContain('No app/game read model rows');
  });
  it('overlays Rust-owned route snapshot activity values without requiring raw agent events', () => {
    const state = resolveSnapshotLiveActivityState(rustOwnedRouteSnapshot());

    expectRustOwnedSnapshotOverlay(state);
  });

  it('projects generated and saved report events into the real activity viewer intent', () => {
    const report = activityReportDocumentSnapshot();
    const history = activityReportHistorySnapshot(report);
    const state = resolveSnapshotLiveActivityState(null, activityReportEventSnapshots());

    expect(state.activityReportEvent?.event).toBe(AgentEvent.ActivityReportSaved);
    expect(state.activityReport).toEqual({ ok: true, state: 'ready', value: report });
    expect(state.activityReportHistoryEvent?.event).toBe(AgentEvent.ActivityReportHistoryReported);
    expect(state.activityReportHistory).toEqual({ ok: true, state: 'ready', value: history });
    expect(state.activityServiceUiSpine.report).toEqual(state.activityReport);
    expect(state.activityServiceUiSpine.reportHistory).toEqual(state.activityReportHistory);

    const intent = createParentPortalActivityUiIntent(
      {
        activityReport: state.activityReport,
        activityReportHistory: state.activityReportHistory,
      },
      5
    );
    expect(intent.reportDocument).toEqual(report);
    expect(intent.reportHistory).toEqual(history);
    expect(intent.reportFiles.map((item) => item.fileName)).toEqual(['activity-report-1.json']);
  });

  it('rejects malformed report event JSON instead of promoting it into UI state', () => {
    const state = resolveSnapshotLiveActivityState(null, [malformedActivityReportEventSnapshot()]);

    expect(state.activityReportEvent?.event).toBe(AgentEvent.ActivityReportGenerated);
    expect(state.activityReport).toBeNull();
    expect(state.activityServiceUiSpine.report).toBeNull();
    expect(state.activityServiceUiSpine.currentState).toBe('unavailable');
  });

  it('keeps the product snapshot path empty when Rust has not supplied route activity yet', () => {
    const state = resolveSnapshotLiveActivityState(null);

    expect(state.recentSummary).toBeNull();
    expect(state.networkFlowEvent).toBeNull();
    expect(state.browserManagedStatus).toBeNull();
    expect(state.localAiRuntimeStatusEvent).toBeNull();
    expect(state.networkRuntimeEventChainStream).toBeNull();
  });

  it('keeps the empty route live activity state aligned with the shared portal-domain defaults', () => {
    expect(EMPTY_ROUTE_LIVE_ACTIVITY_STATE.activityServiceUiSpine.dataOwner).toBe('rust-service-read-model');
    expect(EMPTY_ROUTE_LIVE_ACTIVITY_STATE.activityServiceUiSpine.uiConsumer).toBe('c-owned-activity-ui');
    expect(EMPTY_ROUTE_LIVE_ACTIVITY_STATE.activityServiceUiSpine.viteDataOwner).toBe(false);
    expect(EMPTY_ROUTE_LIVE_ACTIVITY_STATE.activityServiceUiSpine.currentState).toBe('unavailable');
  });
});

describe('portal live activity runtime event projection', () => {
  it('surfaces Rust-owned runtime event-chain snapshots directly', () => {
    const runtimeEventValue = {
      aiAnalysisRef: 'analysis.network.flow.observed.1',
    } as const;
    const runtimeEvent = {
      ok: true,
      eventType: 'network.flow.observed',
      value: runtimeEventValue,
    } satisfies ParentNetworkRuntimeEventResultSnapshot;
    const state = resolveSnapshotLiveActivityState({
      networkRuntimeEventChainStream: {
        streamedEventCount: 1,
        invalidEventCount: 0,
        events: [runtimeEvent],
      },
    });

    expect(state.networkRuntimeEventChainStream).toEqual({
      streamedEventCount: 1,
      invalidEventCount: 0,
      events: [
        {
          ok: true,
          eventType: 'network.flow.observed',
          value: runtimeEventValue,
        },
      ],
    });
  });
});

describe('portal live activity malformed boundary projection', () => {
  it('normalizes optional bridge fields and rejects contradictory result envelopes', () => {
    const networkFlow = networkFlowReadModelSnapshot();
    const tracking = activityTrackingReadModelResultSnapshot();
    const state = resolveSnapshotLiveActivityState({
      browserManagedEvent: {
        event: AgentEvent.BrowserManagedStatusReported,
        payload: null,
      },
      networkFlowReadModel: {
        ...networkFlow,
        rows: networkFlow.rows.map((row) => ({
          ...row,
          localEndpoint: {},
          destinationEndpoint: {},
        })),
      },
      networkRuntimeEventChainStream: {
        streamedEventCount: 1,
        invalidEventCount: 0,
        events: [
          {
            ok: false,
            reason: 'owner-unavailable',
            eventType: 'network.flow.observed',
          },
        ],
      },
      activityTrackingReadModel: {
        ok: true,
        reason: 'invalid-payload',
        value: tracking.value ?? null,
      },
    });

    expect(state.browserManagedEvent).toEqual({
      event: AgentEvent.BrowserManagedStatusReported,
      eventId: null,
      correlationId: null,
      sentAt: null,
      sourcePeerId: null,
      sourceRole: null,
      targetPeerId: null,
      targetRole: null,
      severity: null,
      snapshot: null,
    });
    expect(state.networkFlowReadModel?.rows[0]?.localEndpoint).toEqual({ ip: null, port: null });
    expect(state.networkFlowReadModel?.rows[0]?.destinationEndpoint).toEqual({ ip: null, port: null });
    expect(state.networkRuntimeEventChainStream).toBeNull();
    expect(state.activityTrackingReadModel).toBeNull();
  });

  it('rejects malformed loose records instead of promoting them into product state', () => {
    const state = resolveSnapshotLiveActivityState({
      recentSummary: { schemaVersion: 1, limit: 1, returned: 2 },
      ingestStatus: {
        schemaVersion: 1,
        databaseReady: true,
        eventsIngested: 1,
        eventsStored: 1,
        duplicateEvents: 0,
        localOnlyClaim: true,
      },
      activityAppUseReadModel: {
        ok: true,
        state: 'ready',
        value: { state: 'ready', rows: [] },
      },
      browserEvidenceReadModel: { schemaVersion: 1 },
      browserManagedStatus: { managedState: 'managed', capabilityStatus: 'available' },
    });

    expect([
      state.recentSummary,
      state.ingestStatus,
      state.activityAppUseReadModel,
      state.browserEvidenceReadModel,
      state.browserManagedStatus,
    ]).toEqual([null, null, null, null, null]);
    expect(state.activityServiceUiSpine).toEqual(EMPTY_ROUTE_LIVE_ACTIVITY_STATE.activityServiceUiSpine);
  });
});

describe('portal live activity typed read-model projection', () => {
  it('parses a generated activity adapter result and wires it into the service spine', () => {
    const activityAppUseReadModel = readyActivityAppUseAdapterSnapshot();
    const state = resolveSnapshotLiveActivityState({ activityAppUseReadModel });

    expect(state.activityAppUseReadModel).toEqual(activityAppUseReadModel);
    expect(state.activityServiceUiSpine).toEqual({
      dataOwner: 'rust-service-read-model',
      uiConsumer: 'c-owned-activity-ui',
      viteDataOwner: false,
      currentState: 'ready',
      report: null,
      reportHistory: null,
      screen: null,
      appUse: activityAppUseReadModel,
      browser: null,
      games: null,
      network: null,
    });
  });

  it('preserves the strict Rust app/game platform extension while rejecting unmodeled browser records', () => {
    const activityAppGamePlatformExtensionReadModel = appGamePlatformExtensionSnapshot();
    const state = resolveSnapshotLiveActivityState({
      activityAppGamePlatformExtensionReadModel,
      browserInventoryReadModel: { rows: [] },
      browserInterventionReadModel: { rows: [] },
    });

    expect(state.activityAppGamePlatformExtensionReadModel).toEqual(activityAppGamePlatformExtensionReadModel);
    expect([state.browserInventoryReadModel, state.browserInterventionReadModel]).toEqual([null, null]);
  });

  it('rejects loose, version-skewed, inconsistent, and provider-extended app/game platform records', () => {
    const malformedValues = [
      { ok: true, value: { rows: [] } },
      {
        ...appGamePlatformExtensionSnapshot(),
        value: { ...appGamePlatformExtensionSnapshot().value, schemaVersion: 999 },
      },
      {
        ...appGamePlatformExtensionSnapshot(),
        value: { ...appGamePlatformExtensionSnapshot().value, state: 'ready' },
      },
      {
        ...appGamePlatformExtensionSnapshot(),
        value: {
          ...appGamePlatformExtensionSnapshot().value,
          rows: [
            {
              ...appGamePlatformExtensionSnapshot().value.rows[0],
              providerDispatchTarget: 'must-not-reach-portal',
            },
          ],
        },
      },
    ];

    expect(
      malformedValues.map(
        (activityAppGamePlatformExtensionReadModel) =>
          resolveSnapshotLiveActivityState({ activityAppGamePlatformExtensionReadModel })
            .activityAppGamePlatformExtensionReadModel
      )
    ).toEqual([null, null, null, null]);
  });

  it('parses tracking retention results and reports malformed writes explicitly', () => {
    const acceptedResult = trackingRetentionSettingsWriteResultSnapshot();
    const parsed = resolveSnapshotLiveActivityState({
      activityTrackingRetentionSettingsWriteResult: acceptedResult,
    }).activityTrackingRetentionSettingsWriteResult;
    const failed = resolveSnapshotLiveActivityState({
      activityTrackingRetentionSettingsWriteResult: { schemaVersion: 999 },
    }).activityTrackingRetentionSettingsWriteResult;

    expect(parsed?.parseState).toBe('parsed');
    expect(
      parsed?.parseState === 'parsed' ? [parsed.value.commandId, parsed.value.localServiceStateRevision] : null
    ).toEqual(['tracking-write-1', 7]);
    expect(failed).toEqual({ parseState: 'failed', reason: 'invalid-payload' });
  });
});

function rustOwnedRouteSnapshot(): ParentRouteLiveActivitySnapshot {
  return rustOwnedRouteSnapshotValue;
}

function expectRustOwnedSnapshotOverlay(state: ResolvedLiveActivityState): void {
  expectRustOwnedStateSummary(state);
  expectRustOwnedTrackingState(state);
  expectRustOwnedAppGamePanels(state);
}

function expectRustOwnedStateSummary(state: ResolvedLiveActivityState): void {
  expect(state.recentSummary?.mostRecentSubjectName).toBe('Child Laptop');
  expect(state.browserManagedEvent?.event).toBe(AgentEvent.BrowserManagedStatusReported);
  expect(state.browserManagedStatus?.managedState).toBe('bridge-connected');
  expect(state.browserEvidenceReadModel).toEqual(browserEvidenceReadModelSnapshot());
  expect(state.localAiRuntimeStatusEvent?.event).toBe(AgentEvent.LocalAiRuntimeStatusReported);
  expect(state.networkFlowEvent?.eventId).toBe('evt-rust-network-flow');
  expect(state.networkFlowReadModel?.rows.at(0)?.destinationDomain).toBe('example-network.test');
}

function readyActivityAppUseAdapterSnapshot(): Readonly<Record<string, unknown>> {
  return {
    ok: true,
    state: 'ready',
    value: {
      schemaVersion: 1,
      request: {
        schemaVersion: 1,
        scope: { scopeKind: 'family', familyId: 'family-1', deviceId: null },
        requestedAt: '2026-06-23T00:00:00Z',
        rangeStart: '2026-06-22T00:00:00Z',
        rangeEnd: '2026-06-23T00:00:00Z',
      },
      state: 'ready',
      generatedAt: '2026-06-23T00:00:01Z',
      summary: 'No app rows were returned.',
      rows: [],
    },
  };
}

function appGamePlatformExtensionSnapshot() {
  return {
    ok: true,
    value: {
      schemaVersion: 1,
      state: 'manual-required',
      generatedAt: '2026-06-23T00:00:01Z',
      summary: 'App/game platform extension proof-pack readiness from service projection',
      rows: [
        {
          platform: 'macos',
          state: 'manual-required',
          setupState: 'manual-required',
          proofPackState: 'manual-proof-pack-required',
          authorityTier: 'visibility-only',
          adapterExecutionClaim: 'not-executed',
          broadBlockingClaimed: false,
          privilegedMobileClaimed: false,
          childDeviceDeliveryClaimed: false,
          requiredProofRefs: ['macos-setup-proof'],
        },
      ],
    },
  };
}

function browserEvidenceReadModelSnapshot(): Readonly<Record<string, unknown>> {
  return {
    schemaVersion: 1,
    generatedAt: '2026-06-23T00:00:00Z',
    limit: 10,
    returned: 0,
    latestEventId: null,
    latestObservedAt: null,
    capabilityStatus: null,
    custodyLabel: 'unavailable',
    queryVisibility: 'unavailable',
    rows: [],
  };
}

function trackingRetentionSettingsWriteResultSnapshot(): Readonly<Record<string, unknown>> {
  return {
    schemaVersion: 1,
    commandId: 'tracking-write-1',
    settingsKind: 'retention-window-setting',
    writeState: 'service-write-command-accepted',
    acceptedAt: '2026-06-23T00:00:00Z',
    sourceWriterIntentRefs: ['writer-intent-1'],
    sourceReadModelProofRefs: ['read-model-proof-1'],
    sourceMutationProofRefs: ['mutation-proof-1'],
    appliedRetentionWindowHours: 24,
    appliedDeleteAfterAlertResolutionState: 'retain-after-alert-resolved',
    parentExportState: 'not-prepared',
    remoteSyncState: 'disabled',
    remoteAiState: 'disabled',
    localServiceStateRevision: 7,
    localServiceStateSnapshotRef: 'tracking-state-7',
    durableSettingsStoreRef: 'tracking-settings-store',
    durableSettingsPersistenceState: 'persisted',
    childConfigAckState: 'missing',
    commandTransportClaimState: 'claimed',
    serviceWritePreflightClaimState: 'claimed',
    serviceMutationExecutionState: 'claimed',
    portalWritableUiClaimState: 'unclaimed',
    platformRuntimeClaimState: 'unclaimed',
    childDeviceDeliveryClaimState: 'unclaimed',
    providerDeliveryClaimState: 'unclaimed',
    notificationReceiptClaimState: 'unclaimed',
    physicalDeviceClaimState: 'unclaimed',
    authorityClaimState: 'unclaimed',
    productClaimState: 'unclaimed',
  };
}

function expectRustOwnedTrackingState(state: ResolvedLiveActivityState): void {
  expect(state.activityTrackingReadModelEvent?.eventId).toBe('evt-rust-tracking');
  expect(state.activityTrackingReadModel?.ok ? state.activityTrackingReadModel.value.rows[0]?.deviceId : null).toBe(
    'child-device-1'
  );
}

function expectRustOwnedAppGamePanels(state: ResolvedLiveActivityState): void {
  const notificationPanel = state.appGameNotificationParentSurfacePanel;
  const policyPanel = state.appGamePolicyReadinessPanel;
  const platformPanel = state.appGamePlatformProofStatusPanel;
  const childPanel = state.appGameChildRuntimeTransportReceiptPanel;

  expect(notificationPanel?.summary).toBe('1 manual action');
  expect(notificationPanel?.rows[0]?.title).toBe('app-game-notification-parent-surface-time-limit');
  expect(policyPanel?.title).toBe('App/game policy readiness');
  expect(policyPanel?.summaryDetails[0]?.value).toBe('Manual required');
  expect(platformPanel?.rows[0]?.title).toBe('windows');
  expect(childPanel?.rows[0]?.title).toBe('app-game-child-runtime-transport-receipt-warning');
}
