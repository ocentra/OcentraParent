import { Schema } from '@ocentra-parent/schema-domain/effect';
import {
  AgentMessageTargetSchema,
  AgentPeerSchema,
  AgentProtocolSchemaVersion,
  AgentWebSocketUrlSchema,
} from './primitives';
import { AgentPairingStateSchema, AgentRouteSecurityPolicySchema } from './security';

const decodeAgentWebSocketUrl = Schema.decodeUnknownSync(AgentWebSocketUrlSchema);

export const AgentProtocolDefaults = {
  SchemaVersion: AgentProtocolSchemaVersion,
  WebSocketUrl: decodeAgentWebSocketUrl('ws://127.0.0.1:4477/api/dev/ws'),
  MessageIdPrefix: 'cmd-',
  Peer: {
    PortalDev: AgentPeerSchema.parse({
      peerId: 'portal-dev',
      role: 'portal',
    }),
  },
  Target: {
    LocalhostWindowsAgent: AgentMessageTargetSchema.parse({
      deviceId: 'local-dev-agent',
      platform: 'windows',
      route: 'localhost',
    }),
    LocalNetworkWindowsAgent: AgentMessageTargetSchema.parse({
      deviceId: 'local-dev-agent',
      platform: 'windows',
      route: 'local-network',
    }),
  },
  PairingState: {
    Unpaired: AgentPairingStateSchema.parse('unpaired'),
    Pairing: AgentPairingStateSchema.parse('pairing'),
    Paired: AgentPairingStateSchema.parse('paired'),
    Revoked: AgentPairingStateSchema.parse('revoked'),
  },
  RouteSecurity: {
    Localhost: AgentRouteSecurityPolicySchema.parse({
      route: 'localhost',
      requiresPairing: false,
      allowsAnonymousControl: true,
    }),
    LocalNetwork: AgentRouteSecurityPolicySchema.parse({
      route: 'local-network',
      requiresPairing: true,
      allowsAnonymousControl: false,
    }),
    CloudRelay: AgentRouteSecurityPolicySchema.parse({
      route: 'cloud-relay',
      requiresPairing: true,
      allowsAnonymousControl: false,
    }),
  },
  Host: {
    LoopbackIp: '127.0.0.1',
    LocalhostName: 'localhost',
  },
  Field: {
    Available: 'available',
    ActiveState: 'activeState',
    AdapterId: 'adapterId',
    BrowserEvidenceId: 'browserEvidenceId',
    BrowserFamily: 'browserFamily',
    CapabilityStatus: 'capabilityStatus',
    ActivityDigest: 'activityDigest',
    CustodyLabel: 'custodyLabel',
    DatabaseReady: 'databaseReady',
    Domain: 'domain',
    DuplicateEvents: 'duplicateEvents',
    Entries: 'entries',
    EventsIngested: 'eventsIngested',
    EventsStored: 'eventsStored',
    FirstObservedAt: 'firstObservedAt',
    LastEventId: 'lastEventId',
    LastObservedAt: 'lastObservedAt',
    LatestEventId: 'latestEventId',
    LatestObservedAt: 'latestObservedAt',
    Limit: 'limit',
    ManagedBrowserSessionId: 'managedBrowserSessionId',
    Message: 'message',
    MostRecentKind: 'mostRecentKind',
    MostRecentObserver: 'mostRecentObserver',
    MostRecentSubjectId: 'mostRecentSubjectId',
    MostRecentSubjectKind: 'mostRecentSubjectKind',
    MostRecentSubjectName: 'mostRecentSubjectName',
    Online: 'online',
    Origin: 'origin',
    Reason: 'reason',
    Returned: 'returned',
    SourceId: 'sourceId',
    Transport: 'transport',
    Title: 'title',
    Url: 'url',
  },
  Primitive: {
    String: 'string',
  },
} as const;
