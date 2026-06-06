import { spawnSync } from 'node:child_process';
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

const proofRoot = join('output', 'network-plan-proof', '51b-product-readiness-service-status');
const testRoot = join('test-results', 'network-product-readiness-service-status-proof');
const boundaryPath = join(proofRoot, '51b-product-readiness-service-status.json');
const proofSummaryPath = join(proofRoot, 'proof-summary.json');
const testProofPath = join(testRoot, 'proof.json');

mkdirSync(proofRoot, { recursive: true });
mkdirSync(testRoot, { recursive: true });

const boundary = {
  reportRef: 'network-product-readiness-service-status-row51b',
  command: 'agent.network.product-readiness.status.get',
  event: 'agent.network.product-readiness.status.reported',
  payloadFields: ['networkLiveCaptureCustodyStatus', 'networkProductReadinessStatus'],
  composedRows: ['13a live-capture custody status materializer', '51a product-readiness status materializer'],
  serviceBoundary:
    'The Rust service exposes existing network evidence materializer outputs through a typed WebSocket status event. It does not render portal UI, execute adapters, claim policy authority, or publish enforcement commands.',
  rejectedClaims: [
    'exact URL from network-only evidence',
    'page content',
    'private message',
    'search query',
    'decrypted payload',
    'production SLO',
    'portal policy authority',
    'portal adapter dispatch',
    'live adapter execution',
    'live capture driver invocation',
    'raw artifact creation',
    'enforcement command publication',
  ],
};

writeFileSync(boundaryPath, `${JSON.stringify(boundary, null, 2)}\n`);

const commands = [
  {
    name: 'service-network-product-readiness-status-tests',
    command: 'cargo',
    args: ['test', '-p', 'ocentra-parent-agent-service', 'network_product_readiness_status'],
    log: join(proofRoot, 'service-network-product-readiness-status-tests.log'),
  },
  {
    name: 'agent-protocol-product-readiness-status-command-test',
    command: 'cargo',
    args: ['test', '-p', 'ocentra-parent-agent-protocol', 'network_product_readiness_status_command'],
    log: join(proofRoot, 'agent-protocol-product-readiness-status-command-test.log'),
  },
  {
    name: 'agent-protocol-domain-contract-tests',
    command: npmCommand(),
    args: npmArgs(['--workspace', '@ocentra-parent/agent-protocol-domain', 'run', 'test', '--', 'contracts.test.ts']),
    log: join(proofRoot, 'agent-protocol-domain-contract-tests.log'),
  },
  {
    name: 'network-evidence-live-capture-custody-status-tests',
    command: 'cargo',
    args: ['test', '-p', 'ocentra-network-evidence', 'live_capture_custody_status'],
    log: join(proofRoot, 'network-evidence-live-capture-custody-status-tests.log'),
  },
  {
    name: 'network-evidence-product-readiness-status-tests',
    command: 'cargo',
    args: ['test', '-p', 'ocentra-network-evidence', 'product_readiness_status'],
    log: join(proofRoot, 'network-evidence-product-readiness-status-tests.log'),
  },
  {
    name: 'agent-service-clippy',
    command: 'cargo',
    args: ['clippy', '-p', 'ocentra-parent-agent-service', '--all-targets', '--', '-D', 'warnings'],
    log: join(proofRoot, 'agent-service-clippy.log'),
  },
  {
    name: 'source-shape',
    command: 'node',
    args: ['scripts/check-source-shape.mjs'],
    log: join(proofRoot, 'source-shape.log'),
  },
  {
    name: 'diff-check',
    command: 'git',
    args: ['diff', '--check'],
    log: join(proofRoot, 'diff-check.log'),
  },
];

const commandResults = commands.map(runCommand);
assertSourceContracts();

const checkedAt = new Date().toISOString();
const proof = {
  proof: 'network-product-readiness-service-status',
  checkedAt,
  branch: runText('git', ['branch', '--show-current']).trim(),
  commit: runText('git', ['rev-parse', 'HEAD']).trim(),
  originMain: runText('git', ['rev-parse', 'origin/main']).trim(),
  mergeBase: runText('git', ['merge-base', 'HEAD', 'origin/main']).trim(),
  sourceStatusShort: sourceStatusShort(),
  proofRoot,
  testRoot,
  commands: commandResults,
  artifacts: {
    boundary: boundaryPath,
    proofSummary: proofSummaryPath,
    testProof: testProofPath,
  },
  rowsCovered: [
    'network-plan row 13a service WebSocket exposure supplement',
    'network-plan row 51a service WebSocket exposure supplement',
    'network-plan row 51b product readiness service status',
  ],
  claimsProved: [
    'agent-protocol-domain and agent-protocol expose a typed WebSocket command and report event for network product-readiness status',
    'the Rust service command handler routes agent.network.product-readiness.status.get to a service-backed report event',
    'the service payload serializes row13a live-capture custody status and row51a product-readiness status from ocentra-network-evidence materializers',
    'the live-capture status reports no driver invocation, no raw artifact creation, no remote upload, no exact URL/content, and zero enforcement commands',
    'the product-readiness status carries portal read-model and retention/export refs while keeping UI policy authority, portal adapter dispatch, live adapter execution, and enforcement commands false',
  ],
  claimsNotProved: [
    'parent portal rendering of the product-readiness status',
    'production SLO validation',
    'live capture driver invocation or raw artifact creation',
    'exact URL, page content, private message, search query, or decrypted payload',
    'full policy engine execution',
    'adapter execution, host filtering, live platform mutation, or rollback execution',
    'broker or family-hub delivery',
  ],
};

writeFileSync(proofSummaryPath, `${JSON.stringify(proof, null, 2)}\n`);
writeFileSync(testProofPath, `${JSON.stringify(proof, null, 2)}\n`);
console.log(
  'network-product-readiness-service-status-proof-ok:service,protocol,ts,evidence,clippy,source-shape,diff-check'
);
console.log(`proof=${proofSummaryPath}`);

function assertSourceContracts() {
  const contracts = readText('packages/agent-protocol-domain/src/contracts.ts');
  const defaults = readText('packages/agent-protocol-domain/src/defaults.ts');
  const rustTransport = readText('crates/agent-protocol/src/transport.rs');
  const fieldConstants = readText('crates/agent-protocol/src/constants/field.rs');
  const serviceWebSocket = readText('crates/agent-service/src/websocket.rs');
  const payload = readText('crates/agent-service/src/network_product_readiness_status_payload.rs');
  const serviceTests = readText('crates/agent-service/src/network_product_readiness_status_service_tests.rs');
  const custodyStatus = readText('crates/ocentra-network-evidence/src/live_capture_custody_status.rs');
  const productStatus = readText('crates/ocentra-network-evidence/src/product_readiness_status.rs');

  assertIncludes(contracts, 'agent.network.product-readiness.status.get', 'TypeScript command');
  assertIncludes(contracts, 'agent.network.product-readiness.status.reported', 'TypeScript event');
  assertIncludes(defaults, 'networkProductReadinessStatus', 'TypeScript product status field');
  assertIncludes(defaults, 'networkLiveCaptureCustodyStatus', 'TypeScript custody status field');
  assertIncludes(rustTransport, 'AgentNetworkProductReadinessStatusGet', 'Rust command enum');
  assertIncludes(rustTransport, 'AgentNetworkProductReadinessStatusReported', 'Rust event enum');
  assertIncludes(fieldConstants, 'NETWORK_PRODUCT_READINESS_STATUS', 'Rust product status field constant');
  assertIncludes(fieldConstants, 'NETWORK_LIVE_CAPTURE_CUSTODY_STATUS', 'Rust custody status field constant');
  assertIncludes(
    serviceWebSocket,
    'AgentCommandName::AgentNetworkProductReadinessStatusGet',
    'WebSocket dispatch routes status command'
  );
  assertIncludes(payload, 'build_network_product_readiness_status_report', 'service report builder exists');
  assertIncludes(payload, 'materialize_network_product_readiness_status', 'service payload uses product materializer');
  assertIncludes(
    payload,
    'materialize_network_live_capture_custody_status',
    'service payload uses custody materializer'
  );
  assertIncludes(
    serviceTests,
    'websocket_network_product_readiness_status_command_reports_payload',
    'WebSocket command test covers payload'
  );
  assertIncludes(custodyStatus, 'Serialize, Deserialize', 'custody status is serializable for service payload');
  assertIncludes(
    productStatus,
    'PortalAdapterDispatchClaimRejected',
    'product status rejects portal adapter dispatch claims'
  );
}

function npmCommand() {
  return process.platform === 'win32' ? 'cmd' : 'npm';
}

function npmArgs(args) {
  return process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
}

function readText(path) {
  return readFileSync(path, 'utf8');
}

function assertIncludes(text, expected, label) {
  if (!text.includes(expected)) {
    throw new Error(`${label}: missing ${expected}`);
  }
}

function runCommand(entry) {
  const result = spawnSync(entry.command, entry.args, { encoding: 'utf8', shell: false });
  writeFileSync(entry.log, `${result.stdout ?? ''}${result.stderr ?? ''}`);
  if (result.status !== 0) {
    throw new Error(`${entry.name} failed with exit ${result.status}`);
  }
  return {
    name: entry.name,
    command: [entry.command, ...entry.args].join(' '),
    status: result.status,
    log: entry.log,
  };
}

function runText(command, args) {
  const result = spawnSync(command, args, { encoding: 'utf8', shell: false });
  if (result.status !== 0) {
    throw new Error(`${command} ${args.join(' ')} failed with exit ${result.status}`);
  }
  return `${result.stdout ?? ''}${result.stderr ?? ''}`;
}

function sourceStatusShort() {
  return runText('git', ['status', '--short'])
    .split(/\r?\n/u)
    .filter((line) => {
      if (line.trim().length === 0) {
        return false;
      }
      const filePath = line.slice(3).replaceAll('\\', '/');
      return (
        !filePath.startsWith('output/network-plan-proof/51b-product-readiness-service-status/') &&
        !filePath.startsWith('test-results/network-product-readiness-service-status-proof/')
      );
    })
    .join('\n');
}
