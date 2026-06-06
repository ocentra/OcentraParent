import { spawnSync } from 'node:child_process';
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

const proofRoot = join('output', 'network-plan-proof', '10f-remote-delivery-event-chain-journal-status');
const testRoot = join('test-results', 'network-remote-delivery-event-chain-journal-proof');
mkdirSync(proofRoot, { recursive: true });
mkdirSync(testRoot, { recursive: true });

const securityLogPath = join(proofRoot, '09-security-negative-proof.log');
const validationLogPath = join(proofRoot, '12-validation-commands.log');

assertSourceContracts();

writeFileSync(
  join(proofRoot, 'expected-remote-delivery-event-chain-journal-status.json'),
  `${JSON.stringify(
    {
      acceptedInputs: [
        'local network runtime event-chain payloads',
        'row10b broker/family-hub delivery requirement refs',
        'ocentra-eventing NDJSON journal and projection replay records',
      ],
      eventChainJournalRefs: [
        'network.remote-delivery.event-chain-journal.10f',
        'network.remote-delivery.event-chain-replay.10f',
        'network.remote-delivery.event-chain-export.10f',
        'network.remote-delivery.event-chain.support-status.10f',
      ],
      renderedStates: [
        'projectionReplayMode=ProjectionOnly',
        'journalEntryCount equals replayRecordCount',
        'exportableRemoteEnvelopeCount equals journalEntryCount',
        'unavailableEventCount equals journalEntryCount',
      ],
      parserInvariants: [
        'event-chain journal refs must all cite row10f',
        'projection replay cannot dispatch action handlers',
        'event-chain journal/export boundary cannot carry live broker/family-hub delivery claims',
        'event-chain journal/export boundary cannot carry exact content or adapter-action claims',
      ],
      noClaims: [
        'live broker delivery',
        'live family-hub relay delivery',
        'remote provider delivery',
        'child-device delivery',
        'product-ready remote delivery',
        'policy authority',
        'side-effect authority',
        'enforcement command publication',
        'adapter action execution',
        'exact URL from network-only evidence',
        'decrypted payload',
        'page content',
        'host filtering',
      ],
    },
    null,
    2
  )}\n`
);

const commands = [
  {
    name: 'eventing-journal-replay-test',
    command: 'cargo',
    args: ['test', '-p', 'ocentra-eventing', 'journal_replay'],
    log: join(proofRoot, 'eventing-journal-replay-test.log'),
  },
  {
    name: 'agent-core-remote-delivery-rust-test',
    command: 'cargo',
    args: ['test', '-p', 'ocentra-parent-agent-core', 'network_runtime_remote_delivery_status'],
    log: join(proofRoot, 'agent-core-remote-delivery-rust-test.log'),
  },
  {
    name: 'agent-core-remote-event-chain-journal-test',
    command: 'cargo',
    args: ['test', '-p', 'ocentra-parent-agent-core', 'network_runtime_remote_event_chain_journal'],
    log: join(proofRoot, 'agent-core-remote-event-chain-journal-test.log'),
  },
  {
    name: 'agent-core-build',
    command: 'cargo',
    args: ['build', '-p', 'ocentra-parent-agent-core'],
    log: join(proofRoot, 'agent-core-build.log'),
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

writeFileSync(
  validationLogPath,
  commandResults.map((entry) => `${entry.command} -> ${entry.status}`).join('\n') + '\n'
);
writeFileSync(
  securityLogPath,
  [
    `checkedAt=${new Date().toISOString()}`,
    'asserted=no exact URL/page/message/search claim from network-only evidence',
    'asserted=no decrypted payload or raw PCAP without custody claim',
    'asserted=no live broker/family-hub delivery claim',
    'asserted=no remote provider or child-device delivery claim',
    'asserted=no product-ready remote delivery claim',
    'asserted=no action replay, policy authority, side-effect authority, adapter action, host filtering, or enforcement command publication claim',
  ].join('\n') + '\n'
);

const proof = {
  proof: 'network-remote-delivery-event-chain-journal-proof',
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
    expectedRemoteDeliveryEventChainJournalStatus: join(
      proofRoot,
      'expected-remote-delivery-event-chain-journal-status.json'
    ),
    securityNegativeLog: securityLogPath,
    validationCommands: validationLogPath,
    proofSummary: join(proofRoot, 'proof-summary.json'),
    testProof: join(testRoot, 'proof.json'),
  },
  coveredRows: [
    'network-plan supplemental row 10f remote delivery event-chain journal/export boundary status',
    'network-plan supplemental row 10b broker/family-hub remote delivery status',
    'network-plan workpack 45 eventing delivery-decision and journal/replay consumption',
  ],
  provenBoundaries: [
    'agent-core materializes local network runtime event-chain payloads through ocentra-eventing NDJSON journal records',
    'projection replay reads the remote-delivery event-chain journal as exportable stored envelopes without action-handler dispatch',
    'row10f refs mark journal, replay, export, and support-status boundaries that future broker/family-hub transport can consume',
    'the proof keeps provider delivery, child-device delivery, product-ready remote delivery, policy authority, side-effect authority, adapter execution, enforcement commands, and host filtering false',
    'the proof keeps exact URL, decrypted payload, and page content unavailable from network-only event-chain envelopes',
  ],
  notClaimed: [
    'live broker delivery',
    'live family-hub relay delivery',
    'remote provider delivery',
    'child-device delivery',
    'product-ready remote delivery',
    'cross-process transport implementation',
    'remote retention/delete/export propagation implementation',
    'policy authority',
    'side-effect authority',
    'adapter execution',
    'host filtering',
    'full network-plan completion',
  ],
};

writeFileSync(join(proofRoot, 'proof-summary.json'), `${JSON.stringify(proof, null, 2)}\n`);
writeFileSync(join(testRoot, 'proof.json'), `${JSON.stringify(proof, null, 2)}\n`);
console.log('network-remote-delivery-event-chain-journal-proof-ok:eventing-journal,core,source-shape,diff-check');
console.log(`proof=${join(proofRoot, 'proof-summary.json')}`);

function assertSourceContracts() {
  const protocolConstants = readFileSync('crates/agent-protocol/src/constants/network_flow.rs', 'utf8');
  const coreRuntime = readFileSync('crates/agent-core/src/network_event_runtime.rs', 'utf8');
  const coreProof = readFileSync(
    'crates/agent-core/src/network_event_runtime/remote_delivery_event_chain_journal.rs',
    'utf8'
  );
  const coreStore = readFileSync(
    'crates/agent-core/src/network_event_runtime/remote_delivery_event_chain_store.rs',
    'utf8'
  );
  const coreTests = readFileSync('crates/agent-core/src/network_event_runtime_remote_delivery_tests.rs', 'utf8');
  const coreReadme = readFileSync('crates/agent-core/README.md', 'utf8');
  const featureDoc = readFileSync('docs/features/network-domain-control.md', 'utf8');
  const checklist = readFileSync('docs/plans/network-plan/implementation-checklist.md', 'utf8');
  const workpacks = readFileSync('docs/plans/network-plan/workpacks/README.md', 'utf8');
  const requiredSnippets = [
    [protocolConstants, 'TEST_REMOTE_EVENT_CHAIN_JOURNAL_REF'],
    [coreRuntime, 'prove_network_runtime_remote_event_chain_journal'],
    [coreStore, 'NdjsonEventJournal::with_options'],
    [coreStore, 'journal.replay_projection(ReplayFilter::all())'],
    [coreProof, 'projection_replay_mode: projection.mode'],
    [coreTests, 'network_runtime_remote_event_chain_journal_preserves_export_boundary_without_transport'],
    [coreTests, 'broker_delivery_implemented'],
    [coreReadme, 'journal/export boundary'],
    [featureDoc, 'network-remote-delivery-event-chain-journal-proof'],
    [checklist, '10f remote delivery event-chain journal/export boundary status'],
    [workpacks, '10f'],
  ];
  for (const [haystack, needle] of requiredSnippets) {
    if (!haystack.includes(needle)) {
      throw new Error(`missing source contract snippet: ${needle}`);
    }
  }
}

function runCommand(entry) {
  const result = spawnSync(entry.command, entry.args, {
    encoding: 'utf8',
    shell: false,
  });
  writeFileSync(entry.log, normalizeLogOutput(`${result.stdout ?? ''}${result.stderr ?? ''}`));
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

function normalizeLogOutput(value) {
  const withoutTrailingSpaces = value.replace(/[ \t]+$/gmu, '');
  const withoutBlankTail = withoutTrailingSpaces.replace(/(?:\r?\n){2,}$/u, '\n');
  if (withoutBlankTail.length === 0 || withoutBlankTail.endsWith('\n')) {
    return withoutBlankTail;
  }
  return `${withoutBlankTail}\n`;
}

function runText(command, args) {
  const result = spawnSync(command, args, { encoding: 'utf8', shell: false });
  if (result.status !== 0) {
    throw new Error(`${command} ${args.join(' ')} failed with exit ${result.status}`);
  }
  return `${result.stdout ?? ''}${result.stderr ?? ''}`;
}

function sourceStatusShort() {
  const status = runText('git', ['status', '--short']);
  const generatedProofPrefixes = [
    'output/network-plan-proof/10b-broker-family-hub-delivery-status/',
    'output/network-plan-proof/10f-remote-delivery-event-chain-journal-status/',
    'output/network-plan-proof/10g-remote-delivery-receipt-ledger/',
    'test-results/network-broker-family-hub-delivery-status-proof/',
    'test-results/network-remote-delivery-event-chain-journal-proof/',
    'test-results/network-remote-delivery-receipt-ledger-proof/',
  ];
  return status
    .split(/\r?\n/)
    .filter((line) => {
      if (line.trim().length === 0) {
        return false;
      }
      const filePath = line.slice(3).replaceAll('\\', '/');
      return !generatedProofPrefixes.some((prefix) => filePath.startsWith(prefix));
    })
    .join('\n');
}
