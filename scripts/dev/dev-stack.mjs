#!/usr/bin/env node
import { spawn } from 'node:child_process';

import { ParentDevEnv, ParentDevNetworkMode, resolveParentDevNetworkMode } from './local-dev-config.mjs';

const children = [];
const networkMode = resolveParentDevNetworkMode();
const childEnv = {
  ...process.env,
  [ParentDevEnv.DevNetworkMode]: networkMode,
};

function start(label, args) {
  const child = spawn('npm', ['run', ...args], {
    cwd: process.cwd(),
    stdio: 'inherit',
    shell: process.platform === 'win32',
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
  console.log('[dev] Starting Ocentra Parent in LAN mode.');
}

start('agent', ['dev:agent']);
start('portal', ['dev:portal']);
