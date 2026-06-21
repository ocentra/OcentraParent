import type { AgentProtocolLogFields } from '@ocentra-parent/schema-domain/agent-command-event-contracts';
import { AgentProtocolDefaults } from '@ocentra-parent/schema-domain/agent-protocol-defaults';
import {
  parseActivityMemoryGraphDigest,
  type ActivityMemoryGraphEdge,
  type ActivityMemoryGraphNode,
  type ActivityMemoryGraphNodeId,
  type ActivityMemoryGraphReadModel,
} from '@ocentra-parent/schema-domain/activity-memory-graph';

export type PortalActivityMemoryGraphReadModel = ActivityMemoryGraphReadModel;
export type PortalActivityMemoryGraphNode = ActivityMemoryGraphNode;
export type PortalActivityMemoryGraphEdge = ActivityMemoryGraphEdge;
export type PortalActivityMemoryGraphNodeId = ActivityMemoryGraphNodeId;

export function parseActivityMemoryGraphReadModel(
  payload: AgentProtocolLogFields
): PortalActivityMemoryGraphReadModel | null {
  const digest = payload[AgentProtocolDefaults.Field.ActivityDigest];
  if (typeof digest !== 'string') {
    return null;
  }
  return parseActivityMemoryGraphDigest(digest);
}

