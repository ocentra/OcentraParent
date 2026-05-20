#!/usr/bin/env node
import { spawn } from 'node:child_process';

import {
  ParentDevEnv,
  ParentDevValue,
  isLikelyParentAgentOccupant,
  resolveParentDevNetworkConfig,
} from './local-dev-config.mjs';
import { ensurePortFree } from './port-utils.mjs';

function log(message) {
  console.log(`[dev:agent] ${message}`);
}

const network = resolveParentDevNetworkConfig();
const port = network.agentPort;
const agentEnv = {
  [ParentDevEnv.AgentAddress]: network.agentAddress,
  [ParentDevEnv.AgentAllowedOrigins]: network.allowedOrigins.join(','),
};
if (network.localNetworkEnabled) {
  agentEnv[ParentDevEnv.AgentLocalNetworkEnabled] = ParentDevValue.True;
}

const isFree = await ensurePortFree(port, isLikelyParentAgentOccupant, log, network.agentBindHost);
if (!isFree) {
  throw new Error(`Cannot start Ocentra Parent agent because port ${port} is not available.`);
}

log(`Starting headless Rust agent on ${network.agentAddress} in ${network.mode} mode.`);
const agent = spawn('cargo', ['run', '-p', 'ocentra-parent-agent-service'], {
  cwd: process.cwd(),
  stdio: 'inherit',
  shell: process.platform === 'win32',
  env: {
    ...process.env,
    ...agentEnv,
  },
});

agent.on('exit', (code) => process.exit(code ?? 0));
agent.on('error', (error) => {
  console.error(`[dev:agent] ${error instanceof Error ? error.message : String(error)}`);
  process.exit(1);
});
