import { spawnSync } from 'node:child_process';
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

const proofRoot = join('output', 'network-plan-proof', '04-typescript-event-parity');
const testRoot = join('test-results', 'eventing-network-ts-event-parity-proof');
mkdirSync(proofRoot, { recursive: true });
mkdirSync(testRoot, { recursive: true });

const commands = [
  {
    name: 'agent-protocol-network-runtime-event-tests',
    command: npmCommand(),
    args: npmArgs([
      '--workspace',
      '@ocentra-parent/agent-protocol-domain',
      'run',
      'test',
      '--',
      'network-runtime-events',
    ]),
    log: join(proofRoot, 'agent-protocol-network-runtime-event-tests.log'),
  },
  {
    name: 'agent-protocol-domain-build',
    command: npmCommand(),
    args: npmArgs(['--workspace', '@ocentra-parent/agent-protocol-domain', 'run', 'build']),
    log: join(proofRoot, 'agent-protocol-domain-build.log'),
  },
  {
    name: 'source-shape',
    command: 'node',
    args: ['scripts/check-source-shape.mjs', 'crates/ocentra-eventing'],
    log: join(proofRoot, 'source-shape.log'),
  },
];

const commandResults = commands.map(runCommand);
assertSourceContracts();
const publicImport = await assertPublicImport();

const proof = {
  proof: 'eventing-network-ts-event-parity',
  checkedAt: new Date().toISOString(),
  branch: runText('git', ['branch', '--show-current']).trim(),
  commit: runText('git', ['rev-parse', 'HEAD']).trim(),
  statusShort: runText('git', ['status', '--short']),
  proofRoot,
  testRoot,
  commands: commandResults,
  artifacts: {
    proofSummary: join(proofRoot, 'proof-summary.json'),
    testProof: join(testRoot, 'proof.json'),
  },
  publicImport,
  rowsCovered: [
    'network-plan row 04 TypeScript package parity sub-slice',
    'network-plan row 10 TypeScript event parity sub-slice',
    'eventing-plan rows 45-50 TypeScript public parity supplement',
  ],
  claimsProved: [
    'agent-protocol-domain publicly exports network runtime event contracts through ./network-runtime-events',
    'TypeScript Effect Schema contracts cover the eleven Rust protocol-facing network runtime event payload shapes',
    'event-type constants match the Rust network flow event type constants including portal.read_model.updated',
    'network claim-boundary parsing rejects exact URL, decrypted payload, message content, search query, and adapter-action claims',
    'network AI request parsing rejects raw packet payload inclusion',
    'network enforcement result parsing rejects adapter-action execution claims',
  ],
  claimsNotProved: [
    'broker-backed delivery, relay-hub delivery, or service WebSocket streaming of runtime event chains',
    'host DNS/filter mutation, firewall mutation, WFP/NetworkExtension/VpnService/nftables adapter execution, or enforcement-command execution',
    'portal UI rendering of network runtime event-chain payloads',
    'production retention, replay, delete/export, offset, dedupe, or cross-process durable event delivery',
  ],
};

writeFileSync(join(proofRoot, 'proof-summary.json'), `${JSON.stringify(proof, null, 2)}\n`);
writeFileSync(join(testRoot, 'proof.json'), `${JSON.stringify(proof, null, 2)}\n`);
console.log('eventing-network-ts-event-parity-proof-ok:tests,build,public-import,source-shape');
console.log(`proof=${join(proofRoot, 'proof-summary.json')}`);

function assertSourceContracts() {
  const packageJson = readText('packages/agent-protocol-domain/package.json');
  const source = readText('packages/agent-protocol-domain/src/network-runtime-events.ts');
  const tests = readText('packages/agent-protocol-domain/tests/network-runtime-events.test.ts');
  const rustContracts = readText('crates/agent-protocol/src/network_flow_events.rs');
  const rustConstants = readText('crates/agent-protocol/src/constants/network_flow.rs');
  const readme = readText('packages/agent-protocol-domain/README.md');

  assertIncludes(packageJson, '"./network-runtime-events"', 'public package export exists');
  assertIncludes(source, 'AgentNetworkFlowObservedEventSchema', 'network flow observed schema exists');
  assertIncludes(source, 'AgentNetworkDomainObservedEventSchema', 'network domain observed schema exists');
  assertIncludes(source, 'AgentNetworkActivityClassifiedEventSchema', 'network classification schema exists');
  assertIncludes(source, 'AgentNetworkAiAnalysisRequestedEventSchema', 'network AI request schema exists');
  assertIncludes(source, 'AgentNetworkPolicyDecisionCompletedEventSchema', 'network policy decision schema exists');
  assertIncludes(
    source,
    'AgentNetworkEnforcementCommandIssuedEventSchema',
    'network enforcement command schema exists'
  );
  assertIncludes(source, 'AgentNetworkAuditEntryCommittedEventSchema', 'network audit schema exists');
  assertIncludes(source, 'AgentNetworkPortalReadModelUpdatedEventSchema', 'network portal schema exists');
  assertIncludes(source, 'Network runtime events cannot claim exact URL', 'no-content claim-boundary filter exists');
  assertIncludes(source, 'Network AI events cannot include raw packet payloads', 'raw packet payload filter exists');
  assertIncludes(source, 'Network enforcement result cannot claim adapter action', 'adapter action filter exists');
  assertIncludes(tests, 'portal.read_model.updated', 'portal event type parity is asserted');
  assertIncludes(rustContracts, 'pub struct NetworkPortalReadModelUpdatedEvent', 'Rust portal contract exists');
  assertIncludes(rustConstants, 'EVENT_PORTAL_READ_MODEL_UPDATED', 'Rust portal event constant exists');
  assertIncludes(readme, 'Network runtime event parsing proves public TypeScript parity', 'README records boundary');
}

async function assertPublicImport() {
  const publicModule = await import('@ocentra-parent/agent-protocol-domain/network-runtime-events');
  const eventType = publicModule.AgentNetworkRuntimeEventType.NetworkFlowObserved;
  const result = publicModule.parseAgentNetworkRuntimeEvent({
    eventType,
    payload: {
      schemaVersion: publicModule.AgentNetworkRuntimeEventSchemaVersion,
      flowEventRef: 'event.network.flow.observed.import-proof',
      observedAt: '2026-06-05T06:40:00Z',
      deviceRef: 'device.child.windows-import-proof',
      flowEvidenceRef: 'evidence.network.flow.import-proof',
      custody: 'child-device-query-store',
      evidenceGrade: 'A',
      claimBoundary: {
        exactUrlAvailable: false,
        decryptedHttpsPayloadAvailable: false,
        messageContentAvailable: false,
        searchQueryAvailable: false,
        adapterActionExecuted: false,
      },
    },
  });
  if (!result.ok || result.eventType !== 'network.flow.observed') {
    throw new Error('public network-runtime-events import did not parse network.flow.observed');
  }
  return {
    exportPath: '@ocentra-parent/agent-protocol-domain/network-runtime-events',
    eventType,
    parsed: result.ok,
  };
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
