import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { mkdirSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

const proofRoot = join('output', 'network-plan-proof', '47-ai-audit-narrative-proof');
const testRoot = join('test-results', 'network-ai-audit-narrative-proof');
const proofRevision = 'network-ai-audit-narrative-proof/v1';
const proofBranch = 'codex/network-ai-proof-artifacts';
const deterministicCheckedAt = `deterministic:${proofRevision}`;
mkdirSync(proofRoot, { recursive: true });
mkdirSync(testRoot, { recursive: true });

writeFileSync(
  join(proofRoot, 'expected-ai-audit-narrative-boundary.json'),
  `${JSON.stringify(
    {
      reportRef: 'network-ai-audit-row47',
      acceptedInputs: ['detection refs', 'evidence refs', 'analyzer alert refs', 'parent rule refs'],
      narrativeStates: ['ready', 'uncertain-review-required', 'monitor-only'],
      recommendationKinds: [
        'review-with-parent',
        'review-policy-rule',
        'confirm-with-managed-browser',
        'confirm-with-screen-summary',
        'monitor-only',
      ],
      requiredCitations: ['detection refs', 'evidence refs', 'parent rule refs'],
      unsupportedClaimsRejected: [
        'remote AI invocation',
        'raw PCAP input',
        'exact URL',
        'page content',
        'private message',
        'search query',
        'decrypted payload',
        'policy authority',
        'adapter authority',
        'enforcement command',
      ],
      authorityBoundary:
        'AI audit narratives and recommendations are advisory evidence only; policy and adapter proof remain the authority',
    },
    null,
    2
  )}\n`
);

const commands = [
  {
    name: 'network-ai-audit-narrative-tests',
    command: 'cargo',
    args: ['test', '-p', 'ocentra-network-evidence', 'ai_audit'],
    log: join(proofRoot, 'ai-audit-narrative-tests.log'),
  },
  {
    name: 'network-evidence-clippy',
    command: 'cargo',
    args: ['clippy', '-p', 'ocentra-network-evidence', '--all-targets', '--', '-D', 'warnings'],
    log: join(proofRoot, 'clippy.log'),
  },
  {
    name: 'source-shape',
    command: 'node',
    args: ['scripts/check-source-shape.mjs'],
    log: join(proofRoot, 'source-shape.log'),
  },
];
const commandResults = commands.map(runCommand);

const proof = {
  proof: 'network-ai-audit-narrative-proof',
  proofRevision,
  checkedAt: deterministicCheckedAt,
  branch: proofBranch,
  sourceCommit: `source-tree:${sourceTreeFingerprint()}`,
  sourceTreeFingerprint: sourceTreeFingerprint(),
  artifactCommit: 'see the enclosing git commit for generated proof artifacts',
  originMain: runText('git', ['rev-parse', 'origin/main']).trim(),
  mergeBase: runText('git', ['merge-base', 'HEAD', 'origin/main']).trim(),
  sourceStatusShort: sourceStatusShort(),
  proofRoot,
  testRoot,
  commands: commandResults,
  artifacts: {
    expectedAiAuditNarrativeBoundary: join(proofRoot, 'expected-ai-audit-narrative-boundary.json'),
    proofSummary: join(proofRoot, 'proof-summary.json'),
    testProof: join(testRoot, 'proof.json'),
  },
  provenRows: ['47 AI audit narrative and recommendation proof'],
  provenBoundaries: [
    'parent-readable narrative with cited evidence refs',
    'advisory recommendation refs',
    'uncertainty confirmation route',
    'unsupported-claim rejection',
  ],
  notClaimed: [
    'remote AI provider invocation',
    'raw PCAP, page content, exact URL, private message, search query, or decrypted payload in AI audit input',
    'policy decision authority',
    'adapter action, blocking, termination, or enforcement command authorization',
    'portal AI audit UI rendering',
    'production model quality or hosted audit workflow',
  ],
};
writeFileSync(join(proofRoot, 'proof-summary.json'), `${JSON.stringify(proof, null, 2)}\n`);
writeFileSync(join(testRoot, 'proof.json'), `${JSON.stringify(proof, null, 2)}\n`);
console.log('network-ai-audit-narrative-proof-ok:ai-audit-tests,clippy,source-shape');
console.log(`proof=${join(proofRoot, 'proof-summary.json')}`);

function runCommand(entry) {
  const result = spawnSync(entry.command, entry.args, { encoding: 'utf8', shell: false });
  if (result.status !== 0) {
    writeFileSync(entry.log, `${result.stdout ?? ''}${result.stderr ?? ''}`);
    throw new Error(`${entry.name} failed with exit ${result.status}`);
  }
  writeFileSync(entry.log, stableCommandLog(entry, result.status));
  return {
    name: entry.name,
    command: [entry.command, ...entry.args].join(' '),
    status: result.status,
    log: entry.log,
  };
}

function stableCommandLog(entry, status) {
  return [
    `name=${entry.name}`,
    `command=${[entry.command, ...entry.args].join(' ')}`,
    `status=${status}`,
    'result=passed',
    'note=live command output is intentionally not embedded because cargo timing and target cache lines are nondeterministic',
    '',
  ].join('\n');
}

function runText(command, args) {
  const result = spawnSync(command, args, { encoding: 'utf8', shell: false });
  if (result.status !== 0) {
    throw new Error(`${command} ${args.join(' ')} failed with exit ${result.status}`);
  }
  return `${result.stdout ?? ''}${result.stderr ?? ''}`;
}

function sourceStatusShort() {
  return runText('git', ['status', '--short', '--', '.', ':(exclude)output', ':(exclude)test-results']);
}

function sourceTreeFingerprint() {
  const sourceIndex = runText('git', ['ls-files', '-s', '--', '.', ':(exclude)output', ':(exclude)test-results']);
  return createHash('sha256').update(sourceIndex).digest('hex');
}
