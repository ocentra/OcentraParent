import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';
import { pathToFileURL } from 'node:url';
import { spawn } from 'node:child_process';
import { tsImport } from 'tsx/esm/api';

const repoRoot = process.cwd();
const proofMode = 'child-ios-entitlement-capability-proof';
const outputDir = join(repoRoot, 'test-results', proofMode);
const proofPath = join(outputDir, 'proof.json');
const commands = [];
const proofLabels = [];

await main();

async function main() {
  await mkdir(outputDir, { recursive: true });

  await runNpm([
    'exec',
    '--workspace',
    '@ocentra-parent/child-runtime-domain',
    '--',
    'vitest',
    'run',
    'tests/unit/child-ios-entitlement-capability-proof.test.ts',
  ]);

  const sourceProof = await assertIosSourceProof();
  const runtimeReadModel = await parseRuntimeReadModel(buildRuntimeReadModel());
  const matrixProof = await assertProofMatrix();
  const scriptWiring = await assertScriptWiring();

  const proof = {
    schemaVersion: 1,
    checkedAt: new Date().toISOString(),
    commit: await gitHead(),
    proofMode,
    commands,
    proofLabels,
    evidence: {
      sourceProof,
      contract: 'packages/child-runtime-domain/src/child-ios-entitlement-capability-proof.ts',
      contractTest: 'packages/child-runtime-domain/tests/unit/child-ios-entitlement-capability-proof.test.ts',
      matrix: 'docs/expectations/pre-ai-proof-matrix.json',
      checkpoint: 'docs/checkpoints/child-ios-entitlement-capability-proof-2026-05-31.md',
      output: relativePath(proofPath),
    },
    runtimeReadModel,
    matrixProof,
    scriptWiring,
    childIosEntitlementPackageProved: {
      simulatorTarget: 'ci-mechanical-proof: iOS app target exists in the Xcode project',
      bundleIdentifier: 'ci-mechanical-proof: bundle identifier remains ca.ocentra.parent.agent',
      infoPlist: 'ci-mechanical-proof: basic iOS app plist exists without entitlement or background claims',
      statusSurface: 'simulator-scaffold: AgentStatusViewController exposes manual-required status labels',
      simulatorBuildScript: 'ci-mechanical-proof: simulator package script exists with code signing disabled',
    },
    childIosEntitlementStillManual: [
      'Family Controls entitlement approval and behavior',
      'DeviceActivity schedule and event behavior',
      'Screen Time API authorization and behavior',
      'Network Extension entitlement and filtering behavior',
      'notification authorization and delivery',
      'background execution mode and behavior',
      'Apple signing, provisioning, and entitlement files',
      'TestFlight install and App Store distribution',
      'physical-device install and runtime evidence',
    ],
    nonClaims: [
      'Family Controls, DeviceActivity, Screen Time, or Network Extension implementation',
      'notification permission grant or delivery',
      'background execution behavior',
      'Apple signing, provisioning, entitlement approval, TestFlight, or App Store proof',
      'simulator launch, physical-device install, or device behavior',
      'child-agent parity or external LAN/WebSocket iOS transport',
    ],
  };

  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  console.log(`child-ios-entitlement-capability-proof-ok:${proofLabels.join(',')}`);
  console.log(`evidence=${proofPath}`);
}

async function assertIosSourceProof() {
  const project = await readRepoFile('platforms/ios/OcentraParentAgent.xcodeproj/project.pbxproj');
  const plist = await readRepoFile('platforms/ios/OcentraParentAgent/Info.plist');
  const statusView = await readRepoFile('platforms/ios/OcentraParentAgent/AgentStatusViewController.swift');
  const buildScript = await readRepoFile('scripts/release/ios/build-simulator-app.sh');

  assertIncludes(project, 'OcentraParentAgent.app', 'iOS app product target');
  assertIncludes(project, 'PRODUCT_BUNDLE_IDENTIFIER = ca.ocentra.parent.agent', 'iOS bundle identifier');
  assertIncludes(plist, '<key>CFBundleIdentifier</key>', 'Info.plist bundle identifier key');
  assertIncludes(plist, '<key>LSRequiresIPhoneOS</key>', 'Info.plist iPhone requirement');
  assertNotIncludes(plist, '<key>UIBackgroundModes</key>', 'background modes entitlement claim');
  assertNotIncludes(plist, 'FamilyControls', 'Family Controls framework claim');
  assertNotIncludes(plist, 'DeviceActivity', 'DeviceActivity framework claim');
  assertNotIncludes(plist, 'NetworkExtension', 'Network Extension framework claim');
  assertIncludes(statusView, 'child-ios-entitlement-capability-proof', 'iOS status schema label');
  assertIncludes(statusView, 'family-controls=manual-required', 'Family Controls manual label');
  assertIncludes(statusView, 'device-activity=manual-required', 'DeviceActivity manual label');
  assertIncludes(statusView, 'screen-time=manual-required', 'Screen Time manual label');
  assertIncludes(statusView, 'network-extension=manual-required', 'Network Extension manual label');
  assertIncludes(statusView, 'notifications=manual-required', 'notifications manual label');
  assertIncludes(statusView, 'background-execution=manual-required', 'background execution manual label');
  assertIncludes(statusView, 'signing=manual-required', 'signing manual label');
  assertIncludes(statusView, 'testflight=manual-required', 'TestFlight manual label');
  assertIncludes(statusView, 'device-proof=manual-required', 'device proof manual label');
  assertIncludes(statusView, 'child-agent-parity=not-claimed', 'child-agent parity non-claim label');
  assertIncludes(buildScript, 'xcodebuild', 'iOS simulator build command');
  assertIncludes(buildScript, 'iphonesimulator', 'iOS simulator SDK');
  assertIncludes(buildScript, 'CODE_SIGNING_ALLOWED=NO', 'unsigned simulator package proof');
  proofLabels.push('ios-scaffold.entitlement-source-proof');

  return {
    project: 'platforms/ios/OcentraParentAgent.xcodeproj/project.pbxproj',
    infoPlist: 'platforms/ios/OcentraParentAgent/Info.plist',
    statusView: 'platforms/ios/OcentraParentAgent/AgentStatusViewController.swift',
    simulatorBuildScript: 'scripts/release/ios/build-simulator-app.sh',
  };
}

function buildRuntimeReadModel() {
  return {
    schemaVersion: proofMode,
    bundleId: 'ca.ocentra.parent.agent',
    statusSurfaceClass: 'AgentStatusViewController',
    protocolBridgeProof: {
      bundleId: 'ca.ocentra.parent.agent',
      statusSurfaceClass: 'AgentStatusViewController',
      bridgeState: 'simulator-scaffold',
      externalTransportState: 'not-implemented',
      commands: [
        'child.ios.entitlement.capability.snapshot.get',
        'child.ios.entitlement.package.proof.get',
        'child.ios.entitlement.manual-proof.get',
      ],
      events: [
        'child.ios.entitlement.capability.snapshot.reported',
        'child.ios.entitlement.package.proof.reported',
        'child.ios.entitlement.manual-proof.reported',
      ],
      runtimeOwner: 'ios-swift-scaffold',
      proofRequirement: 'iOS simulator scaffold status surface names manual entitlement states',
      claimBoundary: 'status surface is not external child-agent transport or Apple entitlement proof',
    },
    surfaceProofs: surfaceProofs(),
    packageLifecycleProofs: packageLifecycleProofs(),
    claimBoundaries: {
      simulatorPackage: 'Xcode project target, bundle id, plist, status view, and package script are source proof only',
      familyControls: 'Family Controls remains entitlement-required without Apple approval and device artifacts',
      deviceActivity: 'DeviceActivity remains entitlement-required without schedule and event artifacts',
      screenTime: 'Screen Time API remains entitlement-required without authorization and behavior artifacts',
      networkExtension: 'Network Extension remains entitlement-required without filtering artifacts',
      notifications: 'notification authorization and delivery remain manual-required',
      backgroundExecution: 'background execution remains manual-required without UIBackgroundModes and device proof',
      signingEntitlements: 'signing and entitlements remain signing-required; simulator script disables signing',
      testflight: 'TestFlight and App Store distribution remain device-proof-required or planned',
      deviceProof: 'physical-device install and runtime behavior remain device-proof-required',
      externalTransport: 'no external LAN or WebSocket iOS child-agent transport is claimed',
    },
    updatedAt: new Date().toISOString(),
  };
}

async function parseRuntimeReadModel(readModel) {
  const module = await importTsModule('packages/child-runtime-domain/src/child-ios-entitlement-capability-proof.ts');
  const parsed = module.ChildIosEntitlementCapabilityReadModelSchema.parse(readModel);
  proofLabels.push('child-runtime-domain.child-ios-entitlement-capability-proof-parse');
  return parsed;
}

async function assertProofMatrix() {
  const matrix = JSON.parse(await readRepoFile('docs/expectations/pre-ai-proof-matrix.json'));
  const claim = matrix.claims.find((candidate) => candidate.id === proofMode);
  const scenario = matrix.checkpointScenarios.find((candidate) => candidate.id === proofMode);
  if (!claim || !scenario) {
    throw new Error('Proof matrix is missing child-ios-entitlement-capability-proof claim or scenario.');
  }
  assertArrayIncludes(matrix.requiredCompletedClaimIds, proofMode, 'required completed claim');
  assertArrayIncludes(scenario.ciCommands, `node scripts/test/${proofMode}.mjs`, 'scenario command');
  assertArrayIncludes(claim.ciProof.commands, `node scripts/test/${proofMode}.mjs`, 'claim command');
  proofLabels.push('proof-matrix.child-ios-entitlement-capability-proof');
  return {
    claimId: claim.id,
    platformCoverage: claim.platformCoverage,
    runtimeSurfaceCoverage: claim.runtimeSurfaceCoverage,
  };
}

async function assertScriptWiring() {
  const packageJson = JSON.parse(await readRepoFile('package.json'));
  const childRuntimeDomainPackage = JSON.parse(await readRepoFile('packages/child-runtime-domain/package.json'));
  const script = packageJson.scripts['test:child-ios-entitlement-capability-proof'];
  if (script !== `node scripts/test/${proofMode}.mjs`) {
    throw new Error('Missing root test:child-ios-entitlement-capability-proof script.');
  }
  if (!childRuntimeDomainPackage.exports['./*']) {
    throw new Error('Missing child-runtime-domain wildcard export.');
  }
  proofLabels.push('package-scripts.child-ios-entitlement-capability-proof');
  return {
    rootScript: 'test:child-ios-entitlement-capability-proof',
    childRuntimeDomainExport: './*',
    sourceContract: 'packages/child-runtime-domain/src/child-ios-entitlement-capability-proof.ts',
  };
}

function surfaceProofs() {
  return [
    surfaceProof(
      'simulator-app-target',
      'package-lifecycle',
      'manual-required',
      'declared-in-project',
      'ci-mechanical-proof',
      'ios-xcode-project'
    ),
    surfaceProof(
      'bundle-identifier',
      'package-lifecycle',
      'manual-required',
      'declared-in-project',
      'ci-mechanical-proof',
      'ios-xcode-project'
    ),
    surfaceProof(
      'status-surface',
      'typed-protocol-bridge',
      'scaffold',
      'scaffold-status-label',
      'simulator-scaffold',
      'ios-swift-scaffold'
    ),
    surfaceProof(
      'family-controls-entitlement',
      'family-controls-entitlement',
      'manual-required',
      'not-declared',
      'entitlement-required',
      'apple-entitlement'
    ),
    surfaceProof(
      'device-activity-framework',
      'device-activity',
      'manual-required',
      'not-declared',
      'entitlement-required',
      'apple-device-framework'
    ),
    surfaceProof(
      'screen-time-api',
      'screen-time-api',
      'manual-required',
      'not-declared',
      'entitlement-required',
      'apple-device-framework'
    ),
    surfaceProof(
      'network-extension',
      'network-extension',
      'manual-required',
      'not-declared',
      'entitlement-required',
      'apple-network-extension'
    ),
    surfaceProof(
      'notifications-permission',
      'notifications',
      'manual-required',
      'not-declared',
      'manual-required',
      'apple-notification-permission'
    ),
    surfaceProof(
      'background-execution',
      'background-execution',
      'manual-required',
      'not-declared',
      'manual-required',
      'apple-background-mode'
    ),
    surfaceProof(
      'signing-entitlements',
      'signing-entitlements',
      'manual-required',
      'not-applicable',
      'signing-required',
      'apple-signing'
    ),
    surfaceProof(
      'testflight-distribution',
      'testflight-distribution',
      'manual-required',
      'not-applicable',
      'device-proof-required',
      'apple-testflight'
    ),
    surfaceProof(
      'physical-device-proof',
      'package-lifecycle',
      'manual-required',
      'not-applicable',
      'device-proof-required',
      'apple-device-proof'
    ),
    surfaceProof(
      'app-store-distribution',
      'store-distribution',
      'planned',
      'not-applicable',
      'planned',
      'app-store-connect'
    ),
  ];
}

function packageLifecycleProofs() {
  return [
    lifecycleProof('xcode-project-target', 'ci-mechanical-proof', 'ios-xcode-project'),
    lifecycleProof('bundle-identifier', 'ci-mechanical-proof', 'ios-xcode-project'),
    lifecycleProof('simulator-build-script', 'ci-mechanical-proof', 'ios-simulator-build-script'),
    lifecycleProof('status-view', 'simulator-scaffold', 'ios-swift-scaffold'),
    lifecycleProof('info-plist', 'ci-mechanical-proof', 'ios-info-plist'),
    lifecycleProof('simulator-build', 'manual-required', 'ios-simulator-build-script'),
    lifecycleProof('device-install', 'device-proof-required', 'apple-device-proof'),
    lifecycleProof('testflight-install', 'device-proof-required', 'apple-testflight'),
    lifecycleProof('signing-profile', 'signing-required', 'apple-signing'),
    lifecycleProof('entitlement-review', 'entitlement-required', 'apple-entitlement'),
  ];
}

function surfaceProof(surface, parentCapability, parentCapabilityStatus, declarationState, proofState, runtimeOwner) {
  const proofRequirement = `${surface} remains ${proofState} until Apple artifacts change it`;
  return {
    surface,
    parentCapability,
    parentCapabilityStatus,
    declarationState,
    proofState,
    runtimeOwner,
    proofRequirement,
    claimBoundary: proofRequirement,
  };
}

function lifecycleProof(phase, proofState, runtimeOwner) {
  return {
    phase,
    proofState,
    runtimeOwner,
    proofRequirement: `${phase} proof state is ${proofState}`,
    claimBoundary: `${phase} does not upgrade iOS entitlement or device behavior without Apple artifacts`,
  };
}

async function readRepoFile(path) {
  return readFile(join(repoRoot, path), 'utf8');
}

async function runNpm(args) {
  await runCommand(...npmCommand([...args]));
}

async function runCommand(commandName, args) {
  commands.push([commandName, ...args].join(' '));
  await new Promise((resolve, reject) => {
    const child = spawn(commandName, args, { cwd: repoRoot, stdio: 'inherit', windowsHide: true });
    child.once('exit', (code) =>
      code === 0 ? resolve() : reject(new Error(`${commandName} ${args.join(' ')} exited with ${code}`))
    );
    child.once('error', reject);
  });
}

async function importTsModule(relativePath) {
  return tsImport(pathToFileURL(join(repoRoot, relativePath)).href, import.meta.url);
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

function relativePath(path) {
  return relative(repoRoot, path).replaceAll('\\', '/');
}

function assertIncludes(value, expected, label) {
  if (!value.includes(expected)) {
    throw new Error(`${label}: missing ${expected}`);
  }
}

function assertNotIncludes(value, expected, label) {
  if (value.includes(expected)) {
    throw new Error(`${label}: unexpectedly contains ${expected}`);
  }
}

function assertArrayIncludes(values, expected, label) {
  if (!Array.isArray(values) || !values.includes(expected)) {
    throw new Error(`${label}: missing ${expected}`);
  }
}

function npmCommand(args) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return [command, commandArgs];
}
