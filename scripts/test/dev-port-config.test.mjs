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
} from '../dev/local-dev-config.mjs';
import { isPortAvailable, waitForPort } from '../dev/port-utils.mjs';

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
  assert.equal(config.portalBindHost, ParentDevHost.Wildcard);
  assert.equal(config.agentAddress, `0.0.0.0:${ParentDevPort.Agent}`);
  assert.equal(config.agentWebSocketUrl, `ws://192.168.50.25:${ParentDevPort.Agent}/api/dev/ws`);
  assert.deepEqual(config.allowedOrigins, createAllowedOrigins('192.168.50.25'));
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
