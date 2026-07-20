import {
  ParentAgentPeerDefaults,
  ParentAgentPeerRole,
  ParentAgentProtocolLogLevel,
  ParentAgentTargetDefaults,
  type ParentRouteEventSnapshot,
} from '../generated/parent-ui-bridge';

export function hasCanonicalLanReplayProvenance(replay: ParentRouteEventSnapshot): boolean {
  return [
    replay.sourcePeerId === ParentAgentTargetDefaults.LocalhostWindowsAgent.deviceId,
    replay.sourceRole === ParentAgentPeerRole.AgentService,
    replay.targetPeerId === ParentAgentPeerDefaults.PortalDev.peerId,
    replay.targetRole === ParentAgentPeerRole.Portal,
    replay.severity === ParentAgentProtocolLogLevel.Info,
  ].every(Boolean);
}
