import {
  parentDevBridgeUnavailableDetail,
  ParentBridgeConnectionState,
  ParentHostBridgeRuntime,
  ParentRouteDataSource,
  ParentServiceHealthAuthenticationState,
  ParentServiceHealthReason,
  ParentServiceHealthState,
  type ParentDevBridgeUrl,
  type ParentRouteAgentEndpoint,
  type ParentRouteId,
  type ParentRouteSnapshot,
  type ParentRouteSummary,
  type ParentServiceHealthReason as ParentServiceHealthReasonValue,
} from '../../generated/parent-ui-bridge';
import { PORTAL_HOST_BRIDGE_RUNTIME } from '@ocentra-parent/portal-domain/portal-host-bridge-runtime';

export function createUnavailableDevWebRouteSnapshot(
  parentDevBridgeUrl: ParentDevBridgeUrl,
  route: ParentRouteId
): ParentRouteSnapshot {
  return createUnavailableParentRouteSnapshot(
    route,
    ParentHostBridgeRuntime.AgentEndpointDevWeb,
    parentDevBridgeUnavailableDetail(parentDevBridgeUrl),
    ParentServiceHealthReason.TransportUnavailable
  );
}

export function createSchemaMismatchParentRouteSnapshot(
  route: ParentRouteId,
  agentEndpoint: ParentRouteAgentEndpoint = ParentHostBridgeRuntime.AgentEndpointPending
): ParentRouteSnapshot {
  return createUnavailableParentRouteSnapshot(
    route,
    agentEndpoint,
    PORTAL_HOST_BRIDGE_RUNTIME.RouteSchemaMismatchTitle,
    ParentServiceHealthReason.ResponseSchemaMismatch
  );
}

export function createIdentityMismatchParentRouteSnapshot(
  route: ParentRouteId,
  agentEndpoint: ParentRouteAgentEndpoint = ParentHostBridgeRuntime.AgentEndpointPending
): ParentRouteSnapshot {
  return createUnavailableParentRouteSnapshot(
    route,
    agentEndpoint,
    PORTAL_HOST_BRIDGE_RUNTIME.RouteIdentityMismatchTitle,
    ParentServiceHealthReason.ResponseIdentityMismatch
  );
}

function createUnavailableParentRouteSnapshot(
  route: ParentRouteId,
  agentEndpoint: ParentRouteAgentEndpoint,
  title: ParentRouteSummary[keyof ParentRouteSummary],
  reason: ParentServiceHealthReasonValue
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
    agentEndpoint,
    dataSource: ParentRouteDataSource.Unavailable,
    summary: {
      title,
      routeCapability: ParentHostBridgeRuntime.RouteCapabilityUnavailable,
      parentAccess: ParentHostBridgeRuntime.RouteCapabilityUnavailable,
      household: ParentHostBridgeRuntime.HouseholdUnavailable,
      childDevice: ParentHostBridgeRuntime.ChildDeviceUnavailable,
    },
    serviceHealth: {
      state: ParentServiceHealthState.Unavailable,
      route: null,
      protocolSchemaVersion: null,
      serviceVersion: null,
      transport: null,
      authenticationState: ParentServiceHealthAuthenticationState.Unavailable,
      reason,
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
