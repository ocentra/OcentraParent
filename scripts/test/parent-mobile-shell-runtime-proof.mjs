import { spawn } from 'node:child_process';
import { existsSync } from 'node:fs';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const repoRoot = process.cwd();
const outputDir = join(repoRoot, 'test-results', 'parent-mobile-shell-runtime-proof');
const proofPath = join(outputDir, 'proof.json');
const commands = [];
const proofLabels = [];

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
    serviceAvailability: {
      localService: 'manual-required',
      lanService: 'degraded',
      cloudRelay: 'not-implemented',
      selectedRouteId: 'route-parent-mobile-lan-provider',
    },
    controllerProof: {
      controllerState: 'observer',
      controllerLeaseId: null,
      takeoverRequestAllowed: false,
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
    serviceAvailability: {
      localService: 'manual-required',
      lanService: 'manual-required',
      cloudRelay: 'not-implemented',
      selectedRouteId: null,
    },
    controllerProof: {
      controllerState: 'manual-required',
      controllerLeaseId: null,
      takeoverRequestAllowed: true,
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
}

function summarizeRuntimeModel(readModel) {
  return {
    platform: readModel.platform,
    packageState: readModel.packageProof.packageState,
    controllerState: readModel.controllerProof.controllerState,
    takeoverRequestAllowed: readModel.controllerProof.takeoverRequestAllowed,
    localService: readModel.serviceAvailability.localService,
    lanService: readModel.serviceAvailability.lanService,
    cloudRelay: readModel.serviceAvailability.cloudRelay,
    assistantJobRoute: readModel.assistantJobProof.route,
    assistantJobState: readModel.assistantJobProof.jobState,
    localModelExecutionAllowed: readModel.localModelExecutionAllowed,
    childAgentBehaviorClaim: readModel.childAgentBehaviorClaim,
  };
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
