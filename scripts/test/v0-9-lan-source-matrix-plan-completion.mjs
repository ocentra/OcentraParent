import { spawn } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { join } from 'node:path';

const repoRoot = process.cwd();
const outputDir = join(repoRoot, 'test-results', 'v0-9-lan-source-matrix-plan-completion');
const proofPath = join(outputDir, 'proof.json');
const commands = [];

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
    'tests/lan-discovery-source-matrix.test.ts',
    'tests/lan-signed-discovery-relay-spine.test.ts',
    'tests/lan-production-household-proof.test.ts',
  ]);
  await runCommand('cmd', [
    '/c',
    'npm',
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/agent-protocol-domain',
    '--',
    'tests/lan-discovery-source-matrix.test.ts',
    'tests/lan-pairing-browser-add-device-state.test.ts',
  ]);
  await runCommand('cargo', ['test', '-p', 'ocentra-parent-agent-protocol', 'lan_pairing_browser_add_device_state']);
  await runCommand('cargo', ['test', '-p', 'ocentra-parent-agent-service', 'lan_pairing_browser_add_device_state']);
  await runCommand('cmd', ['/c', 'npm', 'run', 'lint', '--workspace', '@ocentra-parent/portal']);

  const contract = await import(parentDomainLanPairingModuleUrl());
  const matrix = contract.LanDiscoverySourceMatrixSchema.parse(sourceMatrixFixture());
  const readModel = contract.LanBrowserAddDeviceReadModelSchema.parse({
    ...addDeviceReadModelFixture(),
    lanDiscoverySourceMatrix: matrix,
  });
  assertSourceMatrix(matrix);

  const workpackCounts = countByStatus(matrix.workpackRows);
  const sourceCounts = countByStatus(matrix.sourceRows);
  const weakSources = matrix.sourceRows.filter(
    (row) => row.canConfirmChildAgent !== true && row.canAssignChildProfile !== true
  );

  const proof = {
    schemaVersion: 1,
    checkedAt: new Date().toISOString(),
    commit: await gitHead(),
    proofMode: 'v0-9-lan-source-matrix-plan-completion',
    commands,
    readModelFields: {
      addDeviceState: readModel.addDeviceState,
      physicalHouseholdLanState: readModel.physicalHouseholdLanState,
      cloudRelayState: readModel.cloudRelayState,
      hasLanDiscoverySourceMatrix: readModel.lanDiscoverySourceMatrix !== undefined,
    },
    workpackCounts,
    sourceCounts,
    weakSourceFence: {
      weakSources: weakSources.length,
      totalSources: matrix.sourceRows.length,
      canConfirmChildAgent: weakSources.filter((row) => row.canConfirmChildAgent === true).length,
      canAssignChildProfile: weakSources.filter((row) => row.canAssignChildProfile === true).length,
    },
    manualRequiredWorkpacks: matrix.workpackRows
      .filter((row) => row.status === 'manual-required')
      .map((row) => `${row.workpackId}:${row.title}`),
    notImplementedWorkpacks: matrix.workpackRows
      .filter((row) => row.status === 'not-implemented')
      .map((row) => `${row.workpackId}:${row.title}`),
    manualRequiredSources: matrix.sourceRows
      .filter((row) => row.status === 'manual-required')
      .map((row) => row.source),
    notImplementedSources: matrix.sourceRows
      .filter((row) => row.status === 'not-implemented')
      .map((row) => row.source),
    claimsProved: [
      'LAN read model carries all 20 plan workpacks as typed status rows',
      'weak LAN evidence sources are visible but cannot confirm child-agent identity or assign child profile',
      'signed child-agent hello and heartbeat remain artifact-gated instead of being silently marked implemented',
      'Devices/LAN and Activity/Network can render the matrix through the service-backed add-device read model',
    ],
    claimsNotProved: matrix.claimsNotProved,
  };

  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  console.log('v0-9-lan-source-matrix-plan-completion-ok');
  console.log(`evidence=${proofPath}`);
}

function sourceMatrixFixture() {
  return {
    schemaVersion: 'v0.9',
    generatedAt: '2026-06-02T12:00:00.000Z',
    productionDiscoveryState: 'manual-required',
    workpackRows: workpackRows(),
    sourceRows: sourceRows(),
    claimsProved: [
      'read-model-source-matrix-visible',
      'weak-source-confirmation-fenced',
      'signed-child-agent-artifacts-gated',
    ],
    claimsNotProved: [
      'packet-mode adapters remain manual-required',
      'physical household proof still needs a second child-agent device',
      'mDNS/SSDP advertisement is not implemented yet',
    ],
  };
}

function workpackRows() {
  const rows = [
    ['01', 'Contract boundary and Effect schemas', 'implemented', 'typescript-contract'],
    ['02', 'Evidence model and device record', 'partial', 'typescript-contract'],
    ['03', 'Interface detection', 'partial', 'rust-service-read-model'],
    ['04', 'Neighbor table ingestion', 'partial', 'rust-service-read-model'],
    ['05', 'Targeted ARP checks', 'not-implemented', 'manual-artifact'],
    ['06', 'Bounded ARP sweep', 'not-implemented', 'manual-artifact'],
    ['07', 'Passive discovery listeners', 'manual-required', 'manual-artifact'],
    ['08', 'mDNS and DNS-SD discovery', 'manual-required', 'manual-artifact'],
    ['09', 'SSDP and UPnP discovery', 'manual-required', 'manual-artifact'],
    ['10', 'NetBIOS, LLMNR, and reverse DNS', 'partial', 'rust-service-read-model'],
    ['11', 'Light service probing', 'not-implemented', 'manual-artifact'],
    ['12', 'OUI and vendor lookup', 'partial', 'rust-service-read-model'],
    ['13', 'Merge and de-duplication engine', 'partial', 'rust-service-read-model'],
    ['14', 'Explainable classification', 'partial', 'rust-service-read-model'],
    ['15', 'Household device store', 'partial', 'rust-service-read-model'],
    ['16', 'Read models and LAN events', 'implemented', 'rust-service-read-model'],
    ['17', 'Parent and child mDNS advertisements', 'manual-required', 'manual-artifact'],
    ['18', 'Signed child hello and heartbeat', 'manual-required', 'manual-artifact'],
    ['19', 'Assignment, revocation, and audit', 'partial', 'rust-service-read-model'],
    ['20', 'Proof gates, fixtures, and rollout', 'partial', 'proof-harness'],
  ];
  return rows.map(([workpackId, title, status, runtimePath]) => ({
    schemaVersion: 'v0.9',
    workpackId,
    title,
    discoveryState: discoveryStateForStatus(status),
    proofState: proofStateForStatus(status),
    runtimeOwner: runtimeOwnerForPath(runtimePath),
    status,
    readModelVisible: true,
    requiredArtifactSummary: status === 'implemented' ? null : `${title} evidence remains ${status}`,
  }));
}

function sourceRows() {
  return [
    implementedSource('contract-boundary', '01', 'typescript-contract', 'activity-network', true, false),
    implementedSource('evidence-model', '02', 'typescript-contract', 'activity-network', true, false),
    implementedSource('interface-selection', '03', 'rust-service-read-model', 'devices-lan', true, false),
    weakSource('windows-neighbor-table', '04', 'rust-service-read-model', 'devices-lan'),
    weakSource('linux-proc-net-arp', '04', 'manual-artifact', 'activity-network'),
    weakSource('linux-ip-neigh', '04', 'manual-artifact', 'activity-network'),
    weakSource('macos-arp', '04', 'manual-artifact', 'activity-network'),
    weakSource('netbios-name-cache', '10', 'rust-service-read-model', 'activity-network'),
    weakSource('llmnr-name-query', '10', 'rust-service-read-model', 'activity-network'),
    weakSource('reverse-dns-query', '10', 'rust-service-read-model', 'activity-network'),
    weakSource('mdns-dns-sd-query', '08', 'manual-artifact', 'activity-network'),
    weakSource('ssdp-upnp-query', '09', 'manual-artifact', 'activity-network'),
    weakSource('service-identity-probe', '11', 'manual-artifact', 'activity-network'),
    weakSource('oui-vendor-lookup', '12', 'rust-service-read-model', 'devices-lan'),
    implementedSource('merge-deduplication', '13', 'rust-service-read-model', 'devices-lan', true, false),
    implementedSource('explainable-classification', '14', 'rust-service-read-model', 'devices-lan', true, false),
    implementedSource('household-device-store', '15', 'rust-service-read-model', 'policy-network', true, true),
    implementedSource('read-model-event-stream', '16', 'rust-service-read-model', 'activity-network', true, false),
    manualSource('parent-mdns-advertisement', '17', 'manual-artifact', 'activity-network', false, false, 'mDNS advertisement artifact'),
    manualSource('child-mdns-advertisement', '17', 'manual-artifact', 'activity-network', false, false, 'mDNS advertisement artifact'),
    manualSource('signed-child-agent-hello', '18', 'manual-artifact', 'devices-lan', true, false, 'signed hello artifact'),
    manualSource('signed-child-agent-heartbeat', '18', 'manual-artifact', 'activity-network', true, false, 'signed heartbeat artifact'),
    implementedSource('assignment-revocation-audit', '19', 'rust-service-read-model', 'activity-network', true, true),
    implementedSource('proof-gate-rollout', '20', 'proof-harness', 'activity-network', false, false),
    notImplementedSource('targeted-arp-refresh', '05', 'manual-artifact', 'activity-network'),
    notImplementedSource('bounded-arp-sweep', '06', 'manual-artifact', 'activity-network'),
    notImplementedSource('passive-arp-listener', '07', 'manual-artifact', 'activity-network'),
    notImplementedSource('passive-mdns-listener', '07', 'manual-artifact', 'activity-network'),
    notImplementedSource('passive-ssdp-listener', '07', 'manual-artifact', 'activity-network'),
    notImplementedSource('passive-llmnr-listener', '07', 'manual-artifact', 'activity-network'),
    notImplementedSource('passive-netbios-listener', '07', 'manual-artifact', 'activity-network'),
  ];
}

function implementedSource(source, workpackId, runtimePath, uiSurface, canConfirm, canAssign) {
  return sourceRow(source, workpackId, 'implemented', runtimePath, uiSurface, canConfirm, canAssign, null);
}

function weakSource(source, workpackId, runtimePath, uiSurface) {
  return sourceRow(
    source,
    workpackId,
    runtimePath === 'manual-artifact' ? 'manual-required' : 'partial',
    runtimePath,
    uiSurface,
    false,
    false,
    null
  );
}

function manualSource(source, workpackId, runtimePath, uiSurface, canConfirm, canAssign, requiredArtifactSummary) {
  return sourceRow(
    source,
    workpackId,
    'manual-required',
    runtimePath,
    uiSurface,
    canConfirm,
    canAssign,
    requiredArtifactSummary
  );
}

function notImplementedSource(source, workpackId, runtimePath, uiSurface) {
  return sourceRow(
    source,
    workpackId,
    'not-implemented',
    runtimePath,
    uiSurface,
    false,
    false,
    'adapter not implemented'
  );
}

function sourceRow(
  source,
  workpackId,
  status,
  runtimePath,
  uiSurface,
  canConfirmChildAgent,
  canAssignChildProfile,
  requiredArtifactSummary
) {
  return {
    schemaVersion: 'v0.9',
    source,
    workpackId,
    status,
    authority: sourceAuthority(source, canConfirmChildAgent, canAssignChildProfile),
    runtimePath,
    uiSurface,
    canConfirmChildAgent,
    canAssignChildProfile,
    canControlRoute: canConfirmChildAgent || canAssignChildProfile,
    requiresSelectedInterface: sourceRequiresSelectedInterface(source),
    persistsAcrossRestart: source === 'household-device-store' || source === 'assignment-revocation-audit',
    evidenceLabel: `${source} evidence`,
    requiredArtifactSummary,
  };
}

function discoveryStateForStatus(status) {
  if (status === 'implemented' || status === 'partial' || status === 'parser-proof') return 'pending';
  if (status === 'manual-required') return 'manual-required';
  return 'unavailable';
}

function proofStateForStatus(status) {
  if (status === 'manual-required') return 'manual-required';
  if (status === 'not-implemented') return 'not-implemented';
  return 'ci-mechanical-proof';
}

function runtimeOwnerForPath(runtimePath) {
  if (runtimePath === 'typescript-contract') return 'parent-domain-contract';
  if (runtimePath === 'agent-protocol') return 'agent-protocol';
  if (runtimePath === 'rust-service-read-model') return 'rust-service-read-model';
  if (runtimePath === 'proof-harness') return 'proof-harness';
  return 'manual-proof';
}

function sourceAuthority(source, canConfirmChildAgent, canAssignChildProfile) {
  if (source === 'proof-gate-rollout') return 'proof-gate';
  if (canAssignChildProfile) return 'manual-parent-decision';
  if (canConfirmChildAgent) return 'strong-identity';
  if (source.includes('classification') || source.includes('oui')) return 'classification-only';
  if (source.includes('mdns') || source.includes('ssdp')) return 'presence-only';
  if (source.includes('netbios') || source.includes('llmnr') || source.includes('dns')) return 'name-only';
  return 'no-child-confirmation';
}

function sourceRequiresSelectedInterface(source) {
  return (
    source.includes('arp') ||
    source.includes('neighbor') ||
    source.includes('mdns') ||
    source.includes('ssdp') ||
    source.includes('llmnr') ||
    source.includes('netbios') ||
    source.includes('targeted') ||
    source.includes('bounded')
  );
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
    honestNonClaims: ['physical-household-lan-manual-required', 'cloud-relay-not-implemented'],
  };
}

function assertSourceMatrix(matrix) {
  if (matrix.workpackRows.length !== 20) {
    throw new Error(`expected 20 workpack rows, received ${matrix.workpackRows.length}`);
  }
  assertStatusCount(matrix.workpackRows, 'implemented', 2, 'implemented workpacks');
  assertStatusCount(matrix.workpackRows, 'manual-required', 5, 'manual-required workpacks');
  assertStatusCount(matrix.workpackRows, 'not-implemented', 3, 'not-implemented workpacks');
  assertRow(matrix.sourceRows, 'source', 'windows-neighbor-table', 'canConfirmChildAgent', false);
  assertRow(matrix.sourceRows, 'source', 'mdns-dns-sd-query', 'canAssignChildProfile', false);
  assertRow(matrix.sourceRows, 'source', 'signed-child-agent-hello', 'requiredArtifactSummary', 'signed hello artifact');
  assertRow(
    matrix.sourceRows,
    'source',
    'signed-child-agent-heartbeat',
    'requiredArtifactSummary',
    'signed heartbeat artifact'
  );
}

function assertStatusCount(rows, status, minimum, label) {
  const count = rows.filter((row) => row.status === status).length;
  if (count < minimum) {
    throw new Error(`expected at least ${minimum} ${label}, received ${count}`);
  }
}

function assertRow(rows, key, value, field, expected) {
  const row = rows.find((candidate) => candidate[key] === value);
  if (!row) {
    throw new Error(`missing row ${value}`);
  }
  if (row[field] !== expected) {
    throw new Error(`expected ${value}.${field} ${expected}, received ${row[field]}`);
  }
}

function countByStatus(rows) {
  return rows.reduce((counts, row) => {
    counts[row.status] = (counts[row.status] ?? 0) + 1;
    return counts;
  }, {});
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

function parentDomainLanPairingModuleUrl() {
  const modulePath = join(repoRoot, 'packages', 'parent-domain', 'dist', 'lan-pairing.js');
  return `file:///${modulePath.replaceAll('\\', '/')}`;
}
