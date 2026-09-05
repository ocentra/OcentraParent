import {
  EMPTY_PORTAL_LIVE_ACTIVITY_STATE,
  type PortalLiveActivityState,
} from '@ocentra-parent/portal-domain/live-activity-state';
import { parsePortalLanAddDeviceReadModel } from '@ocentra-parent/portal-domain/live-activity-lan-add-device';
import { decodeActivityAppGamePlatformExtensionReadModel } from '@ocentra-parent/portal-domain/route-live-activity-app-game-extension-decoder';
import {
  decodeNetworkFlowReadModel,
  decodeNetworkRuntimeEventChainStream,
} from '@ocentra-parent/portal-domain/route-live-activity-network-decoder';
import { decodePortalRouteEvent } from '@ocentra-parent/portal-domain/route-live-activity-route-event-decoder';
import {
  decodeActivityIngestStatus,
  decodeActivityRecentSummary,
} from '@ocentra-parent/portal-domain/route-live-activity-summary-decoders';
import {
  decodeActivityAppUseReadModel,
  decodeActivityBrowserReadModel,
  decodeActivityGamesReadModel,
  decodeActivityReportDocumentPayload,
  decodeActivityReportHistoryPayload,
  decodeActivityScreenReadModel,
} from '@ocentra-parent/portal-domain/route-live-activity-surface-decoders';
import {
  decodeActivityTrackingReadModel,
  decodeTrackingRetentionSettingsWriteResult,
} from '@ocentra-parent/portal-domain/route-live-activity-tracking-decoder';
import type {
  ParentAppGameAdapterDispatchPanelSnapshot,
  ParentAppGameNotificationParentSurfacePanelSnapshot,
  ParentAppGamePanelSnapshot,
  ParentAppGameTimerParentSurfacePanelSnapshot,
  ParentNetworkEvidenceSummarySnapshot,
  ParentPolicyPreviewPanelSnapshot,
  ParentRouteEventSnapshot,
  ParentRouteLiveActivitySnapshot,
  ParentScreenSummaryPanelSnapshot,
} from '../generated/parent-ui-bridge';
import { ParentAgentActivityReadModelState, ParentAgentEvent } from '../generated/parent-ui-bridge';
import { decodeBrowserEvidenceReadModel, decodeBrowserManagedStatus } from './route-live-activity-state-decoders';

type ResolvedPortalLiveActivityOverrides = {
  readonly activityScreenReadModel: ReturnType<typeof decodeActivityScreenReadModel>;
  readonly activityAppUseReadModel: ReturnType<typeof decodeActivityAppUseReadModel>;
  readonly activityBrowserReadModel: ReturnType<typeof decodeActivityBrowserReadModel>;
  readonly activityGamesReadModel: ReturnType<typeof decodeActivityGamesReadModel>;
  readonly browserInventoryEvent: ReturnType<typeof decodePortalRouteEvent>;
  readonly browserEvidenceEvent: ReturnType<typeof decodePortalRouteEvent>;
  readonly browserEvidenceReadModel: ReturnType<typeof decodeBrowserEvidenceReadModel>;
  readonly browserManagedEvent: ReturnType<typeof decodePortalRouteEvent>;
  readonly browserManagedStatus: ReturnType<typeof decodeBrowserManagedStatus>;
  readonly localAiRuntimeStatusEvent: ReturnType<typeof decodePortalRouteEvent>;
  readonly lanAiJobEvent: ReturnType<typeof decodePortalRouteEvent>;
  readonly parentAssistantBoundaryEvent: ReturnType<typeof decodePortalRouteEvent>;
  readonly networkFlowEvent: ReturnType<typeof decodePortalRouteEvent>;
  readonly networkFlowReadModel: ReturnType<typeof decodeNetworkFlowReadModel>;
  readonly networkRuntimeEventChainStream: ReturnType<typeof decodeNetworkRuntimeEventChainStream>;
  readonly lanPairingBrowserDiscoveryEvent: ReturnType<typeof decodePortalRouteEvent>;
  readonly lanAddDeviceReadModel: ReturnType<typeof parsePortalLanAddDeviceReadModel>;
  readonly browserInterventionEvent: ReturnType<typeof decodePortalRouteEvent>;
  readonly activityTrackingReadModelEvent: ReturnType<typeof decodePortalRouteEvent>;
  readonly activityTrackingReadModel: ReturnType<typeof decodeActivityTrackingReadModel>;
  readonly activityTrackingRetentionSettingsWriteResult: ReturnType<typeof decodeTrackingRetentionSettingsWriteResult>;
  readonly screenSummaryPanel?: ParentScreenSummaryPanelSnapshot | null;
  readonly networkEvidenceSummary?: ParentNetworkEvidenceSummarySnapshot | null;
  readonly policyPreviewPanel?: ParentPolicyPreviewPanelSnapshot | null;
  readonly appGameNotificationParentSurfacePanel: ParentAppGameNotificationParentSurfacePanelSnapshot | null;
  readonly appGamePolicyReadinessPanel: ParentAppGamePanelSnapshot | null;
  readonly appGamePlatformProofStatusPanel: ParentAppGamePanelSnapshot | null;
  readonly appGameChildRuntimeTransportReceiptPanel: ParentAppGamePanelSnapshot | null;
  readonly appGameAdapterDispatchPanel?: ParentAppGameAdapterDispatchPanelSnapshot | null;
  readonly appGameTimerParentSurfacePanel?: ParentAppGameTimerParentSurfacePanelSnapshot | null;
};

export type ResolvedPortalLiveActivityState = Omit<PortalLiveActivityState, keyof ResolvedPortalLiveActivityOverrides> &
  ResolvedPortalLiveActivityOverrides;

export const EMPTY_ROUTE_LIVE_ACTIVITY_STATE = {
  ...EMPTY_PORTAL_LIVE_ACTIVITY_STATE,
  appGameNotificationParentSurfacePanel: null,
  appGamePlatformProofStatusPanel: null,
  appGameChildRuntimeTransportReceiptPanel: null,
  appGamePolicyReadinessPanel: null,
} satisfies ResolvedPortalLiveActivityState;

export function resolveSnapshotLiveActivityState(
  snapshot?: ParentRouteLiveActivitySnapshot | null,
  events: readonly ParentRouteEventSnapshot[] = []
): ResolvedPortalLiveActivityState {
  if (snapshot === null || snapshot === undefined) {
    return withActivityReportEvents(EMPTY_ROUTE_LIVE_ACTIVITY_STATE, events);
  }

  return withActivityReportEvents(
    {
      ...EMPTY_ROUTE_LIVE_ACTIVITY_STATE,
      ...decodeActivitySurfaceSnapshot(snapshot),
      recentSummary: decodeActivityRecentSummary(snapshot.recentSummary),
      ingestStatus: decodeActivityIngestStatus(snapshot.ingestStatus),
      activityAppGamePlatformExtensionReadModel: decodeActivityAppGamePlatformExtensionReadModel(
        snapshot.activityAppGamePlatformExtensionReadModel
      ),
      screenSummaryPanel: optionalSnapshot(snapshot.screenSummaryPanel),
      browserInventoryEvent: decodePortalRouteEvent(snapshot.browserInventoryEvent),
      browserInventoryReadModel: null,
      browserEvidenceEvent: decodePortalRouteEvent(snapshot.browserEvidenceEvent),
      browserEvidenceReadModel: decodeBrowserEvidenceReadModel(snapshot.browserEvidenceReadModel),
      browserManagedEvent: decodePortalRouteEvent(snapshot.browserManagedEvent),
      browserManagedStatus: decodeBrowserManagedStatus(snapshot.browserManagedStatus),
      localAiRuntimeStatusEvent: decodePortalRouteEvent(snapshot.localAiRuntimeStatusEvent),
      lanAiJobEvent: decodePortalRouteEvent(snapshot.lanAiJobEvent),
      parentAssistantBoundaryEvent: decodePortalRouteEvent(snapshot.parentAssistantBoundaryEvent),
      activityMemoryGraphReadModel: optionalSnapshot(snapshot.activityMemoryGraphReadModel),
      networkFlowEvent: decodePortalRouteEvent(snapshot.networkFlowEvent),
      networkFlowReadModel: decodeNetworkFlowReadModel(snapshot.networkFlowReadModel),
      networkEvidenceSummary: optionalSnapshot(snapshot.networkEvidenceSummary),
      networkRuntimeEventChainStream: decodeNetworkRuntimeEventChainStream(snapshot.networkRuntimeEventChainStream),
      lanPairingBrowserDiscoveryEvent: decodePortalRouteEvent(snapshot.lanPairingBrowserDiscoveryEvent),
      lanAddDeviceReadModel: parsePortalLanAddDeviceReadModel(snapshot.lanAddDeviceReadModel),
      policyPreviewPanel: optionalSnapshot(snapshot.policyPreviewPanel),
      appGameNotificationParentSurfacePanel: optionalSnapshot(snapshot.appGameNotificationParentSurfacePanel),
      appGamePolicyReadinessPanel: optionalSnapshot(snapshot.appGamePolicyReadinessPanel),
      appGamePlatformProofStatusPanel: optionalSnapshot(snapshot.appGamePlatformProofStatusPanel),
      appGameChildRuntimeTransportReceiptPanel: optionalSnapshot(snapshot.appGameChildRuntimeTransportReceiptPanel),
      appGameAdapterDispatchPanel: optionalSnapshot(snapshot.appGameAdapterDispatchPanel),
      appGameTimerParentSurfacePanel: optionalSnapshot(snapshot.appGameTimerParentSurfacePanel),
      browserInterventionEvent: decodePortalRouteEvent(snapshot.browserInterventionEvent),
      browserInterventionReadModel: null,
      activityTrackingReadModelEvent: decodePortalRouteEvent(snapshot.activityTrackingReadModelEvent),
      activityTrackingReadModel: decodeActivityTrackingReadModel(snapshot.activityTrackingReadModel),
      activityTrackingPanel: optionalSnapshot(snapshot.activityTrackingPanel),
      activityTrackingRetentionSettingsWriteResult: decodeTrackingRetentionSettingsWriteResult(
        snapshot.activityTrackingRetentionSettingsWriteResult
      ),
    },
    events
  );
}

function withActivityReportEvents(
  state: ResolvedPortalLiveActivityState,
  events: readonly ParentRouteEventSnapshot[]
): ResolvedPortalLiveActivityState {
  const reportEvent = decodePortalRouteEvent(
    events.find(
      (event) =>
        event.event === ParentAgentEvent.ActivityReportGenerated || event.event === ParentAgentEvent.ActivityReportSaved
    )
  );
  const reportHistoryEvent = decodePortalRouteEvent(
    events.find((event) => event.event === ParentAgentEvent.ActivityReportHistoryReported)
  );
  const report = decodeActivityReportDocumentPayload(reportEvent?.payload);
  const reportHistory = decodeActivityReportHistoryPayload(reportHistoryEvent?.payload);
  return {
    ...state,
    activityReportEvent: reportEvent,
    activityReport: report,
    activityReportHistoryEvent: reportHistoryEvent,
    activityReportHistory: reportHistory,
    activityServiceUiSpine: {
      ...state.activityServiceUiSpine,
      currentState: firstReportedActivityState([
        report,
        reportHistory,
        state.activityScreenReadModel,
        state.activityAppUseReadModel,
        state.activityBrowserReadModel,
        state.activityGamesReadModel,
      ]),
      report,
      reportHistory,
    },
  };
}

function decodeActivitySurfaceSnapshot(snapshot: ParentRouteLiveActivitySnapshot) {
  const screen = decodeActivityScreenReadModel(snapshot.activityScreenReadModel);
  const appUse = decodeActivityAppUseReadModel(snapshot.activityAppUseReadModel);
  const browser = decodeActivityBrowserReadModel(snapshot.activityBrowserReadModel);
  const games = decodeActivityGamesReadModel(snapshot.activityGamesReadModel);
  return {
    activityServiceUiSpine: {
      ...EMPTY_PORTAL_LIVE_ACTIVITY_STATE.activityServiceUiSpine,
      currentState: firstReportedActivityState([screen, appUse, browser, games]),
      screen,
      appUse,
      browser,
      games,
    },
    activityScreenReadModel: screen,
    activityAppUseReadModel: appUse,
    activityBrowserReadModel: browser,
    activityGamesReadModel: games,
  };
}

function firstReportedActivityState(
  readModels: readonly (Readonly<{ state: ParentAgentActivityReadModelState }> | null)[]
) {
  for (const readModel of readModels) {
    if (readModel !== null) return readModel.state;
  }
  return ParentAgentActivityReadModelState.Unavailable;
}

function optionalSnapshot<TValue>(value: TValue | null | undefined): TValue | null {
  return value ?? null;
}
