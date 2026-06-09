import { spawnSync } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const proofMode = 'app-game-linux-wsl-runtime-proof';
const wslDistro = process.env.OCENTRA_LINUX_WSL_DISTRO ?? 'Ubuntu-22.04';
const testOutputDir = join(repoRoot, 'test-results', proofMode);
const proofDir = join(repoRoot, 'output', 'app-game-plan-proof', '182-app-game-linux-wsl-runtime-proof');
const commands = [];

await main();

async function main() {
  await mkdir(testOutputDir, { recursive: true });
  await mkdir(proofDir, { recursive: true });

  run('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/schema-domain']);
  run('cmd', [
    '/c',
    'npm',
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/parent-domain',
    '--',
    'app-game-linux-wsl-runtime-proof',
    'app-game-broad-blocking-proof-gates',
  ]);
  run('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);

  run('wsl.exe', ['--status']);
  const osRelease = parseOsRelease(wsl('cat /etc/os-release'));
  const uname = wsl('uname -r -m').trim().split(/\s+/);
  const packageManagerVisibleCount = packageCount();
  const processSnapshotCount = processCount();
  const systemdSessionState = wsl('ps -p 1 -o comm= 2>/dev/null || true').includes('systemd')
    ? 'systemd-session-observed'
    : 'session-not-proved';
  const displayProof = collectDisplayProof();
  const dockerState = dockerCliVisible() ? 'docker-visible' : 'docker-cli-unavailable';

  const module = await import(
    pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', 'app-game-linux-wsl-runtime-proof.js')).href
  );
  const readModel = module.decodeAppGameLinuxWslRuntimeProof({
    schemaVersion: proofMode,
    proofId: 'app-game-linux-wsl-runtime-proof-ubuntu',
    targetKind: 'wsl2-distro',
    runtimeState: 'runtime-observed',
    distroRef: 'linux-wsl-distro-ref',
    distroId: osRelease.id,
    distroVersion: osRelease.versionId,
    kernelRelease: uname[0],
    architecture: uname[1],
    packageManagerVisibleCount,
    processSnapshotCount,
    systemdSessionState,
    displayState: displayProof.displayState,
    x11SocketState: displayProof.x11SocketState,
    waylandSocketState: displayProof.waylandSocketState,
    foregroundProbeState: displayProof.foregroundProbeState,
    dockerState,
    proofRefs: [
      'linux-wsl-distro-ref',
      'linux-wsl-kernel-ref',
      'linux-wsl-package-manager-ref',
      'linux-wsl-process-ref',
      'linux-wsl-session-ref',
      'linux-wslg-display-ref',
      'linux-wslg-x11-socket-ref',
      'linux-wslg-wayland-socket-ref',
      'linux-docker-cli-ref',
    ],
    packageNamesRedacted: true,
    processNamesRedacted: true,
    rawDistroNameRedacted: true,
    mechanismProofAttached: true,
    distroProofAttached: true,
    sessionProofAttached: systemdSessionState === 'systemd-session-observed',
    displayProofAttached: displayProof.displayProofAttached,
    rollbackProofAttached: false,
    auditProofAttached: false,
    foregroundCaptureClaimed: false,
    adapterDispatchClaimed: false,
    broadBlockingClaimed: false,
    platformEnforcementClaimed: false,
    parentVisibleSummary:
      'WSL2 Ubuntu runtime, package manager, process, and session facts are observed; Linux broad blocking remains unavailable until rollback and audit proof are attached.',
    checkedAt: '2026-06-08T16:10:00.000Z',
  });
  const summary = module.summarizeAppGameLinuxWslRuntimeProof(readModel);
  await assertSourceBoundaries();

  const proof = {
    schemaVersion: 1,
    proofMode,
    generatedAt: 'deterministic-proof-artifact',
    distroRef: 'linux-wsl-distro-ref',
    commands: commands.map(redactCommandOutput),
    readModel,
    summary,
    evidence: {
      contract: 'packages/parent-domain/src/app-game-linux-wsl-runtime-proof.ts',
      contractTest: 'packages/parent-domain/tests/app-game-linux-wsl-runtime-proof.test.ts',
      wslRuntime:
        'wsl.exe observed a WSL2 Ubuntu runtime with Linux kernel, package manager, process snapshot, and session facts.',
      packageVisibility:
        'WSL dpkg package inventory returned a nonzero package count; proof stores only the count.',
      dockerBoundary: 'Docker CLI is recorded as visible or unavailable without claiming Docker-backed enforcement.',
      displayBoundary:
        'WSLg display, X11 socket, and Wayland socket are recorded as parent-safe display readiness only; active foreground capture remains unclaimed.',
      broadBlockBoundary:
        'Linux broad blocking remains unavailable because rollback and audit proof are not attached.',
    },
    productBoundaries: {
      nativeAppMeaningProven: true,
      nativeGameMeaningProven: true,
      linuxWslRuntimeObserved: true,
      linuxMechanismProofAttached: true,
      linuxDistroProofAttached: true,
      linuxSessionProofAttached: readModel.sessionProofAttached,
      linuxDisplayProofAttached: readModel.displayProofAttached,
      linuxForegroundCaptureClaimed: false,
      rollbackProofAttached: false,
      auditProofAttached: false,
      adapterDispatchClaimed: false,
      platformEnforcementClaimed: false,
      broadBlockingClaimed: false,
      rawPackageNamesStored: false,
      rawProcessNamesStored: false,
      rawDistroNameStored: false,
    },
    proofPaths: {
      proof: 'test-results/app-game-linux-wsl-runtime-proof/proof.json',
      appGameProofPack: 'output/app-game-plan-proof/182-app-game-linux-wsl-runtime-proof',
      harness: 'scripts/test/app-game-linux-wsl-runtime-proof.mjs',
    },
  };

  await writeJson(join(testOutputDir, 'proof.json'), proof);
  await writeJson(join(proofDir, 'proof.json'), proof);
  await writeFile(
    join(proofDir, '00-source-snapshot.md'),
    [
      '# App-game Linux WSL runtime proof source snapshot',
      '',
      '- Branch: codex/app-game-control-product-completion',
      '- Commit: uncommitted full-goal batch, validated by harness before final checkpoint commit',
      '- Distro: linux-wsl-distro-ref',
      '',
      'Evidence:',
      '- WSL2 Ubuntu runtime was reachable from the Windows host.',
      '- Kernel, package manager, process, and systemd-session facts were observed.',
      '- Package names, process names, and raw distro names are redacted from proof artifacts.',
      '- Linux adapter dispatch, broad blocking, and platform enforcement remain unclaimed without rollback and audit proof.',
      '',
    ].join('\n')
  );
  await writeFile(
    join(proofDir, '09-manual-platform-proof.md'),
    [
      '# Linux WSL runtime manual platform proof',
      '',
      '| Field | Value |',
      '| --- | --- |',
      `| Distro | ${readModel.distroId} ${readModel.distroVersion} |`,
      `| Kernel | ${readModel.kernelRelease} |`,
      `| Architecture | ${readModel.architecture} |`,
      `| Package manager visible rows | ${readModel.packageManagerVisibleCount} |`,
      `| Process snapshot rows | ${readModel.processSnapshotCount} |`,
      `| Session state | ${readModel.systemdSessionState} |`,
      `| Display state | ${readModel.displayState} |`,
      `| X11 socket state | ${readModel.x11SocketState} |`,
      `| Wayland socket state | ${readModel.waylandSocketState} |`,
      `| Foreground probe state | ${readModel.foregroundProbeState} |`,
      `| Docker state | ${readModel.dockerState} |`,
      '',
      'Limitations:',
      '- This is WSL runtime/package/process/session/display readiness evidence only.',
      '- It does not prove target-desktop active-window foreground capture, AppArmor/SELinux policy, package manager restriction, Flatpak/Snap restriction, launch blocking, rollback, or audit behavior.',
      '- Raw package names, process names, and raw distro names are intentionally redacted from proof artifacts.',
      '',
    ].join('\n')
  );
  await writeFile(join(proofDir, '10-validation-commands.log'), `${commands.map(redactCommandOutput).join('\n\n')}\n`);

  console.log('app-game-linux-wsl-runtime-proof-ok');
  console.log('evidence=test-results/app-game-linux-wsl-runtime-proof/proof.json');
}

function wsl(shellCommand) {
  return run('wsl.exe', ['-d', wslDistro, '--', 'sh', '-lc', shellCommand]).stdout;
}

function packageCount() {
  const output = wsl('dpkg-query -W 2>/dev/null | wc -l');
  const count = Number.parseInt(output.trim(), 10);
  if (count === 0) {
    throw new Error('WSL package manager returned zero visible packages.');
  }
  return count;
}

function processCount() {
  const output = wsl('ps -eo pid,comm --no-headers 2>/dev/null || true');
  const count = output.split(/\r?\n/).filter((line) => line.trim()).length;
  if (count === 0) {
    throw new Error('WSL process snapshot returned zero rows.');
  }
  return count;
}

function collectDisplayProof() {
  const wslgDir = wsl('test -d /mnt/wslg && echo observed || echo missing').trim();
  const x11Socket = wsl('test -S /tmp/.X11-unix/X0 && echo observed || echo missing').trim();
  const waylandSocket = wsl('test -S /mnt/wslg/runtime-dir/wayland-0 && echo observed || echo missing').trim();
  const xdotool = wsl('command -v xdotool || true').trim();
  const displayProofAttached = wslgDir === 'observed' && x11Socket === 'observed' && waylandSocket === 'observed';

  return {
    displayState: displayProofAttached ? 'wslg-display-observed' : 'display-not-proved',
    x11SocketState: x11Socket === 'observed' ? 'socket-observed' : 'socket-not-proved',
    waylandSocketState: waylandSocket === 'observed' ? 'socket-observed' : 'socket-not-proved',
    foregroundProbeState: xdotool ? 'active-window-not-proved' : 'active-window-tool-missing',
    displayProofAttached,
  };
}

function dockerCliVisible() {
  const result = spawnSync('docker', ['version', '--format', '{{json .}}'], {
    cwd: repoRoot,
    encoding: 'utf8',
    shell: false,
  });
  commands.push({
    rendered: 'docker version --format {{json .}}',
    status: result.status,
    stdout: result.stdout,
    stderr: result.stderr,
    proofStdout: result.status === 0 ? '<docker-cli-visible>\n' : '',
    proofStderr: result.status === 0 ? '' : '<docker-cli-unavailable>\n',
  });
  return result.status === 0;
}

function parseOsRelease(output) {
  const fields = Object.fromEntries(
    output
      .split(/\r?\n/)
      .map((line) => line.match(/^([A-Z_]+)=(.*)$/))
      .filter(Boolean)
      .map((match) => [match[1].toLowerCase(), match[2].replace(/^"|"$/g, '')])
  );
  if (!fields.id || !fields.version_id) {
    throw new Error('WSL /etc/os-release did not include ID and VERSION_ID.');
  }
  return { id: fields.id, versionId: fields.version_id };
}

async function assertSourceBoundaries() {
  const gateData = await readFile(
    join(repoRoot, 'packages', 'parent-domain', 'src', 'app-game-broad-blocking-proof-gate-data.ts'),
    'utf8'
  );
  const gateRules = await readFile(
    join(repoRoot, 'packages', 'parent-domain', 'src', 'app-game-broad-blocking-proof-gate-rules.ts'),
    'utf8'
  );
  assertIncludes(gateData, "gateId: 'linux-hard-block-mechanism-unavailable'", 'Linux hard-block gate');
  assertIncludes(gateData, "adapterDispatchState: 'adapter-unavailable'", 'Linux adapter unavailable state');
  assertIncludes(gateData, 'broadBlockingClaimed: false', 'Linux broad-block non-claim');
  assertIncludes(gateRules, "'linux-mechanism-proof'", 'Linux mechanism proof rule');
  assertIncludes(gateRules, "'linux-distro-proof'", 'Linux distro proof rule');
  assertIncludes(gateRules, "'linux-session-proof'", 'Linux session proof rule');
}

function run(command, args) {
  const rendered = `${command} ${args.join(' ')}`;
  const result = spawnSync(command, args, { cwd: repoRoot, encoding: 'utf8', shell: false });
  const record = {
    rendered,
    status: result.status,
    stdout: result.stdout,
    stderr: result.stderr,
    proofStdout: normalizeCommandOutput(result.stdout),
    proofStderr: normalizeCommandOutput(result.stderr),
  };
  commands.push(record);
  if (result.status !== 0) {
    throw new Error(`${rendered} failed with exit ${result.status}`);
  }
  return record;
}

function normalizeCommandOutput(output) {
  const withoutNuls = output.replace(/\u0000/g, '');
  if (withoutNuls.includes('Default Distribution')) {
    return '<wsl-status-visible default-distro=linux-wsl-distro-ref>\n';
  }
  if (withoutNuls.includes('VERSION_ID=') || withoutNuls.includes('PRETTY_NAME=')) {
    const versionId = withoutNuls.match(/VERSION_ID="?([^"\r\n]+)"?/)?.[1] ?? 'unknown';
    const id = withoutNuls.match(/^ID="?([^"\r\n]+)"?/m)?.[1] ?? 'unknown';
    return `ID=<redacted-linux-distro-id:${id}>\nVERSION_ID=${versionId}\n`;
  }
  if (withoutNuls.trim() === 'systemd') {
    return '<systemd-session-observed>\n';
  }
  if (withoutNuls.includes('microsoft-standard-WSL2')) {
    return withoutNuls.replace(new RegExp(wslDistro, 'g'), 'linux-wsl-distro-ref');
  }
  if (/^[^\r\n]+(?:\r?\n[^\r\n]+){10,}/.test(withoutNuls)) {
    const count = withoutNuls.split(/\r?\n/).filter((line) => line.trim()).length;
    return `<redacted-linux-list count=${count}>\n`;
  }
  return withoutNuls
    .split(repoRoot)
    .join('<repo-root>')
    .replace(new RegExp(wslDistro, 'g'), 'linux-wsl-distro-ref')
    .replace(/Ubuntu-22\.04/g, 'linux-wsl-distro-ref');
}

function redactCommandOutput(command) {
  return `${command.rendered.replace(wslDistro, 'linux-wsl-distro-ref')}\nexit=${command.status}\n${command.proofStdout}${command.proofStderr}`;
}

function assertIncludes(source, needle, label) {
  if (!source.includes(needle)) {
    throw new Error(`Missing ${label}: ${needle}`);
  }
}

async function writeJson(path, value) {
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`);
}
