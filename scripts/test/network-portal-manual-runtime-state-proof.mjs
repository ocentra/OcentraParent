import { spawnSync } from 'node:child_process';
import { mkdirSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

const proofRoot = join('output', 'network-plan-proof', '36c-network-portal-manual-runtime-state-proof');
const testRoot = join('test-results', 'network-portal-manual-runtime-state-proof');
mkdirSync(proofRoot, { recursive: true });
mkdirSync(testRoot, { recursive: true });

writeFileSync(
  join(proofRoot, 'expected-portal-state.json'),
  `${JSON.stringify(
    {
      source: 'agent.network.flow.read-model.reported activityDigest',
      renderedStateFamilies: [
        'read-model row counts',
        'runtime observed/delivered/failed/publish counts',
        'runtime stored/dead-letter counts',
        'manual-required row count',
        'enforcement-command event count',
        'retention tombstone/exportable/deleted evidence state',
      ],
      invariants: [
        'missing runtime delivery stays Not reported',
        'service digest counts are parsed through activity-domain contracts',
        'portal does not invent risk-budget or performance state',
        'portal does not claim exact URL, decrypted payload, page content, or browser evidence',
      ],
    },
    null,
    2
  )}\n`
);

const commands = [
  {
    name: 'agent-service-network-flow-payload-tests',
    command: 'cargo',
    args: ['test', '-p', 'ocentra-parent-agent-service', 'network_flow_payload'],
    log: join(proofRoot, 'agent-service-network-flow-payload-tests.log'),
  },
  {
    name: 'agent-service-network-flow-digest-tests',
    command: 'cargo',
    args: ['test', '-p', 'ocentra-parent-agent-service', 'network_flow_digest'],
    log: join(proofRoot, 'agent-service-network-flow-digest-tests.log'),
  },
  {
    name: 'activity-domain-build',
    command: 'cmd',
    args: ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/activity-domain'],
    log: join(proofRoot, 'activity-domain-build.log'),
  },
  {
    name: 'activity-domain-network-flow-tests',
    command: 'cmd',
    args: ['/c', 'npm', 'run', 'test', '--workspace', '@ocentra-parent/activity-domain', '--', 'network-flow'],
    log: join(proofRoot, 'activity-domain-network-flow-tests.log'),
  },
  {
    name: 'portal-live-activity-network-flow-tests',
    command: 'cmd',
    args: ['/c', 'npm', 'run', 'test', '--workspace', '@ocentra-parent/portal', '--', 'live-activity-network-flow'],
    log: join(proofRoot, 'portal-live-activity-network-flow-tests.log'),
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
  proof: 'network-portal-manual-runtime-state',
  checkedAt: new Date().toISOString(),
  branch: runText('git', ['branch', '--show-current']).trim(),
  commit: runText('git', ['rev-parse', 'HEAD']).trim(),
  sourceStatusShort: sourceStatusShort(),
  proofRoot,
  testRoot,
  commands: commandResults,
  artifacts: {
    expectedPortalState: join(proofRoot, 'expected-portal-state.json'),
    proofSummary: join(proofRoot, 'proof-summary.json'),
    testProof: join(testRoot, 'proof.json'),
  },
  provenRows: ['36c Parent portal manual/runtime network state rendering proof'],
  provenStateFamilies: [
    'read-model row counts',
    'runtime delivery counts',
    'runtime storage counts',
    'manual-required count',
    'enforcement-command count',
    'retention tombstone/export/delete references',
  ],
  notClaimed: [
    'risk-budget portal rendering',
    'performance/SLO portal rendering',
    'live broker or family-hub transport dispatch',
    'remote acknowledgement ingestion',
    'exact URL, page content, decrypted payload, or private message evidence',
  ],
};

writeFileSync(join(proofRoot, 'proof-summary.json'), `${JSON.stringify(proof, null, 2)}\n`);
writeFileSync(join(testRoot, 'proof.json'), `${JSON.stringify(proof, null, 2)}\n`);
console.log(
  'network-portal-manual-runtime-state-proof-ok:agent-service-payload,digest,activity-domain,portal,source-shape,diff-check'
);
console.log(`proof=${join(proofRoot, 'proof-summary.json')}`);

function runCommand(entry) {
  const result = spawnSync(entry.command, entry.args, { encoding: 'utf8', shell: false });
  writeFileSync(entry.log, normalizedLog(`${result.stdout ?? ''}${result.stderr ?? ''}`));
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

function normalizedLog(text) {
  const withoutTrailingBlankLines = text.replace(/(?:\r?\n)+$/u, '');
  return `${withoutTrailingBlankLines}\n`;
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
    ':(exclude)output/network-plan-proof/36c-network-portal-manual-runtime-state-proof',
    ':(exclude)test-results/network-portal-manual-runtime-state-proof',
  ]);
}
