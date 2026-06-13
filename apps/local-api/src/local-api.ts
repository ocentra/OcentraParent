import {
  AgentCommandName,
  AgentCommandNameSchema,
  AgentEventDeliveryMode,
  type AgentCommandName as AgentCommandNameValue,
} from '@ocentra-parent/agent-protocol-domain/contracts';
import {
  type Infer,
  NonEmptyStringSchema,
  Schema,
  brandedNonEmptyStringSchema,
  withParser,
} from '@ocentra-parent/schema-domain/effect';

const LocalApiPathTextSchema = NonEmptyStringSchema.pipe(
  Schema.filter((value) => value.startsWith('/') || 'Expected local API path to start with /')
);

export const LocalApiRouteIdSchema = brandedNonEmptyStringSchema('LocalApiRouteId');
export const LocalApiPathSchema = withParser(LocalApiPathTextSchema.pipe(Schema.brand('LocalApiPath')));
export const LocalApiOperationIdSchema = brandedNonEmptyStringSchema('LocalApiOperationId');
export const LocalApiRuntimeOwnerSchema = withParser(Schema.Literal('agent-service'));
export const LocalApiTransportSchema = withParser(Schema.Literal('http', 'websocket'));
export const LocalApiHttpMethodSchema = withParser(Schema.Literal('GET'));
export const LocalApiOperationKindSchema = withParser(
  Schema.Literal('health-read', 'dev-log-read', 'browser-intervention-page-read', 'agent-command-websocket')
);

export const LocalApiRouteSchema = withParser(
  Schema.Struct({
    routeId: LocalApiRouteIdSchema,
    path: LocalApiPathSchema,
    method: LocalApiHttpMethodSchema,
    transport: LocalApiTransportSchema,
    owner: LocalApiRuntimeOwnerSchema,
    operation: LocalApiOperationKindSchema,
    command: Schema.Union(AgentCommandNameSchema, Schema.Null),
    deliveryMode: Schema.optionalWith(Schema.Union(Schema.Literal(AgentEventDeliveryMode.RequestResponse), Schema.Null), {
      default: () => null,
    }),
  })
);

export const LocalApiManifestSchema = withParser(
  Schema.Struct({
    routes: Schema.Array(LocalApiRouteSchema),
  })
);

export type LocalApiRouteId = typeof LocalApiRouteIdSchema.Type;
export type LocalApiPath = typeof LocalApiPathSchema.Type;
export type LocalApiOperationId = typeof LocalApiOperationIdSchema.Type;
export type LocalApiRuntimeOwner = Infer<typeof LocalApiRuntimeOwnerSchema>;
export type LocalApiTransport = Infer<typeof LocalApiTransportSchema>;
export type LocalApiHttpMethod = Infer<typeof LocalApiHttpMethodSchema>;
export type LocalApiOperationKind = Infer<typeof LocalApiOperationKindSchema>;
export type LocalApiRoute = Infer<typeof LocalApiRouteSchema>;
export type LocalApiManifest = Infer<typeof LocalApiManifestSchema>;

export const LocalApiRouteId = {
  Health: LocalApiRouteIdSchema.parse('agent-service.health'),
  DevLogSnapshot: LocalApiRouteIdSchema.parse('agent-service.dev-log-snapshot'),
  BrowserInterventionPage: LocalApiRouteIdSchema.parse('agent-service.browser-intervention-page'),
  DevWebSocket: LocalApiRouteIdSchema.parse('agent-service.dev-websocket'),
} as const;

export const LocalApiPath = {
  Health: LocalApiPathSchema.parse('/health'),
  DevLogSnapshot: LocalApiPathSchema.parse('/api/dev/log-snapshot'),
  BrowserInterventionPage: LocalApiPathSchema.parse('/api/browser/intervention/page'),
  DevWebSocket: LocalApiPathSchema.parse('/api/dev/ws'),
} as const;

export const LocalApiManifest = LocalApiManifestSchema.parse({
  routes: [
    {
      routeId: LocalApiRouteId.Health,
      path: LocalApiPath.Health,
      method: 'GET',
      transport: 'http',
      owner: 'agent-service',
      operation: 'health-read',
      command: null,
      deliveryMode: null,
    },
    {
      routeId: LocalApiRouteId.DevLogSnapshot,
      path: LocalApiPath.DevLogSnapshot,
      method: 'GET',
      transport: 'http',
      owner: 'agent-service',
      operation: 'dev-log-read',
      command: null,
      deliveryMode: null,
    },
    {
      routeId: LocalApiRouteId.BrowserInterventionPage,
      path: LocalApiPath.BrowserInterventionPage,
      method: 'GET',
      transport: 'http',
      owner: 'agent-service',
      operation: 'browser-intervention-page-read',
      command: null,
      deliveryMode: null,
    },
    {
      routeId: LocalApiRouteId.DevWebSocket,
      path: LocalApiPath.DevWebSocket,
      method: 'GET',
      transport: 'websocket',
      owner: 'agent-service',
      operation: 'agent-command-websocket',
      command: AgentCommandName.HealthCheck,
      deliveryMode: AgentEventDeliveryMode.RequestResponse,
    },
  ],
} satisfies LocalApiManifest);

export function localApiRouteForCommand(command: AgentCommandNameValue): LocalApiRoute {
  AgentCommandNameSchema.parse(command);
  return LocalApiManifest.routes.find((route) => route.command === command) ?? localApiWebSocketRoute();
}

export function localApiRouteById(routeId: LocalApiRouteId): LocalApiRoute | null {
  const parsedRouteId = LocalApiRouteIdSchema.parse(routeId);
  return LocalApiManifest.routes.find((route) => route.routeId === parsedRouteId) ?? null;
}

function localApiWebSocketRoute(): LocalApiRoute {
  const route = localApiRouteById(LocalApiRouteId.DevWebSocket);
  if (route === null) {
    throw new Error('Expected local API manifest to include the agent-service WebSocket route');
  }
  return route;
}
