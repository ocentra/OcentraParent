import {
  ParentHostBridgeRuntime,
  decodeParentRouteSnapshot,
  type ParentRouteAgentEndpoint,
  type ParentRouteId,
  type ParentRouteSnapshot,
} from '../../generated/parent-ui-bridge';
import {
  createIdentityMismatchParentRouteSnapshot,
  createSchemaMismatchParentRouteSnapshot,
} from './dev-web-unavailable-snapshot';

export function decodeHostRouteSnapshot(
  value: unknown,
  route: ParentRouteId,
  agentEndpoint: ParentRouteAgentEndpoint = ParentHostBridgeRuntime.AgentEndpointPending
): ParentRouteSnapshot {
  let snapshot: ParentRouteSnapshot;
  try {
    snapshot = decodeParentRouteSnapshot(value);
  } catch {
    return createSchemaMismatchParentRouteSnapshot(route, agentEndpoint);
  }
  return snapshot.route === route ? snapshot : createIdentityMismatchParentRouteSnapshot(route, agentEndpoint);
}
