#!/usr/bin/env node
import { spawn } from 'node:child_process';

import { ParentDevEnv, ParentDevNetworkMode, resolveParentDevNetworkMode } from './local-dev-config.mjs';

const children = [];
const networkMode = resolveParentDevNetworkMode();
const desktopProductMode = process.argv.includes('--desktop-product');
const childEnv = {
  ...process.env,
  [ParentDevEnv.DevNetworkMode]: networkMode,
};

function start(label, command, args, options = {}) {
  const child = spawn(command, args, {
    cwd: process.cwd(),
    stdio: 'inherit',
    shell: options.shell ?? process.platform === 'win32',
    env: childEnv,
  });
  children.push(child);
  child.on('exit', (code) => {
    if (code && code !== 0) {
      console.error(`[dev] ${label} exited with ${code}.`);
      stopAll();
      process.exit(code);
    }
  });
  return child;
}

function stopAll() {
  for (const child of children) {
    if (child.pid === undefined) {
      continue;
    }
    if (process.platform === 'win32') {
      spawn('taskkill', ['/PID', String(child.pid), '/T', '/F'], {
        stdio: 'ignore',
      });
    } else {
      child.kill();
    }
  }
}

process.on('SIGINT', () => {
  stopAll();
  process.exit(0);
});

process.on('SIGTERM', () => {
  stopAll();
  process.exit(0);
});

if (networkMode === ParentDevNetworkMode.Lan) {
  console.log(
    desktopProductMode
      ? '[dev] Starting Ocentra Parent desktop product stack in LAN mode.'
      : '[dev] Starting Ocentra Parent web dev stack in LAN mode.'
  );
}

start('agent', 'npm', ['run', 'dev:agent']);

if (desktopProductMode) {
  console.log('[dev] Desktop product stack uses the Tauri host bridge; web dev bridge is disabled.');
  start('portal', 'npm', [
    'run',
    networkMode === ParentDevNetworkMode.Lan ? 'dev:portal:desktop:lan' : 'dev:portal:desktop',
  ]);
} else {
  start('bridge', 'npm', ['run', 'dev:bridge']);
  start('portal', 'npm', ['run', 'dev:portal']);
}
