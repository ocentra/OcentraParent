import { spawn } from 'node:child_process';
import { join } from 'node:path';

export function resolveDebugAgentServicePath() {
  const binaryName = process.platform === 'win32' ? 'ocentra-parent-agent-service.exe' : 'ocentra-parent-agent-service';
  return join(process.cwd(), 'target', 'debug', binaryName);
}

export function spawnVitePortal(port, env) {
  const command = process.platform === 'win32' ? 'cmd.exe' : 'npm';
  const args =
    process.platform === 'win32'
      ? ['/c', `npm exec -- vite --host 127.0.0.1 --port ${port} --strictPort`]
      : ['exec', '--', 'vite', '--host', '127.0.0.1', '--port', String(port), '--strictPort'];

  return spawn(command, args, {
    cwd: join(process.cwd(), 'apps', 'portal'),
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
