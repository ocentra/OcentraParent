import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { mkdirSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

const proofRoot = join('output', 'network-plan-proof', '50-security-readiness-proof');
const testRoot = join('test-results', 'network-security-readiness-proof');
const proofRevision = 'network-security-readiness-proof/v1';
const proofBranch = 'codex/network-security-proof-artifacts';
const deterministicCheckedAt = `deterministic:${proofRevision}`;
mkdirSync(proofRoot, { recursive: true });
mkdirSync(testRoot, { recursive: true });

writeFileSync(
  join(proofRoot, 'expected-security-readiness-boundary.json'),
  `${JSON.stringify(
    {
      reportRef: 'network-readiness-row50',
      requiredGates: [
        'security threat model',
        'privacy and compliance review',
        'retention delete export custody',
        'key rotation and secret handling',
        'rule and model provenance rollback',
        'deployment rollback',
        'support and staff training',
        'staged rollout',
        'known gap signoff',
      ],
      productionGate:
        'Production rollout remains blocked unless external audit or penetration-test signoff proof artifact refs are supplied.',
      externalSignoffArtifactGate:
        'External signoff proof requires signoff, artifact, digest, and scope refs before production-ready state can be represented.',
      unsupportedClaimsRejected: [
        'default remote upload',
        'raw PCAP without custody',
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
        'Readiness proof records gate refs and blocked/ready states only; it does not deploy, enable uploads, publish commands, or claim production rollout without external signoff artifact proof.',
    },
    null,
    2
  )}\n`
);

const commands = [
  {
    name: 'network-security-readiness-tests',
    command: 'cargo',
    args: ['test', '-p', 'ocentra-network-evidence', 'readiness_proof'],
    log: join(proofRoot, 'security-readiness-tests.log'),
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
const sourceTreeFingerprint = trackedSourceFingerprint();

const proof = {
  proof: 'network-security-readiness-proof',
  proofRevision,
  checkedAt: deterministicCheckedAt,
  branch: proofBranch,
  sourceCommit: `source-tree:${sourceTreeFingerprint}`,
  sourceTreeFingerprint,
  artifactCommit: 'see the enclosing git commit for generated proof artifacts',
  originMain: runText('git', ['rev-parse', 'origin/main']).trim(),
  mergeBase: runText('git', ['merge-base', 'HEAD', 'origin/main']).trim(),
  sourceStatusShort: sourceStatusShort(),
  proofRoot,
  testRoot,
  commands: commandResults,
  artifacts: {
    expectedSecurityReadinessBoundary: join(proofRoot, 'expected-security-readiness-boundary.json'),
    proofSummary: join(proofRoot, 'proof-summary.json'),
    testProof: join(testRoot, 'proof.json'),
  },
  provenRows: ['50 Security, privacy, compliance, deployment, support, and staged rollout proof'],
  provenBoundaries: [
    'security, privacy, compliance, retention, hardening, support, and rollout gate refs',
    'production blocked without external audit or penetration-test signoff proof',
    'production-ready state only with external signoff artifact proof refs',
    'unsupported content/upload/authority/enforcement claim rejection',
  ],
  notClaimed: [
    'production deployment or rollout execution',
    'external audit completion unless signoff, artifact, digest, and scope refs are supplied',
    'default remote upload of child network evidence',
    'raw PCAP without custody',
    'exact URL, page content, private message, search query, or decrypted payload availability',
    'policy or adapter authority',
    'enforcement command publication',
    'support materials authored in full outside referenced proof refs',
  ],
};
writeFileSync(join(proofRoot, 'proof-summary.json'), `${JSON.stringify(proof, null, 2)}\n`);
writeFileSync(join(testRoot, 'proof.json'), `${JSON.stringify(proof, null, 2)}\n`);
console.log('network-security-readiness-proof-ok:readiness-tests,clippy,source-shape');
console.log(`proof=${join(proofRoot, 'proof-summary.json')}`);

function runCommand(entry) {
  const result = spawnSync(entry.command, entry.args, { encoding: 'utf8', shell: false });
  if (result.status !== 0) {
    writeFileSync(entry.log, `${result.stdout ?? ''}${result.stderr ?? ''}`);
    throw new Error(`${entry.name} failed with exit ${result.status}`);
  }
  writeFileSync(entry.log, `${entry.name}=passed\ncommand=${[entry.command, ...entry.args].join(' ')}\n`);
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
    ':(exclude)output',
    ':(exclude)test-results',
    ':(exclude)output/network-plan-proof/50-security-readiness-proof',
    ':(exclude)test-results/network-security-readiness-proof',
  ]);
}

function trackedSourceFingerprint() {
  return createHash('sha256')
    .update(runText('git', ['ls-files', '-s', '--', '.', ':(exclude)output', ':(exclude)test-results']))
    .digest('hex');
}
