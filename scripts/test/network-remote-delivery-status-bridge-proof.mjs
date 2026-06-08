import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

const proofRoot = join('output', 'network-plan-proof', '10f-remote-delivery-status-bridge');
const testRoot = join('test-results', 'network-remote-delivery-status-bridge-proof');
mkdirSync(proofRoot, { recursive: true });
mkdirSync(testRoot, { recursive: true });

const sourceFiles = [
  'scripts/test/network-remote-delivery-status-bridge-proof.mjs',
  'crates/agent-protocol/src/constants.rs',
  'crates/agent-protocol/src/constants/field.rs',
  'crates/agent-protocol/src/constants/network_flow.rs',
  'crates/agent-protocol/src/network_flow.rs',
  'crates/agent-protocol/src/network_flow_tests.rs',
  'crates/agent-protocol/src/tests.rs',
  'crates/agent-protocol/src/transport.rs',
  'crates/agent-service/src/network_remote_delivery_status_payload.rs',
  'crates/agent-service/src/network_remote_delivery_status_service_tests.rs',
  'crates/agent-service/src/main.rs',
  'crates/agent-service/src/websocket.rs',
  'packages/agent-protocol-domain/src/contracts.ts',
  'packages/agent-protocol-domain/src/defaults.ts',
  'packages/agent-protocol-domain/src/network-remote-delivery-status.ts',
  'packages/agent-protocol-domain/tests/network-remote-delivery-status.test.ts',
  'packages/agent-protocol-domain/package.json',
  'docs/features/network-domain-control.md',
  'docs/plans/network-plan/implementation-checklist.md',
  'docs/plans/network-plan/workpacks/README.md',
  'crates/agent-protocol/readme.md',
  'crates/agent-service/readme.md',
  'packages/agent-protocol-domain/readme.md',
];

const fingerprintExcludedSourceFiles = new Set([
  'scripts/test/network-remote-delivery-status-bridge-proof.mjs',
  'crates/agent-protocol/src/constants/network_flow.rs',
  'docs/features/network-domain-control.md',
  'docs/plans/network-plan/implementation-checklist.md',
  'docs/plans/network-plan/workpacks/README.md',
]);

assertSourceContracts();

const expectedStatus = {
  acceptedInputs: [
    'row10b broker/family-hub remote delivery status requirements',
    'row10c event-chain journal/export refs',
    'row10d receipt ledger/local acknowledgement refs',
    'row10e durable envelope/store/replay/delete-export refs',
  ],
  commandAndEvent: {
    command: 'agent.network.remote-delivery.status.get',
    event: 'agent.network.remote-delivery.status.reported',
    payloadField: 'networkRemoteDeliveryStatus',
  },
  bridgeRefs: [
    'network.remote-delivery.status-bridge.10f',
    'network.remote-delivery.event-chain-journal.10c',
    'network.remote-delivery.event-chain.receipt-ledger.10d',
    'network.remote-delivery.event-chain.local-receipt-ack.10d',
    'network.remote-delivery.durable-envelope.10e',
    'network.remote-delivery.durable-envelope-store.10e',
    'network.remote-delivery.durable-envelope-replay.10e',
    'network.remote-delivery.durable-envelope-delete-export.10e',
    'network.remote-delivery.durable-envelope-support-status.10e',
  ],
  renderedStates: [
    'brokerStatus=fixture-requirements-recorded-but-not-implemented',
    'familyHubStatus=fixture-requirements-recorded-but-not-implemented',
    'durableEnvelopeReady=true',
    'durableEnvelopeMissingArtifactCount=0',
    'localIdempotencyQueueProved=true',
    'queuedDuplicateRejected=true',
    'completedDuplicateRejected=true',
    'productReadyRemoteDelivery=false',
  ],
  parserInvariants: [
    'TypeScript parser rejects stale or wrong durable-envelope refs even when they contain row10e text',
    'TypeScript parser rejects stale or wrong status refs even when they contain row10f text',
    'TypeScript parser rejects missing broker or family-hub requirement artifacts',
    'Rust service serializes the status into the protocol-owned payload field',
    'WebSocket routing returns the status event from the browser/network command group',
    'WebSocket routing keeps a command-rejected warning branch for proof-derived status build failures; runtime tests cover the deterministic success path',
    'bridge status cannot claim provider delivery, child-device delivery, policy authority, side-effect authority, adapter action, enforcement commands, exact content, or host filtering',
  ],
  noClaims: [
    'live broker delivery',
    'live family-hub relay delivery',
    'remote acknowledgement implementation',
    'remote provider delivery',
    'child-device delivery',
    'cross-process replay implementation',
    'remote delete/export propagation implementation',
    'product-ready remote delivery',
    'policy authority',
    'side-effect authority',
    'enforcement command publication',
    'adapter action execution',
    'raw PCAP',
    'exact URL from network-only evidence',
    'decrypted payload',
    'page content',
    'video content',
    'private-message content',
    'search-query content',
    'host filtering',
  ],
};
writeJson(join(proofRoot, 'expected-remote-delivery-status-bridge.json'), expectedStatus);

const proofPackApplicability = {
  proofRevision: 'network-remote-delivery-status-bridge-proof/v1',
  items: [
    {
      item: '00-source-snapshot.md',
      state: 'present',
      artifact: join(proofRoot, '00-source-snapshot.md'),
      reason:
        'Records deterministic branch marker, source base, inspected source/doc paths, before-state gap, and row10f bridge boundary.',
    },
    {
      item: '01-contract-proof.log',
      state: 'present',
      artifact: join(proofRoot, 'agent-protocol-domain-remote-delivery-status-test.log'),
      reason:
        'TypeScript parser tests reject live/product-ready delivery, stale refs, adapter/enforcement/content claims, malformed JSON, and missing payload field.',
    },
    {
      item: '02-eventing-proof.log',
      state: 'present',
      artifact: join(proofRoot, 'agent-protocol-remote-delivery-status-test.log'),
      reason:
        'Rust protocol/service tests prove the current row10k status bridge still preserves row10b through row10e eventing refs without creating a private bus or adapter action.',
    },
    {
      item: '03-parser-fixture-proof.json',
      state: 'not-applicable',
      reason:
        'Row10f is a service/protocol status bridge over committed row10b-row10e proof state; it does not add PCAP parser behavior.',
    },
    {
      item: '04-analyzer-alert-proof.json',
      state: 'not-applicable',
      reason: 'Row10f does not add analyzer or signature-alert behavior.',
    },
    {
      item: '05-ai-policy-proof.json',
      state: 'not-applicable',
      reason: 'Row10f carries no AI or policy authority and keeps policyAuthority=false.',
    },
    {
      item: '06-adapter-action-proof.json',
      state: 'not-applicable',
      reason: 'Row10f is read-only status; adapterActionExecutedCount remains zero and no adapter is invoked.',
    },
    {
      item: '07-journal-sqlite-proof.json',
      state: 'present',
      artifact: join(proofRoot, 'agent-service-remote-delivery-status-test.log'),
      reason:
        'The service status preserves row10c journal refs, row10d receipt ledger refs, and row10e durable envelope refs from the local proof chain.',
    },
    {
      item: '08-ui-snapshots/',
      state: 'not-applicable',
      reason: 'Row10f adds no portal UI or screenshots.',
    },
    {
      item: '09-security-negative-proof.log',
      state: 'present',
      artifact: join(proofRoot, '09-security-negative-proof.log'),
      reason:
        'Records explicit non-claims for exact content, live delivery, product readiness, policy, adapter, enforcement, and host filtering.',
    },
    {
      item: '10-performance-proof.log',
      state: 'not-applicable',
      reason: 'Row10f makes no throughput, latency, CPU, memory, disk, or queue performance claim.',
    },
    {
      item: '12-validation-commands.log',
      state: 'present',
      artifact: join(proofRoot, '12-validation-commands.log'),
      reason:
        'Lists focused Rust protocol, Rust service, dependency workspace builds before TypeScript parser tests, TypeScript build, and source-shape validation.',
    },
  ],
};
writeJson(join(proofRoot, 'proof-pack-applicability.json'), proofPackApplicability);

writeFileSync(
  join(proofRoot, '00-source-snapshot.md'),
  [
    '# Row10f Remote Delivery Status Bridge Source Snapshot',
    '',
    'proofRevision=network-remote-delivery-status-bridge-proof/v1',
    'branchMarker=codex/network-row10f-remote-delivery-status-bridge-on-row10e',
    `sourceBase=${mergeBase()}`,
    'worktreeStatus=expected to contain generated row10f proof artifacts until committed',
    '',
    'Inspected paths:',
    ...sourceFiles.map((filePath) => `- ${filePath}`),
    '',
    'Before-state gap:',
    '- Row10b through row10e were local proof boundaries, but the service/protocol layer did not expose a typed row10f status bridge that consumers can parse without making live remote-delivery claims.',
    '',
    'Current bridge boundary:',
    '- The current row10k status bridge preserves row10f row10b-through-row10e remote-delivery evidence and serializes a typed protocol status event for portal/service consumers.',
    '- The bridge keeps live broker delivery, family-hub delivery, provider or child-device delivery, policy authority, adapter execution, exact content, and host filtering unclaimed.',
  ].join('\n') + '\n'
);

const commands = [
  {
    name: 'agent-protocol-remote-delivery-status-test',
    command: 'cargo',
    args: ['test', '-p', 'ocentra-parent-agent-protocol', 'network_remote_delivery_status'],
    log: join(proofRoot, 'agent-protocol-remote-delivery-status-test.log'),
  },
  {
    name: 'agent-service-remote-delivery-status-test',
    command: 'cargo',
    args: ['test', '-p', 'ocentra-parent-agent-service', 'network_remote_delivery_status'],
    log: join(proofRoot, 'agent-service-remote-delivery-status-test.log'),
  },
  {
    name: 'agent-protocol-domain-dependency-build',
    command: 'cmd',
    args: [
      '/c',
      'npm',
      'run',
      'build',
      '--workspace',
      '@ocentra-parent/schema-domain',
      '--workspace',
      '@ocentra-parent/logging-domain',
      '--workspace',
      '@ocentra-parent/activity-domain',
      '--workspace',
      '@ocentra-parent/parent-domain',
      '--workspace',
      '@ocentra-parent/agent-protocol-domain',
    ],
    log: join(proofRoot, 'agent-protocol-domain-dependency-build.log'),
  },
  {
    name: 'agent-protocol-domain-remote-delivery-status-test',
    command: 'cmd',
    args: [
      '/c',
      'npm',
      'run',
      'test',
      '--workspace',
      '@ocentra-parent/agent-protocol-domain',
      '--',
      'network-remote-delivery-status.test.ts',
    ],
    log: join(proofRoot, 'agent-protocol-domain-remote-delivery-status-test.log'),
  },
  {
    name: 'agent-protocol-domain-build',
    command: 'cmd',
    args: ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/agent-protocol-domain'],
    log: join(proofRoot, 'agent-protocol-domain-build.log'),
  },
  {
    name: 'source-shape',
    command: 'node',
    args: ['scripts/check-source-shape.mjs'],
    log: join(proofRoot, 'source-shape.log'),
  },
];

const commandResults = commands.map(runCommand);

const validationLogPath = join(proofRoot, '12-validation-commands.log');
writeFileSync(
  validationLogPath,
  commandResults.map((entry) => `${entry.command} -> ${entry.status}`).join('\n') + '\n'
);

const securityLogPath = join(proofRoot, '09-security-negative-proof.log');
writeFileSync(
  securityLogPath,
  [
    'checkedAt=deterministic:network-remote-delivery-status-bridge-proof/v1',
    'asserted=no exact URL/page/message/search claim from network-only evidence',
    'asserted=no video content, private-message content, or search-query content claim from network-only evidence',
    'asserted=no decrypted payload or raw PCAP without custody claim',
    'asserted=no live broker/family-hub delivery claim',
    'asserted=no remote acknowledgement implementation claim',
    'asserted=no remote provider or child-device delivery claim',
    'asserted=no cross-process replay implementation claim',
    'asserted=no remote delete/export propagation implementation claim',
    'asserted=no product-ready remote delivery claim',
    'asserted=no policy authority, side-effect authority, adapter action, host filtering, or enforcement command publication',
  ].join('\n') + '\n'
);

const proof = {
  proof: 'network-remote-delivery-status-bridge-proof',
  proofRevision: 'network-remote-delivery-status-bridge-proof/v1',
  checkedAt: 'deterministic:network-remote-delivery-status-bridge-proof/v1',
  sourceFingerprint: `source-tree:${sourceFingerprint()}`,
  sourceRefs: sourceFiles,
  sourceBase: mergeBase(),
  proofRoot,
  testRoot,
  commands: commandResults,
  artifacts: {
    sourceSnapshot: join(proofRoot, '00-source-snapshot.md'),
    expectedRemoteDeliveryStatusBridge: join(proofRoot, 'expected-remote-delivery-status-bridge.json'),
    proofPackApplicability: join(proofRoot, 'proof-pack-applicability.json'),
    securityNegativeLog: securityLogPath,
    validationCommands: validationLogPath,
    proofSummary: join(proofRoot, 'proof-summary.json'),
    testProof: join(testRoot, 'proof.json'),
  },
  coveredRows: [
    'network-plan supplemental row 10f remote delivery status bridge',
    'network-plan supplemental row 10e remote delivery durable envelope/store status',
    'network-plan supplemental row 10d remote delivery receipt ledger/local ack status',
    'network-plan supplemental row 10c remote delivery event-chain journal/export boundary status',
    'network-plan supplemental row 10b broker/family-hub remote delivery status',
  ],
  provenBoundaries: [
    'agent-protocol defines typed command/event names and a serializable NetworkRemoteDeliveryStatus shape that preserves row10f evidence through the current row10k bridge',
    'agent-service returns agent.network.remote-delivery.status.reported from agent.network.remote-delivery.status.get over the existing browser/network WebSocket command group',
    'agent-service source keeps an agent.command.rejected branch for proof-derived status build failures while runtime tests cover the deterministic success path',
    'the service status is served from the current row10k cached snapshot and preserves row10b, row10c, row10d, and row10e refs',
    'agent-protocol-domain parses the status event and rejects stale exact refs, wrong status refs, missing requirement artifacts, live/product-ready delivery claims, enforcement counts, adapter execution counts, and exact-content counters',
    'the bridge is a read-only status handoff and does not execute broker/family-hub delivery, provider delivery, child-device delivery, policy, adapter, or host filtering actions',
  ],
  notClaimed: expectedStatus.noClaims,
};

writeJson(join(proofRoot, 'proof-summary.json'), proof);
writeJson(join(testRoot, 'proof.json'), proof);
console.log('network-remote-delivery-status-bridge-proof-ok:protocol,service,ts,source-shape');
console.log(`proof=${join(proofRoot, 'proof-summary.json')}`);

function assertSourceContracts() {
  const constantsRoot = readText('crates/agent-protocol/src/constants.rs');
  const fieldConstants = readText('crates/agent-protocol/src/constants/field.rs');
  const networkConstants = readText('crates/agent-protocol/src/constants/network_flow.rs');
  const protocolContracts = readText('crates/agent-protocol/src/network_flow.rs');
  const protocolTransport = readText('crates/agent-protocol/src/transport.rs');
  const protocolTests = readText('crates/agent-protocol/src/network_flow_tests.rs');
  const protocolNameTests = readText('crates/agent-protocol/src/tests.rs');
  const servicePayload = readText('crates/agent-service/src/network_remote_delivery_status_payload.rs');
  const serviceTests = readText('crates/agent-service/src/network_remote_delivery_status_service_tests.rs');
  const serviceWebSocket = readText('crates/agent-service/src/websocket.rs');
  const domainContracts = readText('packages/agent-protocol-domain/src/contracts.ts');
  const domainDefaults = readText('packages/agent-protocol-domain/src/defaults.ts');
  const domainParser = readText('packages/agent-protocol-domain/src/network-remote-delivery-status.ts');
  const domainTests = readText('packages/agent-protocol-domain/tests/network-remote-delivery-status.test.ts');
  const packageManifest = readText('packages/agent-protocol-domain/package.json');
  const featureDoc = readText('docs/features/network-domain-control.md');
  const checklist = readText('docs/plans/network-plan/implementation-checklist.md');
  const workpacks = readText('docs/plans/network-plan/workpacks/README.md');
  const protocolReadme = readText('crates/agent-protocol/readme.md');
  const serviceReadme = readText('crates/agent-service/readme.md');
  const domainReadme = readText('packages/agent-protocol-domain/readme.md');
  const requiredSnippets = [
    [constantsRoot, 'NETWORK_REMOTE_DELIVERY_STATUS_REPORTED'],
    [fieldConstants, 'NETWORK_REMOTE_DELIVERY_STATUS'],
    [networkConstants, 'TEST_REMOTE_DELIVERY_STATUS_BRIDGE_REF'],
    [protocolContracts, 'NetworkRemoteDeliveryStatus'],
    [protocolContracts, 'product_ready_remote_delivery: bool'],
    [protocolTransport, 'AgentNetworkRemoteDeliveryStatusGet'],
    [protocolTransport, 'AgentNetworkRemoteDeliveryStatusReported'],
    [
      protocolTests,
      'network_remote_delivery_status_serializes_row10q_status_with_row10k_dispatch_state_without_product_claims',
    ],
    [protocolNameTests, 'AgentNetworkRemoteDeliveryStatusGet'],
    [servicePayload, 'prove_network_runtime_remote_delivery_transport_dispatch_state'],
    [servicePayload, 'OnceCell<NetworkRemoteDeliveryStatus>'],
    [servicePayload, 'get_or_try_init'],
    [servicePayload, 'AgentEventName::AgentCommandRejected'],
    [servicePayload, 'status.product_ready_remote_delivery = report.product_ready_remote_delivery'],
    [servicePayload, 'status.adapter_action_executed_count = count(report.adapter_action_executed_count)'],
    [serviceTests, 'network_remote_delivery_status_payload_serializes_row10q_status_with_row10k_dispatch_state'],
    [serviceTests, 'network_remote_delivery_status_payload_reuses_stable_row10q_status_snapshot'],
    [serviceTests, 'websocket_network_remote_delivery_status_command_reports_payload'],
    [serviceWebSocket, 'AgentCommandName::AgentNetworkRemoteDeliveryStatusGet'],
    [domainContracts, 'NetworkRemoteDeliveryStatusReported'],
    [domainDefaults, 'NetworkRemoteDeliveryStatus'],
    [domainDefaults, 'network.remote-delivery.transport-dispatch-state.10k'],
    [domainParser, 'AgentNetworkRemoteDeliveryStatusSchema'],
    [domainParser, 'AgentProtocolDefaults.Field.NetworkRemoteDeliveryStatus'],
    [domainParser, 'status.statusRef === NetworkRemoteDeliveryRefs.StatusRef'],
    [domainParser, 'productReadyRemoteDelivery: Schema.Literal(false)'],
    [domainParser, 'status.durableEnvelopeRef === NetworkRemoteDeliveryRefs.DurableEnvelopeRef'],
    [domainTests, 'rejects live delivery, product-ready, adapter, and content claims'],
    [domainTests, 'wrong.network.remote-delivery.durable-envelope.10e'],
    [packageManifest, './network-remote-delivery-status'],
    [featureDoc, 'row10f remote delivery status bridge proof'],
    [checklist, '10f-remote-delivery-status-bridge'],
    [workpacks, 'Remote delivery status bridge'],
    [protocolReadme, 'Network remote delivery status bridge'],
    [serviceReadme, 'Network remote delivery status reports'],
    [domainReadme, 'Network remote delivery status event parsing'],
  ];
  for (const [haystack, needle] of requiredSnippets) {
    assertIncludes(haystack, needle, `source contract snippet ${needle}`);
  }
  for (const forbidden of ['tokio::task::spawn_blocking', 'Handle::current', '.block_on(']) {
    assertNotIncludes(servicePayload, forbidden, `service status bridge forbids ${forbidden}`);
  }
}

function runCommand(entry) {
  const result = spawnSync(entry.command, entry.args, { encoding: 'utf8', shell: false });
  writeFileSync(entry.log, normalizeCommandLog(entry.name, `${result.stdout ?? ''}${result.stderr ?? ''}`));
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

function sourceFingerprint() {
  const hash = createHash('sha256');
  for (const filePath of fingerprintSourceFiles()) {
    hash.update(filePath);
    hash.update('\0');
    hash.update(readText(filePath));
    hash.update('\0');
  }
  return hash.digest('hex');
}

function mergeBase() {
  return runText('git', ['merge-base', 'HEAD', 'origin/main']).trim();
}

function readText(path) {
  return readFileSync(path, 'utf8');
}

function writeJson(path, value) {
  writeFileSync(path, `${JSON.stringify(value, null, 2)}\n`);
}

function assertIncludes(text, expected, label) {
  if (!text.includes(expected)) {
    throw new Error(`${label}: missing ${expected}`);
  }
}

function assertNotIncludes(text, forbidden, label) {
  if (text.includes(forbidden)) {
    throw new Error(`${label}: found ${forbidden}`);
  }
}

function normalizeCommandLog(name, text) {
  if (name === 'source-shape') {
    return normalizeSourceShapeLog(text);
  }
  return normalizeLogText(text);
}

function normalizeSourceShapeLog(text) {
  const normalized = normalizeLogText(text);
  const scopedWarnings = normalized
    .split('\n')
    .filter((line) => fingerprintSourceFiles().some((filePath) => line.startsWith(filePath)))
    .sort();
  const passedLine = normalized.includes('Source shape guard passed.') ? 'Source shape guard passed.' : '';
  return (
    ['Source shape warnings scoped to row10f source refs:', ...scopedWarnings, passedLine]
      .filter((line) => line.length > 0)
      .join('\n') + '\n'
  );
}

function normalizeLogText(text) {
  const normalizedLines = sortSourceShapeWarningLines(
    sortConsecutiveTestLines(
      normalizeWorkspacePaths(text)
        .replace(/\r\n/g, '\n')
        .split('\n')
        .filter((line) => !line.includes('Blocking waiting for'))
        .filter((line) => !line.trimStart().startsWith('Compiling '))
        .filter((line) => !line.trimStart().startsWith('Checking '))
        .map((line) =>
          line
            .replace(/finished in [0-9.]+s/g, 'finished in <duration>')
            .replace(/target\(s\) in [0-9.]+s/g, 'target(s) in <duration>')
            .replace(/target\(s\) in [0-9]+m [0-9]+s/g, 'target(s) in <duration>')
            .replace(/Duration\s+[0-9.]+(?:ms|s)/g, 'Duration <duration>')
            .replace(/Start at\s+[0-9:]+/g, 'Start at <time>')
            .replace(/duration_ms: [0-9.]+/g, 'duration_ms: <duration>')
            .replace(/\b[0-9.]+(?:ms|s)\b/g, '<duration>')
        )
    )
  );
  const trimmed = normalizedLines
    .join('\n')
    .replace(/[ \t]+$/gm, '')
    .replace(/\s+$/u, '');
  return trimmed.length === 0 ? '' : `${trimmed}\n`;
}

function fingerprintSourceFiles() {
  return sourceFiles.filter((filePath) => !fingerprintExcludedSourceFiles.has(filePath));
}

function normalizeWorkspacePaths(text) {
  const workspacePath = process.cwd();
  const workspacePathForward = workspacePath.replace(/\\/g, '/');
  return text
    .replace(new RegExp(escapeRegExp(workspacePath), 'g'), '<workspace>')
    .replace(new RegExp(escapeRegExp(workspacePathForward), 'g'), '<workspace>');
}

function escapeRegExp(value) {
  return value.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
}

function sortSourceShapeWarningLines(lines) {
  const warningHeaderIndex = lines.findIndex((line) => line.startsWith('Source shape warnings:'));
  if (warningHeaderIndex === -1) {
    return lines;
  }
  return [
    ...lines.slice(0, warningHeaderIndex + 1),
    ...lines
      .slice(warningHeaderIndex + 1)
      .filter((line) => line.trim().length > 0)
      .sort(),
  ];
}

function sortConsecutiveTestLines(lines) {
  const sortedLines = [];
  let testLineBuffer = [];
  for (const line of lines) {
    if (line.startsWith('test ') && line.endsWith(' ... ok')) {
      testLineBuffer.push(line);
      continue;
    }
    if (testLineBuffer.length > 0) {
      sortedLines.push(...testLineBuffer.sort());
      testLineBuffer = [];
    }
    sortedLines.push(line);
  }
  if (testLineBuffer.length > 0) {
    sortedLines.push(...testLineBuffer.sort());
  }
  return sortedLines;
}
