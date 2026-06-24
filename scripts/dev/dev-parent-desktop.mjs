#!/usr/bin/env node
import { execSync, spawn } from 'node:child_process';
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import {
  ParentDevEnv,
  createAgentHealthUrl,
  createAgentWebSocketUrl,
  createHttpOrigin,
  resolveParentDevNetworkConfig,
} from './local-dev-config.mjs';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(scriptDir, '..', '..');
const desktopDir = path.join(repoRoot, 'apps', 'parent-desktop');
const tauriDir = path.join(desktopDir, 'src-tauri');
const baseConfigPath = path.join(tauriDir, 'tauri.conf.json');
const generatedConfigDir = path.join(tauriDir, '.generated');
const targetDir = path.join(tauriDir, 'target-parent-dev');
const parentDesktopProcessNames =
  process.platform === 'win32'
    ? ['ocentra-parent-desktop.exe', 'Ocentra Parent.exe']
    : ['ocentra-parent-desktop', 'Ocentra Parent'];

function log(message) {
  console.log(`[dev:parent-desktop] ${message}`);
}

function formatDurationMs(ms) {
  if (ms >= 60_000) return `${(ms / 60_000).toFixed(2)}m`;
  if (ms >= 1000) return `${(ms / 1000).toFixed(2)}s`;
  return `${ms}ms`;
}

async function killParentDesktopIfRunning() {
  try {
    for (const processName of parentDesktopProcessNames) {
      try {
        if (process.platform === 'win32') {
          execSync(`taskkill /IM "${processName}" /F 2>nul`, { stdio: 'pipe' });
        } else {
          execSync(`pkill -x "${processName}" || true`, { stdio: 'pipe' });
        }
      } catch {
        // This alias was not running; keep trying the other known desktop names.
      }
    }
    await new Promise((resolve) => setTimeout(resolve, 500));
  } catch {
    // No stale desktop process was running.
  }
}

function createConnectSrc(network) {
  return [
    "'self'",
    createAgentWebSocketUrl(network.agentPort, network.lanHost),
    createAgentHealthUrl(network.agentPort, network.lanHost),
    createAgentWebSocketUrl(network.agentPort, '127.0.0.1'),
    createAgentHealthUrl(network.agentPort, '127.0.0.1'),
    createAgentWebSocketUrl(network.agentPort, 'localhost'),
    createAgentHealthUrl(network.agentPort, 'localhost'),
  ].join(' ');
}

function createGeneratedConfig(network) {
  const config = JSON.parse(readFileSync(baseConfigPath, 'utf8'));
  config.build = {
    ...(config.build ?? {}),
    beforeDevCommand: 'npm --prefix ../.. run dev',
    devUrl: createHttpOrigin(network.lanHost, network.portalPort),
  };
  config.app = {
    ...(config.app ?? {}),
    security: {
      ...(config.app?.security ?? {}),
      csp: [
        "default-src 'self'",
        `connect-src ${createConnectSrc(network)}`,
        "img-src 'self' asset: https://asset.localhost",
        "script-src 'self'",
        "style-src 'self' 'unsafe-inline'",
      ].join('; '),
    },
  };

  mkdirSync(generatedConfigDir, { recursive: true });
  const generatedPath = path.join(
    generatedConfigDir,
    `tauri.dev.portal-${network.portalPort}.agent-${network.agentPort}.conf.json`
  );
  writeFileSync(generatedPath, `${JSON.stringify(config, null, 2)}\n`, 'utf8');
  return path.relative(desktopDir, generatedPath).replace(/\\/g, '/');
}

async function main() {
  const startedAt = Date.now();
  const network = resolveParentDevNetworkConfig();
  const generatedConfigPath = createGeneratedConfig(network);
  const dryRun = process.argv.includes('--dry-run');
  const devEnv = {
    ...process.env,
    [ParentDevEnv.AgentPort]: String(network.agentPort),
    [ParentDevEnv.PortalPort]: String(network.portalPort),
    [ParentDevEnv.DevNetworkMode]: network.mode,
    [ParentDevEnv.PortalAgentWebSocketUrl]: network.agentWebSocketUrl,
  };

  log(`Using portal ${network.portalBindHost}:${network.portalPort}.`);
  log(`Using agent ${network.agentAddress}.`);
  log(`Generated Tauri config ${generatedConfigPath}.`);
  if (dryRun) {
    log('Dry run complete; not launching Tauri.');
    return;
  }

  const cleanupStartedAt = Date.now();
  await killParentDesktopIfRunning();
  log(`Stale desktop cleanup completed in ${formatDurationMs(Date.now() - cleanupStartedAt)}.`);

  const child = spawn('cargo', ['tauri', 'dev', '-c', generatedConfigPath], {
    cwd: desktopDir,
    stdio: 'inherit',
    shell: process.platform === 'win32',
    env: {
      ...devEnv,
      CARGO_TARGET_DIR: targetDir,
    },
  });

  child.once('spawn', () => {
    log(`cargo tauri dev spawned after ${formatDurationMs(Date.now() - startedAt)}.`);
  });
  child.on('exit', (code) => {
    log(`cargo tauri dev exited after ${formatDurationMs(Date.now() - startedAt)}.`);
    process.exit(code ?? 0);
  });
  child.on('error', (error) => {
    console.error(`[dev:parent-desktop] Fatal: ${error instanceof Error ? error.message : String(error)}`);
    process.exit(1);
  });
}

main().catch((error) => {
  console.error(`[dev:parent-desktop] Fatal: ${error instanceof Error ? error.message : String(error)}`);
  process.exit(1);
});
