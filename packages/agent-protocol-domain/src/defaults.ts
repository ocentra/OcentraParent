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
    Message: 'message',
  },
  Primitive: {
    String: 'string',
  },
} as const;
