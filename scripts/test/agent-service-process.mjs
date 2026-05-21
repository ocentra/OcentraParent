import { spawn } from 'node:child_process';
import { once } from 'node:events';
import { rm } from 'node:fs/promises';
import { join } from 'node:path';
import { setTimeout as delay } from 'node:timers/promises';

export function resolveDebugAgentServicePath(repoRoot = process.cwd()) {
  const binaryName = process.platform === 'win32' ? 'ocentra-parent-agent-service.exe' : 'ocentra-parent-agent-service';
  return join(repoRoot, 'target', 'debug', binaryName);
}

export function spawnVitePortal(port, env, repoRoot = process.cwd()) {
  const command = process.platform === 'win32' ? 'cmd.exe' : 'npm';
  const args =
    process.platform === 'win32'
      ? ['/c', `npm exec -- vite --host 127.0.0.1 --port ${port} --strictPort`]
      : ['exec', '--', 'vite', '--host', '127.0.0.1', '--port', String(port), '--strictPort'];

  return spawn(command, args, {
    cwd: join(repoRoot, 'apps', 'portal'),
    detached: process.platform !== 'win32',
    env,
    stdio: ['ignore', 'pipe', 'pipe'],
  });
}

export function stopProcessTree(child) {
  if (child.pid === undefined) {
    return;
  }

  if (process.platform === 'win32') {
    spawn('taskkill', ['/PID', String(child.pid), '/T', '/F'], { stdio: 'ignore' });
    return;
  }

  try {
    process.kill(-child.pid, 'SIGTERM');
  } catch {
    child.kill('SIGTERM');
  }
}

export async function stopProcessTreeAndWait(child, { shutdownTimeoutMs = 5000, forceTimeoutMs = 2000 } = {}) {
  if (child.exitCode !== null || child.signalCode !== null) {
    return;
  }

  const gracefulExit = waitForExit(child, shutdownTimeoutMs);
  stopProcessTree(child);
  if (await gracefulExit) {
    return;
  }

  forceKillProcessTree(child);
  await waitForExit(child, forceTimeoutMs);
}

export async function removeDirectoryWithRetry(directoryPath, { attempts = 10, delayMs = 250 } = {}) {
  for (let attempt = 1; attempt <= attempts; attempt += 1) {
    try {
      await rm(directoryPath, { recursive: true, force: true });
      return;
    } catch (error) {
      if (attempt === attempts || !isRetriableRemoveError(error)) {
        throw error;
      }
      await delay(delayMs);
    }
  }
}

async function waitForExit(child, timeoutMs) {
  if (child.exitCode !== null || child.signalCode !== null) {
    return true;
  }

  return Promise.race([once(child, 'exit').then(() => true), delay(timeoutMs).then(() => false)]);
}

function forceKillProcessTree(child) {
  if (child.pid === undefined) {
    return;
  }

  if (process.platform === 'win32') {
    spawn('taskkill', ['/PID', String(child.pid), '/T', '/F'], { stdio: 'ignore' });
    return;
  }

  try {
    process.kill(-child.pid, 'SIGKILL');
  } catch {
    child.kill('SIGKILL');
  }
}

function isRetriableRemoveError(error) {
  return error?.code === 'EBUSY' || error?.code === 'ENOTEMPTY' || error?.code === 'EPERM';
}
