import { execFileSync, spawn } from 'node:child_process';
import { existsSync } from 'node:fs';
import path from 'node:path';
import { setInterval as keepAlive, clearInterval } from 'node:timers';
import { setTimeout as delay } from 'node:timers/promises';
import { fileURLToPath } from 'node:url';

import {
  ParentDevEnv,
  ParentDevHost,
  ParentDevPort,
  createAgentAddress,
  createAgentHealthUrl,
  createAgentWebSocketUrl,
  createHttpOrigin,
  isLikelyParentAgentOccupant,
  isLikelyParentPortalOccupant,
} from '../dev/local-dev-config.mjs';
import { ensurePortFree } from '../dev/port-utils.mjs';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const agentPort = ParentDevPort.PortalSmokeAgent;
const portalPort = ParentDevPort.PortalSmokePortal;
const children = [];

await prepareBuildOutputs();
await ensurePortFree(agentPort, isLikelyParentAgentOccupant, console.log);
await ensurePortFree(portalPort, isLikelyParentPortalOccupant, console.log);

const agent = spawnAgent();
children.push(agent);

await waitForHttp(createAgentHealthUrl(agentPort));

const portal = spawnPortal();
children.push(portal);

for (const child of children) {
  child.once('exit', (code) => {
    stopChildren();
    process.exit(code ?? 1);
  });
}

const interval = keepAlive(() => undefined, 1000);
process.once('SIGINT', shutdown);
process.once('SIGTERM', shutdown);

function prepareBuildOutputs() {
  if (!existsSync(path.join(repoRoot, 'packages', 'portal-domain', 'dist', 'contracts.js'))) {
    runCommandSync('npm', ['run', 'build:contracts']);
  }

  if (!existsSync(agentBinaryPath())) {
    runCommandSync('cargo', ['build', '-p', 'ocentra-parent-agent-service']);
  }
}

function spawnAgent() {
  return spawn(agentBinaryPath(), [], {
    cwd: repoRoot,
    detached: process.platform !== 'win32',
    env: {
      ...process.env,
      [ParentDevEnv.AgentAddress]: createAgentAddress(agentPort),
      [ParentDevEnv.AgentAllowedOrigins]: createHttpOrigin(ParentDevHost.Loopback, portalPort),
    },
    stdio: ['ignore', 'inherit', 'inherit'],
  });
}

function spawnPortal() {
  return spawnCommand(
    'npm',
    ['exec', '--', 'vite', '--host', ParentDevHost.Loopback, '--port', String(portalPort), '--strictPort'],
    {
      cwd: path.join(repoRoot, 'apps', 'portal'),
      env: {
        ...process.env,
        [ParentDevEnv.PortalAgentWebSocketUrl]: createAgentWebSocketUrl(agentPort),
      },
    }
  );
}

function runCommandSync(command, args) {
  if (process.platform === 'win32') {
    execFileSync('cmd.exe', ['/d', '/s', '/c', `${command} ${args.join(' ')}`], {
      cwd: repoRoot,
      stdio: 'inherit',
    });
    return;
  }

  execFileSync(command, args, {
    cwd: repoRoot,
    stdio: 'inherit',
  });
}

function spawnCommand(command, args, options) {
  if (process.platform === 'win32') {
    return spawn('cmd.exe', ['/d', '/s', '/c', `${command} ${args.join(' ')}`], {
      ...options,
      stdio: ['ignore', 'inherit', 'inherit'],
    });
  }

  return spawn(command, args, {
    ...options,
    detached: process.platform !== 'win32',
    stdio: ['ignore', 'inherit', 'inherit'],
  });
}

function agentBinaryPath() {
  const binaryName = process.platform === 'win32' ? 'ocentra-parent-agent-service.exe' : 'ocentra-parent-agent-service';
  return path.join(repoRoot, 'target', 'debug', binaryName);
}

async function waitForHttp(url) {
  const startedAt = Date.now();
  while (Date.now() - startedAt < 30000) {
    try {
      const response = await fetch(url);
      if (response.ok) {
        return;
      }
    } catch {
      await delay(250);
    }
  }
  throw new Error(`Timed out waiting for ${url}`);
}

function shutdown() {
  clearInterval(interval);
  stopChildren();
  process.exit(0);
}

function stopChildren() {
  for (const child of children) {
    if (child.pid === undefined) {
      continue;
    }
    if (process.platform === 'win32') {
      spawn('taskkill', ['/PID', String(child.pid), '/T', '/F'], { stdio: 'ignore' });
    } else {
      try {
        process.kill(-child.pid, 'SIGTERM');
      } catch {
        child.kill('SIGTERM');
      }
    }
  }
}
