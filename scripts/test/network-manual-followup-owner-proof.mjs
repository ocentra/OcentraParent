import { spawnSync } from 'node:child_process';
import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

const proofRoot = join('output', 'network-plan-proof', 'manual-followup-owner-ledger');
const testRoot = join('test-results', 'network-manual-followup-owner-proof');
mkdirSync(proofRoot, { recursive: true });
mkdirSync(testRoot, { recursive: true });

const sourceBranch = runText('git', ['branch', '--show-current']).trim();
const sourceCommit = runText('git', ['rev-parse', 'HEAD']).trim();
const sourceOriginMain = runText('git', ['rev-parse', 'origin/main']).trim();
const sourceMergeBase = runText('git', ['merge-base', 'HEAD', 'origin/main']).trim();
const sourceStatusShort = readSourceStatusShort();

const ledgerEntries = [
  manualRow({
    id: 'row10-broker-family-hub-delivery',
    state: 'deferred',
    reason:
      'Live broker and family-hub delivery require custody, auth, encryption, offset, replay, deletion, dedupe, broker config, family identity, and relay-policy artifacts before product delivery can be claimed.',
    followUpOwner: 'E-D network remote-delivery sequencing',
    followUpPath: 'row10b/row10f/row10g/row10h split branches after dependency integration',
    sourceRefs: [
      'docs/features/network-domain-control.md',
      'docs/plans/network-plan/implementation-checklist.md',
      'docs/plans/network-plan/workpacks/README.md',
    ],
    noClaimBoundary: ['no live broker transport', 'no family-hub relay delivery', 'no remote adapter action'],
  }),
  manualRow({
    id: 'raw-capture-retention-delete-export',
    state: 'manual-required',
    reason:
      'Raw PCAP/live-capture retention, remote deletion, and export propagation need live capture artifacts, encrypted storage evidence, quota rotation, custody chain, and private-traffic exclusion proof.',
    followUpOwner: 'E-D network custody and platform adapters',
    followUpPath: 'row13 live-capture custody plus remote delete/export propagation proof',
    sourceRefs: [
      'docs/features/network-domain-control.md',
      'docs/plans/network-plan/implementation-checklist.md',
      'output/network-plan-proof/52-platform-claims-proof/11-manual-platform-proof.md',
    ],
    noClaimBoundary: [
      'no raw PCAP default storage',
      'no production retention claim',
      'no remote deletion propagation claim',
    ],
  }),
  manualRow({
    id: 'live-analyzer-model-policy-execution',
    state: 'deferred',
    reason:
      'Analyzer, local-AI model, and full policy engine execution remain fixture-backed or refs-only until live runtime execution and policy authority proof exists.',
    followUpOwner: 'E-D network AI/policy runtime',
    followUpPath: 'live analyzer/model/policy execution proof after local runtime integration',
    sourceRefs: [
      'docs/features/network-domain-control.md',
      'docs/plans/network-plan/implementation-checklist.md',
      'output/network-plan-proof/51-end-to-end-pipeline-proof/proof-summary.json',
    ],
    noClaimBoundary: ['no live local model execution', 'no remote AI invocation', 'no policy engine execution claim'],
  }),
  manualRow({
    id: 'host-adapter-execution',
    state: 'manual-required',
    reason:
      'DNS, Firewall, WFP, Android VpnService, Apple Network Extension, and Linux adapter rows require exact OS/device/permission/apply/rollback/audit artifacts before host mutation.',
    followUpOwner: 'E-D platform adapter proof owners',
    followUpPath: 'row37-row42 live adapter execution follow-up after platform proof artifacts',
    sourceRefs: [
      'docs/expectations/enforcement.md',
      'output/network-plan-proof/11-manual-platform-proof/11-manual-platform-proof.md',
      'output/network-plan-proof/52-platform-claims-proof/proof-summary.json',
    ],
    noClaimBoundary: ['no host DNS mutation', 'no packet blocking', 'no enforcement command publication'],
  }),
  manualRow({
    id: 'portal-risk-performance-platform-rendering',
    state: 'deferred',
    reason:
      'The current portal drawer renders service network read-model evidence, but broader risk-budget, performance, manual-required, degraded, and platform-state UI coverage is still unrendered.',
    followUpOwner: 'E-D network portal readiness',
    followUpPath: 'portal risk/performance/platform status rendering proof',
    sourceRefs: ['docs/features/network-domain-control.md', 'docs/plans/network-plan/implementation-checklist.md'],
    noClaimBoundary: [
      'no portal policy authority',
      'no local evidence-grade computation in UI',
      'no adapter command dispatch',
    ],
  }),
  manualRow({
    id: 'production-security-support-external-signoff',
    state: 'manual-required',
    reason:
      'Production-ready claims require external audit or penetration-test signoff plus full support, incident, staged-rollout, and training artifacts.',
    followUpOwner: 'Primary release/support coordination with E-D network evidence',
    followUpPath: 'external audit/support rollout proof after production scope is authorized',
    sourceRefs: [
      'docs/features/network-domain-control.md',
      'docs/plans/network-plan/implementation-checklist.md',
      'output/network-plan-proof/11a-hardening-support-proof/proof-summary.json',
    ],
    noClaimBoundary: [
      'no production deployment claim',
      'no external audit execution claim',
      'no default remote upload',
    ],
  }),
  manualRow({
    id: 'ui-screenshot-na-for-non-ui-proof-rows',
    state: 'skipped-non-ui',
    reason:
      'Rust manifest and backend-only proof rows do not change a portal surface, so UI screenshots are explicitly not applicable and proof logs remain the evidence.',
    followUpOwner: 'Owning row implementer when UI changes',
    followUpPath: 'add portal screenshot proof only in branches that change UI rendering',
    sourceRefs: [
      'docs/plans/network-plan/implementation-checklist.md',
      'output/network-plan-proof/52-platform-claims-proof/11-manual-platform-proof.md',
    ],
    noClaimBoundary: ['no UI rendering claim without screenshot or e2e proof'],
  }),
];

validateLedger(ledgerEntries);
assertDocsMentionOpenFollowups();

const commands = [
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

writeJson(join(proofRoot, 'manual-followup-owner-ledger.json'), { ledgerEntries });
writeFileSync(join(proofRoot, 'manual-followup-owner-ledger.md'), renderLedgerMarkdown(ledgerEntries));

const proof = {
  proof: 'network-manual-followup-owner-proof',
  checkedAt: new Date().toISOString(),
  branch: sourceBranch,
  sourceCommit,
  artifactCommit: 'see the enclosing git commit for generated proof artifacts',
  originMain: sourceOriginMain,
  mergeBase: sourceMergeBase,
  sourceStatusShort,
  proofRoot,
  testRoot,
  commands: commandResults,
  artifacts: {
    ledgerJson: join(proofRoot, 'manual-followup-owner-ledger.json'),
    ledgerMarkdown: join(proofRoot, 'manual-followup-owner-ledger.md'),
    proofSummary: join(proofRoot, 'proof-summary.json'),
    testProof: join(testRoot, 'proof.json'),
  },
  provenGates: [
    'every manual/deferred/skipped network proof item carries a reason',
    'every manual/deferred/skipped network proof item carries a follow-up owner',
    'every manual/deferred/skipped network proof item carries source refs',
    'every manual/deferred/skipped network proof item carries a no-claim boundary',
  ],
  knownGaps: ledgerEntries.map((entry) => ({
    id: entry.id,
    state: entry.state,
    owner: entry.followUpOwner,
  })),
};

writeJson(join(proofRoot, 'proof-summary.json'), proof);
writeJson(join(testRoot, 'proof.json'), proof);
console.log('network-manual-followup-owner-proof-ok:ledger,source-shape,diff-check');
console.log(`proof=${join(proofRoot, 'proof-summary.json')}`);

function manualRow(row) {
  return row;
}

function validateLedger(entries) {
  const ids = new Set();
  for (const entry of entries) {
    assertPresent(entry.id, 'id');
    if (ids.has(entry.id)) {
      throw new Error(`duplicate ledger id: ${entry.id}`);
    }
    ids.add(entry.id);
    assertPresent(entry.state, `${entry.id}.state`);
    assertPresent(entry.reason, `${entry.id}.reason`);
    assertPresent(entry.followUpOwner, `${entry.id}.followUpOwner`);
    assertPresent(entry.followUpPath, `${entry.id}.followUpPath`);
    assertList(entry.sourceRefs, `${entry.id}.sourceRefs`);
    assertList(entry.noClaimBoundary, `${entry.id}.noClaimBoundary`);
    for (const sourceRef of entry.sourceRefs) {
      if (!existsSync(sourceRef)) {
        throw new Error(`${entry.id}.sourceRefs missing file: ${sourceRef}`);
      }
      assertTrackedSourceRef(entry.id, sourceRef);
    }
  }
}

function assertTrackedSourceRef(id, sourceRef) {
  const tracked = spawnSync('git', ['ls-files', '--error-unmatch', sourceRef], {
    encoding: 'utf8',
    shell: false,
  });
  if (tracked.status !== 0) {
    throw new Error(`${id}.sourceRefs untracked file: ${sourceRef}`);
  }
}

function assertDocsMentionOpenFollowups() {
  const featureDoc = readFileSync('docs/features/network-domain-control.md', 'utf8');
  const checklist = readFileSync('docs/plans/network-plan/implementation-checklist.md', 'utf8');
  const requiredPhrases = [
    'Broker/family-hub delivery implementation',
    'live adapter execution',
    'portal risk-budget/performance UI rendering',
    'Every failed, skipped, manual, or deferred network proof item',
  ];
  for (const phrase of requiredPhrases) {
    if (!featureDoc.includes(phrase) && !checklist.includes(phrase)) {
      throw new Error(`missing manual-followup source phrase: ${phrase}`);
    }
  }
}

function renderLedgerMarkdown(entries) {
  const rows = entries.map((entry) =>
    [
      `## ${entry.id}`,
      '',
      `- State: ${entry.state}`,
      `- Reason: ${entry.reason}`,
      `- Follow-up owner: ${entry.followUpOwner}`,
      `- Follow-up path: ${entry.followUpPath}`,
      `- Source refs: ${entry.sourceRefs.join(', ')}`,
      `- No-claim boundary: ${entry.noClaimBoundary.join(', ')}`,
      '',
    ].join('\n')
  );
  return `# Network Manual Follow-up Owner Ledger\n\n${rows.join('\n')}`;
}

function runCommand(entry) {
  const result = spawnSync(entry.command, entry.args, { encoding: 'utf8', shell: false });
  writeLog(entry.log, `${result.stdout ?? ''}${result.stderr ?? ''}`);
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

function readSourceStatusShort() {
  return runText('git', [
    'status',
    '--short',
    '--',
    '.',
    ':(exclude)output/network-plan-proof/manual-followup-owner-ledger',
    ':(exclude)test-results/network-manual-followup-owner-proof',
  ]);
}

function assertPresent(value, label) {
  if (typeof value !== 'string' || value.trim().length === 0) {
    throw new Error(`missing ${label}`);
  }
}

function assertList(value, label) {
  if (
    !Array.isArray(value) ||
    value.length === 0 ||
    value.some((item) => typeof item !== 'string' || item.trim().length === 0)
  ) {
    throw new Error(`missing ${label}`);
  }
}

function writeJson(path, value) {
  writeFileSync(path, `${JSON.stringify(value, null, 2)}\n`);
}

function writeLog(path, value) {
  const trimmed = value.trimEnd();
  writeFileSync(path, trimmed.length > 0 ? `${trimmed}\n` : '');
}
