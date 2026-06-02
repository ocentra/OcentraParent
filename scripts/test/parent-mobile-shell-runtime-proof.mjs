import { spawn } from 'node:child_process';
import { existsSync } from 'node:fs';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const repoRoot = process.cwd();
const outputDir = join(repoRoot, 'test-results', 'parent-mobile-shell-runtime-proof');
const proofPath = join(outputDir, 'proof.json');
const commands = [];
const proofLabels = [];

const androidParentMobileCapabilities = [
  capabilityProof(
    'parent-mobile-observer',
    'scaffold',
    'typed observer read model and package launch target',
    'observer state is represented without mobile UX parity'
  ),
  capabilityProof(
    'parent-mobile-controller',
    'manual-required',
    'real mobile package and device controller takeover proof',
    'no parent mobile write authority is claimed from scaffold state'
  ),
  capabilityProof(
    'foreground-mobile-service',
    'manual-required',
    'Android emulator or device foreground-service and notification proof',
    'manifest declaration is not foreground behavior proof'
  ),
  capabilityProof(
    'notifications',
    'manual-required',
    'Android notification permission prompt and delivery proof',
    'permission declaration is not runtime notification proof'
  ),
  capabilityProof(
    'package-lifecycle',
    'manual-required',
    'Android install launch background update and uninstall proof',
    'debug package mechanics are not store or lifecycle proof'
  ),
  capabilityProof(
    'store-distribution',
    'planned',
    'Google Play signing and release-track proof',
    'store distribution is not wired'
  ),
];

const iosParentMobileCapabilities = [
  capabilityProof(
    'parent-mobile-observer',
    'scaffold',
    'typed observer read model and simulator app target',
    'observer state is represented without mobile UX parity'
  ),
  capabilityProof(
    'parent-mobile-controller',
    'manual-required',
    'real signed mobile package and device controller takeover proof',
    'no parent mobile write authority is claimed from simulator scaffold'
  ),
  capabilityProof(
    'foreground-mobile-service',
    'unavailable',
    'iOS has no Android-style foreground service',
    'foreground service is not an iOS parent mobile claim'
  ),
  capabilityProof(
    'notifications',
    'manual-required',
    'iOS notification permission and delivery proof',
    'notification behavior requires device or simulator permission evidence'
  ),
  capabilityProof(
    'background-execution',
    'manual-required',
    'iOS background mode entitlement and device behavior proof',
    'simulator app target is not background execution proof'
  ),
  capabilityProof(
    'signing-entitlements',
    'manual-required',
    'Apple signing team provisioning and entitlement proof',
    'simulator build is not signing or entitlement proof'
  ),
  capabilityProof(
    'testflight-distribution',
    'manual-required',
    'TestFlight build upload install and launch proof',
    'TestFlight distribution is not wired'
  ),
  capabilityProof('store-distribution', 'planned', 'App Store release-track proof', 'store distribution is not wired'),
];

await main();

async function main() {
  await mkdir(outputDir, { recursive: true });

  await runNpm(['run', 'build:contracts']);
  await runNpm([
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/parent-domain',
    '--',
    'tests/parent-mobile-runtime.test.ts',
  ]);

  const runtimeModels = await parentMobileRuntimeModels();
  const packageProof = await assertPackageShells();
  const capabilityProof = await assertParentMobileCapabilityData(runtimeModels);
  const matrixProof = await assertProofMatrix();
  const scriptProof = await assertScriptWiring();

  assertRuntimeModel(runtimeModels.androidObserver, 'android');
  assertRuntimeModel(runtimeModels.iosObserver, 'ios');

  const proof = {
    schemaVersion: 1,
    checkedAt: new Date().toISOString(),
    commit: await gitHead(),
    commands,
    proofLabels,
    evidence: {
      parentDomainContract: relative(
        repoRoot,
        join(repoRoot, 'packages', 'parent-domain', 'src', 'parent-mobile-runtime.ts')
      ),
      parentDomainContractTest: relative(
        repoRoot,
        join(repoRoot, 'packages', 'parent-domain', 'tests', 'parent-mobile-runtime.test.ts')
      ),
      matrix: relative(repoRoot, join(repoRoot, 'docs', 'expectations', 'pre-ai-proof-matrix.json')),
      output: relative(repoRoot, proofPath),
    },
    runtimeProof: {
      androidObserver: summarizeRuntimeModel(runtimeModels.androidObserver),
      iosObserver: summarizeRuntimeModel(runtimeModels.iosObserver),
      platformCapabilities: capabilityProof,
      localModelExecutionDefault: 'disabled-by-default',
      childAgentBehaviorClaim: 'not-claimed',
    },
    packageLaunchProof: packageProof,
    ciProof: {
      matrixClaim: matrixProof,
      scriptWiring: scriptProof,
    },
    knownGaps: [
      'Parent mobile controller takeover remains manual-required until a real Android or iOS mobile package/device proof exists.',
      'LAN AI provider submission is represented through typed route/degraded states; no local model execution runs on parent mobile by default.',
      'Cloud relay remains not implemented and is not counted as LAN or local service proof.',
      'Android notification permission, foreground-service behavior, package lifecycle, and Google Play distribution remain manual-required or planned until emulator/device/store proof exists.',
      'iOS notification permission, background execution, signing, TestFlight, and App Store distribution remain manual-required or planned until Mac/Xcode/device proof exists.',
      'Android/iOS child-agent permissions, device-owner policy, Family Controls, TestFlight, and store signing remain separate manual-required platform claims.',
    ],
  };

  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  console.log(`parent-mobile-shell-runtime-proof-ok:${proofLabels.join(',')}`);
  console.log(`evidence=${proofPath}`);
}

async function parentMobileRuntimeModels() {
  const modulePath = join(repoRoot, 'packages', 'parent-domain', 'dist', 'parent-mobile-runtime.js');
  if (!existsSync(modulePath)) {
    throw new Error(`Missing built parent mobile runtime module: ${modulePath}`);
  }
  const module = await import(`file:///${modulePath.replaceAll('\\', '/')}`);
  const androidObserver = module.ParentMobileRuntimeReadModelSchema.parse({
    schemaVersion: 'v0.9-parent-mobile-shell',
    parentDeviceId: 'parent-mobile-android-observer',
    platform: 'android',
    packageProof: {
      platform: 'android',
      packageState: 'ci-mechanical-proof',
      launchTarget: 'ca.ocentra.parent.agent/.MainActivity',
      proofCommand: 'cmd /c npm run release:package:android',
      signingState: 'manual-required',
      storeDistributionState: 'manual-required',
    },
    serviceAvailability: serviceAvailability('manual-required', 'degraded', 'route-parent-mobile-lan-provider'),
    controllerProof: {
      controllerState: 'observer',
      controllerLeaseId: null,
      takeoverRequestAllowed: false,
      commandAuthorityState: 'observer-read-only',
    },
    assistantJobProof: {
      route: 'lan-ai-provider',
      jobState: 'degraded',
      providerId: null,
      requiredCapabilities: ['chat-completion', 'summarization'],
      evidenceReferenceIds: ['activity-event-parent-mobile-proof'],
      unavailableReason: 'lan-ai-provider-unavailable',
    },
    localModelExecutionState: 'disabled-by-default',
    localModelExecutionAllowed: false,
    childAgentBehaviorClaim: 'not-claimed',
    platformCapabilities: androidParentMobileCapabilities,
    updatedAt: '2026-05-28T16:00:00.000Z',
  });
  const iosObserver = module.ParentMobileRuntimeReadModelSchema.parse({
    schemaVersion: 'v0.9-parent-mobile-shell',
    parentDeviceId: 'parent-mobile-ios-observer',
    platform: 'ios',
    packageProof: {
      platform: 'ios',
      packageState: 'ci-mechanical-proof',
      launchTarget: 'ca.ocentra.parent.agent',
      proofCommand: 'bash scripts/release/ios/build-simulator-app.sh',
      signingState: 'manual-required',
      storeDistributionState: 'manual-required',
    },
    serviceAvailability: serviceAvailability('manual-required', 'manual-required', null),
    controllerProof: {
      controllerState: 'manual-required',
      controllerLeaseId: null,
      takeoverRequestAllowed: true,
      commandAuthorityState: 'controller-takeover-manual-required',
    },
    assistantJobProof: {
      route: 'unavailable',
      jobState: 'unavailable',
      providerId: null,
      requiredCapabilities: ['chat-completion', 'summarization'],
      evidenceReferenceIds: [],
      unavailableReason: 'mobile-package-proof-required',
    },
    localModelExecutionState: 'disabled-by-default',
    localModelExecutionAllowed: false,
    childAgentBehaviorClaim: 'not-claimed',
    platformCapabilities: iosParentMobileCapabilities,
    updatedAt: '2026-05-28T16:00:00.000Z',
  });
  proofLabels.push('parent-mobile-runtime.contract-parse');
  return { androidObserver, iosObserver };
}

async function assertPackageShells() {
  const androidManifest = await readRepoFile('platforms/android/agent/app/src/main/AndroidManifest.xml');
  const androidActivity = await readRepoFile(
    'platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/MainActivity.java'
  );
  const androidService = await readRepoFile(
    'platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/OcentraParentAgentService.java'
  );
  const androidReleaseScript = await readRepoFile('scripts/release/android/build-agent-package.mjs');
  const iosProject = await readRepoFile('platforms/ios/OcentraParentAgent.xcodeproj/project.pbxproj');
  const iosViewController = await readRepoFile('platforms/ios/OcentraParentAgent/AgentStatusViewController.swift');
  const iosReleaseScript = await readRepoFile('scripts/release/ios/build-simulator-app.sh');

  assertIncludes(androidManifest, 'android.intent.action.MAIN', 'Android launch activity');
  assertIncludes(androidManifest, 'FOREGROUND_SERVICE_DATA_SYNC', 'Android foreground service permission');
  assertIncludes(androidManifest, 'POST_NOTIFICATIONS', 'Android notification permission declaration');
  assertIncludes(androidActivity, 'startForegroundService', 'Android service launch path');
  assertIncludes(androidService, 'startForeground', 'Android foreground service start');
  assertIncludes(androidReleaseScript, 'gradlew.bat assembleDebug', 'Android debug package command');
  assertIncludes(iosProject, 'productType = "com.apple.product-type.application"', 'iOS app product target');
  assertIncludes(iosProject, 'PRODUCT_BUNDLE_IDENTIFIER = ca.ocentra.parent.agent', 'iOS bundle id');
  assertIncludes(iosViewController, 'Ocentra Parent Agent iOS scaffold', 'iOS scaffold status surface');
  assertIncludes(iosReleaseScript, 'xcodebuild', 'iOS simulator build command');

  proofLabels.push('parent-mobile-package.launch-targets');
  return {
    android: {
      state: 'ci-mechanical-proof',
      launchTarget: 'ca.ocentra.parent.agent/.MainActivity',
      packageCommand: 'cmd /c npm run release:package:android',
      evidenceFiles: [
        'platforms/android/agent/app/src/main/AndroidManifest.xml',
        'platforms/android/agent/app/src/main/java/ca/ocentra/parent/agent/MainActivity.java',
        'scripts/release/android/build-agent-package.mjs',
      ],
    },
    ios: {
      state: 'ci-mechanical-proof',
      launchTarget: 'ca.ocentra.parent.agent',
      packageCommand: 'bash scripts/release/ios/build-simulator-app.sh',
      evidenceFiles: [
        'platforms/ios/OcentraParentAgent.xcodeproj/project.pbxproj',
        'platforms/ios/OcentraParentAgent/AgentStatusViewController.swift',
        'scripts/release/ios/build-simulator-app.sh',
      ],
    },
  };
}

async function assertParentMobileCapabilityData(runtimeModels) {
  const modulePath = join(repoRoot, 'packages', 'parent-domain', 'dist', 'capabilities.js');
  if (!existsSync(modulePath)) {
    throw new Error(`Missing built platform capability module: ${modulePath}`);
  }
  const module = await import(`file:///${modulePath.replaceAll('\\', '/')}`);
  const platformCapabilities = module.ParentControlPlatformCapabilities;

  assertCapabilityStatus(platformCapabilities, 'android', 'parent-mobile-observer', 'scaffold');
  assertCapabilityStatus(platformCapabilities, 'android', 'parent-mobile-controller', 'manual-required');
  assertCapabilityStatus(platformCapabilities, 'android', 'notifications', 'manual-required');
  assertCapabilityStatus(platformCapabilities, 'android', 'package-lifecycle', 'manual-required');
  assertCapabilityStatus(platformCapabilities, 'ios', 'parent-mobile-observer', 'scaffold');
  assertCapabilityStatus(platformCapabilities, 'ios', 'parent-mobile-controller', 'manual-required');
  assertCapabilityStatus(platformCapabilities, 'ios', 'foreground-mobile-service', 'unavailable');
  assertCapabilityStatus(platformCapabilities, 'ios', 'signing-entitlements', 'manual-required');

  proofLabels.push('parent-mobile-capabilities.platform-boundaries');
  return {
    android: summarizeCapabilityStates(runtimeModels.androidObserver.platformCapabilities),
    ios: summarizeCapabilityStates(runtimeModels.iosObserver.platformCapabilities),
  };
}

async function assertProofMatrix() {
  const matrix = JSON.parse(await readRepoFile('docs/expectations/pre-ai-proof-matrix.json'));
  const claim = matrix.claims.find((candidate) => candidate.id === 'parent-mobile-shell-runtime-proof');
  const scenario = matrix.checkpointScenarios.find((candidate) => candidate.id === 'parent-mobile-shell-runtime-proof');
  if (!claim || !scenario) {
    throw new Error('Proof matrix is missing parent-mobile-shell-runtime-proof claim or checkpoint scenario.');
  }
  assertIncludes(claim.claim, 'Parent mobile shell runtime proof', 'parent mobile claim text');
  assertArrayIncludes(
    claim.ciProof.commands,
    'node scripts/test/parent-mobile-shell-runtime-proof.mjs',
    'claim command'
  );
  assertArrayIncludes(
    scenario.ciCommands,
    'node scripts/test/parent-mobile-shell-runtime-proof.mjs',
    'scenario command'
  );
  assertArrayIncludes(
    matrix.requiredCompletedClaimIds,
    'parent-mobile-shell-runtime-proof',
    'required completed claim'
  );
  proofLabels.push('proof-matrix.parent-mobile-shell-runtime-proof');
  return {
    claimId: claim.id,
    platformCoverage: claim.platformCoverage,
    runtimeSurfaceCoverage: claim.runtimeSurfaceCoverage,
  };
}

async function assertScriptWiring() {
  const packageJson = JSON.parse(await readRepoFile('package.json'));
  const parentDomainPackage = JSON.parse(await readRepoFile('packages/parent-domain/package.json'));
  if (
    packageJson.scripts['test:parent-mobile-shell-runtime'] !==
    'node scripts/test/parent-mobile-shell-runtime-proof.mjs'
  ) {
    throw new Error('Missing root test:parent-mobile-shell-runtime script.');
  }
  if (!parentDomainPackage.exports['./parent-mobile-runtime']) {
    throw new Error('Missing parent-domain parent-mobile-runtime export.');
  }
  proofLabels.push('package-scripts.parent-mobile-shell-runtime');
  return {
    rootScript: 'test:parent-mobile-shell-runtime',
    parentDomainExport: './parent-mobile-runtime',
  };
}

function assertRuntimeModel(readModel, platform) {
  if (readModel.platform !== platform) {
    throw new Error(`Expected ${platform} parent mobile runtime model.`);
  }
  if (readModel.localModelExecutionAllowed !== false) {
    throw new Error(`${platform} parent mobile model must not allow local model execution by default.`);
  }
  if (readModel.childAgentBehaviorClaim !== 'not-claimed') {
    throw new Error(`${platform} parent mobile model must not claim child-agent behavior.`);
  }
  if (readModel.serviceAvailability.cloudRelay !== 'not-implemented') {
    throw new Error(`${platform} parent mobile model must keep cloud relay not implemented.`);
  }
  assertRouteState(readModel, 'parent-cache', 'stale', `${platform} parent cache state`);
  assertRouteState(readModel, 'parent-owned-storage', 'offline', `${platform} parent-owned storage state`);
  if (platform === 'android' && readModel.controllerProof.commandAuthorityState !== 'observer-read-only') {
    throw new Error('Android parent mobile proof must remain observer read-only until real package proof exists.');
  }
  if (platform === 'ios' && readModel.controllerProof.commandAuthorityState !== 'controller-takeover-manual-required') {
    throw new Error('iOS parent mobile proof must keep controller takeover manual-required.');
  }
}

function summarizeRuntimeModel(readModel) {
  return {
    platform: readModel.platform,
    packageState: readModel.packageProof.packageState,
    controllerState: readModel.controllerProof.controllerState,
    takeoverRequestAllowed: readModel.controllerProof.takeoverRequestAllowed,
    commandAuthorityState: readModel.controllerProof.commandAuthorityState,
    localService: readModel.serviceAvailability.localService,
    lanService: readModel.serviceAvailability.lanService,
    cloudRelay: readModel.serviceAvailability.cloudRelay,
    parentCache: readModel.serviceAvailability.parentCache,
    parentOwnedStorage: readModel.serviceAvailability.parentOwnedStorage,
    routeStatuses: Object.fromEntries(
      readModel.serviceAvailability.routeStatuses.map((entry) => [entry.routeKind, entry.state])
    ),
    assistantJobRoute: readModel.assistantJobProof.route,
    assistantJobState: readModel.assistantJobProof.jobState,
    capabilityStates: summarizeCapabilityStates(readModel.platformCapabilities),
    localModelExecutionAllowed: readModel.localModelExecutionAllowed,
    childAgentBehaviorClaim: readModel.childAgentBehaviorClaim,
  };
}

function summarizeCapabilityStates(entries) {
  return Object.fromEntries(entries.map((entry) => [entry.capability, entry.status]));
}

function capabilityProof(capability, status, proofRequirement, claimBoundary) {
  return { capability, status, proofRequirement, claimBoundary };
}

function serviceAvailability(localService, lanService, selectedRouteId) {
  return {
    localService,
    lanService,
    cloudRelay: 'not-implemented',
    parentCache: 'stale',
    parentOwnedStorage: 'offline',
    selectedRouteId,
    routeStatuses: [
      routeStatus('local-service', localService, 'local-service', null),
      routeStatus('lan-service', lanService, 'lan-service', selectedRouteId),
      routeStatus('cloud-relay', 'not-implemented', 'unavailable', null),
      routeStatus('parent-cache', 'stale', 'parent-cache', null),
      routeStatus('parent-owned-storage', 'offline', 'parent-owned-storage', null),
    ],
  };
}

function routeStatus(routeKind, state, custody, selectedRouteId) {
  return {
    routeKind,
    state,
    custody,
    selectedRouteId,
    proofRequirement: `${routeKind} status must stay explicit in the parent mobile shell read model`,
  };
}

function assertRouteState(readModel, routeKind, expectedState, label) {
  const routeStatus = readModel.serviceAvailability.routeStatuses.find((entry) => entry.routeKind === routeKind);
  if (routeStatus?.state !== expectedState) {
    throw new Error(`${label}: expected ${expectedState}, received ${routeStatus?.state ?? 'missing'}`);
  }
}

async function readRepoFile(path) {
  return readFile(join(repoRoot, path), 'utf8');
}

async function runNpm(args) {
  if (process.platform === 'win32') {
    await runCommand('cmd', ['/c', 'npm', ...args]);
    return;
  }
  await runCommand('npm', args);
}

async function runCommand(command, args) {
  commands.push([command, ...args].join(' '));
  await new Promise((resolve, reject) => {
    const child = spawn(command, args, { cwd: repoRoot, stdio: 'inherit', windowsHide: true });
    child.once('exit', (code) => {
      if (code === 0) {
        resolve();
        return;
      }
      reject(new Error(`${command} ${args.join(' ')} exited with ${code}`));
    });
    child.once('error', reject);
  });
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

function assertIncludes(value, expected, label) {
  if (!value.includes(expected)) {
    throw new Error(`${label}: missing ${expected}`);
  }
}

function assertArrayIncludes(values, expected, label) {
  if (!Array.isArray(values) || !values.includes(expected)) {
    throw new Error(`${label}: missing ${expected}`);
  }
}

function assertCapabilityStatus(platformCapabilities, platform, capability, expectedStatus) {
  const platformEntry = platformCapabilities.find((entry) => entry.platform === platform);
  const capabilityEntry = platformEntry?.capabilities.find((entry) => entry.capability === capability);
  if (capabilityEntry?.status !== expectedStatus) {
    throw new Error(
      `${platform} ${capability}: expected ${expectedStatus}, got ${capabilityEntry?.status ?? 'missing'}`
    );
  }
}
