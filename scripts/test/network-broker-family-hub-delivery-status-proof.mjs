import { spawnSync } from 'node:child_process';
import { mkdirSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

const proofRoot = join('output', 'network-plan-proof', '10b-broker-family-hub-delivery-status');
const testRoot = join('test-results', 'network-broker-family-hub-delivery-status-proof');
mkdirSync(proofRoot, { recursive: true });
mkdirSync(testRoot, { recursive: true });

const commands = [
  {
    name: 'network-remote-delivery-status-tests',
    command: 'cargo',
    args: ['test', '-p', 'ocentra-parent-agent-core', 'network_runtime_remote_delivery'],
    log: join(proofRoot, 'remote-delivery-status-tests.log'),
  },
  {
    name: 'network-broker-delivery-tests',
    command: 'cargo',
    args: ['test', '-p', 'ocentra-parent-agent-core', 'network_runtime_broker_delivery'],
    log: join(proofRoot, 'broker-delivery-tests.log'),
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
    name: 'schema-boundaries',
    command: 'cmd',
    args: ['/c', 'npm', 'run', 'lint:schema-boundaries'],
    log: join(proofRoot, 'schema-boundaries.log'),
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

const statusLog = [
  'network row10b broker/family-hub remote delivery status',
  '',
  'brokerStatus=requirements-satisfied-but-not-implemented',
  'familyHubStatus=requirements-satisfied-but-not-implemented',
  'brokerDeliveryImplemented=false',
  'familyHubDeliveryImplemented=false',
  'crossProcessReplayImplemented=false',
  'remoteRetentionDeleteExportPropagationImplemented=false',
  'policyAuthority=false',
  'sideEffectAuthority=false',
  'enforcementCommandEvents=0',
  'adapterActionsExecuted=0',
  '',
  ...commandResults.map((result) => `${result.name}: ${result.command} -> exit ${result.status}; log=${result.log}`),
  '',
];
writeFileSync(join(proofRoot, '10b-remote-delivery-status.log'), statusLog.join('\n'));

const proof = {
  proof: 'network-broker-family-hub-delivery-status',
  checkedAt: new Date().toISOString(),
  branch: runText('git', ['branch', '--show-current']).trim(),
  commit: runText('git', ['rev-parse', 'HEAD']).trim(),
  originMain: runText('git', ['rev-parse', 'origin/main']).trim(),
  mergeBase: runText('git', ['merge-base', 'HEAD', 'origin/main']).trim(),
  sourceStatusShort: sourceStatusShort(),
  proofRoot,
  testRoot,
  commands: commandResults,
  artifacts: {
    statusLog: join(proofRoot, '10b-remote-delivery-status.log'),
    proofSummary: join(proofRoot, 'proof-summary.json'),
    testProof: join(testRoot, 'proof.json'),
  },
  provenRows: [
    '10b Broker/family-hub remote delivery status proof',
    '10 NetworkActivityEvent reusable Rust eventing consumption',
  ],
  provenBehavior: [
    'broker and family-hub relay routes carry custody, auth, encryption, retention, replay, deletion, offset, dedupe, broker config, identity, and relay policy refs',
    'requirements-satisfied broker and family-hub decisions remain explicit status rows rather than live transport claims',
    'local idempotency and overflow dead-letter evidence remains attached to the remote-delivery status',
    'subscriber filters remain scoped to the network event namespace',
    'duplicate and family-hub status proof does not publish enforcement command events or execute adapter actions',
  ],
  notClaimed: [
    'live broker delivery',
    'live family-hub delivery',
    'cross-process durable replay',
    'remote retention/delete/export propagation',
    'policy authority',
    'side-effect authority',
    'adapter execution',
    'host filtering',
    'full network-plan completion',
  ],
};
writeFileSync(join(proofRoot, 'proof-summary.json'), `${JSON.stringify(proof, null, 2)}\n`);
writeFileSync(join(testRoot, 'proof.json'), `${JSON.stringify(proof, null, 2)}\n`);
console.log(
  'network-broker-family-hub-delivery-status-proof-ok:rust,eventing,clippy,schema-boundaries,source-shape,diff-check'
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
    ':(exclude)output/network-plan-proof/10b-broker-family-hub-delivery-status',
    ':(exclude)test-results/network-broker-family-hub-delivery-status-proof',
  ]);
}
