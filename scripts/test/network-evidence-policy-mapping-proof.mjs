import { spawnSync } from 'node:child_process';
import { mkdirSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

const proofRoot = join('output', 'network-plan-proof', '34-evidence-grade-policy-mapping');
const testRoot = join('test-results', 'network-evidence-policy-mapping-proof');
mkdirSync(proofRoot, { recursive: true });
mkdirSync(testRoot, { recursive: true });

writeFileSync(
  join(proofRoot, 'expected-evidence-grade-policy-mapping.json'),
  `${JSON.stringify(
    {
      requiredRefs: ['policy-decision-ref', 'parent-rule-ref', 'evidence-refs'],
      gradeMapping: {
        A: 'dry-run requested action, no adapter or enforcement command',
        B: 'dry-run monitor/warn, parent-review for limit/block',
        C: 'parent-review only',
        D: 'observe-only none',
      },
      adapterState: 'never-authorized in this row; adapter proof remains a later row',
      enforcementState: 'never-authorized in this row',
      localAiRefs: 'optional local-AI result refs are refs only and must be non-empty when present',
    },
    null,
    2
  )}\n`
);

const commands = [
  {
    name: 'network-evidence-policy-mapping-tests',
    command: 'cargo',
    args: ['test', '-p', 'ocentra-network-evidence', 'policy_mapping'],
    log: join(proofRoot, 'policy-mapping-tests.log'),
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
  proof: 'network-evidence-policy-mapping',
  checkedAt: new Date().toISOString(),
  branch: runText('git', ['branch', '--show-current']).trim(),
  commit: runText('git', ['rev-parse', 'HEAD']).trim(),
  statusShort: runText('git', ['status', '--short']),
  proofRoot,
  testRoot,
  commands: commandResults,
  artifacts: {
    expectedEvidenceGradePolicyMapping: join(proofRoot, 'expected-evidence-grade-policy-mapping.json'),
    proofSummary: join(proofRoot, 'proof-summary.json'),
    testProof: join(testRoot, 'proof.json'),
  },
  provenRows: ['34 Evidence-grade policy mapping'],
  notClaimed: [
    'adapter execution or adapter authorization',
    'enforcement command publication',
    'policy engine completeness',
    'portal UI rendering',
    'host DNS/firewall filtering',
    'AI model execution',
  ],
};
writeFileSync(join(proofRoot, 'proof-summary.json'), `${JSON.stringify(proof, null, 2)}\n`);
writeFileSync(join(testRoot, 'proof.json'), `${JSON.stringify(proof, null, 2)}\n`);
console.log('network-evidence-policy-mapping-proof-ok:policy-tests,clippy,source-shape');
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
