#!/usr/bin/env node
import { spawn } from 'node:child_process';

import { isLikelyParentBridgeOccupant, ParentDevEnv, resolveParentDevNetworkConfig } from './local-dev-config.mjs';
import { ensurePortFree } from './port-utils.mjs';

function log(message) {
  console.log(`[dev:bridge] ${message}`);
}

const network = resolveParentDevNetworkConfig();
const port = network.parentBridgePort;
const isFree = await ensurePortFree(port, isLikelyParentBridgeOccupant, log, network.parentBridgeBindHost);
if (!isFree) {
  throw new Error(`Cannot start Ocentra Parent dev bridge because port ${port} is not available.`);
}

log(`Starting Rust parent dev bridge on ${network.parentBridgeAddress}.`);
const bridge = spawn('cargo', ['run', '--manifest-path', 'crates/parent-dev-bridge/Cargo.toml'], {
  cwd: process.cwd(),
  stdio: 'inherit',
  shell: process.platform === 'win32',
  env: {
    ...process.env,
    [ParentDevEnv.AgentAddress]: network.agentConnectAddress,
    [ParentDevEnv.AgentAllowedOrigins]: network.allowedOrigins.join(','),
    [ParentDevEnv.AgentPort]: String(network.agentPort),
    [ParentDevEnv.ParentBridgePort]: String(network.parentBridgePort),
    [ParentDevEnv.DevNetworkMode]: network.mode,
  },
});

bridge.on('exit', (code) => process.exit(code ?? 0));
bridge.on('error', (error) => {
  console.error(`[dev:bridge] ${error instanceof Error ? error.message : String(error)}`);
  process.exit(1);
});
