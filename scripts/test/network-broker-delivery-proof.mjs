import { spawnSync } from 'node:child_process';
import { mkdirSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

const proofRoot = join('output', 'network-plan-proof', '10a-broker-delivery-proof');
const testRoot = join('test-results', 'network-broker-delivery-proof');
mkdirSync(proofRoot, { recursive: true });
mkdirSync(testRoot, { recursive: true });

const commands = [
  {
    name: 'network-broker-delivery-tests',
    command: 'cargo',
    args: ['test', '-p', 'ocentra-parent-agent-core', 'network_runtime_broker_delivery'],
    log: join(proofRoot, 'broker-delivery-tests.log'),
  },
  {
    name: 'network-queue-idempotency-tests',
    command: 'cargo',
    args: [
      'test',
      '-p',
      'ocentra-parent-agent-core',
      'network_runtime_queue_idempotency_rejects_queued_and_completed_duplicates',
    ],
    log: join(proofRoot, 'queue-idempotency-tests.log'),
  },
  {
    name: 'network-queue-overflow-tests',
    command: 'cargo',
    args: ['test', '-p', 'ocentra-parent-agent-core', 'network_runtime_queue_overflow_dead_letters_second_flow'],
    log: join(proofRoot, 'queue-overflow-tests.log'),
  },
  {
    name: 'eventing-delivery-decision-tests',
    command: 'cargo',
    args: ['test', '-p', 'ocentra-eventing', 'delivery_decision'],
    log: join(proofRoot, 'eventing-delivery-decision-tests.log'),
  },
  {
    name: 'agent-core-clippy',
    command: 'cargo',
    args: ['clippy', '-p', 'ocentra-parent-agent-core', '--all-targets', '--', '-D', 'warnings'],
    log: join(proofRoot, 'agent-core-clippy.log'),
  },
  {
    name: 'source-shape',
    command: 'node',
    args: ['scripts/check-source-shape.mjs'],
    log: join(proofRoot, 'source-shape.log'),
  },
];
const commandResults = commands.map(runCommand);

const brokerDeliveryLog = [
  'network row10a broker delivery semantics',
  '',
  'semantic=effectively-once-through-idempotency',
  'brokerDeliveryImplemented=false',
  'familyHubDeliveryImplemented=false',
  'duplicateDetection=queued-and-completed-idempotency-rejection',
  'replay=broker-replay-plan-ref-preserved',
  'droppedEventAudit=queue-overflow-dead-letter-count-preserved',
  'adapterAction=zero-enforcement-command-events-and-zero-adapter-action-executed',
  '',
  ...commandResults.map((result) => `${result.name}: ${result.command} -> exit ${result.status}; log=${result.log}`),
  '',
];
writeFileSync(join(proofRoot, '10a-broker-delivery-proof.log'), brokerDeliveryLog.join('\n'));

const proof = {
  proof: 'network-broker-delivery',
  checkedAt: new Date().toISOString(),
  branch: runText('git', ['branch', '--show-current']).trim(),
  commit: runText('git', ['rev-parse', 'HEAD']).trim(),
  sourceStatusShort: sourceStatusShort(),
  proofRoot,
  testRoot,
  commands: commandResults,
  artifacts: {
    brokerDeliveryProofLog: join(proofRoot, '10a-broker-delivery-proof.log'),
    proofSummary: join(proofRoot, 'proof-summary.json'),
    testProof: join(testRoot, 'proof.json'),
  },
  provenRows: ['10a Broker delivery semantics proof'],
  provenBehavior: [
    'broker route requirements can be satisfied while live broker delivery remains false',
    'network queue idempotency rejects queued and completed duplicate events',
    'queue overflow creates dropped-event dead-letter audit evidence',
    'broker replay, dropped-event audit, and adapter-action ledger refs are preserved',
    'duplicate broker routes do not create duplicate enforcement command events or adapter actions',
  ],
  notClaimed: [
    'live broker delivery',
    'family-hub delivery',
    'cross-process broker transport',
    'production retention/delete/export propagation',
    'adapter execution',
    'enforcement command publication from duplicate broker events',
  ],
};
writeFileSync(join(proofRoot, 'proof-summary.json'), `${JSON.stringify(proof, null, 2)}\n`);
writeFileSync(join(testRoot, 'proof.json'), `${JSON.stringify(proof, null, 2)}\n`);
console.log('network-broker-delivery-proof-ok:tests,clippy,source-shape');
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
    ':(exclude)output/network-plan-proof/10a-broker-delivery-proof',
    ':(exclude)test-results/network-broker-delivery-proof',
  ]);
}
