import {
  parentDevBridgeUnavailableDetail,
  ParentBridgeConnectionState,
  ParentHostBridgeRuntime,
  ParentRouteDataSource,
  type ParentDevBridgeUrl,
  type ParentRouteId,
  type ParentRouteSnapshot,
} from '../../generated/parent-ui-bridge';
import { PORTAL_HOST_BRIDGE_RUNTIME as PortalRuntime } from '@ocentra-parent/portal-domain/parent-portal-service-state';

export function createUnavailableDevWebRouteSnapshot(
  parentDevBridgeUrl: ParentDevBridgeUrl,
  route: ParentRouteId
): ParentRouteSnapshot {
  const timestamp = new Date().toISOString();
  return {
    schemaVersion: ParentHostBridgeRuntime.SchemaVersion,
    route,
    generatedAt: timestamp,
    seasonLabel: ParentHostBridgeRuntime.SeasonLabelLocal,
    lastUpdated: timestamp,
    connectionState: ParentBridgeConnectionState.Error,
    commandEnabled: false,
    agentEndpoint: ParentHostBridgeRuntime.AgentEndpointDevWeb,
    dataSource: ParentRouteDataSource.Unavailable,
    summary: {
      title: parentDevBridgeUnavailableDetail(parentDevBridgeUrl),
      routeCapability: ParentHostBridgeRuntime.RouteCapabilityUnavailable,
      parentAccess: ParentHostBridgeRuntime.RouteCapabilityUnavailable,
      household: ParentHostBridgeRuntime.HouseholdUnavailable,
      childDevice: ParentHostBridgeRuntime.ChildDeviceUnavailable,
    },
    serviceHealth: {
      state: PortalRuntime.UnavailableState,
      route: null,
      protocolSchemaVersion: null,
      serviceVersion: null,
      transport: null,
      authenticationState: PortalRuntime.UnavailableState,
      reason: PortalRuntime.TransportUnavailableReason,
      trace: {
        requestId: null,
        correlationId: null,
        responseEventId: null,
        requestSentAt: null,
        responseSentAt: null,
      },
    },
    diagnosticPanelsEnabled: false,
    parentPortalRows: null,
    parentPortalShellStatus: null,
    liveActivity: null,
    browserPanels: null,
    setupFirstRunPanel: null,
    screenSettingsServiceResponse: null,
  };
}
