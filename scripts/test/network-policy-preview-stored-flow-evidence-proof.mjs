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
  'crates/agent-core/Cargo.toml',
  'crates/agent-protocol/src/constants/field.rs',
  'crates/agent-protocol/src/constants/policy.rs',
  'crates/agent-protocol/src/constants/sqlite.rs',
  'crates/agent-core/src/activity_store_policy_preview.rs',
  'crates/agent-core/src/activity_store_policy_preview_rows.rs',
  'crates/agent-core/src/activity_store_policy_preview_targets.rs',
  'crates/agent-core/src/activity_store_policy_preview_parent_rules.rs',
  'crates/agent-core/src/activity_store_policy_preview_test_fixture.rs',
  'crates/agent-core/src/activity_store_policy_preview_tests.rs',
  'crates/agent-core/src/activity_store_policy_preview_parent_rule_tests.rs',
  'crates/ocentra-network-evidence/src/policy.rs',
  'crates/ocentra-network-evidence/src/tests/policy.rs',
  'crates/agent-core/README.md',
  'crates/agent-service/src/policy_preview_api.rs',
  'crates/agent-service/src/policy_preview_payload.rs',
  'crates/agent-service/src/policy_preview_tests.rs',
  'crates/agent-service/README.md',
  'crates/agent-protocol/src/policy_preview.rs',
  'crates/agent-protocol/README.md',
  'packages/agent-protocol-domain/src/contracts.ts',
  'packages/agent-protocol-domain/src/defaults.ts',
  'packages/agent-protocol-domain/tests/unit/policy-preview-contracts.test.ts',
  'packages/agent-protocol-domain/tests/unit/network-remote-delivery-status.test.ts',
  'packages/agent-protocol-domain/README.md',
  'apps/portal/src/live-activity-state.ts',
  'apps/portal/src/policy-preview-details.ts',
  'apps/portal/tests/policy-preview-live-activity-state.test.ts',
  'docs/features/network-domain-control.md',
  'docs/product-capability-checklist.md',
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
    'parent-rule context scoped to the stored event source device and platform',
  ],
  previewState: [
    'targetType=domain',
    'targetValue=example-network.test',
    'capabilityStatus=ready-for-preview-read-model',
    'decisionAction=ask-parent after row34 grade-B block downgrade',
    'networkEvidenceGrade=B',
    'networkRequestedPolicyAction=block',
    'networkMappedPolicyAction=ask-parent',
    'networkPolicyMappingMode=parent-review',
    'networkAdapterActionAuthorized=false',
    'networkEnforcementCommandAuthorized=false',
    'dryRun=true',
    'enforcementHandoffState=disabled',
    'parentRuleContextReferenceCount=1',
  ],
  evidenceBoundary: [
    'policy preview evidence references include the stored ActivityEvent ref',
    'retention tombstones suppress deleted network flow evidence before preview limit and decisions',
    'wrong-device or wrong-child parent rule contexts are excluded before preview decisions',
    'future, expired, or scheduled-without-proof parent rules are excluded before preview decisions',
    'this slice does not invent query-store, journal, AI, adapter, or enforcement refs',
    'row34 shared evidence-grade mapper downgrades grade-B block requests to parent review',
    'service payload exposes row34 evidence-grade provenance without granting adapter or enforcement authority',
    'portal parser retains row34 evidence-grade provenance and rejects adapter or enforcement authorization claims',
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
    name: 'agent-core-policy-preview-read-model-tests',
    command: 'cargo',
    args: ['test', '-p', 'ocentra-parent-agent-core', 'policy_preview_read_model'],
    log: join(proofRoot, 'agent-core-policy-preview-read-model-tests.log'),
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
    args: ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/network-domain'],
    log: join(proofRoot, 'activity-domain-build.log'),
  },
  {
    name: 'agent-protocol-domain-build',
    command: 'cmd',
    args: ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/agent-protocol-domain'],
    log: join(proofRoot, 'agent-protocol-domain-build.log'),
  },
  {
    name: 'portal-build',
    command: 'cmd',
    args: ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/portal'],
    log: join(proofRoot, 'portal-build.log'),
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
    name: 'portal-policy-preview-live-activity-state-test',
    command: 'cmd',
    args: [
      '/c',
      'npm',
      'exec',
      '--workspace',
      '@ocentra-parent/portal',
      '--',
      'vitest',
      'run',
      'tests/policy-preview-live-activity-state.test.ts',
    ],
    log: join(proofRoot, 'portal-policy-preview-live-activity-state-test.log'),
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
    'checkedAt=deterministic:network-policy-preview-stored-flow-evidence-proof/v2',
    'asserted=stored ActivityStore network flow row can produce a dry-run parent-review policy preview decision',
    'asserted=network retention tombstones suppress deleted flow rows before preview decisions',
    'asserted=parent rule context must cite stored network activity evidence refs and match source device/platform scope',
    'asserted=stale, future, or scheduled-without-proof parent rule contexts are excluded',
    'asserted=row34 evidence-grade mapper downgrades grade-B network block requests to parent review',
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
  proofRevision: 'network-policy-preview-stored-flow-evidence-proof/v2',
  checkedAt: 'deterministic:network-policy-preview-stored-flow-evidence-proof/v2',
  sourceFingerprint: `source-tree:${sourceFingerprint()}`,
  sourceRefs: sourceFiles,
  sourceBase: 'deterministic:policy-preview-source-set/v2',
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
    'network-plan workpack 36 parent UI drawer remains outside this proof; this proof preserves read-model-only policy-preview payload boundaries',
  ],
  provenBoundaries: [
    'agent-core policy preview consumes a stored ActivityStore network flow row and maps destinationDomain into a domain policy target',
    'agent-core policy preview resolves a local parent-rule context only when the parent rule cites the stored network activity event ref',
    'agent-core policy preview suppresses retention-deleted network flow evidence before preview limits, parent-rule matching, or dry-run decisions',
    'agent-core policy preview excludes parent-rule contexts whose device or child scope does not match the stored event source',
    'agent-core policy preview excludes future, expired, and scheduled-without-proof parent rules from preview rows',
    'agent-core policy preview applies the shared row34 evidence-grade mapper so grade-B network block requests become parent-review ask-parent decisions',
    'agent-core policy preview emits a dry-run policy decision with disabled enforcement handoff',
    'agent-service policy-preview payload exposes the latest dry-run decision with network evidence grade, requested action, mapped action, mapping mode, and false adapter/enforcement authorization flags',
    'agent-protocol-domain accepts the policy preview read-model command and reported event payload through the shared contracts',
    'portal live-activity parser retains policy-preview network provenance fields and rejects authorized adapter/enforcement payloads',
    'the proof keeps row34 evidence-grade policy mapping as the grade-specific dependency instead of duplicating grade logic in the preview path',
  ],
  notClaimed: expectedStatus.noClaims,
};

writeJson(join(proofRoot, 'proof-summary.json'), proof);
writeJson(join(testRoot, 'proof.json'), proof);
console.log('network-policy-preview-stored-flow-evidence-proof-ok:core,service,ts,portal,fmt,source-shape,diff-check');
console.log(`proof=${join(proofRoot, 'proof-summary.json')}`);

function assertSourceContracts() {
  const coreCargo = readText('crates/agent-core/Cargo.toml');
  const fieldConstants = readText('crates/agent-protocol/src/constants/field.rs');
  const policyConstants = readText('crates/agent-protocol/src/constants/policy.rs');
  const corePreview = readText('crates/agent-core/src/activity_store_policy_preview.rs');
  const coreRows = readText('crates/agent-core/src/activity_store_policy_preview_rows.rs');
  const coreTargets = readText('crates/agent-core/src/activity_store_policy_preview_targets.rs');
  const coreParentRules = readText('crates/agent-core/src/activity_store_policy_preview_parent_rules.rs');
  const coreTests = readText('crates/agent-core/src/activity_store_policy_preview_tests.rs');
  const coreParentRuleTests = readText('crates/agent-core/src/activity_store_policy_preview_parent_rule_tests.rs');
  const sqliteConstants = readText('crates/agent-protocol/src/constants/sqlite.rs');
  const servicePayload = readText('crates/agent-service/src/policy_preview_payload.rs');
  const coreReadme = readText('crates/agent-core/README.md');
  const serviceReadme = readText('crates/agent-service/README.md');
  const protocolReadme = readText('crates/agent-protocol/README.md');
  const tsReadme = readText('packages/agent-protocol-domain/README.md');
  const tsContracts = readText('packages/agent-protocol-domain/tests/unit/policy-preview-contracts.test.ts');
  const remoteDeliveryTsTest = readText('packages/agent-protocol-domain/tests/unit/network-remote-delivery-status.test.ts');
  const portalLiveActivityState = readText('apps/portal/src/live-activity-state.ts');
  const portalDetailView = readText('apps/portal/src/policy-preview-details.ts');
  const portalPolicyPreviewTest = readText('apps/portal/tests/policy-preview-live-activity-state.test.ts');
  const featureDoc = readText('docs/features/network-domain-control.md');
  const productCapabilityChecklist = readText('docs/product-capability-checklist.md');
  const checklist = readText('docs/plans/network-plan/implementation-checklist.md');
  const workpacks = readText('docs/plans/network-plan/workpacks/README.md');
  const networkPolicy = readText('crates/ocentra-network-evidence/src/policy.rs');
  const networkPolicyTests = readText('crates/ocentra-network-evidence/src/tests/policy.rs');
  const requiredSnippets = [
    [coreCargo, 'ocentra-network-evidence'],
    [fieldConstants, 'NETWORK_EVIDENCE_GRADE'],
    [fieldConstants, 'NETWORK_ENFORCEMENT_COMMAND_AUTHORIZED'],
    [policyConstants, 'REASON_NETWORK_EVIDENCE_GRADE_PARENT_REVIEW'],
    [policyConstants, 'NETWORK_POLICY_MAPPING_MODE_PARENT_REVIEW'],
    [corePreview, 'evaluate_policy_dry_run'],
    [corePreview, 'map_network_evidence_grade_to_policy'],
    [corePreview, 'grade_mapped_network_decision'],
    [corePreview, 'PolicyAction::AskParent'],
    [corePreview, 'preview_network_evidence_mapping'],
    [corePreview, 'network_evidence_grade'],
    [coreRows, 'HashSet'],
    [coreRows, 'device_id'],
    [coreRows, 'NETWORK_RETENTION_DELETED'],
    [coreRows, 'row_deleted'],
    [coreRows, 'SELECT_NETWORK_RETENTION_DELETED_ACTIVITY'],
    [sqliteConstants, 'SELECT_POLICY_PREVIEW_ACTIVITY'],
    [sqliteConstants, 'SELECT_NETWORK_RETENTION_DELETED_ACTIVITY'],
    [sqliteConstants, 'device_id'],
    [sqliteConstants, 'platform'],
    [sqliteConstants, 'kind'],
    [coreTargets, 'DESTINATION_DOMAIN'],
    [coreTargets, 'PolicyTargetType::Domain'],
    [coreParentRules, 'context_scope_matches_row'],
    [coreParentRules, 'context_rule_has_supported_schedule'],
    [coreParentRules, 'context_rule_is_effective_at'],
    [coreTests, 'policy_preview_read_model_evaluates_stored_network_flow_evidence_without_enforcement'],
    [coreTests, 'policy_preview_read_model_fails_closed_when_network_mapping_refs_are_malformed'],
    [coreTests, 'policy_preview_read_model_excludes_network_flow_deleted_by_retention_tombstone'],
    [coreTests, 'policy_preview_read_model_applies_retention_tombstones_before_limit'],
    [coreTests, 'REASON_NETWORK_EVIDENCE_GRADE_PARENT_REVIEW'],
    [coreTests, 'network_evidence_mapping'],
    [coreParentRuleTests, 'policy_preview_read_model_rejects_wrong_device_or_child_rule_contexts'],
    [coreParentRuleTests, 'policy_preview_read_model_rejects_future_or_expired_rule_windows'],
    [coreParentRuleTests, 'policy_preview_read_model_rejects_scheduled_rule_without_schedule_proof'],
    [servicePayload, 'POLICY_HANDOFF_STATE'],
    [servicePayload, 'NETWORK_EVIDENCE_GRADE'],
    [servicePayload, 'NETWORK_ADAPTER_ACTION_AUTHORIZED'],
    [coreReadme, 'Network policy-preview proof'],
    [serviceReadme, 'row10m delete/export readiness'],
    [serviceReadme, 'delete/export propagation, product readiness'],
    [protocolReadme, 'row10s cross-process replay status'],
    [protocolReadme, 'row10k transport-dispatch refs'],
    [tsContracts, 'PolicyPreviewReadModelReported'],
    [tsContracts, 'NetworkEvidenceGrade'],
    [tsContracts, 'NetworkEnforcementCommandAuthorized'],
    [portalLiveActivityState, 'resolvePortalDomainLiveActivityState'],
    [portalDetailView, 'PortalDetails.NetworkEvidenceGrade'],
    [portalPolicyPreviewTest, "networkEvidenceGrade: 'B'"],
    [portalDetailView, 'NetworkAdapterAuthorization'],
    [portalPolicyPreviewTest, 'rejects policy-preview payloads that claim network authorization'],
    [remoteDeliveryTsTest, 'parses row10t external cross-process transport status'],
    [tsReadme, 'row10k transport-dispatch'],
    [featureDoc, 'Policy preview over stored flow evidence'],
    [featureDoc, 'network-policy-preview-stored-flow-evidence-proof'],
    [featureDoc, 'retention-deleted flow rows'],
    [productCapabilityChecklist, 'policy-preview evidence-grade provenance'],
    [productCapabilityChecklist, 'complete live capture/runtime execution plus real OS adapter/platform proof'],
    [checklist, 'policy-preview-stored-flow-evidence'],
    [checklist, 'retention-deleted flow rows'],
    [checklist, 'networkEvidenceGrade'],
    [workpacks, 'policy-preview'],
    [networkPolicy, 'mapped_mode_and_action'],
    [networkPolicyTests, 'policy_mapping_routes_grade_b_block_requests_to_parent_review'],
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
            .replace(/\x1b\[[0-9;]*m/g, '')
            .replace(/^[0-9]{1,2}:[0-9]{2}:[0-9]{2}\s+[ap]\.m\.\s+/u, '<time> ')
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
  return sourceFiles;
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
