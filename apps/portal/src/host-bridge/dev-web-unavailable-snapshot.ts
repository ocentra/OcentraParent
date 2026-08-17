import {
  parentDevBridgeUnavailableDetail,
  ParentHostBridgeRuntime,
  type ParentDevBridgeUrl,
  type ParentRouteId,
  type ParentRouteSnapshot,
} from '../../generated/parent-ui-bridge';

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
    connectionState: 'error',
    commandEnabled: false,
    agentEndpoint: ParentHostBridgeRuntime.AgentEndpointDevWeb,
    dataSource: 'unavailable',
    summary: {
      title: parentDevBridgeUnavailableDetail(parentDevBridgeUrl),
      routeCapability: ParentHostBridgeRuntime.RouteCapabilityUnavailable,
      parentAccess: ParentHostBridgeRuntime.RouteCapabilityUnavailable,
      household: ParentHostBridgeRuntime.HouseholdUnavailable,
      childDevice: ParentHostBridgeRuntime.ChildDeviceUnavailable,
    },
    serviceHealth: {
      state: 'unavailable',
      route: null,
      protocolSchemaVersion: null,
      serviceVersion: null,
      transport: null,
      authenticationState: 'unavailable',
      reason: 'transport-unavailable',
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
