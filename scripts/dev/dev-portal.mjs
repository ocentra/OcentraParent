#!/usr/bin/env node
import { spawn } from 'node:child_process';

import {
  ParentDevEnv,
  ParentDevPort,
  isLikelyParentPortalOccupant,
  resolveParentDevNetworkConfig,
} from './local-dev-config.mjs';
import { ensurePortFree } from './port-utils.mjs';

function log(message) {
  console.log(`[dev:portal] ${message}`);
}

const network = resolveParentDevNetworkConfig();
const port = ParentDevPort.Portal;
const isFree = await ensurePortFree(port, isLikelyParentPortalOccupant, log, network.portalBindHost);
if (!isFree) {
  throw new Error(`Cannot start Ocentra Parent portal because port ${port} is not available.`);
}

log(`Starting Vite portal on ${network.portalBindHost}:${port}; open ${network.portalCommandsUrl}.`);
const portal = spawn(
  'npm',
  [
    'exec',
    '--workspace',
    '@ocentra-parent/portal',
    '--',
    'vite',
    '--host',
    network.portalBindHost,
    '--port',
    String(port),
    '--strictPort',
  ],
  {
    cwd: process.cwd(),
    stdio: 'inherit',
    shell: process.platform === 'win32',
    env: {
      ...process.env,
      [ParentDevEnv.PortalAgentWebSocketUrl]: network.agentWebSocketUrl,
    },
  }
);

portal.on('exit', (code) => process.exit(code ?? 0));
portal.on('error', (error) => {
  console.error(`[dev:portal] ${error instanceof Error ? error.message : String(error)}`);
  process.exit(1);
});
