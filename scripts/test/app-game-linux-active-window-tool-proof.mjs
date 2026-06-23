import { spawn } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const proofMode = 'app-game-linux-active-window-tool-proof';
const outputDir = join(repoRoot, 'test-results', proofMode);
const appGameProofDir = join(repoRoot, 'output', 'app-game-plan-proof', '204-app-game-linux-active-window-tool-proof');
const proofPath = join(outputDir, 'proof.json');
const commands = [];

await main();

async function main() {
  await mkdir(outputDir, { recursive: true });
  await mkdir(appGameProofDir, { recursive: true });

  await runCommand(
    ...npmCommand([
      'run',
      'test',
      '--workspace',
      '@ocentra-parent/app-game-domain',
      '--',
      'app-game-linux-active-window-tool-proof',
    ])
  );
  await runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/schema-domain']));
  await runCommand(...npmCommand(['run', 'build', '--workspace', '@ocentra-parent/app-game-domain']));

  const sourceState = await readWslActiveWindowState();
  const contractModule = await import(
    pathToFileURL(join(repoRoot, 'packages', 'schema-domain', 'dist', 'app-game-linux-active-window-tool-proof.js'))
      .href
  );
  const readModel = contractModule.createAppGameLinuxActiveWindowToolProof({
    toolState: sourceState.toolState,
    activeWindowRefState: sourceState.activeWindowRefState,
    displaySourceObserved: sourceState.displaySourceObserved,
    checkedAt: '2026-06-08T21:55:00.000Z',
  });
  const summary = contractModule.summarizeAppGameLinuxActiveWindowToolProof(readModel);

  const proof = {
    schemaVersion: 1,
    proofMode,
    checkedAt: 'deterministic-proof-artifact',
    commit: await gitHead(),
    commands: commands.map(redactCommandRecord),
    sourceState,
    readModel,
    summary,
    evidence: {
      contract: 'packages/schema-domain/src/app-game-linux-active-window-tool-proof.ts',
      contractTest: 'packages/app-game-domain/tests/unit/app-game-linux-active-window-tool-proof.test.ts',
      wslRuntime: 'wsl.exe Ubuntu-22.04 shell probe',
      activeWindowTool:
        'WSL command -v xdotool/xprop, then DISPLAY=:0 xprop -root _NET_ACTIVE_WINDOW; active-window refs are opaque and raw titles are not queried or stored.',
    },
    claimsProved: [
      'Linux/WSL active-window probe tooling is detected without installing packages',
      'The active-window ref probe is reduced to an opaque observed/not-observed state',
      'Raw window titles, process names, adapter dispatch, platform enforcement, and child-device delivery remain unclaimed',
    ],
    claimsNotProved: [
      'Linux foreground app/window capture',
      'Raw active-window title custody',
      'Linux policy enforcement, rollback, audit, adapter dispatch, provider delivery, or child-device delivery',
    ],
  };

  await writeJson(proofPath, proof);
  await writeJson(join(appGameProofDir, 'proof.json'), proof);
  await writeFile(join(appGameProofDir, '00-source-snapshot.md'), sourceSnapshot(sourceState));
  await writeFile(
    join(appGameProofDir, '10-validation-commands.log'),
    `${commands.map(redactCommandRecord).join('\n\n')}\n`
  );

  console.log('app-game-linux-active-window-tool-proof-ok');
  console.log(`evidence=${relativePath(proofPath)}`);
}

async function readWslActiveWindowState() {
  const xdotool = await wsl('command -v xdotool || true');
  const xprop = await wsl('command -v xprop || true');
  const display = await wsl(
    'test -S /mnt/wslg/.X11-unix/X0 || test -S /tmp/.X11-unix/X0 || test -S /mnt/wslg/runtime-dir/wayland-0; if [ $? -eq 0 ]; then echo observed; else echo missing; fi'
  );
  const tool = xdotool.stdout.trim() ? 'xdotool' : xprop.stdout.trim() ? 'xprop' : 'missing';
  const active = await activeWindowRef(tool);

  return {
    toolState: toolState(tool),
    activeWindowRefState: activeWindowRefState(active),
    displaySourceObserved: display.stdout.trim() === 'observed',
    activeWindowRefRedacted: activeWindowRefState(active) === 'active-window-ref-observed',
  };
}

async function activeWindowRef(tool) {
  if (tool === 'xdotool') {
    const result = await wsl('DISPLAY=:0 xdotool getactivewindow 2>/dev/null || true');
    return result.stdout.trim() || 'unavailable';
  }
  if (tool === 'xprop') {
    const result = await wsl('DISPLAY=:0 xprop -root _NET_ACTIVE_WINDOW 2>/dev/null || true');
    const match = /#\s*(0x[0-9a-f]+)/iu.exec(result.stdout);
    return match?.[1] ?? 'unavailable';
  }
  return 'unavailable';
}

function wsl(script) {
  return runCommand('wsl.exe', ['sh', '-lc', script]);
}

function toolState(tool) {
  if (tool === 'xdotool') {
    return 'xdotool-available';
  }
  if (tool === 'xprop') {
    return 'xprop-available';
  }
  return 'active-window-tool-missing';
}

function activeWindowRefState(active) {
  if (!active || active === 'unavailable') {
    return 'active-window-query-unavailable';
  }
  if (active === '0x0' || active === '0') {
    return 'no-active-window-ref';
  }
  return 'active-window-ref-observed';
}

async function runCommand(command, args, options = {}) {
  const commandLine = [command, ...args].join(' ');
  const result = await new Promise((resolve, reject) => {
    const child = spawn(command, args, { cwd: repoRoot, stdio: ['ignore', 'pipe', 'pipe'], windowsHide: true });
    const stdout = [];
    const stderr = [];
    child.stdout.on('data', (chunk) => stdout.push(String(chunk)));
    child.stderr.on('data', (chunk) => stderr.push(String(chunk)));
    child.once('exit', (code) =>
      resolve({ commandLine, status: code ?? 1, stdout: stdout.join(''), stderr: stderr.join('') })
    );
    child.once('error', reject);
  });
  commands.push(result);
  if (result.status !== 0 && !options.allowFailure) {
    throw new Error(`${commandLine} exited with ${result.status}: ${result.stderr}`);
  }
  return result;
}

async function gitHead() {
  const result = await runCommand('git', ['rev-parse', 'HEAD']);
  return result.stdout.trim();
}

async function writeJson(path, value) {
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`);
}

function sourceSnapshot(sourceState) {
  return [
    '# WP204 Linux active-window tool proof source snapshot',
    '',
    `- Tool state: \`${sourceState.toolState}\``,
    `- Display source observed: \`${sourceState.displaySourceObserved}\``,
    `- Active window ref state: \`${sourceState.activeWindowRefState}\``,
    `- Active window ref redacted: \`${sourceState.activeWindowRefRedacted}\``,
    '',
  ].join('\n');
}

function redactCommandRecord(record) {
  const command = record.commandLine.startsWith('wsl.exe ')
    ? 'wsl.exe sh -lc <linux-active-window-tool-proof-script-redacted>'
    : record.commandLine;
  return [command, `exit=${record.status}`, redactOutput(record.stdout), redactOutput(record.stderr)]
    .filter(Boolean)
    .join('\n');
}

function redactOutput(output) {
  return output
    .split(repoRoot)
    .join('<repo-root>')
    .replace(/0x[0-9a-f]+/giu, '<opaque-window-ref-redacted>')
    .replace(/^\d+$/gmu, '<opaque-window-ref-redacted>');
}

function relativePath(path) {
  return relative(repoRoot, path).replaceAll('\\', '/');
}

function npmCommand(args) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return [command, commandArgs];
}
