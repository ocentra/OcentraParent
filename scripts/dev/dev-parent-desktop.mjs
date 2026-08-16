#!/usr/bin/env node
import { execSync, spawn } from 'node:child_process';
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

import {
  ParentDevEnv,
  ParentDevNetworkMode,
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

function createPortalConnectSrc(network) {
  return [
    ...new Set([
      "'self'",
      createHttpOrigin(network.lanHost, network.portalPort),
      createWebSocketOrigin(network.lanHost, network.portalPort),
      createHttpOrigin('127.0.0.1', network.portalPort),
      createWebSocketOrigin('127.0.0.1', network.portalPort),
      createHttpOrigin('localhost', network.portalPort),
      createWebSocketOrigin('localhost', network.portalPort),
    ]),
  ].join(' ');
}

function createWebSocketOrigin(host, port) {
  return `ws://${host}:${port}`;
}

function createGeneratedConfig(network) {
  const config = JSON.parse(readFileSync(baseConfigPath, 'utf8'));
  const beforeDevCommand =
    network.mode === ParentDevNetworkMode.Lan
      ? 'npm --prefix ../.. run dev:desktop:stack:lan'
      : 'npm --prefix ../.. run dev:desktop:stack';
  config.build = {
    ...(config.build ?? {}),
    beforeDevCommand,
    devUrl: createHttpOrigin(network.lanHost, network.portalPort),
  };
  config.app = {
    ...(config.app ?? {}),
    security: {
      ...(config.app?.security ?? {}),
      csp: [
        "default-src 'self'",
        `connect-src ${createPortalConnectSrc(network)}`,
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

export function createParentDesktopDevEnv(network, baseEnv = process.env) {
  return {
    ...baseEnv,
    [ParentDevEnv.AgentAddress]: network.agentConnectAddress,
    [ParentDevEnv.AgentAllowedOrigins]: network.allowedOrigins.join(','),
    [ParentDevEnv.AgentPort]: String(network.agentPort),
    [ParentDevEnv.ParentBridgePort]: String(network.parentBridgePort),
    [ParentDevEnv.PortalPort]: String(network.portalPort),
    [ParentDevEnv.DevNetworkMode]: network.mode,
    [ParentDevEnv.PortalAgentWebSocketUrl]: network.agentWebSocketUrl,
  };
}

export async function main(argv = process.argv, baseEnv = process.env) {
  const startedAt = Date.now();
  const network = resolveParentDevNetworkConfig(baseEnv, undefined, argv);
  const generatedConfigPath = createGeneratedConfig(network);
  const dryRun = argv.includes('--dry-run');
  const devEnv = createParentDesktopDevEnv(network, baseEnv);

  log(`Using portal ${network.portalBindHost}:${network.portalPort}.`);
  log(`Using agent ${network.agentAddress}.`);
  log(`Parent runtime connects to agent at ${network.agentConnectAddress}.`);
  log('Desktop product host bridge uses Tauri invoke/listen; local dev bridge stays web-only.');
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

function isDirectExecution() {
  const entryPath = process.argv[1];
  return entryPath !== undefined && import.meta.url === pathToFileURL(entryPath).href;
}

if (isDirectExecution()) {
  main().catch((error) => {
    console.error(`[dev:parent-desktop] Fatal: ${error instanceof Error ? error.message : String(error)}`);
    process.exit(1);
  });
}
