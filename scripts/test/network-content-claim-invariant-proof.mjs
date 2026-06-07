import { spawnSync } from 'node:child_process';
import { mkdirSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

const proofRoot = join('output', 'network-plan-proof', '54-content-claim-invariant-proof');
const testRoot = join('test-results', 'network-content-claim-invariant-proof');
mkdirSync(proofRoot, { recursive: true });
mkdirSync(testRoot, { recursive: true });

writeFileSync(
  join(proofRoot, 'expected-content-claim-invariants.json'),
  `${JSON.stringify(
    {
      rejectedNetworkOnlyClaims: [
        'raw network payload',
        'decrypted payload',
        'page content',
        'video content',
        'private message',
        'search query',
        'exact URL',
      ],
      rejectedBypassClaims: [
        'AI policy authority',
        'UI policy authority',
        'network adapter authority',
        'enforcement command publication',
      ],
      downstreamNoClaimBoundaries: [
        'local AI receives summary and evidence refs only',
        'AI audit rejects private message and search query claims',
        'risk budget rejects private message and search query claims',
        'DNS adapter proof rejects exact URL, page content, and decrypted payload claims',
      ],
      notClaimed: [
        'network-only exact page/video/message/search content',
        'decrypted payload inspection',
        'raw packet payload sent to AI',
        'policy execution',
        'adapter action execution',
        'enforcement command publication',
      ],
    },
    null,
    2
  )}\n`
);

const commands = [
  {
    name: 'network-content-claim-pipeline-tests',
    command: 'cargo',
    args: ['test', '-p', 'ocentra-network-evidence', 'end_to_end_pipeline'],
    log: join(proofRoot, 'pipeline-tests.log'),
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
  {
    name: 'diff-check',
    command: 'git',
    args: ['diff', '--check'],
    log: join(proofRoot, 'diff-check.log'),
  },
];
const commandResults = commands.map(runCommand);

const proof = {
  proof: 'network-content-claim-invariant-proof',
  checkedAt: new Date().toISOString(),
  branch: runText('git', ['branch', '--show-current']).trim(),
  sourceCommit: runText('git', ['rev-parse', 'HEAD']).trim(),
  artifactCommit: 'see the enclosing git commit for generated proof artifacts',
  originMain: runText('git', ['rev-parse', 'origin/main']).trim(),
  mergeBase: runText('git', ['merge-base', 'origin/main', 'HEAD']).trim(),
  sourceStatusShort: sourceStatusShort(),
  proofRoot,
  testRoot,
  commands: commandResults,
  artifacts: {
    expectedContentClaimInvariants: join(proofRoot, 'expected-content-claim-invariants.json'),
    proofSummary: join(proofRoot, 'proof-summary.json'),
    testProof: join(testRoot, 'proof.json'),
  },
  provenRows: ['54 Content claim invariant proof'],
  provenRootGates: ['09-security-negative-proof.log'],
  provenInvariants: [
    'network-only evidence cannot claim exact URL',
    'network-only evidence cannot claim page content',
    'network-only evidence cannot claim video content',
    'network-only evidence cannot claim private message content',
    'network-only evidence cannot claim search query content',
    'raw packet or decrypted payload claims are rejected before the product path composes',
    'AI, UI, and network surfaces cannot bypass policy or publish enforcement commands',
  ],
  notClaimed: [
    'live packet capture driver invocation',
    'local model execution',
    'full policy engine execution',
    'host adapter mutation',
    'broker or family-hub delivery',
    'portal risk-budget/performance UI rendering',
    'production enforcement',
  ],
};
writeFileSync(join(proofRoot, 'proof-summary.json'), `${JSON.stringify(proof, null, 2)}\n`);
writeFileSync(join(testRoot, 'proof.json'), `${JSON.stringify(proof, null, 2)}\n`);
console.log('network-content-claim-invariant-proof-ok:pipeline-tests,clippy,source-shape,diff-check');
console.log(`proof=${join(proofRoot, 'proof-summary.json')}`);

function runCommand(entry) {
  const result = spawnSync(entry.command, entry.args, { encoding: 'utf8', shell: false });
  writeFileSync(entry.log, `${result.stdout ?? ''}${result.stderr ?? ''}`);
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

function sourceStatusShort() {
  return runText('git', [
    'status',
    '--short',
    '--',
    '.',
    ':(exclude)output/network-plan-proof/54-content-claim-invariant-proof',
    ':(exclude)test-results/network-content-claim-invariant-proof',
  ]);
}
