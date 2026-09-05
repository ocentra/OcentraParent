import { spawn } from 'node:child_process';
import { readFile } from 'node:fs/promises';
import { join } from 'node:path';
import { pathToFileURL } from 'node:url';
import {
  CHILD_IOS_CANONICAL_PROOF_ROOT,
  CHILD_IOS_LEGACY_PROOF_PATH,
  classifyIosXctestResult,
  writeChildIosProofOutputs,
} from './child-ios-entitlement-capability-proof-artifacts.mjs';

const repoRoot = process.cwd();
const proofMode = 'child-ios-entitlement-capability-proof';
const commands = [];
const commandResults = [];
const proofLabels = [];

await main();

async function main() {
  commandResults.push(
    await runCommand('cargo', [
      'test',
      '-p',
      'ocentra-schema',
      '--test',
      'contract',
      'child_ios_entitlement_capability',
    ])
  );

  const contracts = await importGeneratedModule(
    'packages/schema-domain/dist/generated-child-ios-entitlement-capability-proof-contracts.js'
  );
  const runtimeReadModel = assertGeneratedContract(contracts);
  const sourceProof = await assertIosSourceProof(runtimeReadModel);
  const matrixProof = await assertProofMatrix();
  const scriptWiring = await assertScriptWiring();
  const xctestOutcome = await runChildIosXctest();
  commandResults.push({
    command: xctestOutcome.command,
    exitCode: xctestOutcome.exitCode,
    result: xctestOutcome.status === 'passed' ? 'pass' : xctestOutcome.status === 'skipped' ? 'blocked' : 'fail',
    artifact: CHILD_IOS_LEGACY_PROOF_PATH,
    notes: xctestOutcome.reason,
  });

  const proof = {
    schemaVersion: 1,
    checkedAt: new Date().toISOString(),
    commit: await gitHead(),
    proofMode,
    commands,
    commandResults,
    proofLabels,
    evidence: {
      sourceProof,
      contract: 'crates/schema/src/child_ios_entitlement_capability_proof.rs',
      contractGenerator: 'crates/schema/src/child_ios_entitlement_capability_proof_ts.rs',
      generatedContract: 'packages/schema-domain/src/generated-child-ios-entitlement-capability-proof-contracts.ts',
      contractTest: 'crates/schema/tests/contract/child_ios_entitlement_capability_proof.rs',
      matrix: 'docs/expectations/pre-ai-proof-matrix.json',
      output: CHILD_IOS_LEGACY_PROOF_PATH,
      canonicalProofRoot: CHILD_IOS_CANONICAL_PROOF_ROOT,
    },
    runtimeReadModel,
    matrixProof,
    scriptWiring,
    xctestOutcome,
    nonClaims: [
      'simulator or physical-device launch proof from this CI source-contract lane',
      'Family Controls, DeviceActivity, Screen Time, or Network Extension implementation',
      'notification permission grant, background execution, or recovery behavior',
      'Apple signing, provisioning, entitlement approval, TestFlight, or App Store proof',
      'child-agent parity or external iOS transport',
    ],
  };

  await writeChildIosProofOutputs(repoRoot, proof);
  if (xctestOutcome.status === 'failed') {
    throw new Error(`child iOS XCTest failed: ${xctestOutcome.reason}`);
  }
}

function assertGeneratedContract(module) {
  const readModel = structuredClone(module.GeneratedChildIosEntitlementCapabilityReadModel);
  assertEqual(module.ChildIosEntitlementCapabilityContractRuntime.SchemaVersion, proofMode, 'generated schema version');
  assertEqual(readModel.schemaVersion, proofMode, 'read model schema version');
  assertEqual(readModel.bundleId, 'ca.ocentra.child.agent', 'read model bundle identifier');
  assertEqual(readModel.surfaceProofs.length, 15, 'surface proof count');
  assertEqual(readModel.packageLifecycleProofs.length, 13, 'package lifecycle proof count');
  assertEqual(readModel.protocolBridgeProof.externalTransportState, 'not-implemented', 'external transport state');
  assertEqual(
    readModel.surfaceProofs[3].parentCapabilityStatus,
    'manual-required',
    'Family Controls capability status'
  );
  assertEqual(readModel.surfaceProofs[10].proofState, 'device-proof-required', 'supervision proof state');
  assertEqual(readModel.packageLifecycleProofs[12].proofState, 'not-implemented', 'recovery proof state');
  assertIncludes(
    readModel.protocolBridgeProof.commands,
    'child.ios.entitlement.capability.snapshot.get',
    'snapshot command'
  );
  assertEqual(
    readModel.claimBoundaries.capabilityOnlyState,
    'iOS child runtime remains capability-only; no hidden daemon or persistent background service is claimed',
    'capability-only claim boundary'
  );
  proofLabels.push('rust-generated.child-ios-entitlement-capability-contract');
  return readModel;
}

async function assertIosSourceProof(readModel) {
  const project = await readRepoFile('platforms/ios/OcentraChildAgent.xcodeproj/project.pbxproj');
  const plist = await readRepoFile('platforms/ios/OcentraChildAgent/Info.plist');
  const statusView = await readRepoFile('platforms/ios/OcentraChildAgent/AgentStatusViewController.swift');
  const buildScript = await readRepoFile('scripts/release/ios/build-simulator-app.sh');

  assertTextIncludes(project, 'OcentraChildAgent.app', 'iOS app product target');
  assertTextIncludes(project, `PRODUCT_BUNDLE_IDENTIFIER = ${readModel.bundleId}`, 'bundle identifier');
  assertTextIncludes(plist, '<key>CFBundleIdentifier</key>', 'Info.plist bundle identifier');
  assertTextIncludes(plist, '<key>LSRequiresIPhoneOS</key>', 'Info.plist iPhone requirement');
  for (const forbidden of ['<key>UIBackgroundModes</key>', 'FamilyControls', 'DeviceActivity', 'NetworkExtension']) {
    assertTextExcludes(plist, forbidden, `unproved plist capability ${forbidden}`);
  }
  for (const expected of [
    proofMode,
    'service-mode=capability-only',
    'launch-availability=manual-required',
    'recovery=not-implemented',
    'daemon=not-claimed',
    'child-agent-parity=not-claimed',
    'external-transport=not-implemented',
  ]) {
    assertTextIncludes(statusView, expected, `iOS status label ${expected}`);
  }
  for (const renderedObservation of [
    'bundleIdentity.rendered',
    'familyControls.rendered',
    'deviceActivity.rendered',
    'screenTime.rendered',
    'networkExtension.rendered',
    'notifications.rendered',
    'backgroundExecution.rendered',
    'provisioning.rendered',
    'supervision.rendered',
    'signing.rendered',
    'testFlight.rendered',
    'deviceProof.rendered',
  ]) {
    assertTextIncludes(statusView, renderedObservation, `iOS runtime observation ${renderedObservation}`);
  }
  for (const expected of ['xcodebuild', 'iphonesimulator', 'CODE_SIGNING_ALLOWED=NO']) {
    assertTextIncludes(buildScript, expected, `simulator build token ${expected}`);
  }
  proofLabels.push('ios-scaffold.entitlement-source-proof');
  return {
    project: 'platforms/ios/OcentraChildAgent.xcodeproj/project.pbxproj',
    infoPlist: 'platforms/ios/OcentraChildAgent/Info.plist',
    statusView: 'platforms/ios/OcentraChildAgent/AgentStatusViewController.swift',
    simulatorBuildScript: 'scripts/release/ios/build-simulator-app.sh',
  };
}

async function assertProofMatrix() {
  const matrix = JSON.parse(await readRepoFile('docs/expectations/pre-ai-proof-matrix.json'));
  const claim = matrix.claims.find((candidate) => candidate.id === proofMode);
  const scenario = matrix.checkpointScenarios.find((candidate) => candidate.id === proofMode);
  if (!claim || !scenario) throw new Error(`Proof matrix is missing ${proofMode}.`);
  const command = `node scripts/test/${proofMode}.mjs`;
  assertIncludes(matrix.requiredCompletedClaimIds, proofMode, 'required completed claim');
  assertIncludes(scenario.ciCommands, command, 'scenario command');
  assertIncludes(claim.ciProof.commands, command, 'claim command');
  proofLabels.push('proof-matrix.child-ios-entitlement-capability-proof');
  return { claimId: claim.id, platformCoverage: claim.platformCoverage };
}

async function assertScriptWiring() {
  const packageJson = JSON.parse(await readRepoFile('package.json'));
  const expected =
    'node scripts/enforcer/run-ocentra-enforcer.mjs proof run --proof ocentra-parent.child-ios-entitlement-capability-proof';
  assertEqual(
    packageJson.scripts['test:child-ios-entitlement-capability-proof'],
    expected,
    'root Enforcer proof command'
  );
  proofLabels.push('package-scripts.child-ios-entitlement-capability-proof');
  return { rootScript: 'test:child-ios-entitlement-capability-proof' };
}

async function runCommand(command, args) {
  const commandLine = [command, ...args].join(' ');
  commands.push(commandLine);
  return new Promise((resolveCommand, rejectCommand) => {
    const child = spawn(command, args, { cwd: repoRoot, stdio: 'inherit', windowsHide: true });
    child.once('exit', (code) => {
      if (code !== 0) {
        rejectCommand(new Error(`${commandLine} exited with ${code}`));
        return;
      }
      resolveCommand({
        command: commandLine,
        exitCode: 0,
        result: 'pass',
        artifact: 'n/a',
        notes: 'Rust child iOS capability contract test passed.',
      });
    });
    child.once('error', rejectCommand);
  });
}

async function runChildIosXctest() {
  const args = ['--test', 'platforms/ios/tests/child_capability_identity.test.mjs'];
  const command = [process.execPath, ...args].join(' ');
  commands.push(command);
  const result = await new Promise((resolveCommand, rejectCommand) => {
    const output = [];
    const child = spawn(process.execPath, args, {
      cwd: repoRoot,
      stdio: ['ignore', 'pipe', 'pipe'],
      windowsHide: true,
    });
    child.stdout.on('data', (chunk) => output.push(String(chunk)));
    child.stderr.on('data', (chunk) => output.push(String(chunk)));
    child.once('error', rejectCommand);
    child.once('exit', (code) => resolveCommand({ command, exitCode: code ?? 1, output: output.join('') }));
  });
  return classifyIosXctestResult({ ...result, platform: process.platform });
}

async function gitHead() {
  const chunks = [];
  await new Promise((resolve, reject) => {
    const child = spawn('git', ['rev-parse', 'HEAD'], {
      cwd: repoRoot,
      stdio: ['ignore', 'pipe', 'pipe'],
      windowsHide: true,
    });
    child.stdout.on('data', (chunk) => chunks.push(String(chunk)));
    child.once('exit', (code) => (code === 0 ? resolve() : reject(new Error('git rev-parse HEAD failed'))));
    child.once('error', reject);
  });
  return chunks.join('').trim();
}

function importGeneratedModule(path) {
  return import(pathToFileURL(join(repoRoot, path)).href);
}

function readRepoFile(path) {
  return readFile(join(repoRoot, path), 'utf8');
}

function assertEqual(actual, expected, label) {
  if (actual !== expected) throw new Error(`${label}: expected ${expected}, received ${actual}`);
}

function assertIncludes(values, expected, label) {
  if (!Array.isArray(values) || !values.includes(expected)) {
    throw new Error(`${label}: missing ${expected}`);
  }
}

function assertTextIncludes(value, expected, label) {
  if (!value.includes(expected)) throw new Error(`${label}: missing ${expected}`);
}

function assertTextExcludes(value, expected, label) {
  if (value.includes(expected)) throw new Error(`${label}: unexpectedly contains ${expected}`);
}
