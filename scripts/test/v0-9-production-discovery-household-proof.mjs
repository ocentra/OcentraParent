import { spawn } from 'node:child_process';
import { existsSync } from 'node:fs';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const repoRoot = process.cwd();
const lanDomainRoot = join(repoRoot, 'packages', 'lan-domain');
const productionDiscoveryModulePath = join(lanDomainRoot, 'dist', 'v0-9-production-discovery-household-proof.js');
const outputDir = join(repoRoot, 'output', 'lan-plan-proof', '01-lan-b1-proof-regeneration');
const sourceMatrixProofPath = join(outputDir, '01-lan-source-matrix-plan-completion-proof.json');
const signedRelayProofPath = join(outputDir, '02-lan-signed-discovery-relay-spine-proof.json');
const proofPath = join(outputDir, '03-production-discovery-household-proof.json');
const commands = [];
const sensitiveEvidenceMarkers = ['rawEvidence', 'rawToken', 'activity.sqlite', 'activity.ndjson'];

await main();

async function main() {
  await mkdir(outputDir, { recursive: true });
  await ensureLanDomainBuild();
  await runCommand('cmd', ['/c', 'npx', 'vitest', 'run', 'tests/unit/v0-9-production-discovery-household-proof.test.ts'], lanDomainRoot);

  const productionDiscoveryContract = await import(moduleUrl(productionDiscoveryModulePath));
  const sourceMatrixProof = await readJson(sourceMatrixProofPath);
  const signedRelayProof = await readJson(signedRelayProofPath);
  const readModel =
    productionDiscoveryContract.V09ProductionDiscoveryHouseholdProofReadModelSchema.parse(readModelFixture());

  assertArrayIncludes(sourceMatrixProof.claimsNotProved, 'Physical two-device household LAN readiness.', 'source matrix non-claim');
  assertArrayIncludes(signedRelayProof.manualProofRequired, 'signed-child-agent-hello', 'signed relay manual source');
  assertArrayIncludes(signedRelayProof.manualProofRequired, 'signed-child-agent-heartbeat', 'signed relay manual source');
  assertManualChecklist(readModel.manualHouseholdProofChecklist);

  const proof = {
    schemaVersion: 1,
    checkedAt: new Date().toISOString(),
    commit: await gitHead(),
    proofMode: 'v0-9-production-discovery-household-proof',
    ownerBoundary: 'packages/lan-domain',
    commands,
    artifactPath: relativePath(proofPath),
    evidence: {
      sourceMatrix: relativePath(sourceMatrixProofPath),
      signedDiscoveryRelay: relativePath(signedRelayProofPath),
    },
    proofLabels: [
      'v0.9.production-discovery.states-explicit',
      'v0.9.household-route-checks.machine-checked',
      'v0.9.household-manual-and-cloud-nonclaims-preserved',
    ],
    readModel,
    claimsProved: [
      'Production discovery states remain explicit for local mechanical proof without upgrading physical household readiness.',
      'Route checks, restart recovery, wrong-origin, wrong-device, stale, offline, revoked, and unavailable states remain typed.',
      'Manual household proof checklist and cloud-relay non-claim remain explicit in the lan-domain read model.',
    ],
    claimsNotProved: [
      'Physical household LAN product readiness across two real devices.',
      'Router discovery, firewall prompt handling, or local-network permission behavior.',
      'Real Android or iOS controller authority or background LAN behavior.',
      'Cloud relay routing, storage, authentication, or runtime implementation.',
    ],
  };

  assertNoSensitiveEvidenceMarkers(proof);
  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  console.log('v0-9-production-discovery-household-proof-ok');
  console.log(`evidence=${proofPath}`);
}

async function ensureLanDomainBuild() {
  if (existsSync(productionDiscoveryModulePath)) {
    return;
  }
  await runCommand('cmd', ['/c', 'npm', 'run', 'build'], lanDomainRoot);
}

function readModelFixture() {
  const checkedAt = '2026-05-30T20:50:00.000Z';
  const routeId = 'lan-route-production-discovery-household-proof';
  return {
    schemaVersion: 'v0.9',
    checkedAt,
    proofBoundary: 'local-real-service-not-physical-household-lan',
    productReadinessDecision: 'not-ready-for-product-ready-household-lan-claim',
    productionDiscoveryStates: [
      evidence(routeId, 'production-discovery-states', 'discovered', 'discovered', 'unpaired', 'online', null),
      evidence(routeId, 'production-discovery-states', 'pending', 'pending', 'pairing', 'online', null),
      evidence(routeId, 'production-discovery-states', 'paired', 'paired', 'paired', 'online', null),
      evidence(routeId, 'production-discovery-states', 'revoked', 'revoked', 'revoked', 'online', 'revoked'),
      evidence(routeId, 'production-discovery-states', 'stale', 'stale', 'paired', 'stale', 'stale'),
      evidence(routeId, 'production-discovery-states', 'offline', 'offline', 'paired', 'offline', 'offline'),
      evidence(routeId, 'production-discovery-states', 'unavailable', 'unavailable', 'unpaired', 'offline', 'unsupported-route'),
    ],
    routeChecks: [
      evidence(routeId, 'paired-route-accepted', 'paired', 'paired', 'paired', 'online', null),
      evidence(routeId, 'failed-unpaired-rejected', 'failed-unpaired', 'unavailable', 'unpaired', 'online', 'anonymous'),
      evidence(routeId, 'stale-source-rejected', 'stale', 'stale', 'paired', 'stale', 'stale'),
      evidence(routeId, 'offline-device-rejected', 'offline', 'offline', 'paired', 'offline', 'offline'),
      evidence(routeId, 'revoked-pairing-rejected', 'revoked', 'revoked', 'revoked', 'online', 'revoked'),
      evidence(routeId, 'unavailable-route-rejected', 'unavailable', 'unavailable', 'paired', 'online', 'unsupported-route'),
      evidence(routeId, 'wrong-origin-rejected', 'wrong-origin', 'unavailable', 'paired', 'online', 'wrong-origin'),
      evidence(routeId, 'wrong-device-rejected', 'wrong-device', 'unavailable', 'paired', 'online', 'wrong-device'),
    ],
    restartRecovery: [
      evidence(routeId, 'restart-selected-route-recovered', 'restart-recovered', 'paired', 'paired', 'online', null, 'registry-restored-after-restart'),
      evidence(routeId, 'restart-registry-state-recovered', 'restart-recovered', 'paired', 'paired', 'online', null, 'selected-route-persisted'),
    ],
    sourceDeviceStates: [
      evidence(routeId, 'stale-source-rejected', 'stale', 'stale', 'paired', 'stale', 'stale'),
      evidence(routeId, 'offline-device-rejected', 'offline', 'offline', 'paired', 'offline', 'offline'),
      evidence(routeId, 'revoked-pairing-rejected', 'revoked', 'revoked', 'revoked', 'online', 'revoked'),
      evidence(routeId, 'unavailable-route-rejected', 'unavailable', 'unavailable', 'unpaired', 'offline', 'local-network-disabled'),
      evidence(routeId, 'manual-physical-household-checklist', 'manual-required', 'unavailable', 'unpaired', 'offline', 'local-network-disabled', 'manual-required-physical-route-recovery', 'manual-required', 'manual-proof'),
    ],
    manualHouseholdProofChecklist: [
      manualChecklistItem('two-physical-hosts', 'two named household devices on the same LAN'),
      manualChecklistItem('household-router-reachability', 'router or network reachability artifact'),
      manualChecklistItem('os-firewall-or-local-network-permission', 'firewall or OS local-network permission artifact'),
      manualChecklistItem('allowed-origin-on-physical-controller', 'allowed origin from the physical controller host'),
      manualChecklistItem('physical-route-selection-and-takeover', 'physical route selection and takeover artifact'),
      manualChecklistItem('physical-revocation-and-rejection', 'physical revocation before rejected follow-up control'),
      manualChecklistItem('physical-stale-offline-selected-device', 'stopped or paused selected child service artifact'),
      manualChecklistItem('real-mobile-controller-package', 'real Android or iOS controller package proof'),
      manualChecklistItem('real-mobile-observer-package', 'real Android or iOS observer package proof'),
      manualChecklistItem('real-lan-ai-provider-host', 'real opted-in provider host proof'),
      manualChecklistItem('cloud-relay-separate-proof', 'separate authenticated cloud relay proof'),
    ],
    claimsProved: [
      'local real-service production discovery proof preserves route checks and restart recovery states',
      'wrong-origin and wrong-device evidence remain explicit rejection states',
    ],
    claimsNotProved: [
      'physical household LAN readiness',
      'cloud relay routing storage or authentication',
      'mobile background controller behavior',
    ],
  };
}

function evidence(routeId, check, sourceState, discoveryState, trustState, reachability, rejectionReason, routeRecoveryState = 'fail-closed-unpaired', proofState = 'ci-mechanical-proof', runtimeOwner = 'proof-harness') {
  return {
    schemaVersion: 'v0.9',
    check,
    sourceState,
    routeId,
    discoveryState,
    trustState,
    reachability,
    rejectionReason,
    routeRecoveryState,
    proofState,
    runtimeOwner,
    evidenceLabel: `${check} evidence`,
  };
}

function manualChecklistItem(gate, requiredArtifactSummary) {
  return {
    schemaVersion: 'v0.9',
    gate,
    state: 'manual-required',
    requiredArtifactSummary,
    runtimeOwner: 'manual-proof',
  };
}

function assertManualChecklist(checklist) {
  if (!Array.isArray(checklist) || checklist.length !== 11) {
    throw new Error(`Expected 11 manual household proof checklist items, received ${checklist.length}.`);
  }
}

function assertNoSensitiveEvidenceMarkers(value) {
  const serialized = JSON.stringify(value);
  for (const marker of sensitiveEvidenceMarkers) {
    if (serialized.includes(marker)) {
      throw new Error(`Proof includes sensitive marker ${marker}.`);
    }
  }
}

function assertArrayIncludes(values, expected, label) {
  if (!Array.isArray(values) || !values.includes(expected)) {
    throw new Error(`${label}: missing ${expected}.`);
  }
}

async function runCommand(commandName, args, cwd) {
  commands.push(`${relativePath(cwd)} :: ${[commandName, ...args].join(' ')}`);
  await new Promise((resolve, reject) => {
    const child = spawn(commandName, args, { cwd, stdio: 'inherit', windowsHide: true });
    child.once('exit', (code) => {
      if (code === 0) {
        resolve();
        return;
      }
      reject(new Error(`${commandName} ${args.join(' ')} exited with ${code}.`));
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
    child.once('exit', (code) => (code === 0 ? resolve() : reject(new Error('git rev-parse HEAD failed.'))));
    child.once('error', reject);
  });
  return chunks.join('').trim();
}

function moduleUrl(path) {
  return `file:///${path.replaceAll('\\', '/')}`;
}

function relativePath(path) {
  return relative(repoRoot, path).replaceAll('\\', '/');
}
