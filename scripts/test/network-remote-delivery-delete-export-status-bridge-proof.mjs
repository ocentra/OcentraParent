import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

const proofRoot = join('output', 'network-plan-proof', '10n-remote-delivery-delete-export-status-bridge');
const testRoot = join('test-results', 'network-remote-delivery-delete-export-status-bridge-proof');
mkdirSync(proofRoot, { recursive: true });
mkdirSync(testRoot, { recursive: true });

const sourceFiles = [
  'scripts/test/network-remote-delivery-delete-export-status-bridge-proof.mjs',
  'crates/agent-protocol/src/constants/network_flow.rs',
  'crates/agent-protocol/src/network_flow.rs',
  'crates/agent-protocol/src/network_flow_tests.rs',
  'crates/agent-service/src/network_remote_delivery_status_payload.rs',
  'crates/agent-service/src/network_remote_delivery_status_service_tests.rs',
  'packages/agent-protocol-domain/src/defaults.ts',
  'packages/agent-protocol-domain/src/network-remote-delivery-status.ts',
  'packages/agent-protocol-domain/tests/network-remote-delivery-status.test.ts',
  'crates/agent-protocol/README.md',
  'crates/agent-service/README.md',
  'packages/agent-protocol-domain/README.md',
  'docs/features/network-domain-control.md',
  'docs/plans/network-plan/implementation-checklist.md',
  'docs/plans/network-plan/workpacks/README.md',
];

assertSourceContracts();

const expectedStatus = {
  statusCommandAndEvent: {
    command: 'agent.network.remote-delivery.status.get',
    event: 'agent.network.remote-delivery.status.reported',
    payloadField: 'networkRemoteDeliveryStatus',
  },
  statusBridgeRef: 'network.remote-delivery.delete-export-status-bridge.10n',
  deleteExportStatusRefs: [
    'network.remote-delivery.delete-export-propagation-readiness.10m',
    'network.remote-delivery.remote-delete-readiness.10m',
    'network.remote-delivery.remote-export-readiness.10m',
  ],
  provenStates: [
    'deleteExportReadinessRecordCount equals outboxCandidateCount',
    'remoteDeleteReadyCount equals outboxCandidateCount',
    'remoteExportReadyCount equals outboxCandidateCount',
    'deleteExportRecordsMatchFixtureAcks=true',
    'remoteDeleteExportPropagationImplemented=false',
  ],
  parserInvariants: [
    'Rust protocol serializes row10m readiness refs in the current row10t remote delivery status shape',
    'service payload composes row10t status identity, row10k blocked dispatch refs, and row10m delete/export readiness refs into one cached status object',
    'TypeScript parser rejects stale row10t status refs, stale row10m refs, and mismatched readiness counts',
    'status bridge does not claim live propagation, product-ready delivery, policy authority, adapter execution, enforcement commands, exact content, raw PCAP, or host filtering',
  ],
  noClaims: [
    'actual remote delete/export propagation',
    'live broker dispatch',
    'live family-hub relay dispatch',
    'remote provider delivery',
    'child-device delivery',
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
writeJson(join(proofRoot, 'expected-remote-delivery-delete-export-status-bridge.json'), expectedStatus);

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
    name: 'agent-service-clippy',
    command: 'cargo',
    args: ['clippy', '-p', 'ocentra-parent-agent-service', '--all-targets', '--', '-D', 'warnings'],
    log: join(proofRoot, 'agent-service-clippy.log'),
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
    'checkedAt=deterministic:network-remote-delivery-delete-export-status-bridge-proof/v1',
    'asserted=existing remote delivery status command carries row10m delete/export readiness refs',
    'asserted=Rust and TypeScript parsers reject stale row10m refs and mismatched readiness counts',
    'asserted=remote delete/export propagation remains not implemented and product support remains false',
    'asserted=no exact URL/page/video/message/search claim from network-only evidence',
    'asserted=no decrypted payload or raw PCAP claim',
    'asserted=no live broker/family-hub/provider/child-device delivery claim',
    'asserted=no policy authority, side-effect authority, adapter action, host filtering, or enforcement command publication',
  ].join('\n') + '\n'
);

const proof = {
  proof: 'network-remote-delivery-delete-export-status-bridge-proof',
  proofRevision: 'network-remote-delivery-delete-export-status-bridge-proof/v1',
  checkedAt: 'deterministic:network-remote-delivery-delete-export-status-bridge-proof/v1',
  sourceFingerprint: `source-tree:${sourceFingerprint()}`,
  sourceRefs: sourceFiles,
  sourceBase: mergeBase(),
  proofRoot,
  testRoot,
  commands: commandResults,
  artifacts: {
    expectedRemoteDeliveryDeleteExportStatusBridge: join(
      proofRoot,
      'expected-remote-delivery-delete-export-status-bridge.json'
    ),
    securityNegativeLog: securityLogPath,
    validationCommands: validationLogPath,
    proofSummary: join(proofRoot, 'proof-summary.json'),
    testProof: join(testRoot, 'proof.json'),
  },
  coveredRows: [
    'network-plan supplemental row 10n remote delivery delete export status bridge proof',
    'network-plan supplemental row 10m remote delivery delete export propagation readiness proof',
    'network-plan supplemental row 10k remote delivery transport dispatch state',
    'network-plan supplemental row 10h remote delivery outbox status bridge',
  ],
  provenBoundaries: [
    'Rust protocol serializes row10m delete/export readiness refs inside the current row10t remote delivery status shape',
    'agent-service reports row10t status identity plus row10m delete/export readiness refs and counts through the existing cached remote delivery status payload',
    'TypeScript parser rejects stale row10m readiness refs and readiness counts that do not match the source outbox candidate count',
    'the status bridge keeps remote delete/export propagation implementation false and does not upgrade service payload or product support into live delivery',
    'the status bridge rejects product-ready remote delivery, policy authority, side-effect authority, adapter execution, enforcement command publication, live broker/family-hub delivery, provider delivery, child-device delivery, exact-content, and host-filter claims',
  ],
  notClaimed: expectedStatus.noClaims,
};

writeJson(join(proofRoot, 'proof-summary.json'), proof);
writeJson(join(testRoot, 'proof.json'), proof);
console.log(
  'network-remote-delivery-delete-export-status-bridge-proof-ok:protocol,service,ts,clippy,fmt,source-shape,diff-check'
);
console.log(`proof=${join(proofRoot, 'proof-summary.json')}`);

function assertSourceContracts() {
  const protocolConstants = readText('crates/agent-protocol/src/constants/network_flow.rs');
  const protocolShape = readText('crates/agent-protocol/src/network_flow.rs');
  const protocolTests = readText('crates/agent-protocol/src/network_flow_tests.rs');
  const servicePayload = readText('crates/agent-service/src/network_remote_delivery_status_payload.rs');
  const serviceTests = readText('crates/agent-service/src/network_remote_delivery_status_service_tests.rs');
  const tsDefaults = readText('packages/agent-protocol-domain/src/defaults.ts');
  const tsParser = readText('packages/agent-protocol-domain/src/network-remote-delivery-status.ts');
  const tsTests = readText('packages/agent-protocol-domain/tests/network-remote-delivery-status.test.ts');
  const protocolReadme = readText('crates/agent-protocol/README.md');
  const serviceReadme = readText('crates/agent-service/README.md');
  const tsReadme = readText('packages/agent-protocol-domain/README.md');
  const featureDoc = readText('docs/features/network-domain-control.md');
  const checklist = readText('docs/plans/network-plan/implementation-checklist.md');
  const workpacks = readText('docs/plans/network-plan/workpacks/README.md');
  const requiredSnippets = [
    [protocolShape, 'delete_export_propagation_ref'],
    [protocolShape, 'remote_delete_readiness_ref'],
    [protocolShape, 'delete_export_readiness_record_count'],
    [protocolConstants, 'TEST_REMOTE_DELIVERY_DELETE_EXPORT_STATUS_BRIDGE_REF'],
    [
      protocolTests,
      'network_remote_delivery_status_serializes_row10t_external_transport_status_without_product_claims',
    ],
    [protocolTests, 'deleteExportPropagationRef'],
    [servicePayload, 'prove_network_runtime_remote_delivery_delete_export_propagation'],
    [servicePayload, 'apply_delete_export_status'],
    [servicePayload, 'TEST_REMOTE_DELIVERY_DELETE_EXPORT_STATUS_BRIDGE_REF'],
    [serviceTests, 'assert_remote_delivery_delete_export_status'],
    [tsDefaults, 'DeleteExportPropagationRef'],
    [tsDefaults, 'cross-process-custody-status.10q'],
    [tsParser, 'deleteExportReadinessMatches'],
    [tsParser, 'row10t status identity'],
    [tsTests, 'parses row10t external cross-process transport status from a typed agent event'],
    [tsTests, 'rejects row10k dispatch, row10m readiness, and candidate-count mismatches'],
    [tsTests, 'rejects row10p provider/child readiness and row10q custody mismatches'],
    [tsTests, 'rejects row10r replay and row10t transport mismatches'],
    [protocolReadme, 'row10m'],
    [protocolReadme, 'delete/export readiness refs'],
    [protocolReadme, 'row10t external cross-process transport'],
    [serviceReadme, 'row10m delete/export readiness'],
    [tsReadme, 'row10m delete/export readiness refs'],
    [featureDoc, 'network-remote-delivery-delete-export-status-bridge-proof'],
    [checklist, '10n-remote-delivery-delete-export-status-bridge'],
    [workpacks, '10n'],
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

function sourceFingerprint() {
  const hash = createHash('sha256');
  for (const filePath of sourceFiles.filter((filePath) => !filePath.startsWith('scripts/test/'))) {
    hash.update(filePath);
    hash.update('\0');
    hash.update(readText(filePath));
    hash.update('\0');
  }
  return hash.digest('hex');
}

function mergeBase() {
  const result = spawnSync('git', ['merge-base', 'HEAD', 'origin/main'], {
    encoding: 'utf8',
    shell: false,
  });
  if (result.status !== 0) {
    throw new Error(`git merge-base failed with exit ${result.status}`);
  }
  return result.stdout.trim();
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
    .filter((line) =>
      sourceFiles
        .filter((filePath) => !filePath.startsWith('scripts/test/'))
        .some((filePath) => line.startsWith(filePath))
    )
    .sort();
  const passedLine = normalized.includes('Source shape guard passed.') ? 'Source shape guard passed.' : '';
  return (
    ['Source shape warnings scoped to row10n source refs:', ...scopedWarnings, passedLine]
      .filter((line) => line.length > 0)
      .join('\n') + '\n'
  );
}

function normalizeLogText(text) {
  const workspacePath = process.cwd();
  const workspacePathForward = workspacePath.replace(/\\/g, '/');
  const normalized = text
    .replace(new RegExp(escapeRegExp(workspacePath), 'g'), '<workspace>')
    .replace(new RegExp(escapeRegExp(workspacePathForward), 'g'), '<workspace>')
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
        .replace(/\b[0-9.]+(?:ms|s)\b/g, '<duration>')
    )
    .join('\n')
    .replace(/[ \t]+$/gm, '')
    .replace(/\s+$/u, '');
  return normalized.length === 0 ? '' : `${normalized}\n`;
}

function escapeRegExp(value) {
  return value.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
}
