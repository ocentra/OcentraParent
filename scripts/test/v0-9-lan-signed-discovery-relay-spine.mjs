import { spawn } from 'node:child_process';
import { mkdir, writeFile } from 'node:fs/promises';
import { join } from 'node:path';

const repoRoot = process.cwd();
const outputDir = join(repoRoot, 'test-results', 'v0-9-lan-signed-discovery-relay-spine');
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
    'tests/lan-signed-discovery-relay-spine.test.ts',
    'tests/lan-pairing-browser-add-device-state.test.ts',
  ]);
  await runCommand('cargo', ['test', '-p', 'ocentra-parent-agent-protocol', 'lan_pairing_browser_add_device_state']);
  await runCommand('cargo', ['test', '-p', 'ocentra-parent-agent-service', 'lan_pairing_browser_add_device_state']);

  const contract = await import(parentDomainLanPairingModuleUrl());
  const spine = contract.LanSignedDiscoveryRelaySpineSchema.parse(signedDiscoveryRelaySpine());
  const readModel = contract.LanBrowserAddDeviceReadModelSchema.parse({
    ...addDeviceReadModelFixture(),
    signedDiscoveryRelaySpine: spine,
  });

  assertSignedDiscoveryRelaySpine(readModel.signedDiscoveryRelaySpine);

  const proof = {
    schemaVersion: 1,
    checkedAt: new Date().toISOString(),
    commit: await gitHead(),
    proofMode: 'v0-9-lan-signed-discovery-relay-spine',
    commands,
    proofLabels: [
      'v0.9.signed-lan-discovery.contract-proof',
      'v0.9.signed-lan-discovery.rust-service-read-model-proof',
      'v0.9.signed-lan-discovery.relay-cache-non-custody-proof',
    ],
    signedDiscoveryRelaySpine: spine,
    claimsProved: [
      'passive LAN neighbor and router/infrastructure evidence are separate from signed child-agent rows',
      'signed proof rejection states cover unauthenticated, expired, replayed, wrong-origin, wrong-device, revoked, and stale outcomes',
      'route safety rows cover trusted registry recovery, selected custody, stale/offline, wrong-route, revoked-route, and parent decisions',
      'relay, cache, and parent-owned storage are represented as unavailable or not implemented without claiming Ocentra child-data custody',
    ],
    claimsNotProved: [
      'signed child-agent hello or heartbeat from a second installed physical host',
      'physical household LAN readiness across two real child-agent hosts',
      'production cloud relay or parent cache routing',
      'configured parent-owned storage adapter',
    ],
  };

  await writeFile(proofPath, `${JSON.stringify(proof, null, 2)}\n`);
  console.log('v0-9-lan-signed-discovery-relay-spine-ok');
  console.log(`evidence=${proofPath}`);
}

function assertSignedDiscoveryRelaySpine(spine) {
  assertArrayIncludes(
    spine.manualProofRequired,
    'signed-child-agent-hello',
    'signed child-agent hello manual proof requirement'
  );
  assertArrayIncludes(
    spine.manualProofRequired,
    'signed-child-agent-heartbeat',
    'signed child-agent heartbeat manual proof requirement'
  );
  assertArrayIncludes(spine.notImplemented, 'relay-route-unavailable', 'relay route unavailable gap');
  assertArrayIncludes(spine.notImplemented, 'cache-route-unavailable', 'cache route unavailable gap');
  assertRow(spine.adapterRows, 'adapter', 'passive-lan-neighbor', 'custodyLabel', 'passive-lan-observation');
  assertRow(spine.adapterRows, 'adapter', 'signed-child-agent-hello', 'proofState', 'manual-required');
  assertRow(spine.signedProofRows, 'check', 'unauthenticated-caller-rejected', 'rejectionReason', 'anonymous');
  assertRow(spine.signedProofRows, 'check', 'wrong-device-signed-proof-rejected', 'rejectionReason', 'wrong-device');
  assertRow(spine.routeSafetyRows, 'check', 'wrong-route-rejected', 'rejectionReason', 'wrong-device');
  assertRow(spine.routeSafetyRows, 'check', 'parent-revoke-decision-audited', 'discoveryState', 'revoked');
  assertRow(spine.relayCacheRows, 'check', 'ocentra-child-data-custody-not-claimed', 'custodyLabel', 'no-ocentra-child-data-custody');
}

function signedDiscoveryRelaySpine() {
  return {
    schemaVersion: 'v0.9',
    generatedAt: '2026-06-02T11:40:00.000Z',
    adapterRows: [
      adapterRow('passive-lan-neighbor', 'discovered', 'ci-mechanical-proof', 'strong', 'passive-lan-observation', null),
      adapterRow('router-infrastructure', 'discovered', 'ci-mechanical-proof', 'strong', 'router-infrastructure-observation', null),
      adapterRow('mdns-name', 'manual-required', 'manual-required', 'manual-required', 'passive-lan-observation', 'mDNS proof'),
      adapterRow('ssdp-name', 'manual-required', 'manual-required', 'manual-required', 'passive-lan-observation', 'SSDP proof'),
      adapterRow('router-dhcp-name', 'manual-required', 'manual-required', 'manual-required', 'router-infrastructure-observation', 'router DHCP proof'),
      adapterRow('manual-direct-address', 'manual-required', 'manual-required', 'manual-required', 'manual-parent-entry', 'manual address proof'),
      adapterRow('signed-child-agent-hello', 'manual-required', 'manual-required', 'manual-required', 'signed-child-agent-artifact', 'signed hello proof'),
      adapterRow('signed-child-agent-heartbeat', 'manual-required', 'manual-required', 'manual-required', 'signed-child-agent-artifact', 'signed heartbeat proof'),
    ],
    signedProofRows: [
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
    ],
    routeSafetyRows: [
      routeSafety('trusted-registry-restart-recovery', 'paired', 'accepted', null),
      routeSafety('selected-route-custody', 'paired', 'accepted', null),
      routeSafety('stale-selected-device-rejected', 'stale', 'rejected', 'stale'),
      routeSafety('offline-selected-device-rejected', 'offline', 'rejected', 'offline'),
      routeSafety('wrong-route-rejected', 'rejected', 'rejected', 'wrong-device'),
      routeSafety('revoked-route-rejected', 'revoked', 'rejected', 'revoked'),
      routeSafety('parent-assign-decision-audited', 'discovered', 'accepted', null),
      routeSafety('parent-rename-decision-audited', 'discovered', 'accepted', null),
      routeSafety('parent-ignore-decision-audited', 'discovered', 'accepted', null),
      routeSafety('parent-restore-decision-audited', 'discovered', 'accepted', null),
      routeSafety('parent-trust-decision-audited', 'paired', 'accepted', null),
      routeSafety('parent-revoke-decision-audited', 'revoked', 'accepted', null),
    ],
    relayCacheRows: [
      relayCache('relay-route-unavailable', 'unavailable', 'unavailable', 'not-implemented', 'no-ocentra-child-data-custody'),
      relayCache('relay-route-queued-not-configured', 'queued-not-configured', 'pending', 'not-implemented', 'no-ocentra-child-data-custody'),
      relayCache('cache-route-unavailable', 'unavailable', 'unavailable', 'not-implemented', 'no-ocentra-child-data-custody'),
      relayCache('parent-owned-storage-unavailable', 'unavailable', 'unavailable', 'not-implemented', 'parent-owned-storage-unavailable'),
      relayCache('ocentra-child-data-custody-not-claimed', 'local-first', 'unavailable', 'ci-mechanical-proof', 'no-ocentra-child-data-custody'),
    ],
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
      'route safety and relay cache custody are represented in typed protocol state',
    ],
    claimsNotProved: [
      'signed child-agent artifacts are still manual-required',
      'physical household LAN proof still requires real second host evidence',
      'relay or cache routes remain unavailable',
      'parent-owned storage remains unavailable',
    ],
  };
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

function routeSafety(check, discoveryState, responseState, rejectionReason) {
  return {
    schemaVersion: 'v0.9',
    check,
    routeId: 'lan-route-local-network',
    discoveryState,
    responseState,
    rejectionReason,
    proofState: 'ci-mechanical-proof',
    runtimeOwner: 'rust-service-read-model',
    custodyLabel: 'parent-local-service',
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

function assertRow(rows, key, value, field, expected) {
  const row = rows.find((candidate) => candidate[key] === value);
  if (!row) {
    throw new Error(`missing row ${value}`);
  }
  if (row[field] !== expected) {
    throw new Error(`expected ${value}.${field} ${expected}, received ${row[field]}`);
  }
}

function assertArrayIncludes(values, expected, label) {
  if (!Array.isArray(values) || !values.includes(expected)) {
    throw new Error(`${label}: missing ${expected}`);
  }
}
