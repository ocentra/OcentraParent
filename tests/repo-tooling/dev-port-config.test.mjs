import net from 'node:net';
import test from 'node:test';
import assert from 'node:assert/strict';

import {
  ParentDevEnv,
  ParentDevHost,
  ParentDevNetworkMode,
  ParentDevPort,
  createAllowedOrigins,
  resolveParentDevNetworkConfig,
  isLikelyParentAgentOccupant,
  isLikelyParentPortalOccupant,
} from '../../scripts/dev/local-dev-config.mjs';
import { createParentDesktopDevEnv } from '../../scripts/dev/dev-parent-desktop.mjs';
import { isPortAvailable, waitForPort } from '../../scripts/dev/port-utils.mjs';

test('dev port config avoids Ocentra Games and generic Vite ports', () => {
  const reservedElsewhere = new Set([3000, 5173, 5174, 8787]);
  const parentPorts = Object.values(ParentDevPort);

  assert.equal(new Set(parentPorts).size, parentPorts.length);
  for (const port of parentPorts) {
    assert.equal(reservedElsewhere.has(port), false);
  }
});

test('LAN dev mode binds services to the network without changing fixed ports', () => {
  const config = resolveParentDevNetworkConfig(
    { [ParentDevEnv.DevNetworkMode]: ParentDevNetworkMode.Lan },
    { Ethernet: [{ family: 'IPv4', internal: false, address: '192.168.50.25' }] },
    []
  );

  assert.equal(config.mode, ParentDevNetworkMode.Lan);
  assert.equal(config.agentBindHost, ParentDevHost.Wildcard);
  assert.equal(config.agentConnectHost, ParentDevHost.Loopback);
  assert.equal(config.portalBindHost, ParentDevHost.Wildcard);
  assert.equal(config.agentAddress, `0.0.0.0:${ParentDevPort.Agent}`);
  assert.equal(config.agentConnectAddress, `127.0.0.1:${ParentDevPort.Agent}`);
  assert.equal(config.agentWebSocketUrl, `ws://192.168.50.25:${ParentDevPort.Agent}/api/dev/ws`);
  assert.deepEqual(config.allowedOrigins, createAllowedOrigins('192.168.50.25'));
});

test('dev port config supports explicit lane port overrides', () => {
  const config = resolveParentDevNetworkConfig(
    {
      [ParentDevEnv.AgentPort]: '4677',
      [ParentDevEnv.PortalPort]: '4678',
    },
    {},
    []
  );

  assert.equal(config.mode, ParentDevNetworkMode.Loopback);
  assert.equal(config.agentPort, 4677);
  assert.equal(config.portalPort, 4678);
  assert.equal(config.agentAddress, '127.0.0.1:4677');
  assert.equal(config.agentConnectAddress, '127.0.0.1:4677');
  assert.equal(config.agentHealthUrl, 'http://127.0.0.1:4677/health');
  assert.equal(config.agentWebSocketUrl, 'ws://127.0.0.1:4677/api/dev/ws');
  assert.equal(config.portalCommandsUrl, 'http://127.0.0.1:4678/#/commands');
  assert.deepEqual(config.allowedOrigins, ['http://127.0.0.1:4678', 'http://localhost:4678']);
});

test('dev port config uses portal override for LAN origins', () => {
  const config = resolveParentDevNetworkConfig(
    {
      [ParentDevEnv.DevNetworkMode]: ParentDevNetworkMode.Lan,
      [ParentDevEnv.AgentPort]: '4777',
      [ParentDevEnv.PortalPort]: '4778',
    },
    { Ethernet: [{ family: 'IPv4', internal: false, address: '192.168.50.25' }] },
    []
  );

  assert.equal(config.agentAddress, '0.0.0.0:4777');
  assert.equal(config.agentConnectAddress, '127.0.0.1:4777');
  assert.equal(config.agentWebSocketUrl, 'ws://192.168.50.25:4777/api/dev/ws');
  assert.equal(config.portalCommandsUrl, 'http://192.168.50.25:4778/#/commands');
  assert.deepEqual(config.allowedOrigins, [
    'http://127.0.0.1:4778',
    'http://localhost:4778',
    'http://192.168.50.25:4778',
  ]);
});

test('parent desktop dev env uses the LAN runtime connect address and allowed origins', () => {
  const config = resolveParentDevNetworkConfig(
    {
      [ParentDevEnv.DevNetworkMode]: ParentDevNetworkMode.Lan,
      [ParentDevEnv.AgentPort]: '4777',
      [ParentDevEnv.PortalPort]: '4778',
    },
    { Ethernet: [{ family: 'IPv4', internal: false, address: '192.168.50.25' }] },
    []
  );
  const devEnv = createParentDesktopDevEnv(config, { KEEP_EXISTING_ENV: 'true' });

  assert.equal(devEnv.KEEP_EXISTING_ENV, 'true');
  assert.equal(devEnv[ParentDevEnv.AgentAddress], '127.0.0.1:4777');
  assert.equal(
    devEnv[ParentDevEnv.AgentAllowedOrigins],
    'http://127.0.0.1:4778,http://localhost:4778,http://192.168.50.25:4778'
  );
  assert.equal(devEnv[ParentDevEnv.AgentPort], '4777');
  assert.equal(devEnv[ParentDevEnv.ParentBridgePort], '4779');
  assert.equal(devEnv[ParentDevEnv.PortalPort], '4778');
  assert.equal(devEnv[ParentDevEnv.DevNetworkMode], ParentDevNetworkMode.Lan);
  assert.equal(devEnv[ParentDevEnv.PortalAgentWebSocketUrl], 'ws://192.168.50.25:4777/api/dev/ws');
});

test('dev port config rejects invalid explicit port overrides', () => {
  assert.throws(
    () => resolveParentDevNetworkConfig({ [ParentDevEnv.AgentPort]: 'not-a-port' }, {}, []),
    /OCENTRA_PARENT_AGENT_PORT/u
  );
  assert.throws(() => resolveParentDevNetworkConfig({ [ParentDevEnv.PortalPort]: '70000' }, {}, []), /65535/u);
});

test('dev port occupant predicates only match Ocentra Parent processes', () => {
  assert.equal(
    isLikelyParentAgentOccupant({
      pid: 1,
      name: 'ocentra-parent-agent-service.exe',
      commandLine: 'target debug ocentra-parent-agent-service.exe',
    }),
    true
  );
  assert.equal(
    isLikelyParentPortalOccupant({
      pid: 2,
      name: 'node.exe',
      commandLine: 'E:\\OcentraParent\\node_modules\\vite\\bin\\vite.js',
    }),
    true
  );
  assert.equal(
    isLikelyParentPortalOccupant({
      pid: 3,
      name: 'node.exe',
      commandLine: 'E:\\ocentra-games\\packages\\asset-editor\\node_modules\\vite\\bin\\vite.js',
    }),
    false
  );
});

test('port utility detects a real listening server', async () => {
  const server = net.createServer();
  await new Promise((resolve) => server.listen(0, '127.0.0.1', resolve));
  const address = server.address();
  assert.equal(typeof address, 'object');

  try {
    assert.equal(await isPortAvailable(address.port), false);
    await waitForPort(address.port, 1000);
  } finally {
    await new Promise((resolve) => server.close(resolve));
  }

  assert.equal(await isPortAvailable(address.port), true);
});
