#!/usr/bin/env node
import { spawn } from 'node:child_process';

import { ParentDevEnv, isLikelyParentPortalOccupant, resolveParentDevNetworkConfig } from './local-dev-config.mjs';
import { ensurePortFree } from './port-utils.mjs';

function log(message) {
  console.log(`[dev:portal] ${message}`);
}

const network = resolveParentDevNetworkConfig();
const productHostMode = process.argv.includes('--product-host');
const port = network.portalPort;
const isFree = await ensurePortFree(port, isLikelyParentPortalOccupant, log, network.portalBindHost);
if (!isFree) {
  throw new Error(`Cannot start Ocentra Parent portal because port ${port} is not available.`);
}

log(`Starting Vite portal on ${network.portalBindHost}:${port}; open ${network.portalCommandsUrl}.`);
if (productHostMode) {
  log('Host bridge mode Tauri invoke/listen; local dev bridge env is disabled for this portal process.');
} else {
  log(`Dev web bridge target ${network.parentBridgeUrl}.`);
}
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
    env: productHostMode
      ? { ...process.env }
      : {
          ...process.env,
          [ParentDevEnv.PortalAgentWebSocketUrl]: network.agentWebSocketUrl,
          [ParentDevEnv.PortalParentBridgeUrl]: network.parentBridgeUrl,
        },
  }
);

portal.on('exit', (code) => process.exit(code ?? 0));
portal.on('error', (error) => {
  console.error(`[dev:portal] ${error instanceof Error ? error.message : String(error)}`);
  process.exit(1);
});
