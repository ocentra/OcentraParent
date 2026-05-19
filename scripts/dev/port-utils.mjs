#!/usr/bin/env node
import { exec } from 'node:child_process';
import net from 'node:net';
import { promisify } from 'node:util';

const execAsync = promisify(exec);
const isWindows = process.platform === 'win32';

export async function isPortAvailable(port, host = '127.0.0.1') {
  return new Promise((resolve) => {
    const server = net.createServer();
    server.once('error', () => resolve(false));
    server.once('listening', () => {
      server.close(() => resolve(true));
    });
    server.listen(port, host);
  });
}

export async function waitForPort(port, timeoutMs, host = '127.0.0.1') {
  const startedAt = Date.now();
  while (Date.now() - startedAt < timeoutMs) {
    if (!(await isPortAvailable(port, host))) {
      return;
    }
    await delay(250);
  }
  throw new Error(`Port ${port} did not open within ${timeoutMs}ms.`);
}

export async function getPortOccupants(port) {
  return isWindows ? getWindowsPortOccupants(port) : getUnixPortOccupants(port);
}

export async function ensurePortFree(port, shouldKill, log, host = '127.0.0.1', maxRetries = 5) {
  for (let attempt = 1; attempt <= maxRetries; attempt += 1) {
    if (await isPortAvailable(port, host)) {
      return true;
    }

    const occupants = await getPortOccupants(port);
    const matching = occupants.filter(shouldKill);
    if (matching.length === 0) {
      logExternalOccupants(port, occupants, log);
      return false;
    }

    log(`Killing ${matching.length} stale Parent process(es) on port ${port}.`);
    for (const occupant of matching) {
      log(`Killing ${occupant.name || 'unknown'} PID ${occupant.pid}.`);
      await killProcess(occupant.pid);
    }

    await delay(500 * attempt);
  }

  return isPortAvailable(port, host);
}

export async function killProcess(pid) {
  try {
    if (isWindows) {
      await execAsync(`taskkill /F /T /PID ${pid}`, { windowsHide: true });
    } else {
      await execAsync(`kill -9 ${pid}`);
    }
    await delay(500);
    return true;
  } catch {
    return false;
  }
}

async function getWindowsPortOccupants(port) {
  try {
    const { stdout } = await execAsync(`netstat -ano | findstr :${port}`, {
      windowsHide: true,
    });
    const pids = stdout
      .split(/\r?\n/)
      .map((line) => line.match(/LISTENING\s+(\d+)/)?.[1])
      .filter(Boolean)
      .map((pid) => Number(pid));
    return Promise.all([...new Set(pids)].map(readWindowsProcess));
  } catch {
    return [];
  }
}

async function readWindowsProcess(pid) {
  const command = [
    'powershell',
    '-NoProfile',
    '-Command',
    `"$p = Get-CimInstance Win32_Process -Filter \\"ProcessId = ${pid}\\"; if ($p) { $p.Name + '\t' + $p.CommandLine }"`,
  ].join(' ');

  try {
    const { stdout } = await execAsync(command, { windowsHide: true });
    const [name = '', commandLine = ''] = stdout.trim().split('\t');
    return { pid, name, commandLine };
  } catch {
    return { pid, name: '', commandLine: '' };
  }
}

async function getUnixPortOccupants(port) {
  try {
    const { stdout } = await execAsync(`lsof -i :${port} -t`);
    const pids = stdout
      .split(/\r?\n/)
      .filter(Boolean)
      .map((pid) => Number(pid));
    return Promise.all([...new Set(pids)].map(readUnixProcess));
  } catch {
    return [];
  }
}

async function readUnixProcess(pid) {
  try {
    const [{ stdout: name }, { stdout: commandLine }] = await Promise.all([
      execAsync(`ps -p ${pid} -o comm=`),
      execAsync(`ps -p ${pid} -o command=`),
    ]);
    return { pid, name: name.trim(), commandLine: commandLine.trim() };
  } catch {
    return { pid, name: '', commandLine: '' };
  }
}

function logExternalOccupants(port, occupants, log) {
  const summary = occupants.map((occupant) => `${occupant.name || 'unknown'} PID ${occupant.pid}`).join(', ');
  log(`Port ${port} is held by non-Parent process(es): ${summary || 'unknown'}.`);
}

function delay(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}
