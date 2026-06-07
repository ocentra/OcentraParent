import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

const proofRoot = join('output', 'network-plan-proof', 'policy-preview-stored-flow-evidence');
const testRoot = join('test-results', 'network-policy-preview-stored-flow-evidence-proof');
mkdirSync(proofRoot, { recursive: true });
mkdirSync(testRoot, { recursive: true });

const sourceFiles = [
  'scripts/test/network-policy-preview-stored-flow-evidence-proof.mjs',
  'crates/agent-core/src/activity_store_policy_preview.rs',
  'crates/agent-core/src/activity_store_policy_preview_rows.rs',
  'crates/agent-core/src/activity_store_policy_preview_targets.rs',
  'crates/agent-core/src/activity_store_policy_preview_parent_rules.rs',
  'crates/agent-core/src/activity_store_policy_preview_test_fixture.rs',
  'crates/agent-core/src/activity_store_policy_preview_tests.rs',
  'crates/agent-core/README.md',
  'crates/agent-service/src/policy_preview_api.rs',
  'crates/agent-service/src/policy_preview_payload.rs',
  'crates/agent-service/src/policy_preview_tests.rs',
  'crates/agent-service/README.md',
  'crates/agent-protocol/src/policy_preview.rs',
  'crates/agent-protocol/README.md',
  'packages/agent-protocol-domain/src/contracts.ts',
  'packages/agent-protocol-domain/src/defaults.ts',
  'packages/agent-protocol-domain/tests/policy-preview-contracts.test.ts',
  'packages/agent-protocol-domain/tests/network-remote-delivery-status.test.ts',
  'packages/agent-protocol-domain/README.md',
  'docs/features/network-domain-control.md',
  'docs/plans/network-plan/implementation-checklist.md',
  'docs/plans/network-plan/workpacks/README.md',
];

assertSourceContracts();

const expectedStatus = {
  acceptedInputs: [
    'stored ActivityStore network flow row',
    'network domain subject with destinationDomain field',
    'local parent rule context targeting the stored network domain',
    'parent-rule target_evidence_refs matching the stored network activity event ref',
  ],
  previewState: [
    'targetType=domain',
    'targetValue=example-network.test',
    'capabilityStatus=ready',
    'decisionAction=block',
    'dryRun=true',
    'enforcementHandoffState=disabled',
    'parentRuleContextReferenceCount=1',
  ],
  evidenceBoundary: [
    'policy preview evidence references include the stored ActivityEvent ref',
    'this slice does not invent query-store, journal, AI, adapter, or enforcement refs',
    'row34 remains the evidence-grade policy mapper proof for A/B/C/D grade behavior',
  ],
  noClaims: [
    'exact URL from network-only evidence',
    'page content',
    'video content',
    'private-message content',
    'search-query content',
    'decrypted payload',
    'raw PCAP',
    'AI model execution',
    'full policy engine execution',
    'adapter authorization',
    'adapter action execution',
    'enforcement command publication',
    'host filtering',
    'live broker delivery',
    'family-hub delivery',
    'product-ready remote delivery',
  ],
};
writeJson(join(proofRoot, 'expected-policy-preview-stored-flow-evidence.json'), expectedStatus);

const commands = [
  {
    name: 'agent-core-network-flow-policy-preview-test',
    command: 'cargo',
    args: [
      'test',
      '-p',
      'ocentra-parent-agent-core',
      'policy_preview_read_model_evaluates_stored_network_flow_evidence_without_enforcement',
    ],
    log: join(proofRoot, 'agent-core-network-flow-policy-preview-test.log'),
  },
  {
    name: 'agent-service-policy-preview-payload-test',
    command: 'cargo',
    args: ['test', '-p', 'ocentra-parent-agent-service', 'policy_preview_payload'],
    log: join(proofRoot, 'agent-service-policy-preview-payload-test.log'),
  },
  {
    name: 'schema-domain-build',
    command: 'cmd',
    args: ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/schema-domain'],
    log: join(proofRoot, 'schema-domain-build.log'),
  },
  {
    name: 'logging-domain-build',
    command: 'cmd',
    args: ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/logging-domain'],
    log: join(proofRoot, 'logging-domain-build.log'),
  },
  {
    name: 'activity-domain-build',
    command: 'cmd',
    args: ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/activity-domain'],
    log: join(proofRoot, 'activity-domain-build.log'),
  },
  {
    name: 'agent-protocol-domain-build',
    command: 'cmd',
    args: ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/agent-protocol-domain'],
    log: join(proofRoot, 'agent-protocol-domain-build.log'),
  },
  {
    name: 'agent-protocol-domain-policy-preview-contract-test',
    command: 'cmd',
    args: [
      '/c',
      'npm',
      'exec',
      '--workspace',
      '@ocentra-parent/agent-protocol-domain',
      '--',
      'vitest',
      'run',
      'tests/policy-preview-contracts.test.ts',
    ],
    log: join(proofRoot, 'agent-protocol-domain-policy-preview-contract-test.log'),
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
    'checkedAt=deterministic:network-policy-preview-stored-flow-evidence-proof/v1',
    'asserted=stored ActivityStore network flow row can produce a policy preview decision',
    'asserted=parent rule context must cite stored network activity evidence refs',
    'asserted=policy preview remains dry-run with disabled enforcement handoff',
    'asserted=no exact URL/page/video/message/search claim from network-only evidence',
    'asserted=no decrypted payload or raw PCAP claim',
    'asserted=no AI execution claim',
    'asserted=no full policy engine execution claim',
    'asserted=no adapter authorization, adapter action, host filtering, or enforcement command publication',
  ].join('\n') + '\n'
);

const proof = {
  proof: 'network-policy-preview-stored-flow-evidence-proof',
  proofRevision: 'network-policy-preview-stored-flow-evidence-proof/v1',
  checkedAt: 'deterministic:network-policy-preview-stored-flow-evidence-proof/v1',
  sourceFingerprint: `source-tree:${sourceFingerprint()}`,
  sourceRefs: sourceFiles,
  sourceBase: mergeBase(),
  proofRoot,
  testRoot,
  commands: commandResults,
  artifacts: {
    expectedPolicyPreviewStoredFlowEvidence: join(proofRoot, 'expected-policy-preview-stored-flow-evidence.json'),
    securityNegativeLog: securityLogPath,
    validationCommands: validationLogPath,
    proofSummary: join(proofRoot, 'proof-summary.json'),
    testProof: join(testRoot, 'proof.json'),
  },
  coveredRows: [
    'network feature item Policy preview over stored flow evidence',
    'network-plan workpack 34 evidence-grade policy mapping remains the grade mapper dependency',
    'network-plan workpack 36 parent UI network evidence drawer remains read-model-only',
  ],
  provenBoundaries: [
    'agent-core policy preview consumes a stored ActivityStore network flow row and maps destinationDomain into a domain policy target',
    'agent-core policy preview resolves a local parent-rule context only when the parent rule cites the stored network activity event ref',
    'agent-core policy preview emits a dry-run policy decision with disabled enforcement handoff',
    'agent-service policy-preview payload exposes the latest dry-run decision without an adapter or enforcement claim',
    'agent-protocol-domain accepts the policy preview read-model command and reported event payload through the shared contracts',
    'the proof keeps row34 evidence-grade policy mapping as the grade-specific dependency instead of duplicating grade logic in the preview path',
  ],
  notClaimed: expectedStatus.noClaims,
};

writeJson(join(proofRoot, 'proof-summary.json'), proof);
writeJson(join(testRoot, 'proof.json'), proof);
console.log('network-policy-preview-stored-flow-evidence-proof-ok:core,service,ts,fmt,source-shape,diff-check');
console.log(`proof=${join(proofRoot, 'proof-summary.json')}`);

function assertSourceContracts() {
  const corePreview = readText('crates/agent-core/src/activity_store_policy_preview.rs');
  const coreTargets = readText('crates/agent-core/src/activity_store_policy_preview_targets.rs');
  const coreTests = readText('crates/agent-core/src/activity_store_policy_preview_tests.rs');
  const servicePayload = readText('crates/agent-service/src/policy_preview_payload.rs');
  const coreReadme = readText('crates/agent-core/README.md');
  const serviceReadme = readText('crates/agent-service/README.md');
  const protocolReadme = readText('crates/agent-protocol/README.md');
  const tsReadme = readText('packages/agent-protocol-domain/README.md');
  const tsContracts = readText('packages/agent-protocol-domain/tests/policy-preview-contracts.test.ts');
  const remoteDeliveryTsTest = readText('packages/agent-protocol-domain/tests/network-remote-delivery-status.test.ts');
  const featureDoc = readText('docs/features/network-domain-control.md');
  const checklist = readText('docs/plans/network-plan/implementation-checklist.md');
  const workpacks = readText('docs/plans/network-plan/workpacks/README.md');
  const requiredSnippets = [
    [corePreview, 'evaluate_policy_dry_run'],
    [coreTargets, 'DESTINATION_DOMAIN'],
    [coreTargets, 'PolicyTargetType::Domain'],
    [coreTests, 'policy_preview_read_model_evaluates_stored_network_flow_evidence_without_enforcement'],
    [servicePayload, 'POLICY_HANDOFF_STATE'],
    [coreReadme, 'Network policy-preview proof'],
    [serviceReadme, 'row10h typed status bridge'],
    [protocolReadme, 'row10h outbox-status bridge'],
    [tsContracts, 'PolicyPreviewReadModelReported'],
    [remoteDeliveryTsTest, 'parses row10h outbox status from a typed agent event'],
    [tsReadme, 'row10h outbox-status'],
    [featureDoc, 'Policy preview over stored flow evidence'],
    [featureDoc, 'network-policy-preview-stored-flow-evidence-proof'],
    [checklist, 'policy-preview-stored-flow-evidence'],
    [workpacks, 'policy-preview'],
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
    ['Source shape warnings scoped to policy-preview source refs:', ...scopedWarnings, passedLine]
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
