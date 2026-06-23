import { spawn } from 'node:child_process';
import { existsSync } from 'node:fs';
import { mkdir, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const repoRoot = process.cwd();
const lanDomainRoot = join(repoRoot, 'packages', 'lan-domain');
const outputDir = join(repoRoot, 'output', 'lan-plan-proof', '01-lan-b1-proof-regeneration');
const proofPath = join(outputDir, '02-lan-signed-discovery-relay-spine-proof.json');
const commands = [];

await main();

async function main() {
  await mkdir(outputDir, { recursive: true });
  await ensureLanDomainBuild();
  await runCommand(
    'cmd',
    ['/c', 'npx', 'vitest', 'run', 'tests/unit/lan-signed-discovery-relay-spine.test.ts'],
    lanDomainRoot
  );

  const signedDiscoveryRelaySpineContract = await import('@ocentra-parent/schema-domain/lan-relay-spine');
  const pairingDeviceContract = await import('@ocentra-parent/schema-domain/lan-pairing-device');
  const spine = signedDiscoveryRelaySpineContract.LanRelaySpineSchema.parse(signedDiscoveryRelaySpineFixture());
  const readModel = pairingDeviceContract.LanBrowserAddDeviceReadModelSchema.parse({
    ...addDeviceReadModelFixture(),
    signedDiscoveryRelaySpine: spine,
  });

  assertSignedDiscoveryRelaySpine(readModel.signedDiscoveryRelaySpine);

  const proof = {
    schemaVersion: 1,
    checkedAt: new Date().toISOString(),
    commit: await gitHead(),
    proofMode: 'v0-9-lan-signed-discovery-relay-spine',
    ownerBoundary: 'packages/lan-domain',
    commands,
    artifactPath: relativePath(proofPath),
    proofLabels: [
      'v0.9.signed-lan-discovery.contract-proof',
      'v0.9.signed-lan-discovery.manual-boundary-preserved',
      'v0.9.signed-lan-discovery.no-custody-nonclaim',
    ],
    adapterCount: spine.adapterRows.length,
    signedProofCheckCount: spine.signedProofRows.length,
    routeSafetyCheckCount: spine.routeSafetyRows.length,
    relayCacheCheckCount: spine.relayCacheRows.length,
    manualProofRequired: spine.manualProofRequired,
    notImplemented: spine.notImplemented,
    claimsProved: [
      'Signed LAN discovery adapter rows stay typed and explicit in lan-domain.',
      'Manual-required signed child-agent hello and heartbeat boundaries remain preserved.',
      'Wrong-origin, wrong-device, replay, stale, revoked, and unauthenticated rejection states remain explicit.',
      'Relay, cache, and storage rows keep non-custody and not-implemented boundaries visible.',
    ],
    claimsNotProved: [
      'Real signed child-agent hello and heartbeat artifacts from a second installed host.',
      'Physical household LAN readiness across two real child-agent hosts.',
      'Production relay or cache routing.',
      'Parent-owned storage adapter implementation.',
    ],
  };

  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  console.log('v0-9-lan-signed-discovery-relay-spine-ok');
  console.log(`evidence=${proofPath}`);
}

async function ensureLanDomainBuild() {
  if (existsSync(signedDiscoveryRelaySpineModulePath) && existsSync(pairingDeviceModulePath)) {
    return;
  }
  await runNpm(['run', 'build'], lanDomainRoot);
}

function signedDiscoveryRelaySpineFixture() {
  return {
    schemaVersion: 'v0.9',
    generatedAt: '2026-06-02T11:40:00.000Z',
    adapterRows: adapterRows(),
    signedProofRows: signedProofRows(),
    routeSafetyRows: routeSafetyRows(),
    relayCacheRows: relayCacheRows(),
    manualProofRequired: [
      'mdns-name',
      'ssdp-name',
      'router-dhcp-name',
      'manual-direct-address',
      'signed-child-agent-hello',
      'signed-child-agent-heartbeat',
    ],
    notImplemented: [
      'relay-route-unavailable',
      'relay-route-queued-not-configured',
      'cache-route-unavailable',
      'parent-owned-storage-unavailable',
    ],
    claimsProved: [
      'passive and router LAN discovery are separated from controllable child-agent signed discovery rows',
      'signed proof rejection states include unauthenticated, expired, replayed, wrong-origin, wrong-device, revoked, and stale outcomes',
      'route safety rows keep registry recovery, selected route custody, and parent decisions explicit',
      'relay and cache rows are local-first and do not claim Ocentra child-data custody',
    ],
    claimsNotProved: [
      'signed child-agent hello and heartbeat artifacts from a second installed device are still manual-required',
      'physical household LAN readiness still requires two real child-agent hosts',
      'relay or cache production routing remains unavailable or not implemented',
      'parent-owned storage is unavailable until a parent-selected storage adapter exists',
    ],
  };
}

function adapterRows() {
  return [
    adapterRow('passive-lan-neighbor', 'discovered', 'ci-mechanical-proof', 'strong', 'passive-lan-observation', null),
    adapterRow(
      'router-infrastructure',
      'discovered',
      'ci-mechanical-proof',
      'strong',
      'router-infrastructure-observation',
      null
    ),
    adapterRow(
      'mdns-name',
      'manual-required',
      'manual-required',
      'manual-required',
      'passive-lan-observation',
      'mDNS packet proof'
    ),
    adapterRow(
      'ssdp-name',
      'manual-required',
      'manual-required',
      'manual-required',
      'passive-lan-observation',
      'SSDP packet proof'
    ),
    adapterRow(
      'router-dhcp-name',
      'manual-required',
      'manual-required',
      'manual-required',
      'router-infrastructure-observation',
      'router DHCP proof'
    ),
    adapterRow(
      'manual-direct-address',
      'manual-required',
      'manual-required',
      'manual-required',
      'manual-parent-entry',
      'manual direct address proof'
    ),
    adapterRow(
      'signed-child-agent-hello',
      'manual-required',
      'manual-required',
      'manual-required',
      'signed-child-agent-artifact',
      'signed hello proof'
    ),
    adapterRow(
      'signed-child-agent-heartbeat',
      'manual-required',
      'manual-required',
      'manual-required',
      'signed-child-agent-artifact',
      'signed heartbeat proof'
    ),
  ];
}

function signedProofRows() {
  return [
    signedProof('signed-hello-manual-required', 'manual-required', 'queued', null, 'manual-required'),
    signedProof('signed-heartbeat-manual-required', 'manual-required', 'queued', null, 'manual-required'),
    signedProof('accepted-signed-child-agent-manual-required', 'manual-required', 'queued', null, 'manual-required'),
    signedProof('unauthenticated-caller-rejected', 'rejected', 'rejected', 'anonymous', 'ci-mechanical-proof'),
    signedProof('expired-signed-proof-rejected', 'expired', 'rejected', 'expired', 'ci-mechanical-proof'),
    signedProof('replayed-signed-proof-rejected', 'rejected', 'rejected', 'replayed', 'ci-mechanical-proof'),
    signedProof('wrong-origin-signed-proof-rejected', 'rejected', 'rejected', 'wrong-origin', 'ci-mechanical-proof'),
    signedProof('wrong-device-signed-proof-rejected', 'rejected', 'rejected', 'wrong-device', 'ci-mechanical-proof'),
    signedProof('revoked-signed-proof-rejected', 'revoked', 'rejected', 'revoked', 'ci-mechanical-proof'),
    signedProof('stale-signed-proof-rejected', 'stale', 'rejected', 'stale', 'ci-mechanical-proof'),
  ];
}

function routeSafetyRows() {
  return [
    routeSafety(
      'trusted-registry-restart-recovery',
      'lan-route-local-network',
      'paired',
      'accepted',
      null,
      'parent-local-service'
    ),
    routeSafety(
      'selected-route-custody',
      'lan-route-local-network',
      'paired',
      'accepted',
      null,
      'parent-local-service'
    ),
    routeSafety(
      'stale-selected-device-rejected',
      'lan-route-local-network',
      'stale',
      'rejected',
      'stale',
      'parent-local-service'
    ),
    routeSafety(
      'offline-selected-device-rejected',
      'lan-route-local-network',
      'offline',
      'rejected',
      'offline',
      'parent-local-service'
    ),
    routeSafety(
      'wrong-route-rejected',
      'lan-route-local-network',
      'rejected',
      'rejected',
      'wrong-device',
      'parent-local-service'
    ),
    routeSafety(
      'revoked-route-rejected',
      'lan-route-local-network',
      'revoked',
      'rejected',
      'revoked',
      'parent-local-service'
    ),
    routeSafety(
      'parent-assign-decision-audited',
      'lan-route-local-network',
      'discovered',
      'accepted',
      null,
      'parent-local-service'
    ),
    routeSafety(
      'parent-rename-decision-audited',
      'lan-route-local-network',
      'discovered',
      'accepted',
      null,
      'parent-local-service'
    ),
    routeSafety(
      'parent-ignore-decision-audited',
      'lan-route-local-network',
      'discovered',
      'accepted',
      null,
      'parent-local-service'
    ),
    routeSafety(
      'parent-restore-decision-audited',
      'lan-route-local-network',
      'discovered',
      'accepted',
      null,
      'parent-local-service'
    ),
    routeSafety(
      'parent-trust-decision-audited',
      'lan-route-local-network',
      'paired',
      'accepted',
      null,
      'parent-local-service'
    ),
    routeSafety(
      'parent-revoke-decision-audited',
      'lan-route-local-network',
      'revoked',
      'accepted',
      null,
      'parent-local-service'
    ),
  ];
}

function relayCacheRows() {
  return [
    relayCache(
      'relay-route-unavailable',
      'unavailable',
      'unavailable',
      'not-implemented',
      'no-ocentra-child-data-custody'
    ),
    relayCache(
      'relay-route-queued-not-configured',
      'queued-not-configured',
      'pending',
      'not-implemented',
      'no-ocentra-child-data-custody'
    ),
    relayCache(
      'cache-route-unavailable',
      'unavailable',
      'unavailable',
      'not-implemented',
      'no-ocentra-child-data-custody'
    ),
    relayCache(
      'parent-owned-storage-unavailable',
      'unavailable',
      'unavailable',
      'not-implemented',
      'parent-owned-storage-unavailable'
    ),
    relayCache(
      'ocentra-child-data-custody-not-claimed',
      'local-first',
      'unavailable',
      'ci-mechanical-proof',
      'no-ocentra-child-data-custody'
    ),
  ];
}

function addDeviceReadModelFixture() {
  return {
    schemaVersion: 'v0.9',
    generatedAt: '2026-06-02T11:40:00.000Z',
    discoverySource: 'local-service',
    addDeviceState: 'pending',
    localServiceDiscoveryState: 'pending',
    physicalHouseholdLanState: 'manual-required',
    cloudRelayState: 'unavailable',
    scanSummary: {
      schemaVersion: 'v0.9',
      sourceLabels: ['local-service'],
      scannedDeviceCount: 1,
      agentDeviceCount: 1,
      passiveDeviceCount: 0,
      infrastructureDeviceCount: 0,
      unsupportedDeviceCount: 0,
    },
    discoveredDevices: [],
    canonicalHouseholdDevices: [],
    pairingRequests: [],
    trustedDeviceRegistry: [],
    householdDeviceDecisions: [],
    trustedDeviceIds: [],
    revokedDeviceIds: [],
    selectedDeviceReadiness: {
      schemaVersion: 'v0.9',
      selectedChildDeviceId: null,
      routeId: null,
      pairingId: null,
      trustState: 'unpaired',
      reachability: 'offline',
      readyForControl: false,
      staleAt: null,
      offlineAt: null,
    },
    controllerAuthority: 'active-controller',
    observerAuthority: 'observer',
    routeRequirementLabels: ['allowed-origin'],
    auditCheckLabels: ['anonymous', 'wrong-origin', 'wrong-device'],
    honestNonClaims: ['physical-household-lan-manual-required', 'cloud-relay-not-implemented'],
  };
}

function adapterRow(adapter, discoveryState, proofState, sourceConfidence, custodyLabel, requiredArtifactSummary) {
  return {
    schemaVersion: 'v0.9',
    adapter,
    discoveryState,
    proofState,
    sourceConfidence,
    custodyLabel,
    runtimeOwner: proofState === 'manual-required' ? 'manual-proof' : 'rust-service-read-model',
    evidenceLabel: `${adapter} adapter boundary`,
    requiredArtifactSummary,
  };
}

function signedProof(check, discoveryState, responseState, rejectionReason, proofState) {
  return {
    schemaVersion: 'v0.9',
    check,
    discoveryState,
    responseState,
    rejectionReason,
    proofState,
    runtimeOwner: proofState === 'manual-required' ? 'manual-proof' : 'rust-service-read-model',
    evidenceLabel: `${check} signed proof state`,
  };
}

function routeSafety(check, routeId, discoveryState, responseState, rejectionReason, custodyLabel) {
  return {
    schemaVersion: 'v0.9',
    check,
    routeId,
    discoveryState,
    responseState,
    rejectionReason,
    proofState: 'ci-mechanical-proof',
    runtimeOwner: 'rust-service-read-model',
    custodyLabel,
    evidenceLabel: `${check} route safety state`,
  };
}

function relayCache(check, decisionState, discoveryState, proofState, custodyLabel) {
  return {
    schemaVersion: 'v0.9',
    check,
    decisionState,
    discoveryState,
    proofState,
    runtimeOwner: proofState === 'ci-mechanical-proof' ? 'rust-service-read-model' : 'manual-proof',
    custodyLabel,
    evidenceLabel: `${check} relay cache state`,
  };
}

function assertSignedDiscoveryRelaySpine(spine) {
  if (!spine) {
    throw new Error('Signed discovery relay spine was not carried on the add-device read model.');
  }
  assertArrayIncludes(spine.manualProofRequired, 'signed-child-agent-hello', 'signed child-agent hello manual proof');
  assertArrayIncludes(
    spine.manualProofRequired,
    'signed-child-agent-heartbeat',
    'signed child-agent heartbeat manual proof'
  );
  assertArrayIncludes(spine.notImplemented, 'relay-route-unavailable', 'relay unavailable gap');
  assertArrayIncludes(spine.notImplemented, 'cache-route-unavailable', 'cache unavailable gap');
  assertRow(spine.adapterRows, 'adapter', 'passive-lan-neighbor', 'custodyLabel', 'passive-lan-observation');
  assertRow(spine.routeSafetyRows, 'check', 'wrong-route-rejected', 'rejectionReason', 'wrong-device');
  assertRow(
    spine.relayCacheRows,
    'check',
    'ocentra-child-data-custody-not-claimed',
    'custodyLabel',
    'no-ocentra-child-data-custody'
  );
}

function assertRow(rows, key, value, field, expected) {
  const row = rows.find((candidate) => candidate[key] === value);
  if (!row) {
    throw new Error(`Missing row ${value}.`);
  }
  if (row[field] !== expected) {
    throw new Error(`Expected ${value}.${field} to equal ${expected}, received ${row[field]}.`);
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

async function runNpm(args, cwd) {
  if (process.platform === 'win32') {
    await runCommand('cmd', ['/c', 'npm', ...args], cwd);
    return;
  }
  await runCommand('npm', args, cwd);
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

function relativePath(path) {
  return relative(repoRoot, path).replaceAll('\\', '/');
}
