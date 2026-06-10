import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

const proofRoot = join('output', 'network-plan-proof', '13b-live-capture-execution-proof');
const testRoot = join('test-results', 'network-live-capture-execution-proof');
const commandLogRoot = join(proofRoot, 'command-logs');
mkdirSync(proofRoot, { recursive: true });
mkdirSync(testRoot, { recursive: true });
mkdirSync(commandLogRoot, { recursive: true });

const sourceRefs = [
  'crates/ocentra-network-evidence/src/live_capture.rs',
  'crates/ocentra-network-evidence/src/live_capture_execution.rs',
  'crates/ocentra-network-evidence/src/live_capture_execution/types.rs',
  'crates/ocentra-network-evidence/src/live_capture_execution/validation.rs',
  'crates/ocentra-network-evidence/src/lib.rs',
  'crates/ocentra-network-evidence/src/tests.rs',
  'crates/ocentra-network-evidence/src/tests/live_capture.rs',
  'crates/ocentra-network-evidence/src/tests/live_capture_execution.rs',
  'crates/ocentra-network-evidence/src/tests/live_capture_execution_fixtures.rs',
  'crates/ocentra-network-evidence/README.md',
  'docs/features/network-domain-control.md',
  'docs/plans/network-plan/implementation-checklist.md',
  'docs/plans/network-plan/current-network-snapshot.md',
  'scripts/test/network-live-capture-execution-proof.mjs',
];

const boundary = {
  reportRef: 'network-live-capture-execution-row13b',
  sourceRows: ['13 Live pcap/Npcap/libpcap capture adapter', '03a Live capture storage custody proof'],
  supportedExecutionStates: ['bounded-executed', 'manual-required', 'unavailable', 'degraded'],
  driverBackedExecutionRequires: [
    'proof-ready row13 live-capture proof',
    'driver invocation ref',
    'interface observation ref',
    'permission ref',
    'bounded window ref',
    'clean stop ref',
    'custody ref',
    'retention/delete/export ref',
    'metadata-only sanitization ref',
    'private traffic exclusion ref',
  ],
  metadataSnapshotBoundary:
    'Windows netstat/IP-helper style snapshots may be recorded as metadata observations, but they cannot substitute for Npcap/libpcap packet capture.',
  unsupportedClaimsRejected: [
    'raw artifact creation',
    'netstat metadata substitution for live capture',
    'unbounded capture',
    'raw PCAP without custody',
    'exact URL',
    'decrypted payload',
    'page content',
    'private message',
    'search query',
    'policy authority',
    'adapter authority',
    'host filtering',
    'enforcement command',
  ],
};
writeJson(join(proofRoot, 'live-capture-execution-boundary.json'), boundary);
writeJson(join(proofRoot, 'local-host-observations.json'), collectLocalHostObservations());

const commands = [
  {
    name: 'network-live-capture-execution-tests',
    command: 'cargo',
    args: ['test', '-p', 'ocentra-network-evidence', 'live_capture_execution'],
  },
  {
    name: 'network-evidence-clippy',
    command: 'cargo',
    args: ['clippy', '-p', 'ocentra-network-evidence', '--all-targets', '--', '-D', 'warnings'],
  },
  {
    name: 'source-shape',
    command: 'node',
    args: ['scripts/check-source-shape.mjs'],
  },
  {
    name: 'diff-check',
    command: 'git',
    args: ['diff', '--check'],
  },
];
const commandResults = commands.map(runCommand);
writeFileSync(join(proofRoot, 'validation-commands.log'), validationCommandsLog(commandResults));

const proof = {
  schemaVersion: 1,
  proof: 'network-live-capture-execution',
  checkedAt: new Date().toISOString(),
  branch: runText('git', ['branch', '--show-current']).trim(),
  commit: runText('git', ['rev-parse', 'HEAD']).trim(),
  statusShort: runText('git', ['status', '--short']),
  sourceRefs,
  sourceFingerprint: `source-tree:${sourceFingerprint(sourceRefs)}`,
  proofRoot,
  testRoot,
  commands: commandResults,
  artifacts: {
    boundary: join(proofRoot, 'live-capture-execution-boundary.json'),
    localHostObservations: join(proofRoot, 'local-host-observations.json'),
    commandLog: join(proofRoot, 'validation-commands.log'),
    proofSummary: join(proofRoot, 'proof-summary.json'),
    testProof: join(testRoot, 'proof.json'),
  },
  provenRows: ['13b Live-capture bounded execution/status proof'],
  provenBoundaries: [
    'bounded driver execution is accepted only with proof-ready row13 capture refs and all row13b execution/custody refs',
    'metadata-only Windows snapshots remain observable but cannot substitute for Npcap/libpcap capture',
    'manual-required, unavailable, and degraded states stay visible without packet capture claims',
    'raw artifact creation, raw PCAP without custody, content, policy, adapter, host-filter, and enforcement claims are rejected',
    'Windows, Android SDK/emulator, WSL/Linux, and Apple CI/manual-unavailable host observations are recorded as proof context without product support claims',
  ],
  notClaimed: [
    'production live capture support',
    'raw artifact creation',
    'remote upload',
    'raw PCAP without custody',
    'exact URL, decrypted payload, page content, private message, or search query visibility',
    'policy authority',
    'adapter execution or authority',
    'host filtering',
    'enforcement command publication',
    'macOS or iOS local device proof from this Windows host',
  ],
};
writeJson(join(proofRoot, 'proof-summary.json'), proof);
writeJson(join(testRoot, 'proof.json'), proof);
console.log('network-live-capture-execution-proof-ok:execution-tests,clippy,source-shape,diff-check');
console.log(`proof=${join(proofRoot, 'proof-summary.json')}`);

function collectLocalHostObservations() {
  const androidSdkRoot =
    process.env.ANDROID_HOME ?? process.env.ANDROID_SDK_ROOT ?? join(process.env.LOCALAPPDATA ?? '', 'Android', 'Sdk');
  const adbPath = existingCommand(join(androidSdkRoot, 'platform-tools', 'adb.exe'), 'adb');
  const emulatorPath = existingCommand(join(androidSdkRoot, 'emulator', 'emulator.exe'), 'emulator');
  const probes = {
    windowsNetstat: runProbe('netstat', ['-ano']),
    windowsNpcapService: runProbe('sc.exe', ['query', 'npcap']),
    windowsPacketCaptureTool: runProbe('where.exe', ['dumpcap']),
    androidDevices: runProbe(adbPath, ['devices', '-l']),
    androidAvds: runProbe(emulatorPath, ['-list-avds']),
    wslStatus: runProbe('wsl.exe', ['--status']),
    wslCaptureTools: runProbe('wsl.exe', [
      'sh',
      '-lc',
      'command -v tcpdump || command -v dumpcap || command -v tshark || true',
    ]),
  };
  const windowsNpcapDriverAvailable = probes.windowsNpcapService.summary.status === 0;
  const windowsMetadataSnapshotExecuted = probes.windowsNetstat.summary.status === 0;
  const androidDeviceDetected = /\n[^\n]*\sdevice\s/u.test(probes.androidDevices.stdout);
  const androidAvdDetected = probes.androidAvds.stdout.trim().length > 0;
  const linuxCaptureToolObserved = probes.wslCaptureTools.stdout.trim().length > 0;

  return {
    checkedAt: new Date().toISOString(),
    hostPlatform: process.platform,
    windows: {
      npcapDriverProbeState: windowsNpcapDriverAvailable ? 'driver-observed' : 'manual-required',
      metadataSnapshotProbeState: windowsMetadataSnapshotExecuted ? 'metadata-observed' : 'unavailable',
      metadataSnapshotSubstitutesForLiveCapture: false,
      probes: [
        probes.windowsNetstat.summary,
        probes.windowsNpcapService.summary,
        probes.windowsPacketCaptureTool.summary,
      ],
    },
    android: {
      sdkProbeState:
        probes.androidDevices.summary.status === 0 || probes.androidAvds.summary.status === 0
          ? 'manual-required'
          : 'unavailable',
      deviceDetected: androidDeviceDetected,
      avdDetected: androidAvdDetected,
      liveVpnServiceOrPacketCaptureExecuted: false,
      probes: [probes.androidDevices.summary, probes.androidAvds.summary],
    },
    linux: {
      wslProbeState: probes.wslStatus.summary.status === 0 ? 'observed' : 'unavailable',
      captureToolObserved: linuxCaptureToolObserved,
      liveLibpcapCaptureExecuted: false,
      probes: [probes.wslStatus.summary, probes.wslCaptureTools.summary],
    },
    apple: {
      macOsLocalProbeState: process.platform === 'darwin' ? 'local-host' : 'ci-only-manual-device-required',
      iosLocalProbeState: 'ci-only-manual-device-required',
      localWindowsHostCanProveAppleNetworkExtension: false,
    },
    unsupportedClaims: {
      exactUrlClaimed: false,
      decryptedPayloadClaimed: false,
      pageContentClaimed: false,
      rawPcapWithoutCustodyClaimed: false,
      policyAuthorityClaimed: false,
      adapterAuthorityClaimed: false,
      hostFilteringClaimed: false,
      enforcementCommandClaimed: false,
    },
  };
}

function existingCommand(path, fallback) {
  return path && existsSync(path) ? path : fallback;
}

function runCommand(entry) {
  const result = spawnSync(entry.command, entry.args, { encoding: 'utf8', shell: false });
  const log = join(commandLogRoot, `${safeName(entry.name)}.log`);
  writeFileSync(log, normalizeCommandOutput(`${result.stdout ?? ''}${result.stderr ?? ''}`));
  if (result.status !== 0) {
    throw new Error(`${entry.name} failed with exit ${result.status}; log=${log}`);
  }
  return {
    name: entry.name,
    command: [entry.command, ...entry.args].join(' '),
    status: result.status,
    log,
  };
}

function runProbe(command, args) {
  const result = spawnSync(command, args, { encoding: 'utf8', shell: false });
  const stdout = normalizeProbeOutput(result.stdout ?? '');
  const stderr = normalizeProbeOutput(result.stderr ?? '');
  return {
    stdout,
    stderr,
    summary: {
      command: [command, ...args].join(' '),
      status: result.status,
      stdoutLineCount: lineCount(stdout),
      stderrLineCount: lineCount(stderr),
      stdoutSha256: hashText(stdout),
      stderrSha256: hashText(stderr),
      error: result.error?.message,
    },
  };
}

function runText(command, args) {
  const result = spawnSync(command, args, { encoding: 'utf8', shell: false });
  if (result.status !== 0) {
    throw new Error(`${command} ${args.join(' ')} failed with exit ${result.status}`);
  }
  return `${result.stdout ?? ''}${result.stderr ?? ''}`;
}

function sourceFingerprint(paths) {
  const hash = createHash('sha256');
  for (const path of paths) {
    hash.update(path);
    hash.update('\0');
    hash.update(readFileSync(path, 'utf8').replace(/\r\n/gu, '\n'));
    hash.update('\0');
  }
  return hash.digest('hex');
}

function writeJson(path, value) {
  writeFileSync(path, `${JSON.stringify(value, null, 2)}\n`);
}

function validationCommandsLog(results) {
  return `${results.map((result) => `${result.name}: ${result.command} -> exit ${result.status}; log=${result.log}`).join('\n')}\n`;
}

function safeName(value) {
  return value.replace(/[^a-zA-Z0-9_.-]/gu, '-');
}

function normalizeProbeOutput(value) {
  return value.replace(/\r\n/gu, '\n').trim();
}

function lineCount(value) {
  return value.length === 0 ? 0 : value.split('\n').length;
}

function hashText(value) {
  return createHash('sha256').update(value).digest('hex');
}

function normalizeCommandOutput(value) {
  return `${value
    .replace(/\r\n/gu, '\n')
    .replace(/\\/gu, '/')
    .replace(/target\/debug\/deps\/[^\s)]+/gu, 'target/debug/deps/<test-binary>')
    .replace(/\b\d+\.\d+s\b/gu, '<duration>s')
    .replace(/\b\d+\.\d{2}ms\b/gu, '<duration>ms')
    .replace(/target\(s\) in [^\n]+/gu, 'target(s) in <duration>')
    .replace(/finished in [^\n]+/giu, 'finished in <duration>')
    .trim()}\n`;
}
