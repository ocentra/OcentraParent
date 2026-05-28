import { spawn } from 'node:child_process';
import { existsSync } from 'node:fs';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const repoRoot = process.cwd();
const outputDir = join(repoRoot, 'test-results', 'enforcement-lan-mobile-product-proof');
const proofPath = join(outputDir, 'proof.json');
const commands = [];
const proofLabels = [];

await main();

async function main() {
  await mkdir(outputDir, { recursive: true });

  await runCommand('cmd', ['/c', 'npm', 'run', 'build:contracts']);
  await runCommand('cmd', [
    '/c',
    'npm',
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/parent-domain',
    '--',
    'tests/capabilities.test.ts',
  ]);
  await runCommand('cmd', ['/c', 'node', 'scripts/test/platform-os-lan-mobile-proof.mjs']);

  const aggregateProof = await readJson(join(repoRoot, 'test-results', 'platform-os-lan-mobile-proof', 'proof.json'));
  const productionProof = await readJson(
    join(repoRoot, 'test-results', 'platform-lan-enforcement-production-proof', 'proof.json')
  );
  const lanProof = await readJson(
    join(repoRoot, 'test-results', 'v0-9-production-lan-multidevice-hardening', 'proof.json')
  );
  const householdLanReadiness = await readJson(
    join(repoRoot, 'test-results', 'v0-9-household-lan-proof-readiness', 'proof.json')
  );
  const capabilities = await platformCapabilities();
  const matrix = await readJson(join(repoRoot, 'docs', 'expectations', 'pre-ai-proof-matrix.json'));

  assertOsEnforcement(aggregateProof, capabilities);
  assertProductionLan(productionProof, lanProof, householdLanReadiness);
  assertMobileProductStates(capabilities);
  assertProofMatrix(matrix);

  const proof = {
    schemaVersion: 1,
    checkedAt: new Date().toISOString(),
    commit: await gitHead(),
    commands,
    proofLabels,
    evidence: {
      aggregateProof: relative(repoRoot, join(repoRoot, 'test-results', 'platform-os-lan-mobile-proof', 'proof.json')),
      productionProof: relative(
        repoRoot,
        join(repoRoot, 'test-results', 'platform-lan-enforcement-production-proof', 'proof.json')
      ),
      lanProof: relative(
        repoRoot,
        join(repoRoot, 'test-results', 'v0-9-production-lan-multidevice-hardening', 'proof.json')
      ),
      householdLanReadiness: relative(
        repoRoot,
        join(repoRoot, 'test-results', 'v0-9-household-lan-proof-readiness', 'proof.json')
      ),
    },
    productProof: {
      osEnforcement: capabilitySummary(capabilities, 'windows', [
        'owned-process-terminate',
        'app-time-limit',
        'app-blocking',
        'network-domain-blocking',
        'managed-browser-control',
        'unmanaged-browser-detection',
      ]),
      managedBrowserIntervention: aggregateProof.osEnforcementProof.managedBrowserIntervention,
      productionLan: {
        localClaims: lanProof.claimsProvedLocally,
        notLocalClaims: lanProof.claimsNotProvedLocally,
        readinessGate: householdLanReadiness.readinessGate,
        observedLocalServiceStates: householdLanReadiness.observedLocalServiceStates,
        cloudRelayDecision: productionProof.productionTruth.cloudRelayDecision,
      },
      androidChild: capabilitySummary(capabilities, 'android', [
        'foreground-mobile-service',
        'local-storage',
        'typed-protocol-bridge',
        'usage-stats',
        'accessibility-service',
        'vpn-dns-filtering',
        'device-owner-policy',
        'managed-profile',
        'package-lifecycle',
      ]),
      iosChild: capabilitySummary(capabilities, 'ios', [
        'family-controls-entitlement',
        'device-activity',
        'screen-time-api',
        'network-extension',
        'notifications',
        'background-execution',
        'signing-entitlements',
        'testflight-distribution',
      ]),
    },
  };

  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  console.log(`enforcement-lan-mobile-product-proof-ok:${proofLabels.join(',')}`);
  console.log(`evidence=${proofPath}`);
}

function assertOsEnforcement(aggregateProof, capabilities) {
  assertCapability(capabilities, 'windows', 'owned-process-terminate', 'implemented');
  assertCapability(capabilities, 'windows', 'app-time-limit', 'implemented');
  assertCapability(capabilities, 'windows', 'app-blocking', 'manual-required');
  assertCapability(capabilities, 'windows', 'network-domain-blocking', 'manual-required');
  assertCapability(capabilities, 'windows', 'managed-browser-control', 'implemented');
  assertCapability(capabilities, 'windows', 'unmanaged-browser-detection', 'implemented');
  assertEqual(
    aggregateProof.osEnforcementProof.processTerminate?.adapterKind,
    'process-control',
    'owned process terminate adapter'
  );
  assertEqual(
    aggregateProof.osEnforcementProof.broadAppDomainBrowser.includes('manual-required'),
    true,
    'broad app/domain/browser non-claim'
  );
  assertOneOf(
    aggregateProof.osEnforcementProof.managedBrowserIntervention.proofState,
    ['actually-enforced', 'manual-required'],
    'managed browser intervention state'
  );
  proofLabels.push('v0.8.os-enforcement.product-capability-states');
}

function assertProductionLan(productionProof, lanProof, householdLanReadiness) {
  assertEqual(
    productionProof.productionTruth.cloudRelayDecision.includes('no cloud relay behavior is implemented'),
    true,
    'cloud relay non-claim'
  );
  assertArrayIncludes(
    lanProof.claimsProvedLocally,
    'active controller write authority rejects observer writes, stale intents, replay, wrong device, and denied takeover',
    'controller conflict and rejection claim'
  );
  assertArrayIncludes(
    lanProof.claimsNotProvedLocally,
    'real household router discovery across two physical devices',
    'physical household discovery boundary'
  );
  const checklist = lanProof.manualTwoDeviceChecklist?.[0];
  assertArrayIncludes(
    checklist?.requiredArtifacts,
    'parent and child host names or IPs showing two distinct LAN devices',
    'two-device artifact'
  );
  assertEqual(
    householdLanReadiness.productReadinessDecision,
    'not-ready-for-product-ready-household-lan-claim',
    'household LAN product readiness decision'
  );
  assertEqual(
    householdLanReadiness.readinessGate.physicalHouseholdLan.state,
    'manual-required',
    'household physical LAN gate'
  );
  assertEqual(householdLanReadiness.readinessGate.cloudRelay.state, 'not-implemented', 'cloud relay LAN gate');
  proofLabels.push('v0.9.production-lan.household-manual-proof-boundary');
  proofLabels.push('v0.9.production-lan.household-readiness-gate');
}

function assertMobileProductStates(capabilities) {
  assertCapability(capabilities, 'android', 'parent-mobile-observer', 'scaffold');
  assertCapability(capabilities, 'android', 'parent-mobile-controller', 'manual-required');
  assertCapability(capabilities, 'android', 'package-lifecycle', 'manual-required');
  assertCapability(capabilities, 'ios', 'parent-mobile-observer', 'scaffold');
  assertCapability(capabilities, 'ios', 'parent-mobile-controller', 'manual-required');
  assertCapability(capabilities, 'ios', 'signing-entitlements', 'manual-required');
  assertCapability(capabilities, 'ios', 'testflight-distribution', 'manual-required');
  proofLabels.push('mobile-platform.package-signing-capability-states');
}

function assertProofMatrix(matrix) {
  const command = 'node scripts/test/enforcement-lan-mobile-product-proof.mjs';
  if (!matrix.requiredCompletedClaimIds.includes('enforcement-lan-mobile-product-proof')) {
    throw new Error('Proof matrix required claims are missing enforcement-lan-mobile-product-proof.');
  }
  const claim = matrix.claims.find((candidate) => candidate.id === 'enforcement-lan-mobile-product-proof');
  if (!claim) {
    throw new Error('Proof matrix is missing enforcement-lan-mobile-product-proof claim.');
  }
  assertEqual(claim.platformCoverage.windows, 'real-local-windows-proof', 'product proof Windows coverage');
  assertEqual(claim.runtimeSurfaceCoverage.cloudRelay.state, 'not-implemented', 'cloud relay product coverage');
  assertArrayIncludes(claim.ciProof.commands, command, 'product proof claim command');
  const scenario = matrix.checkpointScenarios.find(
    (candidate) => candidate.id === 'enforcement-lan-mobile-product-proof'
  );
  if (!scenario) {
    throw new Error('Proof matrix is missing enforcement-lan-mobile-product-proof scenario.');
  }
  assertArrayIncludes(scenario.ciCommands, command, 'product proof matrix command');
  proofLabels.push('proof-matrix.enforcement-lan-mobile-product-proof');
}

function capabilitySummary(capabilities, platform, names) {
  const entry = capabilities.find((candidate) => candidate.platform === platform);
  return names.map((name) => {
    const capability = entry?.capabilities.find((candidate) => candidate.capability === name);
    if (!capability) {
      throw new Error(`${platform}.${name}: missing capability`);
    }
    return {
      platform,
      capability: capability.capability,
      status: capability.status,
      note: capability.note,
    };
  });
}

function assertCapability(capabilities, platform, capabilityName, expectedStatus) {
  const entry = capabilities.find((candidate) => candidate.platform === platform);
  const capability = entry?.capabilities.find((candidate) => candidate.capability === capabilityName);
  assertEqual(capability?.status, expectedStatus, `${platform}.${capabilityName}`);
}

async function platformCapabilities() {
  const modulePath = join(repoRoot, 'packages', 'parent-domain', 'dist', 'capabilities.js');
  if (!existsSync(modulePath)) {
    throw new Error(`Missing built capabilities module: ${modulePath}`);
  }
  const module = await import(`file:///${modulePath.replaceAll('\\', '/')}`);
  return module.ParentControlPlatformCapabilities;
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

async function readJson(path) {
  if (!existsSync(path)) {
    throw new Error(`Missing proof artifact: ${path}`);
  }
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

function assertArrayIncludes(values, expected, label) {
  if (!Array.isArray(values) || !values.includes(expected)) {
    throw new Error(`${label}: missing ${expected}`);
  }
}

function assertEqual(actual, expected, label) {
  if (actual !== expected) {
    throw new Error(`${label}: expected ${expected}, received ${actual}`);
  }
}

function assertOneOf(actual, expectedValues, label) {
  if (!expectedValues.includes(actual)) {
    throw new Error(`${label}: expected one of ${expectedValues.join(', ')}, received ${actual}`);
  }
}
