import {
  EMPTY_PORTAL_LIVE_ACTIVITY_STATE,
  type PortalLiveActivityState,
} from '@ocentra-parent/portal-domain/live-activity-state';
import type {
  ParentAppGameAdapterDispatchPanelSnapshot,
  ParentAppGameNotificationParentSurfacePanelSnapshot,
  ParentAppGamePanelSnapshot,
  ParentAppGameTimerParentSurfacePanelSnapshot,
  ParentNetworkEvidenceSummarySnapshot,
  ParentPolicyPreviewPanelSnapshot,
  ParentRouteLiveActivitySnapshot,
  ParentScreenSummaryPanelSnapshot,
} from '../generated/parent-ui-bridge';

type ResolvedPortalLiveActivityState = PortalLiveActivityState & {
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

export const EMPTY_ROUTE_LIVE_ACTIVITY_STATE = {
  ...EMPTY_PORTAL_LIVE_ACTIVITY_STATE,
  appGameNotificationParentSurfacePanel: null,
  appGamePlatformProofStatusPanel: null,
  appGameChildRuntimeTransportReceiptPanel: null,
  appGamePolicyReadinessPanel: null,
} satisfies ResolvedPortalLiveActivityState;

export function resolveSnapshotLiveActivityState(
  snapshot?: ParentRouteLiveActivitySnapshot | null
): ResolvedPortalLiveActivityState {
  if (snapshot === null || snapshot === undefined) {
    return EMPTY_ROUTE_LIVE_ACTIVITY_STATE;
  }
  // The Rust-owned bridge snapshot is structurally compatible here even when
  // the generated TS surface widens some fields to unknown records.
  const resolvedSnapshot = snapshot as Partial<ResolvedPortalLiveActivityState>;
  return {
    ...EMPTY_ROUTE_LIVE_ACTIVITY_STATE,
    ...resolvedSnapshot,
  };
}
