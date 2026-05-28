import { spawn } from 'node:child_process';
import { existsSync } from 'node:fs';
import { mkdir, readdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const repoRoot = process.cwd();
const outputDir = join(repoRoot, 'test-results', 'platform-os-lan-mobile-proof');
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
  await runCommand('cmd', ['/c', 'node', 'scripts/test/platform-lan-enforcement-production-proof.mjs']);

  const productionProof = await readJson(
    join(repoRoot, 'test-results', 'platform-lan-enforcement-production-proof', 'proof.json')
  );
  const v08Production = await latestJson(join(repoRoot, 'test-results', 'v0-8-production-enforcement-hardening'));
  const v09Production = await readJson(
    join(repoRoot, 'test-results', 'v0-9-production-lan-multidevice-hardening', 'proof.json')
  );
  const matrix = await readJson(join(repoRoot, 'docs', 'expectations', 'pre-ai-proof-matrix.json'));
  const capabilities = await platformCapabilities();

  assertProductionProof(productionProof);
  assertProcessTerminateProof(v08Production.data);
  assertHouseholdLanProof(v09Production);
  assertMobileCapabilities(capabilities);
  assertProofMatrix(matrix);

  const proof = {
    schemaVersion: 1,
    checkedAt: new Date().toISOString(),
    commit: await gitHead(),
    commands,
    proofLabels,
    evidence: {
      productionProof: relative(
        repoRoot,
        join(repoRoot, 'test-results', 'platform-lan-enforcement-production-proof', 'proof.json')
      ),
      v08ProductionEnforcement: relative(repoRoot, v08Production.path),
      v09ProductionLanMultidevice: relative(
        repoRoot,
        join(repoRoot, 'test-results', 'v0-9-production-lan-multidevice-hardening', 'proof.json')
      ),
    },
    osEnforcementProof: {
      processTerminate: v08Production.data.assertions.find(
        (assertion) => assertion.id === 'process-terminate-owned-process'
      ),
      appTimeLimit: productionProof.productionTruth.v08RealAdapterState,
      broadAppDomainBrowser:
        'manual-required or unavailable except where a real owned process or managed-browser proof harness can run on the host',
      networkDomainBlocking:
        'not implemented as a silent OS block; requires OS-approved adapter and manual host evidence before claim upgrade',
      rollbackRestartAudit: productionProof.manualProofRequirements.windowsEnforcement,
    },
    productionLanProof: {
      localMultiServiceClaims: v09Production.claimsProvedLocally,
      householdNonClaims: v09Production.claimsNotProvedLocally,
      manualTwoDeviceChecklist: v09Production.manualTwoDeviceChecklist,
      cloudRelayDecision: productionProof.productionTruth.cloudRelayDecision,
    },
    mobilePlatformProof: {
      parentMobile: capabilitySummary(
        capabilities,
        ['android', 'ios'],
        ['parent-mobile-observer', 'parent-mobile-controller']
      ),
      androidChild: capabilitySummary(
        capabilities,
        ['android'],
        [
          'foreground-mobile-service',
          'local-storage',
          'typed-protocol-bridge',
          'usage-stats',
          'accessibility-service',
          'vpn-dns-filtering',
          'device-owner-policy',
          'managed-profile',
        ]
      ),
      iosChild: capabilitySummary(
        capabilities,
        ['ios'],
        [
          'family-controls-entitlement',
          'device-activity',
          'screen-time-api',
          'network-extension',
          'notifications',
          'background-execution',
          'testflight-distribution',
        ]
      ),
    },
  };

  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  console.log(`platform-os-lan-mobile-proof-ok:${proofLabels.join(',')}`);
  console.log(`evidence=${proofPath}`);
}

function assertProductionProof(proof) {
  assertEqual(
    proof.productionTruth.cloudRelayDecision.includes('no cloud relay behavior is implemented'),
    true,
    'cloud relay non-claim'
  );
  assertEqual(
    proof.manualProofRequirements.parentMobileAndChildPlatforms.length,
    3,
    'mobile/manual platform proof checklist'
  );
  proofLabels.push('cloud-relay.explicit-not-implemented');
}

function assertProcessTerminateProof(evidence) {
  assertOneOf(
    evidence.serviceScope?.processTerminateServiceProof,
    ['actually-enforced', 'unsupported-platform'],
    'process terminate service proof'
  );
  const assertion = evidence.assertions.find((candidate) => candidate.id === 'process-terminate-owned-process');
  if (!assertion) {
    throw new Error('Missing process-terminate-owned-process assertion.');
  }
  assertOneOf(assertion.status, ['actually-enforced', 'unavailable'], 'process terminate assertion status');
  assertEqual(assertion.adapterKind, 'process-control', 'process terminate adapter');
  proofLabels.push('v0.8.process-terminate-owned-process-proof');
}

function assertHouseholdLanProof(evidence) {
  assertArrayIncludes(
    evidence.claimsProvedLocally,
    'trusted registry persists selected route and recovers it after restart',
    'route persistence/recovery'
  );
  assertArrayIncludes(
    evidence.claimsNotProvedLocally,
    'real household router discovery across two physical devices',
    'physical household discovery non-claim'
  );
  if (!Array.isArray(evidence.manualTwoDeviceChecklist) || evidence.manualTwoDeviceChecklist.length !== 1) {
    throw new Error('Expected one manual two-device household checklist.');
  }
  proofLabels.push('v0.9.household-two-device-manual-checklist');
}

function assertMobileCapabilities(capabilities) {
  assertCapability(capabilities, 'android', 'parent-mobile-observer', 'scaffold');
  assertCapability(capabilities, 'android', 'parent-mobile-controller', 'manual-required');
  assertCapability(capabilities, 'android', 'foreground-mobile-service', 'manual-required');
  assertCapability(capabilities, 'android', 'local-storage', 'scaffold');
  assertCapability(capabilities, 'android', 'typed-protocol-bridge', 'scaffold');
  assertCapability(capabilities, 'android', 'usage-stats', 'manual-required');
  assertCapability(capabilities, 'android', 'accessibility-service', 'manual-required');
  assertCapability(capabilities, 'android', 'vpn-dns-filtering', 'manual-required');
  assertCapability(capabilities, 'android', 'device-owner-policy', 'manual-required');
  assertCapability(capabilities, 'android', 'managed-profile', 'manual-required');
  assertCapability(capabilities, 'ios', 'parent-mobile-observer', 'scaffold');
  assertCapability(capabilities, 'ios', 'parent-mobile-controller', 'manual-required');
  assertCapability(capabilities, 'ios', 'family-controls-entitlement', 'manual-required');
  assertCapability(capabilities, 'ios', 'device-activity', 'manual-required');
  assertCapability(capabilities, 'ios', 'screen-time-api', 'manual-required');
  assertCapability(capabilities, 'ios', 'network-extension', 'manual-required');
  assertCapability(capabilities, 'ios', 'notifications', 'manual-required');
  assertCapability(capabilities, 'ios', 'background-execution', 'manual-required');
  assertCapability(capabilities, 'ios', 'testflight-distribution', 'manual-required');
  proofLabels.push('mobile-platform.capability-specific-states');
}

function assertProofMatrix(matrix) {
  const claim = matrix.claims.find((candidate) => candidate.id === 'platform-os-lan-mobile-proof');
  if (!claim) {
    throw new Error('Proof matrix is missing platform-os-lan-mobile-proof claim.');
  }
  assertEqual(claim.platformCoverage.windows, 'real-local-windows-proof', 'Windows proof state');
  assertEqual(claim.platformCoverage.android, 'manual-required', 'Android proof state');
  assertEqual(claim.platformCoverage.ios, 'manual-required', 'iOS proof state');
  const scenario = matrix.checkpointScenarios.find((candidate) => candidate.id === 'platform-os-lan-mobile-proof');
  if (!scenario) {
    throw new Error('Proof matrix is missing platform-os-lan-mobile-proof checkpoint scenario.');
  }
  assertSetHas(
    new Set(scenario.ciCommands),
    'node scripts/test/platform-os-lan-mobile-proof.mjs',
    'OS/LAN/mobile proof command is matrix-listed'
  );
  proofLabels.push('proof-matrix.platform-os-lan-mobile-proof-states');
}

function assertCapability(capabilities, platform, capabilityName, expectedStatus) {
  const platformEntry = capabilities.find((entry) => entry.platform === platform);
  const capability = platformEntry?.capabilities.find((candidate) => candidate.capability === capabilityName);
  assertEqual(capability?.status, expectedStatus, `${platform}.${capabilityName}`);
}

function capabilitySummary(capabilities, platforms, names) {
  return capabilities
    .filter((entry) => platforms.includes(entry.platform))
    .flatMap((entry) =>
      entry.capabilities
        .filter((capability) => names.includes(capability.capability))
        .map((capability) => ({
          platform: entry.platform,
          capability: capability.capability,
          status: capability.status,
          note: capability.note,
        }))
    );
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

async function latestJson(directory) {
  const entries = await readdir(directory, { withFileTypes: true });
  const jsonFiles = [];
  for (const entry of entries) {
    if (entry.isFile() && entry.name.endsWith('.json')) {
      const path = join(directory, entry.name);
      jsonFiles.push({ path, data: JSON.parse(await readFile(path, 'utf8')) });
    }
  }
  if (jsonFiles.length === 0) {
    throw new Error(`No JSON evidence files found in ${directory}`);
  }
  jsonFiles.sort((left, right) => left.path.localeCompare(right.path));
  return jsonFiles.at(-1);
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

function assertSetHas(set, value, label) {
  if (!set.has(value)) {
    throw new Error(`${label}: missing ${value}`);
  }
}
