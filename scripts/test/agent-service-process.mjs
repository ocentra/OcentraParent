import { spawn, spawnSync } from 'node:child_process';
import { once } from 'node:events';
import { rm } from 'node:fs/promises';
import { join } from 'node:path';
import { setTimeout as delay } from 'node:timers/promises';

const LOCAL_NETWORK_ENVIRONMENT_KEYS = Object.freeze([
  'OCENTRA_PARENT_DEV_NETWORK',
  'OCENTRA_PARENT_AGENT_LOCAL_NETWORK_ENABLED',
]);

export function createLoopbackOnlyTestEnvironment(environment = process.env) {
  const loopbackEnvironment = { ...environment };
  for (const key of LOCAL_NETWORK_ENVIRONMENT_KEYS) {
    delete loopbackEnvironment[key];
  }
  return loopbackEnvironment;
}

export function resolveDebugAgentServicePath(repoRoot = process.cwd()) {
  const binaryName = process.platform === 'win32' ? 'ocentra-parent-agent-service.exe' : 'ocentra-parent-agent-service';
  return join(repoRoot, 'target', 'debug', binaryName);
}

export function resolveAgentServiceManifestPath(repoRoot = process.cwd()) {
  return join(repoRoot, 'crates', 'agent-service', 'Cargo.toml');
}

export function resolveDebugParentDevBridgePath(repoRoot = process.cwd()) {
  const binaryName = process.platform === 'win32' ? 'ocentra-parent-dev-bridge.exe' : 'ocentra-parent-dev-bridge';
  return join(repoRoot, 'target', 'debug', binaryName);
}

export function resolveParentDevBridgeManifestPath(repoRoot = process.cwd()) {
  return join(repoRoot, 'crates', 'parent-dev-bridge', 'Cargo.toml');
}

export function buildPortalE2eRustServices(repoRoot = process.cwd()) {
  const targetDir = join(repoRoot, 'target');
  const manifests = [resolveAgentServiceManifestPath(repoRoot), resolveParentDevBridgeManifestPath(repoRoot)];

  for (const manifestPath of manifests) {
    const result = spawnSync('cargo', ['build', '--quiet', '--manifest-path', manifestPath], {
      cwd: repoRoot,
      env: {
        ...process.env,
        CARGO_TARGET_DIR: targetDir,
      },
      shell: process.platform === 'win32',
      stdio: 'inherit',
    });
    if (result.error !== undefined) {
      throw result.error;
    }
    if (result.status !== 0) {
      throw new Error(`Failed to build portal E2E Rust service from ${manifestPath}`);
    }
  }
}

export function spawnAgentService(env, repoRoot = process.cwd()) {
  return spawn('cargo', ['run', '--quiet', '--manifest-path', resolveAgentServiceManifestPath(repoRoot)], {
    cwd: repoRoot,
    detached: process.platform !== 'win32',
    env: {
      ...env,
      CARGO_TARGET_DIR: join(repoRoot, 'target'),
    },
    shell: process.platform === 'win32',
    stdio: ['ignore', 'inherit', 'inherit'],
  });
}

export function spawnParentDevBridge(env, repoRoot = process.cwd()) {
  return spawn('cargo', ['run', '--quiet', '--manifest-path', resolveParentDevBridgeManifestPath(repoRoot)], {
    cwd: repoRoot,
    detached: process.platform !== 'win32',
    env: {
      ...env,
      CARGO_TARGET_DIR: join(repoRoot, 'target'),
    },
    shell: process.platform === 'win32',
    stdio: ['ignore', 'inherit', 'inherit'],
  });
}

export async function ensureParentDevBridgeBinaryUnlocked(
  repoRoot = process.cwd(),
  { attempts = 20, delayMs = 250 } = {}
) {
  if (process.platform !== 'win32') {
    return;
  }

  const binaryPath = resolveDebugParentDevBridgePath(repoRoot);
  for (let attempt = 1; attempt <= attempts; attempt += 1) {
    terminateWindowsProcessImage('ocentra-parent-dev-bridge.exe');
    try {
      await rm(binaryPath, { force: true });
      return;
    } catch (error) {
      if (attempt === attempts || !isRetriableWindowsBinaryUnlockError(error)) {
        throw error;
      }
      await delay(delayMs);
    }
  }
}

export function spawnVitePortal(port, env, repoRoot = process.cwd()) {
  const command = process.platform === 'win32' ? 'cmd.exe' : 'npm';
  const args =
    process.platform === 'win32'
      ? ['/c', `npm exec -- vite --host 127.0.0.1 --port ${port} --strictPort --force`]
      : ['exec', '--', 'vite', '--host', '127.0.0.1', '--port', String(port), '--strictPort', '--force'];

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

function isRetriableWindowsBinaryUnlockError(error) {
  return error?.code === 'EACCES' || error?.code === 'EBUSY' || error?.code === 'ENOTEMPTY' || error?.code === 'EPERM';
}

function terminateWindowsProcessImage(imageName) {
  spawnSync('taskkill', ['/IM', imageName, '/T', '/F'], {
    stdio: 'ignore',
    windowsHide: true,
  });
}
