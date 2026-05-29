import { spawn } from 'node:child_process';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const repoRoot = process.cwd();
const outputDir = join(repoRoot, 'test-results', 'v0-9-production-lan-mobile-controller-proof');
const proofPath = join(outputDir, 'proof.json');
const commands = [];
const proofLabels = [];

await main();

async function main() {
  await mkdir(outputDir, { recursive: true });

  await runCommand('cmd', ['/c', 'npm', 'run', 'build:contracts']);
  await runCommand('cargo', ['build', '-p', 'ocentra-parent-agent-service']);
  await runCommand('cmd', ['/c', 'node', 'scripts/test/v0-9-household-lan-proof-readiness.mjs']);
  await runCommand('cmd', ['/c', 'node', 'scripts/test/parent-mobile-shell-runtime-proof.mjs']);

  const productionLanProof = await readJson(
    join(repoRoot, 'test-results', 'v0-9-production-lan-multidevice-hardening', 'proof.json')
  );
  const readinessProof = await readJson(
    join(repoRoot, 'test-results', 'v0-9-household-lan-proof-readiness', 'proof.json')
  );
  const parentMobileProof = await readJson(
    join(repoRoot, 'test-results', 'parent-mobile-shell-runtime-proof', 'proof.json')
  );

  assertProductionLanProof(productionLanProof);
  assertHouseholdReadiness(readinessProof);
  assertParentMobileProof(parentMobileProof);

  const proof = {
    schemaVersion: 1,
    checkedAt: new Date().toISOString(),
    commit: await gitHead(),
    proofMode: 'v0-9-production-lan-mobile-controller-proof',
    commands,
    proofLabels,
    evidence: {
      productionLanMultidevice: relative(
        repoRoot,
        join(repoRoot, 'test-results', 'v0-9-production-lan-multidevice-hardening', 'proof.json')
      ),
      householdLanReadiness: relative(
        repoRoot,
        join(repoRoot, 'test-results', 'v0-9-household-lan-proof-readiness', 'proof.json')
      ),
      parentMobileRuntime: relative(
        repoRoot,
        join(repoRoot, 'test-results', 'parent-mobile-shell-runtime-proof', 'proof.json')
      ),
    },
    localServiceProof: {
      twoServiceBoundary: productionLanProof.localTwoServiceProof,
      discovery: productionLanProof.discoveryProof,
      controllerAuthority: productionLanProof.controllerAuthorityProof,
      parentMobileControllerObserver: productionLanProof.parentMobileControllerObserverProof,
    },
    mobileProof: {
      androidObserver: parentMobileProof.runtimeProof.androidObserver,
      iosObserver: parentMobileProof.runtimeProof.iosObserver,
      packageLaunchProof: parentMobileProof.packageLaunchProof,
      knownGaps: parentMobileProof.knownGaps,
    },
    manualProofGates: {
      physicalHouseholdLan: readinessProof.readinessGate.physicalHouseholdLan,
      parentMobileControllerObserver: readinessProof.readinessGate.parentMobileControllerObserver,
      cloudRelay: readinessProof.readinessGate.cloudRelay,
    },
    claimsProved: [
      'local two-service Rust proof covers discovery, selected route recovery, controller lease, observer read-only, wrong-device, replay, stale, and revocation-before-control states',
      'parent mobile backend proof keeps Android observer read-only and iOS controller takeover manual-required without claiming mobile UX parity',
      'cloud relay remains not implemented and is not counted as LAN proof',
    ],
    claimsNotProved: [
      'two physical household devices through a router/firewall path',
      'parent mobile controller write authority from a real Android or iOS package',
      'mobile background LAN behavior, signing, stores, device-owner policy, or iOS Family Controls',
      'cloud relay routing, storage, or authentication',
    ],
  };

  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  console.log(`v0-9-production-lan-mobile-controller-proof-ok:${proofLabels.join(',')}`);
  console.log(`evidence=${proofPath}`);
}

function assertProductionLanProof(proof) {
  assertEqual(proof.localTwoServiceProof.serviceCount, 2, 'local two-service proof count');
  assertEqual(
    proof.localTwoServiceProof.wrongDeviceRejected,
    'wrong-agent-port-rejected-as-wrong-device',
    'wrong device rejection'
  );
  assertArrayLengthAtLeast(proof.controllerAuthorityProof.observerReadOnlyRejected, 2, 'observer read-only proof');
  assertArrayLengthAtLeast(proof.controllerAuthorityProof.leaseLifecycle, 6, 'lease lifecycle proof');
  assertArrayLengthAtLeast(proof.controllerAuthorityProof.dishonestStateRejections, 8, 'dishonest state rejections');
  assertEqual(
    proof.controllerAuthorityProof.revocationBeforeControl.routeRevokedAssertion,
    'first-child-agent:route-revoked',
    'revocation assertion'
  );
  assertEqual(
    proof.controllerAuthorityProof.revocationBeforeControl.controlRejectedAssertion,
    'first-child-agent:revoked-control-rejected',
    'revoked control rejection'
  );
  assertEqual(
    proof.parentMobileControllerObserverProof.mobileWriteAuthorityState,
    'manual-required-real-mobile-package-proof',
    'parent mobile write authority state'
  );
  assertEqual(proof.cloudRelayDecision.state, 'not-implemented', 'cloud relay non-implementation');
  proofLabels.push('v0.9.production-lan.controller-authority-structured-proof');
}

function assertHouseholdReadiness(proof) {
  assertEqual(proof.productReadinessDecision, 'not-ready-for-product-ready-household-lan-claim', 'readiness decision');
  assertEqual(proof.readinessGate.physicalHouseholdLan.state, 'manual-required', 'physical household LAN gate');
  assertEqual(
    proof.readinessGate.parentMobileControllerObserver.state,
    'manual-required',
    'parent mobile controller observer gate'
  );
  assertEqual(proof.readinessGate.cloudRelay.state, 'not-implemented', 'cloud relay readiness gate');
  proofLabels.push('v0.9.production-lan.manual-gates-preserved');
}

function assertParentMobileProof(proof) {
  assertEqual(
    proof.runtimeProof.androidObserver.commandAuthorityState,
    'observer-read-only',
    'Android observer command authority'
  );
  assertEqual(
    proof.runtimeProof.iosObserver.commandAuthorityState,
    'controller-takeover-manual-required',
    'iOS controller command authority'
  );
  assertEqual(proof.runtimeProof.androidObserver.cloudRelay, 'not-implemented', 'Android cloud relay state');
  assertEqual(proof.runtimeProof.iosObserver.cloudRelay, 'not-implemented', 'iOS cloud relay state');
  assertEqual(proof.runtimeProof.localModelExecutionDefault, 'disabled-by-default', 'parent mobile local model state');
  assertEqual(proof.runtimeProof.childAgentBehaviorClaim, 'not-claimed', 'parent mobile child-agent non-claim');
  proofLabels.push('parent-mobile.controller-observer-boundaries');
}

async function runCommand(commandName, args) {
  commands.push([commandName, ...args].join(' '));
  await new Promise((resolve, reject) => {
    const child = spawn(commandName, args, { cwd: repoRoot, stdio: 'inherit', windowsHide: true });
    child.once('exit', (code) => {
      if (code === 0) {
        resolve();
        return;
      }
      reject(new Error(`${commandName} ${args.join(' ')} exited with ${code}`));
    });
    child.once('error', reject);
  });
}

async function readJson(path) {
  return JSON.parse(await readFile(path, 'utf8'));
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

function assertArrayLengthAtLeast(value, minimum, label) {
  if (!Array.isArray(value) || value.length < minimum) {
    throw new Error(`${label}: expected at least ${minimum} entries, received ${value?.length ?? 'non-array'}`);
  }
}

function assertEqual(actual, expected, label) {
  if (actual !== expected) {
    throw new Error(`${label}: expected ${expected}, received ${actual}`);
  }
}
