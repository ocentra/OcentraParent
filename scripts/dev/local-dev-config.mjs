import os from 'node:os';

export const ParentDevNetworkMode = {
  Loopback: 'loopback',
  Lan: 'lan',
};

export const ParentDevHost = {
  Loopback: '127.0.0.1',
  Localhost: 'localhost',
  Wildcard: '0.0.0.0',
};

export const ParentDevPort = {
  Agent: 4477,
  Portal: 4478,
  WebSocketSmokeAgent: 4488,
  PortalSmokeAgent: 4489,
  PortalSmokePortal: 4490,
  LanWebSocketSmokeAgent: 4491,
};

export const ParentDevUrl = {
  AgentHealth: createAgentHealthUrl(ParentDevPort.Agent),
  AgentWebSocket: createAgentWebSocketUrl(ParentDevPort.Agent),
  PortalCommands: createPortalCommandsUrl(ParentDevPort.Portal),
};

export const ParentDevEnv = {
  AgentAddress: 'OCENTRA_PARENT_AGENT_ADDR',
  AgentAllowedOrigins: 'OCENTRA_PARENT_AGENT_ALLOWED_ORIGINS',
  AgentLocalNetworkEnabled: 'OCENTRA_PARENT_AGENT_LOCAL_NETWORK_ENABLED',
  DevNetworkMode: 'OCENTRA_PARENT_DEV_NETWORK',
  LanHost: 'OCENTRA_PARENT_LAN_HOST',
  PortalAgentWebSocketUrl: 'VITE_AGENT_WS_URL',
};

export const ParentDevValue = {
  True: 'true',
};

export function resolveParentDevNetworkConfig(
  env = process.env,
  interfaces = os.networkInterfaces(),
  args = process.argv
) {
  const mode = resolveParentDevNetworkMode(env, args);
  const lanHost =
    mode === ParentDevNetworkMode.Lan
      ? resolveParentLanHost(env[ParentDevEnv.LanHost], interfaces)
      : ParentDevHost.Loopback;
  const agentBindHost = mode === ParentDevNetworkMode.Lan ? ParentDevHost.Wildcard : ParentDevHost.Loopback;
  const portalBindHost = mode === ParentDevNetworkMode.Lan ? ParentDevHost.Wildcard : ParentDevHost.Loopback;
  const allowedOrigins = createAllowedOrigins(lanHost);

  return {
    mode,
    lanHost,
    agentBindHost,
    portalBindHost,
    agentAddress: createAgentAddress(ParentDevPort.Agent, agentBindHost),
    agentHealthUrl: createAgentHealthUrl(ParentDevPort.Agent, lanHost),
    agentWebSocketUrl: createAgentWebSocketUrl(ParentDevPort.Agent, lanHost),
    portalCommandsUrl: createPortalCommandsUrl(ParentDevPort.Portal, lanHost),
    allowedOrigins,
    localNetworkEnabled: mode === ParentDevNetworkMode.Lan,
  };
}

export function resolveParentDevNetworkMode(env = process.env, args = process.argv) {
  if (args.includes('--lan') || env[ParentDevEnv.DevNetworkMode] === ParentDevNetworkMode.Lan) {
    return ParentDevNetworkMode.Lan;
  }
  return ParentDevNetworkMode.Loopback;
}

export function resolveParentLanHost(explicitHost, interfaces = os.networkInterfaces()) {
  if (explicitHost !== undefined && explicitHost.trim().length > 0) {
    return explicitHost.trim();
  }

  for (const entries of Object.values(interfaces)) {
    for (const entry of entries ?? []) {
      if (entry.family === 'IPv4' && !entry.internal) {
        return entry.address;
      }
    }
  }

  throw new Error(`Cannot resolve LAN host. Set ${ParentDevEnv.LanHost} to this PC's local network IP.`);
}

export function createAllowedOrigins(host) {
  return [
    ...new Set([
      createHttpOrigin(ParentDevHost.Loopback),
      createHttpOrigin(ParentDevHost.Localhost),
      createHttpOrigin(host),
    ]),
  ];
}

export function createHttpOrigin(host, port = ParentDevPort.Portal) {
  return `http://${host}:${port}`;
}

export function createAgentAddress(port, host = ParentDevHost.Loopback) {
  return `${host}:${port}`;
}

export function createAgentHealthUrl(port, host = ParentDevHost.Loopback) {
  return `http://${host}:${port}/health`;
}

export function createAgentWebSocketUrl(port, host = ParentDevHost.Loopback) {
  return `ws://${host}:${port}/api/dev/ws`;
}

export function createPortalCommandsUrl(port, host = ParentDevHost.Loopback) {
  return `http://${host}:${port}/#/commands`;
}

export function isLikelyParentAgentOccupant(occupant) {
  const text = `${occupant.name} ${occupant.commandLine}`.toLowerCase();
  return (
    text.includes('ocentra-parent-agent-service') ||
    text.includes('ocentra_parent_agent_service') ||
    text.includes('ocentra_parent_agent') ||
    text.includes('ocentra_parent_agent_addr')
  );
}

export function isLikelyParentPortalOccupant(occupant) {
  const text = `${occupant.name} ${occupant.commandLine}`.toLowerCase();
  return text.includes('ocentraparent') || text.includes('ocentra-parent') || text.includes('@ocentra-parent/portal');
}
