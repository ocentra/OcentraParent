import { spawnSync } from 'node:child_process';
import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { dirname, join } from 'node:path';

const proofRoot = 'output/network-plan-proof/proof-pack-reconciliation';
const testRoot = 'test-results/network-proof-pack-reconciliation';

const docsRead = [
  '.ocentra-ai/rules/ocentra-parent-rules.mdc',
  '.ocentra-ai/rules/ocentra-parent-validation.mdc',
  '.ocentra-ai/rules/ocentra-parent-source-shape.mdc',
  '.ocentra-ai/rules/ocentra-parent-test-rules.mdc',
  '.ocentra-ai/rules/ocentra-parent-domain-boundaries.mdc',
  'docs/feature-list.md',
  'docs/features/network-domain-control.md',
  'docs/expectations/network-flow-evidence.md',
  'docs/expectations/enforcement.md',
  'docs/plans/network-plan/README.md',
  'docs/plans/network-plan/implementation-checklist.md',
  'docs/plans/network-plan/workpacks/README.md',
];

const pendingBranches = [
  {
    branch: 'codex/network-content-claim-invariant-proof',
    expectedHead: '5f883b36ec717b5347340fb13cc2613c63e8a7e3',
    reason:
      'pipeline-level unsupported video/private-message/search content claim fields are branch-only until integrated',
  },
  {
    branch: 'codex/network-manual-followup-owner-ledger-proof',
    expectedHead: 'c6477df42f815f52fc769166fa9275dbdd666d4d',
    reason: 'manual/deferred follow-up owner ledger sourceRef fix is branch-only until integrated',
  },
];

const gates = [
  {
    id: '00-source-snapshot.md',
    status: 'current-main-proved',
    sources: docsRead,
    summary: 'current branch, commit, inspected docs, and pre-reconciliation gaps are recorded',
  },
  {
    id: '01-contract-proof.log',
    status: 'current-main-proved',
    sources: [
      'packages/activity-domain/src/network-contracts.ts',
      'packages/activity-domain/tests/network-contracts.test.ts',
      'packages/agent-protocol-domain/src/network-runtime-events.ts',
      'packages/agent-protocol-domain/tests/network-runtime-events.test.ts',
      'crates/agent-protocol/src/network_flow_events.rs',
      'crates/agent-protocol/src/network_flow_tests.rs',
      'scripts/test/network-contract-boundary-proof.mjs',
      'scripts/test/eventing-network-ts-event-parity-proof.mjs',
      'output/eventing-plan-proof/45-50-network-protocol-contracts/proof-summary.json',
    ],
    summary: 'TypeScript network contracts, Rust protocol parity, and public TS event-contract parity are cited',
  },
  {
    id: '02-eventing-proof.log',
    status: 'current-main-proved',
    sources: [
      'crates/ocentra-eventing/src/lib.rs',
      'crates/ocentra-eventing/src/bus.rs',
      'crates/ocentra-eventing/src/envelope.rs',
      'crates/ocentra-eventing/src/queue.rs',
      'crates/ocentra-eventing/src/request.rs',
      'crates/ocentra-eventing/src/replay.rs',
      'crates/ocentra-eventing/src/testkit.rs',
      'crates/agent-core/src/network_event_runtime.rs',
      'crates/agent-core/src/network_event_runtime/queue.rs',
      'crates/agent-core/src/network_event_runtime_tests.rs',
      'crates/agent-core/src/network_event_runtime_queue_tests.rs',
      'crates/agent-core/src/network_event_runtime_broker_delivery_tests.rs',
      'crates/agent-service/src/network_runtime_delivery.rs',
      'crates/agent-service/src/network_runtime_delivery_tests.rs',
      'crates/agent-service/src/network_runtime_stream_payload.rs',
      'crates/agent-service/src/network_runtime_stream_tests.rs',
      'scripts/test/eventing-network-runtime-proof.mjs',
      'scripts/test/eventing-network-service-runtime-delivery-proof.mjs',
      'scripts/test/eventing-network-service-event-chain-stream-proof.mjs',
      'scripts/test/network-broker-delivery-proof.mjs',
      'output/eventing-plan-proof/reusable-eventing-runtime/proof-summary.json',
      'output/eventing-plan-proof/57-network-workpack-10-reusable-crate/proof-summary.json',
      'output/eventing-plan-proof/58-network-event-chain-exact-refs/proof-summary.json',
      'output/eventing-plan-proof/63-delivery-decision-proof/proof-summary.json',
    ],
    summary: 'network consumes reusable Rust eventing and keeps broker/family-hub transport requirement-gated',
  },
  {
    id: '03-parser-fixture-proof.json',
    status: 'current-main-proved',
    sources: [
      'crates/ocentra-network-evidence/src/pcap.rs',
      'crates/ocentra-network-evidence/src/packet.rs',
      'crates/ocentra-network-evidence/src/dns.rs',
      'crates/ocentra-network-evidence/src/tls.rs',
      'crates/ocentra-network-evidence/src/http.rs',
      'crates/ocentra-network-evidence/src/encrypted_dns.rs',
      'crates/ocentra-network-evidence/src/zeek.rs',
      'crates/ocentra-network-evidence/src/tests/visibility.rs',
      'crates/ocentra-network-evidence/src/tests/zeek.rs',
      'scripts/test/network-pcap-replay-proof.mjs',
      'scripts/test/network-packet-dns-parser-proof.mjs',
      'scripts/test/network-visibility-parser-proof.mjs',
      'scripts/test/network-zeek-analyzer-comparison-proof.mjs',
    ],
    summary: 'PCAP replay, parser fixtures, visibility parsers, and analyzer comparison artifacts are cited',
  },
  {
    id: '04-analyzer-alert-proof.json',
    status: 'current-main-proved',
    sources: [
      'crates/ocentra-network-evidence/src/zeek.rs',
      'crates/ocentra-network-evidence/src/signature_alert.rs',
      'crates/ocentra-network-evidence/src/tests/zeek.rs',
      'crates/ocentra-network-evidence/src/tests/signature_alert.rs',
      'scripts/test/network-zeek-analyzer-comparison-proof.mjs',
      'scripts/test/network-signature-alert-ingestion-proof.mjs',
    ],
    summary: 'Zeek-style summaries and Suricata/Snort-compatible signature alert fixtures are cited',
  },
  {
    id: '05-ai-policy-proof.json',
    status: 'current-main-proved',
    sources: [
      'crates/ocentra-network-evidence/src/ai_detection.rs',
      'crates/ocentra-network-evidence/src/ai_audit.rs',
      'crates/ocentra-network-evidence/src/risk_budget.rs',
      'crates/ocentra-network-evidence/src/policy.rs',
      'crates/ocentra-network-evidence/src/pipeline.rs',
      'crates/ocentra-network-evidence/src/tests/ai_detection.rs',
      'crates/ocentra-network-evidence/src/tests/ai_audit.rs',
      'crates/ocentra-network-evidence/src/tests/risk_budget.rs',
      'crates/ocentra-network-evidence/src/tests/policy.rs',
      'crates/ocentra-network-evidence/src/tests/pipeline.rs',
      'scripts/test/network-ai-detection-fixture-proof.mjs',
      'scripts/test/network-ai-audit-narrative-proof.mjs',
      'scripts/test/network-risk-budget-threshold-proof.mjs',
      'scripts/test/network-evidence-policy-mapping-proof.mjs',
      'scripts/test/network-end-to-end-pipeline-proof.mjs',
      'output/network-plan-proof/51-end-to-end-pipeline-proof/proof-summary.json',
    ],
    summary: 'AI detection/audit, risk budget, policy mapping, and end-to-end bypass guards are cited',
  },
  {
    id: '06-adapter-action-proof.json',
    status: 'current-main-partial',
    sources: [
      'crates/ocentra-network-evidence/src/dns_adapter.rs',
      'crates/ocentra-network-evidence/src/windows_firewall_adapter.rs',
      'crates/ocentra-network-evidence/src/windows_wfp_gate.rs',
      'crates/ocentra-network-evidence/src/android_vpn_service_gate.rs',
      'crates/ocentra-network-evidence/src/apple_network_extension_gate.rs',
      'crates/ocentra-network-evidence/src/linux_adapter_gate.rs',
      'crates/ocentra-network-evidence/src/platform_claims.rs',
      'crates/ocentra-network-evidence/src/action_result.rs',
      'crates/ocentra-network-evidence/src/tests/dns_adapter.rs',
      'crates/ocentra-network-evidence/src/tests/windows_firewall_adapter.rs',
      'crates/ocentra-network-evidence/src/tests/windows_wfp_gate.rs',
      'crates/ocentra-network-evidence/src/tests/android_vpn_service_gate.rs',
      'crates/ocentra-network-evidence/src/tests/apple_network_extension_gate.rs',
      'crates/ocentra-network-evidence/src/tests/linux_adapter_gate.rs',
      'crates/ocentra-network-evidence/src/tests/platform_claims.rs',
      'crates/ocentra-network-evidence/src/tests/action_result.rs',
      'output/network-plan-proof/52-platform-claims-proof/proof-summary.json',
      'output/network-plan-proof/53-action-result-state-proof/proof-summary.json',
    ],
    summary: 'dry-run/manual/apply-ready/result states are proved; live adapter execution remains unclaimed',
  },
  {
    id: '07-journal-sqlite-proof.json',
    status: 'current-main-partial',
    sources: [
      'crates/agent-core/src/activity_store_network_flow.rs',
      'crates/agent-core/src/activity_store_network_flow_rows.rs',
      'crates/agent-core/src/activity_store_network_flow_tests.rs',
      'crates/agent-service/src/network_runtime_delivery.rs',
      'crates/agent-service/src/network_runtime_delivery_tests.rs',
      'crates/agent-service/src/network_runtime_stream_payload.rs',
      'crates/agent-service/src/network_runtime_stream_tests.rs',
      'scripts/test/eventing-network-service-runtime-delivery-proof.mjs',
      'scripts/test/eventing-network-service-event-chain-stream-proof.mjs',
      'output/network-plan-proof/03a-live-capture-storage-proof/proof-summary.json',
      'output/network-plan-proof/51-end-to-end-pipeline-proof/proof-summary.json',
    ],
    summary:
      'service-local read-model, event-chain projection, tombstones, and retention refs are proved; raw PCAP remote delete/export propagation remains open',
  },
  {
    id: '08-ui-snapshots/README.md',
    status: 'current-main-partial',
    sources: [
      'apps/portal/e2e/network-evidence-drawer-proof.spec.ts',
      'apps/portal/src/NetworkEvidenceDrawerRoutePanel.tsx',
      'apps/portal/src/network-evidence-drawer.ts',
      'apps/portal/src/network-flow-read-model.ts',
      'apps/portal/tests/live-activity-network-flow.test.ts',
      'output/network-plan-proof/portal-risk-performance-platform-status/proof-summary.json',
      'test-results/network-portal-risk-performance-platform-proof/proof.json',
    ],
    summary:
      'service-backed drawer and platform/status UI are proved; broader risk-budget, audit narrative, and performance SLO UI remain open',
  },
  {
    id: '09-security-negative-proof.log',
    status: 'current-main-partial',
    sources: [
      'packages/activity-domain/tests/network-contracts.test.ts',
      'packages/agent-protocol-domain/tests/network-runtime-events.test.ts',
      'crates/ocentra-network-evidence/src/tests/pipeline.rs',
      'crates/ocentra-network-evidence/src/tests/action_result.rs',
      'crates/ocentra-network-evidence/src/tests/readiness.rs',
      'scripts/test/network-portal-source-gate-proof.mjs',
      'scripts/test/network-security-readiness-proof.mjs',
      'scripts/test/network-end-to-end-pipeline-proof.mjs',
      'scripts/test/network-action-result-state-proof.mjs',
      'output/network-plan-proof/36-portal-source-gate/proof-summary.json',
      'output/network-plan-proof/51-end-to-end-pipeline-proof/proof-summary.json',
      'output/network-plan-proof/53-action-result-state-proof/proof-summary.json',
    ],
    summary:
      'main proves no exact URL/page/message/search/decrypted-payload, no weak-evidence enforcement, and no UI/AI/network bypass; explicit pipeline video-content invariant is pending branch-only evidence',
  },
  {
    id: '10-performance-proof.log',
    status: 'current-main-proved',
    sources: [
      'crates/ocentra-network-evidence/src/performance.rs',
      'crates/ocentra-network-evidence/src/tests/performance.rs',
      'scripts/test/network-performance-benchmark-proof.mjs',
    ],
    summary: 'deterministic latency, throughput, resource, queue, and high-concurrency fixture metrics are cited',
  },
];

mkdirSync(localPath(proofRoot), { recursive: true });
mkdirSync(localPath(testRoot), { recursive: true });

const branch = runText('git', ['branch', '--show-current']).trim();
const commit = runText('git', ['rev-parse', 'HEAD']).trim();
const originMain = runText('git', ['rev-parse', 'origin/main']).trim();
const mergeBase = runText('git', ['merge-base', 'HEAD', 'origin/main']).trim();
const sourceStatusShort = runText('git', [
  'status',
  '--short',
  '--',
  '.',
  `:(exclude)${proofRoot}`,
  `:(exclude)${testRoot}`,
]);

const sourceChecks = verifySources([...docsRead, ...gates.flatMap((gate) => gate.sources)]);
const pendingBranchChecks = pendingBranches.map(readPendingBranchHead);

writeMarkdown('00-source-snapshot.md', sourceSnapshot());
writeMarkdown('01-contract-proof.log', gateLog('01-contract-proof.log'));
writeMarkdown('02-eventing-proof.log', gateLog('02-eventing-proof.log'));
writeJson('03-parser-fixture-proof.json', gateJson('03-parser-fixture-proof.json'));
writeJson('04-analyzer-alert-proof.json', gateJson('04-analyzer-alert-proof.json'));
writeJson('05-ai-policy-proof.json', gateJson('05-ai-policy-proof.json'));
writeJson('06-adapter-action-proof.json', gateJson('06-adapter-action-proof.json'));
writeJson('07-journal-sqlite-proof.json', gateJson('07-journal-sqlite-proof.json'));
writeMarkdown('08-ui-snapshots/README.md', uiSnapshotReadme());
writeMarkdown('09-security-negative-proof.log', gateLog('09-security-negative-proof.log'));
writeMarkdown('10-performance-proof.log', gateLog('10-performance-proof.log'));
writeJson('11-manual-deferred-followups.json', manualDeferredFollowups());
writeMarkdown('12-validation-commands.log', validationCommandsLog());

const proof = {
  proof: 'network-proof-pack-reconciliation',
  checkedAt: new Date().toISOString(),
  branch,
  commit,
  originMain,
  mergeBase,
  sourceStatusShort,
  proofRoot,
  testRoot,
  sourceChecks,
  gates,
  pendingBranchChecks,
  artifacts: {
    sourceSnapshot: `${proofRoot}/00-source-snapshot.md`,
    contractProof: `${proofRoot}/01-contract-proof.log`,
    eventingProof: `${proofRoot}/02-eventing-proof.log`,
    parserFixtureProof: `${proofRoot}/03-parser-fixture-proof.json`,
    analyzerAlertProof: `${proofRoot}/04-analyzer-alert-proof.json`,
    aiPolicyProof: `${proofRoot}/05-ai-policy-proof.json`,
    adapterActionProof: `${proofRoot}/06-adapter-action-proof.json`,
    journalSqliteProof: `${proofRoot}/07-journal-sqlite-proof.json`,
    uiSnapshotReadme: `${proofRoot}/08-ui-snapshots/README.md`,
    securityNegativeProof: `${proofRoot}/09-security-negative-proof.log`,
    performanceProof: `${proofRoot}/10-performance-proof.log`,
    manualDeferredFollowups: `${proofRoot}/11-manual-deferred-followups.json`,
    validationCommands: `${proofRoot}/12-validation-commands.log`,
    proofSummary: `${proofRoot}/proof-summary.json`,
    testProof: `${testRoot}/proof.json`,
  },
  provenRootGates: gates.filter((gate) => gate.status === 'current-main-proved').map((gate) => gate.id),
  partialRootGates: gates.filter((gate) => gate.status === 'current-main-partial').map((gate) => gate.id),
  currentMainNotClaimed: [
    'live broker or family-hub delivery',
    'live packet capture driver invocation',
    'raw PCAP remote delete/export propagation',
    'live local-AI model execution',
    'full production policy engine execution',
    'live DNS/firewall/WFP/VpnService/NetworkExtension/Linux adapter execution',
    'host mutation or packet blocking',
    'production SLO validation',
    'production rollout or external audit execution',
  ],
};
writeJson('proof-summary.json', proof);
writeFileSync(localPath(`${testRoot}/proof.json`), `${JSON.stringify(proof, null, 2)}\n`);

console.log('network-proof-pack-reconciliation-ok:tracked-sources,current-main-proof-pack,pending-branch-boundaries');
console.log(`proof=${proofRoot}/proof-summary.json`);

function verifySources(paths) {
  return [...new Set(paths)].map((repoPath) => {
    if (!existsSync(localPath(repoPath))) {
      throw new Error(`missing proof source: ${repoPath}`);
    }
    runText('git', ['ls-files', '--error-unmatch', repoPath]);
    if (repoPath.endsWith('.json')) {
      JSON.parse(readFileSync(localPath(repoPath), 'utf8'));
    }
    return { path: repoPath, tracked: true, jsonParsed: repoPath.endsWith('.json') };
  });
}

function readPendingBranchHead(entry) {
  const output = runText('git', ['ls-remote', 'origin', `refs/heads/${entry.branch}`]).trim();
  const [head = ''] = output.split(/\s+/);
  if (head !== entry.expectedHead) {
    throw new Error(`pending branch ${entry.branch} expected ${entry.expectedHead} but found ${head || 'missing'}`);
  }
  return { ...entry, remoteHead: head, currentMainEvidence: false };
}

function sourceSnapshot() {
  return `# Network Proof Pack Reconciliation Source Snapshot

Branch: ${branch}
Commit: ${commit}
Origin main: ${originMain}
Merge base: ${mergeBase}

Source status excluding this proof pack:

\`\`\`text
${sourceStatusShort || '(clean)'}
\`\`\`

Inspected docs:

${docsRead.map((path) => `- ${path}`).join('\n')}

Before-state gaps preserved by this reconciliation:

- Live broker/family-hub delivery remains unclaimed.
- Live packet capture driver invocation and raw PCAP remote delete/export propagation remain unclaimed.
- Live local-AI model execution and full production policy engine execution remain unclaimed.
- Live platform adapter execution, host mutation, packet blocking, and production enforcement remain unclaimed.
- Broader risk-budget, audit narrative, and production performance SLO UI remain open.
- Pipeline-level exact video/private-message/search content claim rejection and the manual/deferred owner ledger are pending branch-only evidence until those branches integrate.
`;
}

function gateLog(id) {
  const gate = findGate(id);
  return [
    `${id}`,
    `status=${gate.status}`,
    `summary=${gate.summary}`,
    '',
    'sources:',
    ...gate.sources.map((source) => `- ${source}`),
    '',
    'not-claimed:',
    ...proofNotClaimed(id).map((claim) => `- ${claim}`),
    '',
  ].join('\n');
}

function gateJson(id) {
  const gate = findGate(id);
  return {
    gate: id,
    status: gate.status,
    summary: gate.summary,
    sources: gate.sources,
    notClaimed: proofNotClaimed(id),
  };
}

function uiSnapshotReadme() {
  const gate = findGate('08-ui-snapshots/README.md');
  return `# Network Proof Pack UI Snapshot Reconciliation

Status: ${gate.status}

Current-main UI proof:

- output/network-plan-proof/36-parent-ui-network-evidence-drawer/08-ui-snapshots/network-evidence-drawer.png
- output/network-plan-proof/36-parent-ui-network-evidence-drawer/proof-summary.json
- output/network-plan-proof/portal-risk-performance-platform-status/proof-summary.json

Partial reason:

- The current service-backed drawer and platform/status proof cover endpoint/domain/process attribution, active/tombstone/exportable counts, retention delete refs, and degraded adapter state.
- Broader risk-budget scoring UI, AI audit narrative UI, production performance SLO UI, and live manual-required host adapter execution remain open.
`;
}

function manualDeferredFollowups() {
  return {
    status: 'current-main-partial',
    currentMainCoverage: [
      'proof-pack reconciliation records manual-required and unavailable gaps for adapter, journal, UI, security, and production gates',
      'existing proof summaries name not-claimed boundaries and manual-required platform rows',
    ],
    pendingBranchEvidence: pendingBranchChecks,
    followUps: [
      {
        owner: 'E-D network runtime lane',
        gap: 'broker/family-hub delivery implementation',
        requiredBeforeClaim: [
          'custody refs',
          'auth/encryption refs',
          'retention/replay/delete refs',
          'relay/family identity refs',
        ],
      },
      {
        owner: 'platform adapter lanes',
        gap: 'live DNS/firewall/WFP/VpnService/NetworkExtension/Linux adapter execution',
        requiredBeforeClaim: ['device/permission artifacts', 'apply/result/rollback logs', 'audit refs'],
      },
      {
        owner: 'E-D or primary integration',
        gap: 'pipeline-level exact video/private-message/search content-claim invariant',
        requiredBeforeClaim: ['merge codex/network-content-claim-invariant-proof or equivalent current-main proof'],
      },
      {
        owner: 'E-D or primary integration',
        gap: 'manual/deferred follow-up owner ledger',
        requiredBeforeClaim: [
          'merge codex/network-manual-followup-owner-ledger-proof or equivalent current-main proof',
        ],
      },
    ],
  };
}

function validationCommandsLog() {
  return [
    'network-proof-pack-reconciliation validation commands',
    '',
    'Inside this harness:',
    '- git ls-files --error-unmatch for every cited source path',
    '- JSON.parse for every cited JSON source path',
    '- git ls-remote origin for pending branch heads',
    '',
    'Expected handoff validation outside this harness:',
    '- node --check scripts/test/network-proof-pack-reconciliation.mjs',
    '- node scripts/test/network-proof-pack-reconciliation.mjs',
    '- git diff --check',
    '- git diff --check origin/main..HEAD',
    '- cmd /c npm run lanes:guard',
    '- cmd /c npm run hub:guard',
    '',
  ].join('\n');
}

function proofNotClaimed(id) {
  const common = [
    'exact URL/page/content from network-only evidence',
    'decrypted payload capture',
    'UI, AI, or network bypass of policy authority',
    'weak-evidence enforcement',
  ];
  if (id === '06-adapter-action-proof.json') {
    return [...common, 'live adapter execution', 'host mutation', 'published enforcement command'];
  }
  if (id === '07-journal-sqlite-proof.json') {
    return [...common, 'raw PCAP remote delete/export propagation', 'broker/family-hub retention propagation'];
  }
  if (id === '08-ui-snapshots/README.md') {
    return [...common, 'portal policy execution', 'portal adapter execution', 'production SLO UI completion'];
  }
  if (id === '09-security-negative-proof.log') {
    return [...common, 'pipeline-level exact video content rejection on current main'];
  }
  if (id === '10-performance-proof.log') {
    return [...common, 'production SLO validation', 'real-time production guarantee'];
  }
  return common;
}

function findGate(id) {
  const gate = gates.find((candidate) => candidate.id === id);
  if (!gate) {
    throw new Error(`unknown gate ${id}`);
  }
  return gate;
}

function writeMarkdown(relativePath, content) {
  const target = `${proofRoot}/${relativePath}`;
  mkdirSync(dirname(localPath(target)), { recursive: true });
  writeFileSync(localPath(target), content.endsWith('\n') ? content : `${content}\n`);
}

function writeJson(relativePath, value) {
  const target = `${proofRoot}/${relativePath}`;
  mkdirSync(dirname(localPath(target)), { recursive: true });
  writeFileSync(localPath(target), `${JSON.stringify(value, null, 2)}\n`);
}

function localPath(repoPath) {
  return join(...repoPath.split('/'));
}

function runText(command, args) {
  const result = spawnSync(command, args, { encoding: 'utf8', shell: false });
  if (result.status !== 0) {
    throw new Error(`${command} ${args.join(' ')} failed with exit ${result.status}: ${result.stderr ?? ''}`);
  }
  return `${result.stdout ?? ''}${result.stderr ?? ''}`;
}
