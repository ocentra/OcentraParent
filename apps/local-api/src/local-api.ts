import { AgentCommandNameSchema, type AgentCommandName } from '@ocentra-parent/schema-domain/agent-command-event-contracts';
import {
  LocalApiManifest,
  LocalApiRouteId,
  type LocalApiRoute,
  type LocalApiRouteId as LocalApiRouteIdValue,
} from '@ocentra-parent/schema-domain/local-api-contracts';

export function localApiRouteForCommand(command: AgentCommandName): LocalApiRoute {
  AgentCommandNameSchema.parse(command);
  return LocalApiManifest.routes.find((route) => route.command === command) ?? localApiWebSocketRoute();
}

export function localApiRouteById(routeId: LocalApiRouteIdValue): LocalApiRoute | null {
  return LocalApiManifest.routes.find((route) => route.routeId === routeId) ?? null;
}

function localApiWebSocketRoute(): LocalApiRoute {
  const route = localApiRouteById(LocalApiRouteId.DevWebSocket);
  if (route === null) {
    throw new Error('Expected local API manifest to include the agent-service WebSocket route');
  }
  return route;
}
