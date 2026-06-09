import { spawn } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';

const repoRoot = process.cwd();
const proofMode = 'app-game-platform-proof-status-proof';
const outputDir = join(repoRoot, 'test-results', proofMode);
const proofPath = join(outputDir, 'proof.json');
const appGameProofDir = join(repoRoot, 'output', 'app-game-plan-proof', '183-app-game-platform-proof-status-surface');
const serviceProofDir = join(
  repoRoot,
  'output',
  'app-game-plan-proof',
  '184-app-game-platform-proof-status-service-surface'
);
const detailProofDir = join(
  repoRoot,
  'output',
  'app-game-plan-proof',
  '195-app-game-platform-proof-status-preflight-detail-refs'
);
const appleCiProofDir = join(
  repoRoot,
  'output',
  'app-game-plan-proof',
  '196-app-game-apple-ci-platform-proof-preflight'
);
const dockerProofDir = join(
  repoRoot,
  'output',
  'app-game-plan-proof',
  '197-app-game-linux-docker-host-preflight'
);
const commands = [];

await main();

async function main() {
  await mkdir(outputDir, { recursive: true });
  await mkdir(appGameProofDir, { recursive: true });
  await mkdir(serviceProofDir, { recursive: true });
  await mkdir(detailProofDir, { recursive: true });
  await mkdir(appleCiProofDir, { recursive: true });
  await mkdir(dockerProofDir, { recursive: true });

  await runCommand('cmd', ['/c', 'npm', 'run', 'build:contracts']);
  await runCommand('cargo', ['test', '-p', 'ocentra-parent-agent-protocol', 'app_game_platform_proof_status']);
  await runCommand('cargo', ['test', '-p', 'ocentra-parent-agent-service', 'app_game_platform_proof_status']);
  await runCommand('cmd', [
    '/c',
    'npm',
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/agent-protocol-domain',
    '--',
    'app-game-platform-proof-status',
  ]);
  await runCommand('cmd', [
    '/c',
    'npm',
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/parent-domain',
    '--',
    'app-game-platform-proof-status',
    'app-game-android-physical-device-proof',
    'app-game-android-authority-preflight',
    'app-game-android-accessibility-overlay-preflight',
    'app-game-android-accessibility-runtime-proof',
    'app-game-android-usage-events-replay',
    'app-game-linux-active-window-tool-proof',
    'app-game-linux-foreground-capture-readiness',
    'app-game-linux-docker-host-preflight',
    'app-game-linux-wsl-runtime-proof',
    'app-game-windows-broad-blocking-authority-preflight',
    'app-game-windows-local-policy-evidence-proof',
    'app-game-apple-ci-platform-proof-preflight',
  ]);
  await runCommand('cmd', [
    '/c',
    'npm',
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/portal-domain',
    '--',
    'app-game-platform-proof-status-panel',
  ]);
  await runCommand('cmd', [
    '/c',
    'npm',
    'exec',
    '--workspace',
    '@ocentra-parent/portal',
    '--',
    'vitest',
    'run',
    'tests/app-game-platform-proof-status-route-panel.test.ts',
  ]);
  await runCommand('node', ['scripts/test/app-game-android-physical-device-proof.mjs']);
  await runCommand('node', ['scripts/test/app-game-android-authority-preflight-proof.mjs']);
  await runCommand('node', ['scripts/test/app-game-android-accessibility-overlay-preflight-proof.mjs']);
  await runCommand('node', ['scripts/test/app-game-android-accessibility-runtime-proof.mjs']);
  await runCommand('node', ['scripts/test/app-game-linux-wsl-runtime-proof.mjs']);
  await runCommand('node', ['scripts/test/app-game-android-usage-events-replay-proof.mjs']);
  await runCommand('node', ['scripts/test/app-game-linux-foreground-capture-readiness-proof.mjs']);
  await runCommand('node', ['scripts/test/app-game-linux-active-window-tool-proof.mjs']);
  await runCommand('node', ['scripts/test/app-game-windows-broad-blocking-authority-preflight-proof.mjs']);
  await runCommand('node', ['scripts/test/app-game-windows-local-policy-evidence-proof.mjs']);
  await runCommand('node', ['scripts/test/app-game-apple-ci-platform-proof-preflight-proof.mjs']);
  await runCommand('node', ['scripts/test/app-game-linux-docker-host-preflight-proof.mjs']);
  await runCommand('cmd', ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/parent-domain']);

  const androidProof = await readJson(join(repoRoot, 'test-results', 'app-game-android-physical-device-proof', 'proof.json'));
  const linuxProof = await readJson(join(repoRoot, 'test-results', 'app-game-linux-wsl-runtime-proof', 'proof.json'));
  const dockerProof = await readJson(
    join(repoRoot, 'test-results', 'app-game-linux-docker-host-preflight-proof', 'proof.json')
  );
  const androidAccessibilityRuntimeProof = await readJson(
    join(repoRoot, 'test-results', 'app-game-android-accessibility-runtime-proof', 'proof.json')
  );
  const linuxActiveWindowToolProof = await readJson(
    join(repoRoot, 'test-results', 'app-game-linux-active-window-tool-proof', 'proof.json')
  );
  const windowsLocalPolicyEvidenceProof = await readJson(
    join(repoRoot, 'test-results', 'app-game-windows-local-policy-evidence-proof', 'proof.json')
  );
  const androidAuthorityModule = await import(
    pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', 'app-game-android-authority-preflight.js')).href
  );
  const androidAccessibilityModule = await import(
    pathToFileURL(
      join(repoRoot, 'packages', 'parent-domain', 'dist', 'app-game-android-accessibility-overlay-preflight.js')
    ).href
  );
  const replayModule = await import(
    pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', 'app-game-android-usage-events-replay.js')).href
  );
  const linuxForegroundModule = await import(
    pathToFileURL(
      join(repoRoot, 'packages', 'parent-domain', 'dist', 'app-game-linux-foreground-capture-readiness.js')
    ).href
  );
  const linuxDockerModule = await import(
    pathToFileURL(
      join(repoRoot, 'packages', 'parent-domain', 'dist', 'app-game-linux-docker-host-preflight.js')
    ).href
  );
  const module = await import(
    pathToFileURL(join(repoRoot, 'packages', 'parent-domain', 'dist', 'app-game-platform-proof-status.js')).href
  );
  const appleCiModule = await import(
    pathToFileURL(
      join(repoRoot, 'packages', 'parent-domain', 'dist', 'app-game-apple-ci-platform-proof-preflight.js')
    ).href
  );
  const windowsModule = await import(
    pathToFileURL(
      join(repoRoot, 'packages', 'parent-domain', 'dist', 'app-game-windows-broad-blocking-authority-preflight.js')
    ).href
  );
  const androidAuthorityPreflight = androidAuthorityModule.createAppGameAndroidAuthorityPreflightReadModel({
    androidProof: androidProof.readModel,
    generatedAt: '2026-06-08T16:19:10.000Z',
  });
  const androidAccessibilityOverlayPreflight =
    androidAccessibilityModule.createAppGameAndroidAccessibilityOverlayPreflightReadModel({
      androidProof: androidProof.readModel,
      accessibilitySettings: {
        accessibilityEnabled: true,
        enabledServiceCount: 1,
        serviceNamesRedacted: true,
        settingsReadable: true,
      },
      generatedAt: '2026-06-08T16:19:20.000Z',
    });
  const androidUsageEventsReplay = replayModule.createAppGameAndroidUsageEventsReplayReadModel({
    androidProof: androidProof.readModel,
    generatedAt: '2026-06-08T16:19:00.000Z',
  });
  const linuxForegroundCaptureReadiness = linuxForegroundModule.createAppGameLinuxForegroundCaptureReadiness({
    linuxProof: linuxProof.readModel,
    generatedAt: '2026-06-08T16:19:30.000Z',
  });
  const linuxDockerHostPreflight = linuxDockerModule.createAppGameLinuxDockerHostPreflightReadModel({
    dockerCliObserved: dockerProof.readModel.dockerCliObserved,
    dockerDaemonObserved: dockerProof.readModel.dockerDaemonObserved,
    contextCount: dockerProof.readModel.contextCount,
    imageCount: dockerProof.readModel.imageCount,
    containerCount: dockerProof.readModel.containerCount,
    generatedAt: '2026-06-08T16:19:35.000Z',
  });
  const windowsBroadBlockingAuthorityPreflight =
    windowsModule.createAppGameWindowsBroadBlockingAuthorityPreflightReadModel({
      generatedAt: '2026-06-08T16:19:40.000Z',
    });
  const appleCiPlatformProofPreflight = appleCiModule.createAppGameAppleCiPlatformProofPreflightReadModel({
    generatedAt: '2026-06-08T16:19:50.000Z',
  });
  const readModel = module.createAppGamePlatformProofStatusReadModel({
    androidProof: androidProof.readModel,
    androidAuthorityPreflight,
    androidAccessibilityOverlayPreflight,
    androidAccessibilityRuntimeProof: androidAccessibilityRuntimeProof.readModel,
    androidUsageEventsReplay,
    linuxProof: linuxProof.readModel,
    linuxForegroundCaptureReadiness,
    linuxActiveWindowToolProof: linuxActiveWindowToolProof.readModel,
    linuxDockerHostPreflight,
    windowsBroadBlockingAuthorityPreflight,
    windowsLocalPolicyEvidenceProof: windowsLocalPolicyEvidenceProof.readModel,
    appleCiPlatformProofPreflight,
    generatedAt: '2026-06-08T16:20:00.000Z',
  });
  const summary = module.summarizeAppGamePlatformProofStatus(readModel);

  assertEqual(summary.platformProofObservedCount, 5, 'platform proof count');
  assertEqual(summary.visibilityOnlyCount, 5, 'visibility-only count');
  assertEqual(summary.enforcementReadyCount, 0, 'enforcement-ready count');
  assertPositive(summary.openGapCount, 'open gap count');

  const serviceSurface = {
    command: 'agent.activity.app-game.platform-proof-status.read-model.get',
    event: 'agent.activity.app-game.platform-proof-status.read-model.reported',
    payloadField: 'appGamePlatformProofStatusReadModel',
    readModelId: 'app-game-platform-proof-status',
    rows: ['windows', 'android', 'linux', 'macos', 'ios'],
    androidProofRefs: [
      'android-adb-host-toolchain-ref',
      'android-physical-device-proof-ref',
      'android-usage-events-foreground-ref',
      'android-accessibility-runtime-proof-ref',
    ],
    androidOpenGaps: [
      'android-device-owner-not-proved',
      'android-durable-usage-events-replay-not-proved',
      'platform-enforcement-not-proved',
      'child-device-delivery-not-proved',
    ],
    linuxProofRefs: [
      'linux-wsl-host-toolchain-ref',
      'linux-wslg-display-ref',
      'linux-wslg-x11-socket-ref',
      'linux-wslg-wayland-socket-ref',
      'linux-active-window-tool-proof-ref',
    ],
    linuxOpenGaps: [
      'linux-native-service-not-proved',
      'linux-foreground-capture-not-proved',
      'linux-rollback-not-proved',
      'platform-enforcement-not-proved',
      'child-device-delivery-not-proved',
    ],
    enforcementReadyCount: 0,
    claims: {
      adapterDispatchClaimed: false,
      broadInstalledAppBlockingClaimed: false,
      platformEnforcementClaimed: false,
      providerDeliveryClaimed: false,
      childDeviceDeliveryClaimed: false,
      privateDiagnosticsClaimed: false,
    },
  };

  const proof = {
    schemaVersion: 1,
    proofMode,
    checkedAt: 'deterministic-proof-artifact',
    commit: await gitHead(),
    commands,
    readModel,
    summary,
    evidence: {
      contract: 'packages/parent-domain/src/app-game-platform-proof-status.ts',
      contractTest: 'packages/parent-domain/tests/app-game-platform-proof-status.test.ts',
      windowsBroadBlockingAuthorityPreflight:
        'packages/parent-domain/src/app-game-windows-broad-blocking-authority-preflight.ts',
      windowsBroadBlockingAuthorityPreflightTest:
        'packages/parent-domain/tests/app-game-windows-broad-blocking-authority-preflight.test.ts',
      windowsLocalPolicyEvidence:
        'packages/parent-domain/src/app-game-windows-local-policy-evidence-proof.ts',
      windowsLocalPolicyEvidenceTest:
        'packages/parent-domain/tests/app-game-windows-local-policy-evidence-proof.test.ts',
      androidAuthorityPreflight:
        'packages/parent-domain/src/app-game-android-authority-preflight.ts',
      androidAuthorityPreflightTest:
        'packages/parent-domain/tests/app-game-android-authority-preflight.test.ts',
      androidAccessibilityOverlayPreflight:
        'packages/parent-domain/src/app-game-android-accessibility-overlay-preflight.ts',
      androidAccessibilityOverlayPreflightTest:
        'packages/parent-domain/tests/app-game-android-accessibility-overlay-preflight.test.ts',
      androidAccessibilityRuntimeProof:
        'packages/parent-domain/src/app-game-android-accessibility-runtime-proof.ts',
      androidAccessibilityRuntimeProofTest:
        'packages/parent-domain/tests/app-game-android-accessibility-runtime-proof.test.ts',
      appleCiPlatformProofPreflight:
        'packages/parent-domain/src/app-game-apple-ci-platform-proof-preflight.ts',
      appleCiPlatformProofPreflightTest:
        'packages/parent-domain/tests/app-game-apple-ci-platform-proof-preflight.test.ts',
      portalIntent: 'packages/portal-domain/src/app-game-platform-proof-status-panel.ts',
      portalTest: 'packages/portal-domain/tests/app-game-platform-proof-status-panel.test.ts',
      protocolContract: 'packages/agent-protocol-domain/src/app-game-platform-proof-status.ts',
      protocolTest: 'packages/agent-protocol-domain/tests/app-game-platform-proof-status.test.ts',
      androidUsageEventsReplay:
        'packages/parent-domain/src/app-game-android-usage-events-replay.ts',
      androidUsageEventsReplayTest:
        'packages/parent-domain/tests/app-game-android-usage-events-replay.test.ts',
      linuxForegroundCaptureReadiness:
        'packages/parent-domain/src/app-game-linux-foreground-capture-readiness.ts',
      linuxForegroundCaptureReadinessTest:
        'packages/parent-domain/tests/app-game-linux-foreground-capture-readiness.test.ts',
      linuxActiveWindowToolProof:
        'packages/parent-domain/src/app-game-linux-active-window-tool-proof.ts',
      linuxActiveWindowToolProofTest:
        'packages/parent-domain/tests/app-game-linux-active-window-tool-proof.test.ts',
      linuxDockerHostPreflight:
        'packages/parent-domain/src/app-game-linux-docker-host-preflight.ts',
      linuxDockerHostPreflightTest:
        'packages/parent-domain/tests/app-game-linux-docker-host-preflight.test.ts',
      rustProtocol: 'crates/agent-protocol/src/app_game_platform_proof_status.rs',
      rustService: 'crates/agent-service/src/activity_api/app_game_platform_proof_status_payload.rs',
      portalRoute: 'apps/portal/src/AppGamePlatformProofStatusRoutePanel.tsx',
      portalRouteTest: 'apps/portal/tests/app-game-platform-proof-status-route-panel.test.ts',
      androidProof: 'test-results/app-game-android-physical-device-proof/proof.json',
      androidReplayProof: 'test-results/app-game-android-usage-events-replay-proof/proof.json',
      linuxProof: 'test-results/app-game-linux-wsl-runtime-proof/proof.json',
      linuxForegroundReadinessProof:
        'test-results/app-game-linux-foreground-capture-readiness-proof/proof.json',
      linuxActiveWindowToolProofArtifact:
        'test-results/app-game-linux-active-window-tool-proof/proof.json',
      linuxDockerHostPreflightProof: 'test-results/app-game-linux-docker-host-preflight-proof/proof.json',
      windowsBroadBlockingProof:
        'test-results/app-game-windows-broad-blocking-authority-preflight-proof/proof.json',
      windowsLocalPolicyEvidenceProof:
        'test-results/app-game-windows-local-policy-evidence-proof/proof.json',
      androidAuthorityProof: 'test-results/app-game-android-authority-preflight-proof/proof.json',
      androidAccessibilityProof:
        'test-results/app-game-android-accessibility-overlay-preflight-proof/proof.json',
      androidAccessibilityRuntimeProofArtifact:
        'test-results/app-game-android-accessibility-runtime-proof/proof.json',
      appleCiProof: 'test-results/app-game-apple-ci-platform-proof-preflight-proof/proof.json',
    },
    windowsBroadBlockingAuthorityPreflight,
    androidAuthorityPreflight,
    androidAccessibilityOverlayPreflight,
    androidAccessibilityRuntimeProof,
    appleCiPlatformProofPreflight,
    androidUsageEventsReplay,
    linuxForegroundCaptureReadiness,
    linuxActiveWindowToolProof,
    linuxDockerHostPreflight,
    windowsLocalPolicyEvidenceProof,
    serviceSurface,
    claimsProved: [
      'Android physical device proof is visible in the parent-safe platform status surface',
      'Android authority preflight blockers are visible without claiming Device Owner or Profile Owner authority',
      'Android Accessibility overlay preflight blockers are visible with enabled-service names redacted',
      'Android Accessibility runtime declaration proof is visible without claiming overlay execution',
      'Android UsageEvents replay readiness is visible in the parent-domain platform status surface',
      'Windows broad-blocking authority preflight blockers are visible without claiming AppLocker, App Control, rollback, audit, or adapter dispatch readiness',
      'Windows local AppLocker/App Control policy evidence is visible without storing raw policy XML or claiming broad blocking',
      'macOS and iOS CI-required proof blockers are visible without claiming Windows-local Apple proof, adapter dispatch, or platform enforcement',
      'Android usage-events foreground activity evidence is visible as a proof ref without claiming durable replay',
      'Linux WSL runtime proof is visible in the parent-safe platform status surface',
      'Linux WSLg display and socket readiness refs are visible without claiming foreground capture',
      'Linux foreground capture readiness is visible without claiming active foreground capture',
      'Linux active-window tool availability is visible without raw title custody or foreground capture claims',
      'Linux Docker host preflight is visible without storing raw context, image, or container identifiers',
      'The status surface keeps native platform enforcement and child delivery unclaimed',
      'The portal intent renders platform proof rows as review-only visibility status',
      'The Rust agent protocol exposes a stable platform proof status command and event',
      'The Rust service routes the command to a live host-capability-backed read model',
      'The app portal route parses the reported read model and renders it on App/Game Sessions',
    ],
    claimsNotProved: [
      'Android Device Owner or Profile Owner authority',
      'Android app hide, suspend, uninstall, managed configuration, or usage event replay',
      'Linux X11 or Wayland foreground capture',
      'Linux AppArmor, SELinux, package, Flatpak, Snap, rollback, or audit enforcement',
      'Linux Docker container policy, container blocking, or container enforcement',
      'macOS Screen Time or Endpoint Security runtime proof on this Windows host',
      'iOS Family Controls or MDM runtime proof on this Windows host',
    ],
  };

  await writeJson(proofPath, proof);
  await writeJson(join(appGameProofDir, 'proof.json'), proof);
  await writeJson(join(serviceProofDir, 'proof.json'), proof);
  await writeJson(join(detailProofDir, 'proof.json'), proof);
  await writeJson(join(appleCiProofDir, 'platform-status-proof.json'), proof);
  await writeJson(join(dockerProofDir, 'platform-status-proof.json'), proof);
  await writeFile(
    join(appGameProofDir, '00-source-snapshot.md'),
    [
      '# App-game platform proof status source snapshot',
      '',
      '- Branch: codex/app-game-control-product-completion',
      '- Commit: uncommitted full-goal batch, validated by harness before final checkpoint commit',
      '- Parent read model: packages/parent-domain/src/app-game-platform-proof-status.ts',
      '- Portal intent: packages/portal-domain/src/app-game-platform-proof-status-panel.ts',
      '',
      'Evidence:',
      '- Android physical-device proof is surfaced as visibility-only platform evidence.',
      '- Linux WSL runtime proof is surfaced as visibility-only platform evidence.',
      '- Enforcement, broad blocking, adapter dispatch, and child delivery remain unclaimed until platform authority proof is attached.',
      '',
    ].join('\n')
  );
  await writeFile(join(appGameProofDir, '10-validation-commands.log'), `${commands.join('\n')}\n`);
  await writeFile(
    join(serviceProofDir, '00-source-snapshot.md'),
    [
      '# App-game platform proof status service surface source snapshot',
      '',
      '- Branch: codex/app-game-control-product-completion',
      '- Commit: uncommitted full-goal batch, validated by harness before final checkpoint commit',
      '- TypeScript protocol: packages/agent-protocol-domain/src/app-game-platform-proof-status.ts',
      '- Rust protocol: crates/agent-protocol/src/app_game_platform_proof_status.rs',
      '- Rust service: crates/agent-service/src/activity_api/app_game_platform_proof_status_payload.rs',
      '- Portal route: apps/portal/src/AppGamePlatformProofStatusRoutePanel.tsx',
      '',
      'Evidence:',
      '- The service command returns Windows, Android, Linux, macOS, and iOS platform proof status rows.',
      '- Host capability signals are parent-safe refs only, not raw local paths or private diagnostics.',
      '- Adapter dispatch, broad blocking, platform enforcement, provider delivery, and child delivery stay unclaimed.',
      '',
    ].join('\n')
  );
  await writeFile(join(serviceProofDir, '10-validation-commands.log'), `${commands.join('\n')}\n`);
  await writeFile(
    join(detailProofDir, '00-source-snapshot.md'),
    [
      '# App-game platform proof status preflight detail refs',
      '',
      '- Branch: codex/app-game-control-product-completion',
      '- Commit: uncommitted full-goal batch, validated by harness before final checkpoint commit',
      '- Parent read model: packages/parent-domain/src/app-game-platform-proof-status.ts',
      '- Windows authority preflight: packages/parent-domain/src/app-game-windows-broad-blocking-authority-preflight.ts',
      '- Android authority preflight: packages/parent-domain/src/app-game-android-authority-preflight.ts',
      '- Android Accessibility overlay preflight: packages/parent-domain/src/app-game-android-accessibility-overlay-preflight.ts',
      '',
      'Evidence:',
      '- Windows, Android, and Linux rows share one platform proof status read model.',
      '- macOS and iOS CI-required rows share the same platform proof status read model.',
      '- Windows broad-blocking, Android authority, and Android Accessibility rows stay visibility-only.',
      '- Adapter dispatch, broad blocking, platform enforcement, provider delivery, and child delivery remain unclaimed.',
      '',
    ].join('\n')
  );
  await writeFile(join(detailProofDir, '10-validation-commands.log'), `${commands.join('\n')}\n`);
  await writeFile(
    join(appleCiProofDir, '01-platform-status-consumption.md'),
    [
      '# Apple CI preflight status consumption',
      '',
      '- Platform status consumes the Apple CI preflight rows as macOS/iOS visibility-only rows.',
      '- This does not claim local Windows proof for macOS or iOS.',
      '- CI/device artifacts remain required before adapter dispatch or platform enforcement.',
      '',
    ].join('\n')
  );
  await writeFile(
    join(dockerProofDir, '01-platform-status-consumption.md'),
    [
      '# Linux Docker host preflight status consumption',
      '',
      '- Platform status consumes the Linux Docker host preflight as a Linux visibility-only proof ref.',
      '- Docker context, image, and container inventories are represented as counts only.',
      '- This does not claim Docker container policy, adapter dispatch, platform enforcement, or child delivery.',
      '',
    ].join('\n')
  );

  console.log('app-game-platform-proof-status-proof-ok');
  console.log(`evidence=${relative(repoRoot, proofPath)}`);
}

async function runCommand(command, args) {
  const commandLine = [command, ...args].join(' ');
  commands.push(commandLine);
  await new Promise((resolve, reject) => {
    const child = spawn(command, args, { cwd: repoRoot, stdio: 'inherit', windowsHide: true });
    child.once('exit', (code) => (code === 0 ? resolve() : reject(new Error(`${commandLine} exited with ${code}`))));
    child.once('error', reject);
  });
}

async function readJson(path) {
  return JSON.parse(await readFile(path, 'utf8'));
}

async function writeJson(path, value) {
  await writeFile(path, `${JSON.stringify(value, null, 2)}\n`);
}

async function gitHead() {
  const chunks = [];
  await new Promise((resolve, reject) => {
    const child = spawn('git', ['rev-parse', 'HEAD'], { cwd: repoRoot, stdio: ['ignore', 'pipe', 'pipe'] });
    child.stdout.on('data', (chunk) => chunks.push(String(chunk)));
    child.once('exit', (code) => (code === 0 ? resolve() : reject(new Error('git rev-parse HEAD failed'))));
    child.once('error', reject);
  });
  return chunks.join('').trim();
}

function assertEqual(actual, expected, label) {
  if (actual !== expected) {
    throw new Error(`${label}: expected ${expected}, received ${actual}`);
  }
}

function assertPositive(actual, label) {
  if (actual <= 0) {
    throw new Error(`${label}: expected positive count, received ${actual}`);
  }
}
