import { spawnSync } from 'node:child_process';
import { mkdirSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

const proofRoot = join('output', 'eventing-plan-proof', 'full-eventing-plan');
const testRoot = join('test-results', 'eventing-full-plan-proof');
const logRoot = join(proofRoot, 'command-logs');
mkdirSync(proofRoot, { recursive: true });
mkdirSync(testRoot, { recursive: true });
mkdirSync(logRoot, { recursive: true });

const proofScripts = [
  'eventing-branded-fixture-parity-proof.mjs',
  'eventing-command-boundary-proof.mjs',
  'eventing-compatibility-matrix-proof.mjs',
  'eventing-contract-registry-proof.mjs',
  'eventing-delivery-semantics-proof.mjs',
  'eventing-duplicate-subscriber-proof.mjs',
  'eventing-enforcement-journal-action-proof.mjs',
  'eventing-family-variant-proof.mjs',
  'eventing-handler-policy-proof.mjs',
  'eventing-journal-replay-proof.mjs',
  'eventing-lifecycle-clear-proof.mjs',
  'eventing-lock-await-proof.mjs',
  'eventing-manual-clock-proof.mjs',
  'eventing-network-protocol-contract-proof.mjs',
  'eventing-network-backpressure-proof.mjs',
  'eventing-network-delivery-decision-proof.mjs',
  'eventing-network-runtime-proof.mjs',
  'eventing-network-service-event-chain-stream-proof.mjs',
  'eventing-network-service-runtime-delivery-proof.mjs',
  'eventing-network-ts-event-parity-proof.mjs',
  'eventing-parent-child-protocol-contract-proof.mjs',
  'eventing-parent-child-runtime-proof.mjs',
  'eventing-production-shutdown-proof.mjs',
  'eventing-queue-policy-proof.mjs',
  'eventing-request-response-proof.mjs',
  'eventing-runtime-proof.mjs',
  'eventing-runtime-lifecycle-proof.mjs',
  'eventing-metrics-testkit-proof.mjs',
  'eventing-source-safety-proof.mjs',
  'eventing-topology-manifest-proof.mjs',
  'eventing-type-safety-source-gate-proof.mjs',
  'eventing-ui-typed-intent-boundary-proof.mjs',
];

const directCommands = [
  {
    name: 'ocentra-eventing-tests',
    command: 'cargo',
    args: ['test', '-p', 'ocentra-eventing'],
  },
  {
    name: 'ocentra-eventing-clippy',
    command: 'cargo',
    args: ['clippy', '-p', 'ocentra-eventing', '--all-targets', '--', '-D', 'warnings'],
  },
  {
    name: 'source-shape',
    command: 'node',
    args: ['scripts/check-source-shape.mjs'],
  },
  {
    name: 'git-diff-check',
    command: 'git',
    args: ['diff', '--check', '--', '.', ':(exclude)output', ':(exclude)test-results'],
  },
];

const scriptResults = proofScripts.map((scriptName) =>
  runCommand({
    name: scriptName.replace(/\.mjs$/, ''),
    command: 'node',
    args: [join('scripts', 'test', scriptName)],
  })
);
const directResults = directCommands.map(runCommand);
const commands = [...scriptResults, ...directResults];

writeFileSync(join(proofRoot, '00-source-snapshot.md'), sourceSnapshot());
writeGroupedLog('01-contract-proof.log', commands, [
  'eventing-type-safety-source-gate-proof',
  'eventing-contract-registry-proof',
  'eventing-family-variant-proof',
  'eventing-topology-manifest-proof',
  'eventing-compatibility-matrix-proof',
  'eventing-branded-fixture-parity-proof',
  'eventing-parent-child-protocol-contract-proof',
  'eventing-network-protocol-contract-proof',
  'eventing-network-ts-event-parity-proof',
  'ocentra-eventing-tests',
]);
writeGroupedLog('02-dispatch-proof.log', commands, [
  'eventing-runtime-lifecycle-proof',
  'eventing-delivery-semantics-proof',
  'eventing-handler-policy-proof',
  'eventing-duplicate-subscriber-proof',
  'eventing-family-variant-proof',
  'ocentra-eventing-tests',
]);
writeGroupedLog('03-queue-retry-timeout-proof.log', commands, [
  'eventing-queue-policy-proof',
  'eventing-network-backpressure-proof',
  'eventing-handler-policy-proof',
  'eventing-manual-clock-proof',
  'eventing-production-shutdown-proof',
  'ocentra-eventing-tests',
]);
writeGroupedLog('04-request-response-proof.log', commands, [
  'eventing-request-response-proof',
  'eventing-manual-clock-proof',
  'ocentra-eventing-tests',
]);
writeGroupedLog('05-journal-replay-proof.log', commands, [
  'eventing-journal-replay-proof',
  'eventing-enforcement-journal-action-proof',
  'ocentra-eventing-tests',
]);
writeGroupedLog('06-parent-runtime-boundary-proof.log', commands, [
  'eventing-parent-child-runtime-proof',
  'eventing-network-runtime-proof',
  'eventing-network-service-runtime-delivery-proof',
  'eventing-network-service-event-chain-stream-proof',
  'eventing-enforcement-journal-action-proof',
]);
writeGroupedLog('07-ui-boundary-proof.log', commands, [
  'eventing-ui-typed-intent-boundary-proof',
  'eventing-command-boundary-proof',
]);
writeGroupedLog('08-security-negative-proof.log', commands, [
  'eventing-command-boundary-proof',
  'eventing-network-delivery-decision-proof',
  'eventing-source-safety-proof',
  'eventing-lock-await-proof',
  'eventing-queue-policy-proof',
  'eventing-production-shutdown-proof',
  'eventing-type-safety-source-gate-proof',
]);
writeFileSync(
  join(proofRoot, '09-manual-platform-proof.md'),
  [
    '# Manual Platform Proof',
    '',
    'N/A for the reusable eventing crate proof pack.',
    '',
    'The eventing plan establishes local typed runtime/event bus behavior and protocol/runtime boundaries.',
    'It does not claim broker delivery, relay-hub delivery, platform adapter execution, host filtering, or device OS support.',
    '',
  ].join('\n')
);
writeGroupedLog(
  '10-validation-commands.log',
  commands,
  commands.map((entry) => entry.name)
);
writeGroupedLog('11-network-consumer-proof.log', commands, [
  'eventing-network-protocol-contract-proof',
  'eventing-network-ts-event-parity-proof',
  'eventing-network-runtime-proof',
  'eventing-network-backpressure-proof',
  'eventing-network-service-runtime-delivery-proof',
  'eventing-network-service-event-chain-stream-proof',
  'eventing-network-delivery-decision-proof',
]);

const proof = {
  proof: 'eventing-full-plan',
  checkedAt: new Date().toISOString(),
  branch: runText('git', ['branch', '--show-current']).trim(),
  commit: runText('git', ['rev-parse', 'HEAD']).trim(),
  originMain: runText('git', ['rev-parse', 'origin/main']).trim(),
  mergeBase: runText('git', ['merge-base', 'HEAD', 'origin/main']).trim(),
  statusShort: runText('git', ['status', '--short']),
  proofRoot,
  testRoot,
  commands,
  requiredProofPack: [
    '00-source-snapshot.md',
    '01-contract-proof.log',
    '02-dispatch-proof.log',
    '03-queue-retry-timeout-proof.log',
    '04-request-response-proof.log',
    '05-journal-replay-proof.log',
    '06-parent-runtime-boundary-proof.log',
    '07-ui-boundary-proof.log',
    '08-security-negative-proof.log',
    '09-manual-platform-proof.md',
    '10-validation-commands.log',
    '11-network-consumer-proof.log',
  ].map((name) => join(proofRoot, name)),
  provenRows: [
    '05-41 reusable eventing crate runtime rows',
    '42-62 parent/controller, child-agent, network, UI, enforcement, and command-boundary consumer rows',
    '63-78 reusable eventing type-safety, compatibility, lifecycle, topology, delivery, and source-safety rows',
  ],
  networkConsumerProof: {
    proofLog: join(proofRoot, '11-network-consumer-proof.log'),
    proves:
      'network consumes ocentra-eventing for typed publish/routing, queue/drain, request-response, service read-model delivery, service event-chain streaming, TypeScript parity, and broker/relay-hub manual-required delivery decisions without adding network business logic to crates/ocentra-eventing',
  },
  notClaimed: [
    'broker-backed delivery',
    'relay-hub delivery',
    'platform adapter execution',
    'host DNS/filter enforcement',
    'portal-owned business event publishing',
  ],
};

writeFileSync(join(proofRoot, 'proof-summary.json'), `${JSON.stringify(proof, null, 2)}\n`);
writeFileSync(join(testRoot, 'proof.json'), `${JSON.stringify(proof, null, 2)}\n`);
console.log('eventing-full-plan-proof-ok:all-eventing-harnesses');
console.log(`proof=${join(proofRoot, 'proof-summary.json')}`);

function runCommand(entry) {
  const result = spawnSync(entry.command, entry.args, { encoding: 'utf8', shell: false });
  const safeName = entry.name.replace(/[^a-zA-Z0-9_.-]/g, '-');
  const log = join(logRoot, `${safeName}.log`);
  writeFileSync(log, `${result.stdout ?? ''}${result.stderr ?? ''}`);
  if (result.status !== 0) {
    throw new Error(`${entry.name} failed with exit ${result.status}; log=${log}`);
  }
  return {
    name: entry.name,
    command: [entry.command, ...entry.args].join(' '),
    status: result.status,
    log,
  };
}

function writeGroupedLog(filename, commands, names) {
  const selected = commands.filter((entry) => names.includes(entry.name));
  const body = selected
    .map((entry) => [`command=${entry.command}`, `status=${entry.status}`, `log=${entry.log}`].join('\n'))
    .join('\n\n');
  writeFileSync(join(proofRoot, filename), `${body}\n`);
}

function sourceSnapshot() {
  return [
    '# Eventing Full Plan Source Snapshot',
    '',
    `branch: ${runText('git', ['branch', '--show-current']).trim()}`,
    `head: ${runText('git', ['rev-parse', 'HEAD']).trim()}`,
    `origin/main: ${runText('git', ['rev-parse', 'origin/main']).trim()}`,
    `merge-base: ${runText('git', ['merge-base', 'HEAD', 'origin/main']).trim()}`,
    '',
    '## Status',
    '',
    '```text',
    runText('git', ['status', '--short']).trimEnd(),
    '```',
    '',
    '## Inspected Paths',
    '',
    '- crates/ocentra-eventing',
    '- crates/agent-protocol',
    '- crates/agent-core',
    '- crates/agent-service',
    '- apps/portal/src/transport.ts',
    '- docs/plans/eventing-plan',
    '- output/eventing-plan-proof',
    '',
    '## Before-State Gap',
    '',
    'The row-level eventing proof artifacts existed, but the eventing checklist did not have one consolidated proof pack tying the source snapshot, grouped logs, manual platform non-claims, and validation commands together for PR-ready handoff.',
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
