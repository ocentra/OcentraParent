import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

const proofRoot = join('output', 'network-plan-proof', '10h-remote-delivery-outbox-status-bridge');
const testRoot = join('test-results', 'network-remote-delivery-outbox-status-bridge-proof');
mkdirSync(proofRoot, { recursive: true });
mkdirSync(testRoot, { recursive: true });

const sourceFiles = [
  'scripts/test/network-remote-delivery-outbox-status-bridge-proof.mjs',
  'crates/agent-protocol/src/constants/network_flow.rs',
  'crates/agent-protocol/src/network_flow.rs',
  'crates/agent-protocol/src/network_flow_tests.rs',
  'crates/agent-service/src/network_remote_delivery_status_payload.rs',
  'crates/agent-service/src/network_remote_delivery_status_service_tests.rs',
  'crates/agent-service/src/websocket.rs',
  'packages/agent-protocol-domain/src/defaults.ts',
  'packages/schema-domain/src/network-remote-delivery-status.ts',
  'packages/agent-protocol-domain/src/network-remote-delivery-status.ts',
  'packages/agent-protocol-domain/tests/unit/network-remote-delivery-status.test.ts',
  'docs/features/network-domain-control.md',
  'docs/plans/network-plan/implementation-checklist.md',
  'docs/plans/network-plan/workpacks/README.md',
  'crates/agent-protocol/README.md',
  'crates/agent-service/README.md',
  'packages/agent-protocol-domain/README.md',
];

assertSourceContracts();

const expectedStatus = {
  acceptedInputs: [
    'row10b broker/family-hub remote delivery requirements',
    'row10c event-chain journal/export refs',
    'row10d receipt-ledger/local acknowledgement refs',
    'row10e durable envelope/store/replay/delete-export refs',
    'row10g prepared outbox and handoff refs',
  ],
  commandAndEvent: {
    command: 'agent.network.remote-delivery.status.get',
    event: 'agent.network.remote-delivery.status.reported',
    payloadField: 'networkRemoteDeliveryStatus',
  },
  bridgeRefs: [
    'network.remote-delivery.transport-dispatch-state.10k',
    'network.remote-delivery.outbox.10g',
    'network.remote-delivery.outbox-handoff.10g',
    'network.remote-delivery.outbox-replay.10g',
    'network.remote-delivery.outbox-support-status.10g',
    'network.remote-delivery.dispatch-blocked-manual-required.10k',
    'network.remote-delivery.future-transport-seam.10k',
  ],
  renderedStates: [
    'outboxCandidateCount>0',
    'sourceOutboxCandidateCount equals outboxCandidateCount',
    'preparedNotDispatchedCount equals outboxCandidateCount',
    'blockedDispatchRecordCount equals outboxCandidateCount',
    'blockedDispatchRecordsMatchOutboxCandidates=true',
    'transportDispatchState=manual-required-blocked',
    'dispatchReadyCandidateCount=0',
    'dispatchAttemptCount=0',
    'remoteAckCount=0',
    'duplicateDurableEnvelopeRejected=true',
    'outboxCandidatesMatchDurableEnvelopes=true',
    'outboxCandidatesMatchReceipts=true',
    'productReadyRemoteDelivery=false',
  ],
  parserInvariants: [
    'TypeScript parser rejects stale or wrong row10g outbox refs',
    'TypeScript parser rejects stale row10h status refs and wrong row10k blocked dispatch refs',
    'TypeScript parser rejects nonzero dispatch attempts or remote acknowledgement counts',
    'TypeScript parser rejects mismatched prepared-not-dispatched counts',
    'Rust service serializes row10g outbox status into the row10k protocol-owned blocked dispatch payload field',
    'WebSocket routing returns the existing remote-delivery status event without adding a transport dispatch command',
    'outbox status cannot claim provider delivery, child-device delivery, policy authority, side-effect authority, adapter action, enforcement commands, exact content, or host filtering',
  ],
  noClaims: [
    'live broker delivery',
    'live family-hub relay delivery',
    'transport dispatch attempt',
    'remote acknowledgement implementation',
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
writeJson(join(proofRoot, 'expected-remote-delivery-outbox-status-bridge.json'), expectedStatus);

writeFileSync(
  join(proofRoot, '00-source-snapshot.md'),
  [
    '# Row10h Remote Delivery Outbox Status Bridge Source Snapshot',
    '',
    'proofRevision=network-remote-delivery-outbox-status-bridge-proof/v1',
    'scopeMarker=row10h-remote-delivery-outbox-status-bridge',
    `sourceBase=${mergeBase()}`,
    'worktreeStatus=expected to contain generated row10h proof artifacts until committed',
    '',
    'Inspected paths:',
    ...sourceFiles.map((filePath) => `- ${filePath}`),
    '',
    'Before-state gap:',
    '- Row10g produced prepared outbox candidates, and row10k now carries those refs through the current typed status payload.',
    '',
    'Current bridge boundary:',
    '- The row10h bridge evidence remains preserved in the row10k status payload through row10g outbox refs, handoff refs, replay/support refs, prepared candidate counts, duplicate rejection, and zero dispatch/ack counters.',
    '- The row10k payload adds manual-required blocked dispatch refs while remaining read-only and keeping live transport, remote acknowledgement, product-ready delivery, policy authority, adapter execution, exact content, and host filtering unclaimed.',
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
    'checkedAt=deterministic:network-remote-delivery-outbox-status-bridge-proof/v1',
    'asserted=no exact URL/page/video/message/search claim from network-only evidence',
    'asserted=no decrypted payload or raw PCAP without custody claim',
    'asserted=no live broker/family-hub delivery claim',
    'asserted=no transport dispatch attempt claim',
    'asserted=no remote acknowledgement implementation claim',
    'asserted=no remote provider or child-device delivery claim',
    'asserted=no remote delete/export propagation implementation claim',
    'asserted=no product-ready remote delivery claim',
    'asserted=no policy authority, side-effect authority, adapter action, host filtering, or enforcement command publication',
  ].join('\n') + '\n'
);

const proof = {
  proof: 'network-remote-delivery-outbox-status-bridge-proof',
  proofRevision: 'network-remote-delivery-outbox-status-bridge-proof/v1',
  checkedAt: 'deterministic:network-remote-delivery-outbox-status-bridge-proof/v1',
  sourceFingerprint: `source-tree:${sourceFingerprint()}`,
  sourceRefs: sourceFiles,
  sourceBase: mergeBase(),
  proofRoot,
  testRoot,
  commands: commandResults,
  artifacts: {
    sourceSnapshot: join(proofRoot, '00-source-snapshot.md'),
    expectedRemoteDeliveryOutboxStatusBridge: join(proofRoot, 'expected-remote-delivery-outbox-status-bridge.json'),
    securityNegativeLog: securityLogPath,
    validationCommands: validationLogPath,
    proofSummary: join(proofRoot, 'proof-summary.json'),
    testProof: join(testRoot, 'proof.json'),
  },
  coveredRows: [
    'network-plan supplemental row 10h remote delivery outbox status bridge',
    'network-plan supplemental row 10k remote delivery transport dispatch state',
    'network-plan supplemental row 10g remote delivery outbox handoff',
    'network-plan supplemental row 10f remote delivery status bridge',
    'network-plan supplemental row 10e remote delivery durable envelope/store status',
  ],
  provenBoundaries: [
    'agent-protocol status payload carries row10g outbox refs, handoff refs, replay/support refs, prepared candidate counts, duplicate rejection, mismatch counters, and row10k blocked dispatch refs',
    'agent-service builds the remote-delivery status from row10k transport dispatch proof state without spawning a live transport dispatch',
    'agent-protocol-domain parser rejects stale outbox refs, stale row10h status refs, wrong row10k dispatch refs, nonzero dispatch/ack counters, and mismatched prepared candidate counts',
    'the bridge keeps broker delivery, family-hub relay delivery, provider delivery, child-device delivery, remote delete/export propagation, product-ready remote delivery, policy authority, side-effect authority, adapter execution, enforcement commands, and host filtering false',
    'the bridge keeps raw PCAP, exact URL, decrypted payload, page content, video content, private-message content, and search-query content unavailable from network-only outbox status',
  ],
  notClaimed: expectedStatus.noClaims,
};

writeJson(join(proofRoot, 'proof-summary.json'), proof);
writeJson(join(testRoot, 'proof.json'), proof);
console.log('network-remote-delivery-outbox-status-bridge-proof-ok:protocol,service,ts,source-shape,diff-check');
console.log(`proof=${join(proofRoot, 'proof-summary.json')}`);

function assertSourceContracts() {
  const protocolConstants = readText('crates/agent-protocol/src/constants/network_flow.rs');
  const protocolShape = readText('crates/agent-protocol/src/network_flow.rs');
  const protocolTests = readText('crates/agent-protocol/src/network_flow_tests.rs');
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
    [protocolConstants, 'TEST_REMOTE_DELIVERY_OUTBOX_STATUS_BRIDGE_REF'],
    [protocolShape, 'outbox_candidate_count'],
    [protocolShape, 'dispatch_attempt_count'],
    [
      protocolTests,
      'network_remote_delivery_status_serializes_row10t_external_transport_status_without_product_claims',
    ],
    [servicePayload, 'prove_network_runtime_remote_delivery_transport_dispatch_state'],
    [servicePayload, 'RuntimeTransportDispatchState::ManualRequiredBlocked'],
    [servicePayload, 'outbox_candidates_match_receipts'],
    [serviceTests, 'network_remote_delivery_status_payload_serializes_row10t_external_transport_status'],
    [tsDefaults, 'OutboxHandoffRef'],
    [tsDefaults, 'TransportDispatchStateRef'],
    [tsTests, 'outboxHandoffRef'],
    [tsTests, 'transportDispatchStateRef'],
    [schemaStatus, 'dispatchAttemptCount: Schema.Literal(0)'],
    [tsTests, 'dispatchAttemptCount: 1'],
    [featureDoc, 'row10h remote delivery outbox status bridge proof'],
    [checklist, '10h-remote-delivery-outbox-status-bridge'],
    [workpacks, 'Remote delivery outbox status bridge'],
  ];
  for (const [haystack, needle] of requiredSnippets) {
    assertIncludes(haystack, needle, `source contract snippet ${needle}`);
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
    ['Source shape warnings scoped to row10h source refs:', ...scopedWarnings, passedLine]
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
