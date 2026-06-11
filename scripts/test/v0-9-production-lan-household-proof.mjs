import { spawn } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { join } from 'node:path';

const repoRoot = process.cwd();
const outputDir = join(repoRoot, 'test-results', 'v0-9-production-lan-household-proof');
const proofPath = join(outputDir, 'proof.json');
const commands = [];

await main();

async function main() {
  await mkdir(outputDir, { recursive: true });

  await runCommand(...npmCommand(['run', 'build:contracts']));
  await runCommand(
    ...npmCommand([
      'run',
      'test',
      '--workspace',
      '@ocentra-parent/parent-domain',
      '--',
      'tests/lan-production-household-proof.test.ts',
      'tests/lan-pairing-browser-add-device-state.test.ts',
      'tests/household-device-spine.test.ts',
    ])
  );
  await runCommand(
    ...npmCommand([
      'run',
      'test',
      '--workspace',
      '@ocentra-parent/agent-protocol-domain',
      '--',
      'tests/lan-pairing-browser-add-device-state.test.ts',
    ])
  );
  await runCommand('cargo', ['test', '-p', 'ocentra-parent-agent-protocol', 'lan_pairing_browser_add_device_state']);
  await runCommand('cargo', ['test', '-p', 'ocentra-parent-agent-service', 'lan_pairing_browser_add_device_state']);

  const contract = await import(parentDomainLanPairingModuleUrl());
  const readModel = contract.LanBrowserAddDeviceReadModelSchema.parse(addDeviceReadModelFixture());
  const productionHouseholdProof = contract.LanProductionHouseholdProofSummarySchema.parse(
    readModel.productionHouseholdProof
  );

  assertProductionHouseholdProof(productionHouseholdProof);
  assertCanonicalMergedState(readModel);

  const proof = {
    schemaVersion: 1,
    checkedAt: new Date().toISOString(),
    commit: await gitHead(),
    proofMode: 'v0-9-production-lan-household-proof',
    commands,
    proofLabels: [
      'v0.9.production-lan-household.contract-proof',
      'v0.9.production-lan-household.rust-service-read-model-proof',
      'v0.9.production-lan-household.manual-boundaries-preserved',
    ],
    productionHouseholdProof,
    claimsProved: [
      'merged LAN add-device read model includes typed production household proof state',
      'passive neighbor and router/infrastructure discovery state stay separate from controllable child-agent targets',
      'trusted registry, parent decisions, route custody, stale/offline selected-device, and revocation states remain machine-checked through contract and Rust service tests',
    ],
    claimsNotProved: [
      'physical household LAN readiness across two distinct real child-agent hosts',
      'signed LAN hello and heartbeat from a second installed child agent',
      'mDNS, SSDP, or router DHCP name discovery from real household network artifacts',
      'cloud relay route, cache route, or Ocentra-hosted child activity custody',
      'Android child-agent parity, iOS child-agent parity, and production store signing',
    ],
  };

  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  console.log('v0-9-production-lan-household-proof-ok');
  console.log(`evidence=${proofPath}`);
}

function assertProductionHouseholdProof(proof) {
  assertArrayIncludes(proof.manualProofRequired, 'signed-lan-hello', 'signed LAN hello manual gate');
  assertArrayIncludes(proof.manualProofRequired, 'signed-lan-heartbeat', 'signed LAN heartbeat manual gate');
  assertArrayIncludes(proof.manualProofRequired, 'second-physical-child-agent', 'second child-agent manual gate');
  assertArrayIncludes(proof.manualProofRequired, 'android-child-agent-parity', 'Android parity manual gate');
  assertArrayIncludes(proof.manualProofRequired, 'ios-child-agent-parity', 'iOS parity manual gate');
  assertArrayIncludes(proof.manualProofRequired, 'store-signing', 'store signing manual gate');
  assertArrayIncludes(proof.notImplemented, 'relay-route', 'relay route non-implementation');
  assertArrayIncludes(proof.notImplemented, 'cache-route', 'cache route non-implementation');
  assertStatus(proof, 'passive-neighbor-discovery', 'ci-mechanical-proof');
  assertStatus(proof, 'route-custody', 'ci-mechanical-proof');
  assertStatus(proof, 'relay-route', 'not-implemented');
  assertStatus(proof, 'cache-route', 'not-implemented');
}

function assertCanonicalMergedState(readModel) {
  const childAgent = readModel.canonicalHouseholdDevices.find((device) => device.classification === 'child-agent');
  if (!childAgent) {
    throw new Error('merged LAN read model did not include a child-agent row');
  }
  assertArrayIncludes(childAgent.sourceLabels, 'network-neighbor', 'child-agent network neighbor source');
  assertArrayIncludes(childAgent.sourceLabels, 'trusted-registry', 'child-agent trusted registry source');
  assertArrayIncludes(childAgent.policyTargetSurfaces, 'policy', 'child-agent policy target');
  const router = readModel.canonicalHouseholdDevices.find(
    (device) => device.classification === 'network-infrastructure'
  );
  if (!router || router.enrollable || router.childAgentInventory !== null) {
    throw new Error('router/infrastructure row must stay visible but non-enrollable');
  }
}

function assertStatus(proof, capability, proofState) {
  const row = proof.statusRows.find((candidate) => candidate.capability === capability);
  if (!row) {
    throw new Error(`missing production household proof row ${capability}`);
  }
  if (row.proofState !== proofState) {
    throw new Error(`expected ${capability} proofState ${proofState}, received ${row.proofState}`);
  }
}

function addDeviceReadModelFixture() {
  return {
    schemaVersion: 'v0.9',
    generatedAt: '2026-06-02T09:40:00.000Z',
    discoverySource: 'physical-household-lan',
    addDeviceState: 'pending',
    localServiceDiscoveryState: 'pending',
    physicalHouseholdLanState: 'discovered',
    cloudRelayState: 'unavailable',
    scanSummary: {
      schemaVersion: 'v0.9',
      sourceLabels: ['local-service', 'windows-neighbor-table'],
      scannedDeviceCount: 2,
      agentDeviceCount: 1,
      passiveDeviceCount: 1,
      infrastructureDeviceCount: 1,
      unsupportedDeviceCount: 0,
    },
    discoveredDevices: [],
    canonicalHouseholdDevices: [canonicalChildAgentDevice(), canonicalRouterDevice()],
    pairingRequests: [],
    trustedDeviceRegistry: [trustedDeviceRegistryEntry()],
    householdDeviceDecisions: [householdDecision('rename'), householdDecision('ignore')],
    productionHouseholdProof: productionHouseholdProof(),
    trustedDeviceIds: ['child-device-1'],
    revokedDeviceIds: [],
    selectedDeviceReadiness: {
      schemaVersion: 'v0.9',
      selectedChildDeviceId: 'child-device-1',
      routeId: 'lan-route-local-network',
      pairingId: 'pairing-child-device-1',
      trustState: 'paired',
      reachability: 'online',
      readyForControl: true,
      staleAt: null,
      offlineAt: null,
    },
    controllerAuthority: 'active-controller',
    observerAuthority: 'observer',
    routeRequirementLabels: ['allowed-origin', 'target-device-match', 'non-replayed-intent'],
    auditCheckLabels: ['wrong-origin', 'wrong-device', 'replayed', 'stale', 'revoked'],
    honestNonClaims: ['physical-household-lan-manual-required', 'cloud-relay-not-implemented'],
  };
}

function productionHouseholdProof() {
  return {
    schemaVersion: 'v0.9',
    generatedAt: '2026-06-02T09:40:00.000Z',
    statusRows: [
      manualStatus('signed-lan-hello'),
      manualStatus('signed-lan-heartbeat'),
      ciStatus('passive-neighbor-discovery', 'discovered'),
      ciStatus('router-neighbor-discovery', 'discovered'),
      manualStatus('mdns-name-discovery'),
      manualStatus('ssdp-name-discovery'),
      manualStatus('router-dhcp-name-discovery'),
      ciStatus('trusted-registry', 'paired'),
      ciStatus('parent-assignment', 'discovered'),
      ciStatus('parent-rename', 'discovered'),
      ciStatus('parent-ignore', 'discovered'),
      ciStatus('parent-revocation', 'revoked'),
      ciStatus('route-custody', 'paired'),
      ciStatus('stale-selected-device', 'stale'),
      ciStatus('offline-selected-device', 'offline'),
      notImplementedStatus('relay-route'),
      notImplementedStatus('cache-route'),
      manualStatus('second-physical-child-agent'),
      manualStatus('android-child-agent-parity'),
      manualStatus('ios-child-agent-parity'),
      manualStatus('store-signing'),
    ],
    manualProofRequired: [
      'signed-lan-hello',
      'signed-lan-heartbeat',
      'mdns-name-discovery',
      'ssdp-name-discovery',
      'router-dhcp-name-discovery',
      'second-physical-child-agent',
      'android-child-agent-parity',
      'ios-child-agent-parity',
      'store-signing',
    ],
    notImplemented: ['relay-route', 'cache-route'],
    claimsProved: [
      'passive Windows neighbor evidence is represented in typed LAN read-model state',
      'trusted registry, route custody, stale/offline, and parent decisions are represented in typed LAN read-model state',
    ],
    claimsNotProved: [
      'physical household LAN readiness remains manual-required until two physical child-agent hosts and router/firewall artifacts are attached',
      'signed LAN hello and heartbeat remain manual-required until a second installed child agent signs them',
      'cloud relay routing storage and authentication are not implemented in this LAN proof',
      'Android child-agent parity remains manual-required until real device permission and transport artifacts are attached',
      'iOS child-agent parity remains manual-required until entitlement device and transport artifacts are attached',
      'store signing remains manual-required until signing store and release artifacts are attached',
    ],
  };
}

function ciStatus(capability, discoveryState) {
  return status(capability, discoveryState, 'ci-mechanical-proof', 'rust-service-read-model', null);
}

function manualStatus(capability) {
  return status(
    capability,
    'manual-required',
    'manual-required',
    'manual-proof',
    `${capability} requires a real household artifact`
  );
}

function notImplementedStatus(capability) {
  return status(capability, 'unavailable', 'not-implemented', 'manual-proof', null);
}

function status(capability, discoveryState, proofState, runtimeOwner, requiredArtifactSummary) {
  return {
    schemaVersion: 'v0.9',
    capability,
    discoveryState,
    proofState,
    runtimeOwner,
    evidenceLabel: `${capability} proof row`,
    requiredArtifactSummary,
  };
}

function canonicalChildAgentDevice() {
  return {
    schemaVersion: 'v0.9',
    canonicalDeviceId: 'lan-physical-mac-54271e97c331',
    displayName: 'GAMEDEV',
    classification: 'child-agent',
    roleBadges: ['child-agent'],
    enrollable: true,
    discoveryState: 'paired',
    trustState: 'paired',
    routeId: 'lan-route-local-network',
    routeState: 'local-network',
    networkMode: 'local-network',
    sourceLabels: ['local-service', 'network-neighbor', 'trusted-registry'],
    networkIdentity: networkIdentity('lan-physical-mac-54271e97c331'),
    childAgentInventory: {
      deviceName: 'GAMEDEV',
      platform: 'windows',
      os: 'windows',
      cpuModel: 'AMD Ryzen 9 3900X 12-Core Processor',
      cpuCores: '12 cores / 24 logical',
      memoryTotal: '63 GiB',
      gpuModel: 'GeForce RTX 2070 SUPER',
      gpuDriver: '456.71',
      gpuMemory: '8192 MiB',
      nvidiaSmi: 'GeForce RTX 2070 SUPER driver 456.71 8192 MiB VRAM',
      networkInterfaces: ['Ethernet 2'],
      capabilities: ['direct-websocket', 'device-inventory', 'pairing-route'],
      roleState: 'implemented',
      routeState: 'local-network',
      pairingTrustState: 'paired',
    },
    policyTargetSurfaces: ['devices', 'policy', 'browser', 'app', 'screen', 'network', 'activity', 'tracking', 'ai'],
  };
}

function canonicalRouterDevice() {
  return {
    schemaVersion: 'v0.9',
    canonicalDeviceId: 'lan-physical-mac-001122334455',
    displayName: 'LAN 192.168.2.1',
    classification: 'network-infrastructure',
    roleBadges: [],
    enrollable: false,
    discoveryState: 'discovered',
    trustState: 'unpaired',
    routeId: null,
    routeState: 'unavailable',
    networkMode: 'local-network',
    sourceLabels: ['network-neighbor'],
    networkIdentity: {
      ...networkIdentity('lan-physical-mac-001122334455'),
      hostname: null,
      ipAddresses: ['192.168.2.1'],
      macAddress: '00-11-22-33-44-55',
      confidence: 'network-neighbor',
    },
    childAgentInventory: null,
    policyTargetSurfaces: ['devices', 'network'],
  };
}

function networkIdentity(deviceId) {
  return {
    hostname: 'GAMEDEV',
    ipAddresses: ['192.168.2.42'],
    macAddress: '54-27-1e-97-c3-31',
    macVendor: null,
    networkInterfaces: ['Ethernet 2'],
    reachability: 'online',
    confidence: 'agent-confirmed',
    staleAt: null,
    offlineAt: null,
    evidenceRecords: [
      evidenceRecord(deviceId, 'local-service', 'ip-address', '192.168.2.42', 'ip:192.168.2.42', 'confirmed'),
      evidenceRecord(
        deviceId,
        'windows-neighbor-table',
        'mac-address',
        '54-27-1e-97-c3-31',
        'mac:54271e97c331',
        'strong'
      ),
      evidenceRecord(
        deviceId,
        'child-agent-heartbeat',
        'child-agent-presence',
        'ocentra-local-service',
        'agent:local-dev-agent',
        'manual-required'
      ),
    ],
  };
}

function evidenceRecord(deviceId, source, evidenceKind, value, mergeKey, confidence) {
  return {
    schemaVersion: 'v0.9',
    evidenceId: `lan-evidence-${source}-${evidenceKind}`,
    source,
    evidenceKind,
    deviceId,
    value,
    normalizedValue: String(value).toLowerCase(),
    firstSeenAt: '2026-06-02T09:40:00.000Z',
    lastSeenAt: '2026-06-02T09:40:00.000Z',
    expiresAt: null,
    confidence,
    mergeKey,
    note: null,
  };
}

function householdDecision(actionKind) {
  return {
    schemaVersion: 'v0.9',
    actionId: `household-action-${actionKind}`,
    actionKind,
    canonicalDeviceId: 'lan-physical-mac-54271e97c331',
    childProfileId: actionKind === 'assign' ? 'child-profile-1' : null,
    displayName: actionKind === 'rename' ? 'GAMEDEV Study PC' : null,
    parentActorId: 'parent-actor-1',
    decidedAt: '2026-06-02T09:40:00.000Z',
    revokedAt: null,
  };
}

function trustedDeviceRegistryEntry() {
  return {
    schemaVersion: 'v0.9',
    pairingId: 'pairing-child-device-1',
    childDevice: deviceRef('child-device-1', 'Mia Windows PC'),
    parentDevice: deviceRef('parent-device-1', 'Parent Windows PC'),
    routeId: 'lan-route-local-network',
    origin: 'http://127.0.0.1:4678',
    proofDigest: 'sha256:lan-proof',
    trustState: 'paired',
    trustedAt: '2026-06-02T09:40:00.000Z',
    expiresAt: '2099-06-02T09:40:00.000Z',
    revokedAt: null,
  };
}

function deviceRef(deviceId, label) {
  return {
    deviceId,
    childProfileId: null,
    label,
    platform: 'windows',
  };
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
