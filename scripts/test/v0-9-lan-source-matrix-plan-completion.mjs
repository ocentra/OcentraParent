import { spawn } from 'node:child_process';
import { existsSync } from 'node:fs';
import { mkdir, writeFile } from 'node:fs/promises';
import { join, relative } from 'node:path';

const repoRoot = process.cwd();
const lanDomainRoot = join(repoRoot, 'packages', 'lan-domain');
const outputDir = join(repoRoot, 'output', 'lan-plan-proof', '01-lan-b1-proof-regeneration');
const proofPath = join(outputDir, '01-lan-source-matrix-plan-completion-proof.json');
const commands = [];

await main();

async function main() {
  await mkdir(outputDir, { recursive: true });
  await ensureLanDomainBuild();
  await runCommand(
    'cmd',
    [
      '/c',
      'npx',
      'vitest',
      'run',
      'tests/unit/lan-discovery-source-matrix.test.ts',
      'tests/unit/lan-pairing-browser-add-device-state.test.ts',
    ],
    lanDomainRoot
  );

  const sourceMatrixContract = await import('@ocentra-parent/schema-domain/lan-source-matrix');
  const pairingDeviceContract = await import('@ocentra-parent/schema-domain/lan-pairing-device');
  const matrix = sourceMatrixContract.LanDiscoverySourceMatrixSchema.parse(sourceMatrixFixture());
  const readModel = pairingDeviceContract.LanBrowserAddDeviceReadModelSchema.parse({
    ...addDeviceReadModelFixture(),
    lanDiscoverySourceMatrix: matrix,
  });

  assertSourceMatrix(matrix);

  const proof = {
    schemaVersion: 1,
    checkedAt: new Date().toISOString(),
    commit: await gitHead(),
    proofMode: 'v0-9-lan-source-matrix-plan-completion',
    ownerBoundary: 'packages/lan-domain',
    commands,
    artifactPath: relativePath(proofPath),
    readModelFields: {
      discoverySource: readModel.discoverySource,
      addDeviceState: readModel.addDeviceState,
      localServiceDiscoveryState: readModel.localServiceDiscoveryState,
      physicalHouseholdLanState: readModel.physicalHouseholdLanState,
      cloudRelayState: readModel.cloudRelayState,
      hasLanDiscoverySourceMatrix: readModel.lanDiscoverySourceMatrix !== undefined,
    },
    workpackCounts: countByStatus(matrix.workpackRows),
    sourceCounts: countByStatus(matrix.sourceRows),
    manualRequiredWorkpacks: matrix.workpackRows
      .filter((row) => row.status === 'manual-required')
      .map((row) => `${row.workpackId}:${row.title}`),
    notImplementedWorkpacks: matrix.workpackRows
      .filter((row) => row.status === 'not-implemented')
      .map((row) => `${row.workpackId}:${row.title}`),
    manualRequiredSources: matrix.sourceRows.filter((row) => row.status === 'manual-required').map((row) => row.source),
    weakSourceFence: {
      weakSources: matrix.sourceRows.filter((row) => !row.canConfirmChildAgent && !row.canAssignChildProfile).length,
      signedSources: matrix.sourceRows
        .filter((row) => row.source === 'signed-child-agent-hello' || row.source === 'signed-child-agent-heartbeat')
        .map((row) => ({
          source: row.source,
          canConfirmChildAgent: row.canConfirmChildAgent,
          requiredArtifactSummary: row.requiredArtifactSummary,
        })),
    },
    claimsProved: [
      'The authoritative 01-20 LAN workpack model is represented as a typed lan-domain source matrix.',
      'Weak discovery sources remain fenced from child confirmation and profile assignment.',
      'Signed child-agent sources remain artifact-gated instead of being upgraded to implemented proof.',
      'The browser add-device read model carries the source matrix without claiming portal or service runtime closure.',
    ],
    claimsNotProved: [
      'Rust service/runtime execution for the LAN source matrix.',
      'Portal rendering or screenshot proof for Devices/LAN, Activity/Network, or policy surfaces.',
      'Physical two-device household LAN readiness.',
      'Packet-mode ARP, passive listener, or advertisement proof.',
    ],
  };

  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  console.log('v0-9-lan-source-matrix-plan-completion-ok');
  console.log(`evidence=${proofPath}`);
}

async function ensureLanDomainBuild() {
  if (existsSync(sourceMatrixModulePath) && existsSync(pairingDeviceModulePath)) {
    return;
  }
  await runNpm(['run', 'build'], lanDomainRoot);
}

function sourceMatrixFixture() {
  return {
    schemaVersion: 'v0.9',
    generatedAt: '2026-06-02T15:55:00.000Z',
    workpackRows: workpackRows(),
    sourceRows: sourceRows(),
    claimsProved: [
      'all LAN plan workpacks are represented in a service-backed source matrix read model',
      'weak LAN discovery sources cannot confirm child identity or assign child profiles',
    ],
    claimsNotProved: [
      'packet-mode ARP sweep and passive listeners remain gated until packet driver artifacts exist',
      'physical household LAN completion remains manual-required until real two-host proof is attached',
      'mDNS/SSDP advertisement and responder behavior remains manual-required until fixtures and LAN captures exist',
    ],
  };
}

function workpackRows() {
  return [
    workpack('01', 'Contract boundary and Effect schemas', 'discovered', 'ci-mechanical-proof', 'implemented', null),
    workpack('02', 'Evidence model and device record', 'pending', 'ci-mechanical-proof', 'partial', null),
    workpack('03', 'Interface detection', 'pending', 'ci-mechanical-proof', 'partial', null),
    workpack('04', 'Neighbor table ingestion', 'discovered', 'ci-mechanical-proof', 'partial', null),
    workpack('05', 'Targeted ARP checks', 'unavailable', 'not-implemented', 'not-implemented', packetArtifact()),
    workpack('06', 'Bounded ARP sweep', 'unavailable', 'not-implemented', 'not-implemented', packetArtifact()),
    workpack(
      '07',
      'Passive discovery listeners',
      'manual-required',
      'manual-required',
      'manual-required',
      packetArtifact()
    ),
    workpack(
      '08',
      'mDNS and DNS-SD discovery',
      'manual-required',
      'manual-required',
      'manual-required',
      mdnsArtifact()
    ),
    workpack('09', 'SSDP and UPnP discovery', 'manual-required', 'manual-required', 'manual-required', mdnsArtifact()),
    workpack('10', 'NetBIOS, LLMNR, and reverse DNS', 'pending', 'ci-mechanical-proof', 'partial', null),
    workpack('11', 'Light service probing', 'unavailable', 'not-implemented', 'not-implemented', packetArtifact()),
    workpack('12', 'OUI and vendor lookup', 'pending', 'ci-mechanical-proof', 'partial', null),
    workpack('13', 'Merge and de-duplication engine', 'pending', 'ci-mechanical-proof', 'partial', null),
    workpack('14', 'Explainable classification', 'pending', 'ci-mechanical-proof', 'partial', null),
    workpack('15', 'Household device store', 'pending', 'ci-mechanical-proof', 'partial', null),
    workpack('16', 'Read models and LAN events', 'discovered', 'ci-mechanical-proof', 'implemented', null),
    workpack(
      '17',
      'Parent and child mDNS advertisements',
      'manual-required',
      'manual-required',
      'manual-required',
      mdnsArtifact()
    ),
    workpack(
      '18',
      'Signed child hello and heartbeat',
      'manual-required',
      'manual-required',
      'manual-required',
      signedArtifact()
    ),
    workpack('19', 'Assignment, revocation, and audit', 'pending', 'ci-mechanical-proof', 'partial', null),
    workpack('20', 'Proof gates, fixtures, and rollout', 'pending', 'ci-mechanical-proof', 'partial', null),
  ];
}

function sourceRows() {
  return [
    source('windows-neighbor-table', '04', 'implemented', 'weak-identity', false, false, null),
    source('mdns-dns-sd-query', '08', 'manual-required', 'name-only', false, false, mdnsArtifact()),
    source('ssdp-upnp-query', '09', 'manual-required', 'name-only', false, false, mdnsArtifact()),
    source('netbios-name-cache', '10', 'manual-required', 'name-only', false, false, mdnsArtifact()),
    source('service-identity-probe', '11', 'manual-required', 'classification-only', false, false, packetArtifact()),
    source('oui-vendor-lookup', '12', 'manual-required', 'classification-only', false, false, mdnsArtifact()),
    source('signed-child-agent-hello', '18', 'manual-required', 'strong-identity', true, false, signedArtifact()),
    source('signed-child-agent-heartbeat', '18', 'manual-required', 'strong-identity', true, false, signedArtifact()),
  ];
}

function addDeviceReadModelFixture() {
  return {
    schemaVersion: 'v0.9',
    generatedAt: '2026-06-02T12:00:00.000Z',
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
    honestNonClaims: ['physical-household-lan-manual-required'],
  };
}

function workpack(workpackId, title, discoveryState, proofState, status, requiredArtifactSummary) {
  return {
    schemaVersion: 'v0.9',
    workpackId,
    title,
    discoveryState,
    proofState,
    runtimeOwner:
      proofState === 'manual-required' || proofState === 'not-implemented' ? 'manual-proof' : 'rust-service-read-model',
    status,
    readModelVisible: true,
    requiredArtifactSummary,
  };
}

function source(
  sourceKind,
  workpackId,
  status,
  authority,
  canConfirmChildAgent,
  canAssignChildProfile,
  requiredArtifactSummary
) {
  return {
    schemaVersion: 'v0.9',
    source: sourceKind,
    workpackId,
    status,
    authority,
    runtimePath: status === 'implemented' ? 'rust-service-read-model' : 'manual-artifact',
    uiSurface: status === 'implemented' ? 'devices-lan' : 'proof-report',
    canConfirmChildAgent,
    canAssignChildProfile,
    canControlRoute: canConfirmChildAgent,
    requiresSelectedInterface: true,
    persistsAcrossRestart: canConfirmChildAgent,
    evidenceLabel: sourceKind,
    requiredArtifactSummary,
  };
}

function assertSourceMatrix(matrix) {
  if (matrix.workpackRows.length !== 20) {
    throw new Error(`Expected 20 workpack rows, received ${matrix.workpackRows.length}.`);
  }
  assertStatusCount(matrix.workpackRows, 'implemented', 2, 'implemented workpacks');
  assertStatusCount(matrix.workpackRows, 'manual-required', 5, 'manual-required workpacks');
  assertStatusCount(matrix.workpackRows, 'not-implemented', 3, 'not-implemented workpacks');
  assertRow(matrix.sourceRows, 'source', 'windows-neighbor-table', 'canAssignChildProfile', false);
  assertRow(matrix.sourceRows, 'source', 'mdns-dns-sd-query', 'canConfirmChildAgent', false);
  assertRow(matrix.sourceRows, 'source', 'signed-child-agent-heartbeat', 'requiredArtifactSummary', signedArtifact());
}

function assertStatusCount(rows, status, minimum, label) {
  const count = rows.filter((row) => row.status === status).length;
  if (count < minimum) {
    throw new Error(`Expected at least ${minimum} ${label}, received ${count}.`);
  }
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

function countByStatus(rows) {
  return rows.reduce((counts, row) => {
    counts[row.status] = (counts[row.status] ?? 0) + 1;
    return counts;
  }, {});
}

function packetArtifact() {
  return 'Attach packet-driver or controlled packet IO proof with selected interface, subnet cap, timing, and malformed packet fixtures.';
}

function mdnsArtifact() {
  return 'Attach mDNS/DNS-SD and SSDP/UPnP fixtures or LAN captures with sanitized host or service names.';
}

function signedArtifact() {
  return 'Attach signed child-agent hello and heartbeat payloads with nonce, family hash, route id, and replay rejection logs.';
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
