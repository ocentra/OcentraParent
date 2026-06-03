import { spawn, spawnSync } from 'node:child_process';
import { mkdirSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

const proofRoot = join('output', 'screen-plan-proof', 'linux-wslg');
const selectedWindowDir = join(proofRoot, 'selected-window');
const title = 'OcentraWslgProof';
const cargo = process.env.CARGO ?? '/root/.cargo/bin/cargo';
const targetDir = process.env.CARGO_TARGET_DIR ?? '/tmp/ocentra-parent-target-screen';
const examplePath = join(targetDir, 'debug', 'examples', 'screen_capture_real_proof');

if (process.platform !== 'linux') {
  throw new Error('screen-capture-linux-wslg-proof must run inside Linux/WSL.');
}

for (const command of ['xclock', 'xwininfo', 'xwd', 'convert', 'identify']) {
  requireCommand(command);
}

rmSync(proofRoot, { recursive: true, force: true });
mkdirSync(selectedWindowDir, { recursive: true });

run(cargo, ['build', '-p', 'ocentra-parent-screen-capture-adapter', '--example', 'screen_capture_real_proof'], {
  CARGO_TARGET_DIR: targetDir,
});

const xclock = spawn('xclock', ['-name', title, '-title', title, '-geometry', '220x220+80+80']);
xclock.stdout.on('data', (chunk) => appendLog('xclock-stdout.log', chunk));
xclock.stderr.on('data', (chunk) => appendLog('xclock-stderr.log', chunk));

try {
  await delay(1000);
  run(examplePath, [selectedWindowDir], {
    OCENTRA_SCREEN_CAPTURE_WINDOW_TITLE_CONTAINS: title,
  });
} finally {
  xclock.kill();
}

const metadata = readJson(join(selectedWindowDir, '02-capture-metadata.json'));
const deletion = readJson(join(selectedWindowDir, '04-deletion-proof.json'));
const summary = {
  proof: 'screen-capture-linux-wslg-proof',
  platform: process.platform,
  session: {
    display: process.env.DISPLAY ?? null,
    waylandDisplay: process.env.WAYLAND_DISPLAY ?? null,
    xdgSessionType: process.env.XDG_SESSION_TYPE ?? null,
  },
  selectedWindow: {
    captured: metadata.captured === true,
    status: metadata.status,
    actualScope: metadata.actualScope,
    width: metadata.width,
    height: metadata.height,
    imageByteSize: metadata.imageByteSize,
    titlePresent: metadata.titlePresent,
    windowId: metadata.windowId,
  },
  custody: {
    rawImageDeleted: deletion.rawImageDeleted === true,
    existsAfterDelete: deletion.existsAfterDelete,
    encryptedQueueContainsRawDigest: deletion.encryptedQueueContainsRawDigest,
  },
  selectedWindowArtifact: selectedWindowDir,
  degradedIsCaptureProof: false,
  nonClaims: [
    'This proves WSLg/X11 selected-window capture only.',
    'It does not claim WSLg root display capture, native Linux Wayland portal capture, or macOS/iOS/Android physical parity.',
  ],
};

if (
  summary.selectedWindow.captured !== true ||
  summary.selectedWindow.status !== 'available' ||
  summary.selectedWindow.actualScope !== 'selectedWindow' ||
  !Number.isInteger(summary.selectedWindow.width) ||
  !Number.isInteger(summary.selectedWindow.height) ||
  summary.selectedWindow.width <= 0 ||
  summary.selectedWindow.height <= 0 ||
  !Number.isInteger(summary.selectedWindow.imageByteSize) ||
  summary.selectedWindow.imageByteSize <= 0 ||
  summary.custody.rawImageDeleted !== true ||
  summary.custody.existsAfterDelete !== false ||
  summary.custody.encryptedQueueContainsRawDigest !== false
) {
  writeJson(join(proofRoot, 'proof-summary.json'), summary);
  throw new Error(`Linux WSLg selected-window proof failed: ${JSON.stringify(summary, null, 2)}`);
}

writeJson(join(proofRoot, 'proof-summary.json'), summary);
console.log(`screen-capture-linux-wslg-proof-ok:${summary.selectedWindow.width}x${summary.selectedWindow.height}`);

function requireCommand(command) {
  const result = spawnSync('bash', ['-lc', `command -v ${command}`], {
    cwd: process.cwd(),
    encoding: 'utf8',
  });
  if (result.status !== 0) {
    throw new Error(`Required Linux proof command missing: ${command}`);
  }
}

function run(command, args, env = {}) {
  const result = spawnSync(command, args, {
    cwd: process.cwd(),
    encoding: 'utf8',
    env: {
      ...process.env,
      ...env,
    },
  });
  writeFileSync(join(proofRoot, `${safeName(command)}-stdout.log`), result.stdout ?? '');
  writeFileSync(join(proofRoot, `${safeName(command)}-stderr.log`), result.stderr ?? '');
  if (result.status !== 0) {
    throw new Error(`${command} ${args.join(' ')} failed with ${result.status}`);
  }
}

function appendLog(name, chunk) {
  writeFileSync(join(proofRoot, name), chunk, { flag: 'a' });
}

function safeName(command) {
  return command.replaceAll('/', '_').replaceAll('\\', '_').replaceAll(':', '_');
}

function readJson(path) {
  return JSON.parse(readFileSync(path, 'utf8'));
}

function writeJson(path, value) {
  writeFileSync(path, `${JSON.stringify(value, null, 2)}\n`);
}

function delay(milliseconds) {
  return new Promise((resolve) => {
    setTimeout(resolve, milliseconds);
  });
}
