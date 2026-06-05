import { spawnSync } from 'node:child_process';
import { mkdirSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

const proofRoot = join('output', 'network-plan-proof', '51-end-to-end-pipeline-proof');
const testRoot = join('test-results', 'network-end-to-end-pipeline-proof');
mkdirSync(proofRoot, { recursive: true });
mkdirSync(testRoot, { recursive: true });

writeFileSync(
  join(proofRoot, 'expected-end-to-end-pipeline.json'),
  `${JSON.stringify(
    {
      path: [
        'trigger-ref',
        'typed-event-ref',
        'evidence-bundle',
        'local-ai-queue-refs-only',
        'ai-detection',
        'ai-audit',
        'risk-budget',
        'policy-decision',
        'adapter-proof-state',
        'audit-event',
        'portal-read-model',
        'retention-delete-export',
      ],
      requiredRefFamilies: [
        'trigger',
        'typed event',
        'evidence',
        'AI detection/audit',
        'policy decision',
        'adapter capability/action artifacts',
        'audit',
        'portal/read-model',
        'retention/delete/export',
      ],
      noBypassInvariants: [
        'weak or unavailable evidence cannot authorize adapter apply',
        'AI remains advisory',
        'portal/UI has no policy authority',
        'network evidence has no adapter authority',
        'no enforcement command is published by this proof path',
      ],
      notClaimed: [
        'live packet capture driver invocation',
        'local model execution',
        'full policy engine execution',
        'host DNS/firewall mutation',
        'broker or family-hub delivery',
        'portal risk-budget/performance UI rendering',
      ],
    },
    null,
    2
  )}\n`
);

const commands = [
  {
    name: 'agent-core-network-runtime-typed-event-refs',
    command: 'cargo',
    args: [
      'test',
      '-p',
      'ocentra-parent-agent-core',
      'network_runtime_chain_carries_exact_refs_without_direct_enforcement_shortcut',
    ],
    log: join(proofRoot, 'agent-core-typed-event-refs.log'),
  },
  {
    name: 'agent-core-network-runtime-weak-evidence-no-enforcement',
    command: 'cargo',
    args: [
      'test',
      '-p',
      'ocentra-parent-agent-core',
      'manual_required_network_evidence_does_not_publish_enforcement_command',
    ],
    log: join(proofRoot, 'agent-core-weak-evidence-no-enforcement.log'),
  },
  {
    name: 'network-evidence-end-to-end-pipeline-tests',
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
];
const commandResults = commands.map(runCommand);

const proof = {
  proof: 'network-end-to-end-pipeline',
  checkedAt: new Date().toISOString(),
  branch: runText('git', ['branch', '--show-current']).trim(),
  commit: runText('git', ['rev-parse', 'HEAD']).trim(),
  sourceStatusShort: sourceStatusShort(),
  proofRoot,
  testRoot,
  commands: commandResults,
  artifacts: {
    expectedEndToEndPipeline: join(proofRoot, 'expected-end-to-end-pipeline.json'),
    proofSummary: join(proofRoot, 'proof-summary.json'),
    testProof: join(testRoot, 'proof.json'),
  },
  provenRows: ['51 Integrated event + network product path proof'],
  provenRootGates: [
    'typed local event-chain refs are preserved before product-path composition',
    'evidence bundle to AI audit to policy to adapter proof preserves exact refs',
    'weak/unavailable evidence cannot publish enforcement commands',
    'AI/UI/network cannot bypass policy',
    'retention/delete/export refs are part of the same proof path',
  ],
  notClaimed: [
    'live packet capture driver invocation',
    'local model execution or remote AI',
    'full policy engine execution',
    'notification provider delivery',
    'host DNS/firewall/WFP/VPN/NetworkExtension/Linux adapter mutation',
    'broker or family-hub transport',
    'portal risk-budget/performance UI rendering',
    'production SLO or external audit completion',
  ],
};
writeFileSync(join(proofRoot, 'proof-summary.json'), `${JSON.stringify(proof, null, 2)}\n`);
writeFileSync(join(testRoot, 'proof.json'), `${JSON.stringify(proof, null, 2)}\n`);
console.log(
  'network-end-to-end-pipeline-proof-ok:agent-core-event-refs,agent-core-weak-no-enforcement,pipeline-tests,clippy,source-shape'
);
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
    ':(exclude)output/network-plan-proof/51-end-to-end-pipeline-proof',
    ':(exclude)test-results/network-end-to-end-pipeline-proof',
  ]);
}
