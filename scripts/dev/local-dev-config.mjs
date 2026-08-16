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
  ParentBridge: 4479,
  WebSocketSmokeAgent: 4488,
  PortalSmokeAgent: 4489,
  PortalSmokePortal: 4490,
  LanWebSocketSmokeAgent: 4491,
};

export const ParentDevUrl = {
  AgentHealth: createAgentHealthUrl(ParentDevPort.Agent),
  AgentWebSocket: createAgentWebSocketUrl(ParentDevPort.Agent),
  ParentBridge: createParentDevBridgeUrl(ParentDevPort.ParentBridge),
  PortalCommands: createPortalCommandsUrl(ParentDevPort.Portal),
};

export const ParentDevEnv = {
  AgentAddress: 'OCENTRA_PARENT_AGENT_ADDR',
  AgentAllowedOrigins: 'OCENTRA_PARENT_AGENT_ALLOWED_ORIGINS',
  AgentLocalNetworkEnabled: 'OCENTRA_PARENT_AGENT_LOCAL_NETWORK_ENABLED',
  AgentPort: 'OCENTRA_PARENT_AGENT_PORT',
  ActivityDbPath: 'OCENTRA_PARENT_ACTIVITY_DB_PATH',
  DevLogDir: 'OCENTRA_PARENT_DEV_LOG_DIR',
  DevNetworkMode: 'OCENTRA_PARENT_DEV_NETWORK',
  LanHost: 'OCENTRA_PARENT_LAN_HOST',
  ParentBridgePort: 'OCENTRA_PARENT_PARENT_BRIDGE_PORT',
  PortalAgentWebSocketUrl: 'VITE_AGENT_WS_URL',
  PortalParentBridgeUrl: 'VITE_PARENT_DEV_BRIDGE_URL',
  PortalPort: 'OCENTRA_PARENT_PORTAL_PORT',
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
  const agentPort = resolveParentDevPort(env[ParentDevEnv.AgentPort], ParentDevPort.Agent, ParentDevEnv.AgentPort);
  const portalPort = resolveParentDevPort(env[ParentDevEnv.PortalPort], ParentDevPort.Portal, ParentDevEnv.PortalPort);
  const parentBridgePort = resolveParentDevPort(
    env[ParentDevEnv.ParentBridgePort],
    defaultParentBridgePort(portalPort),
    ParentDevEnv.ParentBridgePort
  );
  const lanHost =
    mode === ParentDevNetworkMode.Lan
      ? resolveParentLanHost(env[ParentDevEnv.LanHost], interfaces)
      : ParentDevHost.Loopback;
  const agentBindHost = mode === ParentDevNetworkMode.Lan ? ParentDevHost.Wildcard : ParentDevHost.Loopback;
  const agentConnectHost = ParentDevHost.Loopback;
  const portalBindHost = mode === ParentDevNetworkMode.Lan ? ParentDevHost.Wildcard : ParentDevHost.Loopback;
  const parentBridgeBindHost = mode === ParentDevNetworkMode.Lan ? ParentDevHost.Wildcard : ParentDevHost.Loopback;
  const allowedOrigins = createAllowedOrigins(lanHost, portalPort);

  return {
    mode,
    lanHost,
    agentPort,
    portalPort,
    parentBridgePort,
    agentBindHost,
    agentConnectHost,
    portalBindHost,
    parentBridgeBindHost,
    agentAddress: createAgentAddress(agentPort, agentBindHost),
    agentConnectAddress: createAgentAddress(agentPort, agentConnectHost),
    agentHealthUrl: createAgentHealthUrl(agentPort, lanHost),
    agentWebSocketUrl: createAgentWebSocketUrl(agentPort, lanHost),
    parentBridgeAddress: createAgentAddress(parentBridgePort, parentBridgeBindHost),
    parentBridgeUrl: createParentDevBridgeUrl(parentBridgePort, lanHost),
    portalCommandsUrl: createPortalCommandsUrl(portalPort, lanHost),
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

export function resolveParentDevPort(value, defaultPort, envName) {
  if (value === undefined || value.trim().length === 0) {
    return defaultPort;
  }

  const port = Number(value.trim());
  if (!Number.isInteger(port) || port < 1 || port > 65535) {
    throw new Error(`${envName} must be an integer TCP port between 1 and 65535.`);
  }
  return port;
}

function defaultParentBridgePort(portalPort) {
  return portalPort >= 65535 ? ParentDevPort.ParentBridge : portalPort + 1;
}

export function createAllowedOrigins(host, port = ParentDevPort.Portal) {
  return [
    ...new Set([
      createHttpOrigin(ParentDevHost.Loopback, port),
      createHttpOrigin(ParentDevHost.Localhost, port),
      createHttpOrigin(host, port),
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

export function createParentDevBridgeUrl(port, host = ParentDevHost.Loopback) {
  return `http://${host}:${port}/api/parent-ui`;
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

export function isLikelyParentBridgeOccupant(occupant) {
  const text = `${occupant.name} ${occupant.commandLine}`.toLowerCase();
  return text.includes('ocentra-parent-dev-bridge') || text.includes('ocentra_parent_dev_bridge');
}
