import {
  GeneratedPortalAgentProtocolField,
  type GeneratedPortalAgentProtocolPayload,
  decodeGeneratedPortalActivityMemoryGraphDigest,
  type GeneratedPortalActivityMemoryGraphEdgeSnapshot,
  type GeneratedPortalActivityMemoryGraphNodeId,
  type GeneratedPortalActivityMemoryGraphNodeSnapshot,
  type GeneratedPortalActivityMemoryGraphReadModelSnapshot,
  type GeneratedPortalRouteEventPayloadRecord,
} from './generated-portal-contracts';

export type PortalActivityMemoryGraphReadModel = GeneratedPortalActivityMemoryGraphReadModelSnapshot;
export type PortalActivityMemoryGraphNode = GeneratedPortalActivityMemoryGraphNodeSnapshot;
export type PortalActivityMemoryGraphEdge = GeneratedPortalActivityMemoryGraphEdgeSnapshot;
export type PortalActivityMemoryGraphNodeId = GeneratedPortalActivityMemoryGraphNodeId;

export function parseActivityMemoryGraphReadModel(
  payload: GeneratedPortalAgentProtocolPayload | GeneratedPortalRouteEventPayloadRecord | null | undefined
): PortalActivityMemoryGraphReadModel | null {
  const digest = payload?.[GeneratedPortalAgentProtocolField.ActivityDigest];
  if (typeof digest !== 'string') {
    return null;
  }
  return decodeGeneratedPortalActivityMemoryGraphDigest(digest);
}
