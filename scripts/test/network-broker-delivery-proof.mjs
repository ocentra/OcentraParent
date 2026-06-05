import { spawnSync } from 'node:child_process';
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

const proofRoot = join('output', 'network-plan-proof', '10a-broker-delivery-proof');
const testRoot = join('test-results', 'network-broker-delivery-proof');
mkdirSync(proofRoot, { recursive: true });
mkdirSync(testRoot, { recursive: true });
const sourceSnapshot = {
  branch: runText('git', ['branch', '--show-current']).trim(),
  commit: runText('git', ['rev-parse', 'HEAD']).trim(),
  originMain: runText('git', ['rev-parse', 'origin/main']).trim(),
  mergeBase: runText('git', ['merge-base', 'HEAD', 'origin/main']).trim(),
  sourceStatusShort: sourceStatusShort(),
};

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
    args: ['test', '-p', 'ocentra-parent-agent-core', 'network_runtime_queue_overflow_dead_letters_oldest_flow'],
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
assertSourceContracts();

const brokerDeliveryLog = [
  'network row10a broker delivery semantics',
  '',
  'semantic=local-idempotency-queue-proof',
  'brokerDeliveryImplemented=false',
  'relayHubDeliveryImplemented=false',
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
  branch: sourceSnapshot.branch,
  commit: sourceSnapshot.commit,
  originMain: sourceSnapshot.originMain,
  mergeBase: sourceSnapshot.mergeBase,
  sourceStatusShort: sourceSnapshot.sourceStatusShort,
  proofRoot,
  testRoot,
  commands: commandResults,
  artifacts: {
    brokerDeliveryProofLog: join(proofRoot, '10a-broker-delivery-proof.log'),
    proofSummary: join(proofRoot, 'proof-summary.json'),
    testProof: join(testRoot, 'proof.json'),
  },
  provenRows: ['network-plan row 10 eventing consumption closure', '10a Broker delivery semantics proof'],
  provenBehavior: [
    'row 10 is complete for reusable local and service network runtime eventing consumption',
    'broker route requirements can be satisfied while live broker delivery remains false',
    'local network queue idempotency rejects queued and completed duplicate events',
    'queue overflow creates dropped-event dead-letter audit evidence',
    'broker replay, dropped-event audit, and adapter-action ledger refs are preserved',
    'duplicate broker routes do not create duplicate enforcement command events or adapter actions',
  ],
  notClaimed: [
    'live broker delivery',
    'relay-hub delivery',
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

function assertSourceContracts() {
  const brokerDelivery = readText('crates/agent-core/src/network_event_runtime/broker_delivery.rs');
  const brokerDeliveryTests = readText('crates/agent-core/src/network_event_runtime_broker_delivery_tests.rs');
  const eventingReadme = readText('crates/ocentra-eventing/README.md');
  const checklist = readText('docs/plans/network-plan/implementation-checklist.md');
  const featureDoc = readText('docs/features/network-domain-control.md');

  assertIncludes(
    brokerDelivery,
    'EventDeliveryRouteKind::ExternalTransport',
    'network broker proof uses the generic external transport decision'
  );
  assertIncludes(
    brokerDelivery,
    'queue_network_runtime_flow_rejects_duplicate_idempotency',
    'network broker proof composes reusable queue idempotency'
  );
  assertIncludes(
    brokerDelivery,
    'enforcement_command_event_count',
    'network broker proof counts enforcement command events'
  );
  assertIncludes(
    brokerDeliveryTests,
    'network_runtime_broker_delivery_semantics_preserve_refs_without_live_broker',
    'broker delivery test names the no live broker boundary'
  );
  assertIncludes(
    brokerDeliveryTests,
    'assert!(!report.external_transport_delivery_implemented)',
    'broker delivery test rejects live external transport implementation claim'
  );
  assertIncludes(
    eventingReadme,
    'Network-only bus, external queue, request broker, or platform transport',
    'reusable eventing crate does not own network transport machinery'
  );
  assertIncludes(
    checklist,
    '| 10   | NetworkActivityEvent contracts and reusable Rust eventing consumption                                                     | [x]',
    'network checklist row 10 is closed'
  );
  assertIncludes(
    checklist,
    'Live broker/family-hub delivery and cross-process transport remain explicit non-claims',
    'network checklist preserves broker family-hub non-claim'
  );
  assertIncludes(
    featureDoc,
    '- [x] Reusable Rust eventing, detection, AI audit, and risk-budget contracts.',
    'network feature checklist closes reusable eventing line'
  );
  assertIncludes(
    featureDoc,
    'Broker/family-hub delivery implementation',
    'network feature doc keeps broker family-hub implementation in current gap'
  );
}

function readText(path) {
  return readFileSync(path, 'utf8');
}

function assertIncludes(text, expected, label) {
  if (!text.includes(expected)) {
    throw new Error(`${label}: missing ${expected}`);
  }
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
