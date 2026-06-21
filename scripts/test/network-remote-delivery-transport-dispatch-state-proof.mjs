import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

const proofRoot = join('output', 'network-plan-proof', '10k-remote-delivery-transport-dispatch-state');
const testRoot = join('test-results', 'network-remote-delivery-transport-dispatch-state-proof');
mkdirSync(proofRoot, { recursive: true });
mkdirSync(testRoot, { recursive: true });

const sourceFiles = [
  'scripts/test/network-remote-delivery-transport-dispatch-state-proof.mjs',
  'crates/agent-protocol/src/constants/network_flow.rs',
  'crates/agent-protocol/src/network_flow.rs',
  'crates/agent-protocol/src/network_flow_tests.rs',
  'crates/agent-core/src/lib.rs',
  'crates/agent-core/src/network_event_runtime.rs',
  'crates/agent-core/src/network_event_runtime/remote_delivery_transport_dispatch_state.rs',
  'crates/agent-core/src/network_event_runtime/remote_delivery_transport_dispatch_state_types.rs',
  'crates/agent-core/src/network_event_runtime/remote_delivery_no_enforcement_invariant.rs',
  'crates/agent-core/src/network_event_runtime/remote_delivery_no_enforcement_invariant_types.rs',
  'crates/agent-core/src/network_event_runtime/remote_delivery_dispatch_readiness.rs',
  'crates/agent-core/src/network_event_runtime/remote_delivery_dispatch_readiness_types.rs',
  'crates/agent-core/src/network_event_runtime/remote_delivery_outbox_handoff_types.rs',
  'crates/agent-service/src/network_remote_delivery_status_payload.rs',
  'crates/agent-service/src/network_remote_delivery_status_service_tests.rs',
  'packages/agent-protocol-domain/src/defaults.ts',
  'packages/schema-domain/src/network-remote-delivery-status.ts',
  'packages/agent-protocol-domain/src/network-remote-delivery-status.ts',
  'packages/agent-protocol-domain/tests/unit/network-remote-delivery-status.test.ts',
  'docs/features/network-domain-control.md',
  'docs/plans/network-plan/implementation-checklist.md',
  'docs/plans/network-plan/workpacks/README.md',
];

assertSourceContracts();

const expectedStatus = {
  acceptedInputs: [
    'row10j available-metadata no-enforcement invariant',
    'row10i dispatch-readiness refs',
    'row10g prepared outbox candidates',
    'row10b through row10e remote delivery metadata refs',
  ],
  statusCommandAndEvent: {
    command: 'agent.network.remote-delivery.status.get',
    event: 'agent.network.remote-delivery.status.reported',
    payloadField: 'networkRemoteDeliveryStatus',
  },
  transportDispatchStateRefs: [
    'network.remote-delivery.transport-dispatch-state.10k',
    'network.remote-delivery.dispatch-blocked-manual-required.10k',
    'network.remote-delivery.future-transport-seam.10k',
  ],
  provenStates: [
    'state=ManualRequiredBlocked',
    'blockedDispatchRecordCount equals sourceOutboxCandidateCount',
    'blockedDispatchRecordCount equals row10j manualRequiredCandidateCount',
    'sourceOutboxState=PreparedNotDispatched',
    'blockedState=ManualRequiredBlocked',
    'dispatchReadyCandidateCount=0',
    'dispatchAttemptCount=0',
    'remoteAckCount=0',
  ],
  parserInvariants: [
    'available remote metadata becomes blocked manual-required dispatch state without sending transport',
    'blocked dispatch records preserve event id, event type, correlation id, outbox ref, and handoff ref',
    'future transport seam refs exist without claiming live broker or family-hub delivery',
    'service status command serializes row10k blocked dispatch state while preserving row10g outbox refs',
    'service status command caches the deterministic row10k protocol snapshot instead of rebuilding the full proof chain on every request',
    'TypeScript parser rejects stale row10h status refs, stale row10k dispatch refs, and candidate-count mismatches',
    'manual-required blocked dispatch cannot publish enforcement commands',
  ],
  noClaims: [
    'live broker dispatch',
    'live family-hub relay dispatch',
    'transport dispatch attempt',
    'remote acknowledgement',
    'remote provider delivery',
    'child-device delivery',
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
writeJson(join(proofRoot, 'expected-remote-delivery-transport-dispatch-state.json'), expectedStatus);

const commands = [
  {
    name: 'agent-core-remote-delivery-transport-dispatch-state-test',
    command: 'cargo',
    args: ['test', '-p', 'ocentra-parent-agent-core', 'network_runtime_remote_delivery_transport_dispatch_state'],
    log: join(proofRoot, 'agent-core-remote-delivery-transport-dispatch-state-test.log'),
  },
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
      '@ocentra-parent/network-domain',
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
    name: 'agent-core-clippy',
    command: 'cargo',
    args: ['clippy', '-p', 'ocentra-parent-agent-core', '--all-targets', '--', '-D', 'warnings'],
    log: join(proofRoot, 'agent-core-clippy.log'),
  },
  {
    name: 'rust-format',
    command: 'cargo',
    args: ['fmt', '--all', '--check'],
    log: join(proofRoot, 'rust-format.log'),
  },
  {
    name: 'source-shape',
    command: 'node',
    args: ['scripts/check-source-shape.mjs'],
    log: join(proofRoot, 'source-shape.log'),
  },
  {
    name: 'git-diff-check',
    command: 'git',
    args: ['diff', '--check'],
    log: join(proofRoot, 'git-diff-check.log'),
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
    'checkedAt=deterministic:network-remote-delivery-transport-dispatch-state-proof/v1',
    'asserted=available remote metadata maps to manual-required blocked transport dispatch state',
    'asserted=no exact URL/page/video/message/search claim from network-only evidence',
    'asserted=no decrypted payload or raw PCAP claim',
    'asserted=no live broker/family-hub dispatch claim',
    'asserted=no transport dispatch attempt claim',
    'asserted=no remote acknowledgement claim',
    'asserted=no remote provider or child-device delivery claim',
    'asserted=no remote delete/export propagation implementation claim',
    'asserted=no product-ready remote delivery claim',
    'asserted=no policy authority, side-effect authority, adapter action, host filtering, or enforcement command publication',
  ].join('\n') + '\n'
);

const proof = {
  proof: 'network-remote-delivery-transport-dispatch-state-proof',
  proofRevision: 'network-remote-delivery-transport-dispatch-state-proof/v1',
  checkedAt: 'deterministic:network-remote-delivery-transport-dispatch-state-proof/v1',
  sourceFingerprint: `source-tree:${sourceFingerprint()}`,
  sourceRefs: sourceFiles,
  sourceBase: mergeBase(),
  proofRoot,
  testRoot,
  commands: commandResults,
  artifacts: {
    expectedRemoteDeliveryTransportDispatchState: join(
      proofRoot,
      'expected-remote-delivery-transport-dispatch-state.json'
    ),
    securityNegativeLog: securityLogPath,
    validationCommands: validationLogPath,
    proofSummary: join(proofRoot, 'proof-summary.json'),
    testProof: join(testRoot, 'proof.json'),
  },
  coveredRows: [
    'network-plan supplemental row 10k remote delivery transport dispatch state',
    'network-plan supplemental row 10j remote delivery no-enforcement invariant',
    'network-plan supplemental row 10i remote delivery dispatch readiness',
    'network-plan supplemental row 10g remote delivery outbox handoff',
    'network-plan supplemental rows 10b through 10f remote delivery refs',
  ],
  provenBoundaries: [
    'agent-core consumes the row10j available-metadata invariant and renders row10g prepared outbox candidates as manual-required blocked dispatch records',
    'blocked dispatch records preserve event id, event type, correlation id, source outbox state, outbox refs, handoff refs, dispatch-state refs, blocked-dispatch refs, and future transport seam refs',
    'blocked dispatch records equal the source outbox candidate count and row10j manual-required candidate count',
    'the service status command serializes row10k blocked dispatch refs while preserving row10g outbox refs, durable refs, and receipt refs',
    'the service status command uses an async OnceCell cache so repeated reads reuse the stable row10k protocol snapshot without spawn_blocking or block_on',
    'the TypeScript status parser rejects stale row10h status refs, wrong row10k dispatch refs, nonzero dispatch-ready/dispatch/ack counters, and mismatched blocked-dispatch candidate counts',
    'dispatch-ready candidates, dispatch attempts, and remote acknowledgements stay zero',
    'the proof rejects policy authority, side-effect authority, adapter execution, enforcement command publication, live broker/family-hub delivery, remote acknowledgement, provider delivery, child-device delivery, remote delete/export propagation, and product-ready remote delivery claims',
    'the proof rejects raw PCAP, exact URL, decrypted payload, page content, video content, private-message content, and search-query content claims from network-only evidence',
  ],
  notClaimed: [
    'live broker dispatch',
    'live family-hub relay dispatch',
    'transport dispatch attempt',
    'remote acknowledgement',
    'remote provider delivery',
    'child-device delivery',
    'remote delete/export propagation implementation',
    'product-ready remote delivery',
    'cross-process transport implementation',
    'policy authority',
    'side-effect authority',
    'adapter execution',
    'host filtering',
    'full network-plan completion',
  ],
};

writeJson(join(proofRoot, 'proof-summary.json'), proof);
writeJson(join(testRoot, 'proof.json'), proof);
console.log(
  'network-remote-delivery-transport-dispatch-state-proof-ok:core,protocol,service,ts,clippy,fmt,source-shape,diff-check'
);
console.log(`proof=${join(proofRoot, 'proof-summary.json')}`);

function assertSourceContracts() {
  const protocolConstants = readText('crates/agent-protocol/src/constants/network_flow.rs');
  const protocolShape = readText('crates/agent-protocol/src/network_flow.rs');
  const protocolTests = readText('crates/agent-protocol/src/network_flow_tests.rs');
  const coreLib = readText('crates/agent-core/src/lib.rs');
  const coreRuntime = readText('crates/agent-core/src/network_event_runtime.rs');
  const coreProof = readText('crates/agent-core/src/network_event_runtime/remote_delivery_transport_dispatch_state.rs');
  const coreTypes = readText(
    'crates/agent-core/src/network_event_runtime/remote_delivery_transport_dispatch_state_types.rs'
  );
  const servicePayload = readText('crates/agent-service/src/network_remote_delivery_status_payload.rs');
  const serviceTests = readText('crates/agent-service/src/network_remote_delivery_status_service_tests.rs');
  const tsDefaults = readText('packages/agent-protocol-domain/src/defaults.ts');
  const schemaStatus = readText('packages/schema-domain/src/network-remote-delivery-status.ts');
  const tsParser = readText('packages/agent-protocol-domain/src/network-remote-delivery-status.ts');
  const tsTests = readText('packages/agent-protocol-domain/tests/unit/network-remote-delivery-status.test.ts');
  const featureDoc = readText('docs/features/network-domain-control.md');
  const checklist = readText('docs/plans/network-plan/implementation-checklist.md');
  const workpacks = readText('docs/plans/network-plan/workpacks/README.md');
  const requiredSnippets = [
    [protocolConstants, 'TEST_REMOTE_DELIVERY_TRANSPORT_DISPATCH_STATE_REF'],
    [protocolConstants, 'TEST_REMOTE_DELIVERY_DISPATCH_BLOCKED_MANUAL_REF'],
    [protocolConstants, 'TEST_REMOTE_DELIVERY_FUTURE_TRANSPORT_SEAM_REF'],
    [protocolShape, 'transport_dispatch_state_ref'],
    [protocolShape, 'NetworkRemoteDeliveryTransportDispatchState'],
    [
      protocolTests,
      'network_remote_delivery_status_serializes_row10t_external_transport_status_without_product_claims',
    ],
    [coreLib, 'prove_network_runtime_remote_delivery_transport_dispatch_state'],
    [coreRuntime, 'remote_delivery_transport_dispatch_state'],
    [coreProof, 'ManualRequiredBlocked'],
    [coreProof, 'has_unsupported_claims'],
    [coreProof, 'network_runtime_remote_delivery_transport_dispatch_state_blocks_without_transport'],
    [coreProof, 'network_runtime_remote_delivery_transport_dispatch_state_rejects_action_claims'],
    [coreTypes, 'NetworkRuntimeRemoteDeliveryTransportDispatchStateReport'],
    [servicePayload, 'prove_network_runtime_remote_delivery_transport_dispatch_state'],
    [servicePayload, 'OnceCell<NetworkRemoteDeliveryStatus>'],
    [servicePayload, 'get_or_try_init'],
    [servicePayload, 'RuntimeTransportDispatchState::ManualRequiredBlocked'],
    [serviceTests, 'network_remote_delivery_status_payload_serializes_row10t_external_transport_status'],
    [serviceTests, 'network_remote_delivery_status_payload_reuses_stable_row10t_status_snapshot'],
    [tsDefaults, 'TransportDispatchStateRef'],
    [tsTests, 'transportDispatchStateRef'],
    [tsTests, 'parses row10t external cross-process transport status from a typed agent event'],
    [schemaStatus, 'dispatchAttemptCount: Schema.Literal(0)'],
    [featureDoc, 'network-remote-delivery-transport-dispatch-state-proof'],
    [checklist, '10k-remote-delivery-transport-dispatch-state'],
    [workpacks, '10k'],
  ];
  for (const [haystack, needle] of requiredSnippets) {
    assertIncludes(haystack, needle, `source contract snippet ${needle}`);
  }
  for (const forbidden of ['tokio::task::spawn_blocking', 'Handle::current', '.block_on(']) {
    assertNotIncludes(servicePayload, forbidden, `service status cache forbids ${forbidden}`);
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
    ['Source shape warnings scoped to row10k source refs:', ...scopedWarnings, passedLine]
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
  return sourceFiles.filter((filePath) => !filePath.startsWith('scripts/test/'));
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
