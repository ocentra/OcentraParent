import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { mkdirSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

const proofRoot = join('output', 'network-plan-proof', '46-ai-detection-fixture-proof');
const testRoot = join('test-results', 'network-ai-detection-fixture-proof');
const proofRevision = 'network-ai-detection-fixture-proof/v1';
const proofBranch = 'codex/network-ai-proof-artifacts';
const deterministicCheckedAt = `deterministic:${proofRevision}`;
mkdirSync(proofRoot, { recursive: true });
mkdirSync(testRoot, { recursive: true });

writeFileSync(
  join(proofRoot, 'expected-ai-detection-fixture-gate.json'),
  `${JSON.stringify(
    {
      fixtureSetRef: 'network-ai-fixtures-row46',
      acceptedInputs: ['summary refs', 'evidence refs', 'fixture labels', 'analyzer alert refs'],
      metrics: ['precision', 'recall', 'accuracy', 'average confidence drift'],
      thresholds: {
        precisionBasisPoints: 6000,
        recallBasisPoints: 6000,
        averageDriftBasisPoints: 500,
      },
      explicitStates: [
        'meets-fixture-gate',
        'below-quality-threshold',
        'drift-exceeded',
        'below-quality-and-drift-exceeded',
      ],
      unsupportedInputsRejected: [
        'model execution claim',
        'remote AI claim',
        'raw PCAP input',
        'exact URL claim',
        'page content',
        'decrypted payload',
      ],
      authorityBoundary:
        'fixture evaluation creates detection evidence only; it does not publish policy, adapter, or enforcement commands',
    },
    null,
    2
  )}\n`
);

const commands = [
  {
    name: 'network-ai-detection-fixture-tests',
    command: 'cargo',
    args: ['test', '-p', 'ocentra-network-evidence', 'ai_detection'],
    log: join(proofRoot, 'ai-detection-fixture-tests.log'),
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
  proof: 'network-ai-detection-fixture-proof',
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
    expectedAiDetectionFixtureGate: join(proofRoot, 'expected-ai-detection-fixture-gate.json'),
    proofSummary: join(proofRoot, 'proof-summary.json'),
    testProof: join(testRoot, 'proof.json'),
  },
  provenRows: ['46 AI detection model fixture evaluation and drift/precision proof'],
  provenMetrics: ['precision', 'recall', 'accuracy', 'average-confidence-drift'],
  notClaimed: [
    'local AI model execution or worker runtime',
    'remote AI provider invocation',
    'raw PCAP, page content, exact URL, message content, search query, or decrypted payload in AI input',
    'production model quality or hosted drift monitoring',
    'policy decision authority',
    'adapter action, blocking, termination, or enforcement command authorization',
  ],
};
writeFileSync(join(proofRoot, 'proof-summary.json'), `${JSON.stringify(proof, null, 2)}\n`);
writeFileSync(join(testRoot, 'proof.json'), `${JSON.stringify(proof, null, 2)}\n`);
console.log('network-ai-detection-fixture-proof-ok:ai-detection-tests,clippy,source-shape');
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
