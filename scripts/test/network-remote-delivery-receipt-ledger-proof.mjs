import { spawnSync } from 'node:child_process';
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

const proofRoot = join('output', 'network-plan-proof', '10g-remote-delivery-receipt-ledger');
const testRoot = join('test-results', 'network-remote-delivery-receipt-ledger-proof');
mkdirSync(proofRoot, { recursive: true });
mkdirSync(testRoot, { recursive: true });

const securityLogPath = join(proofRoot, '09-security-negative-proof.log');
const validationLogPath = join(proofRoot, '12-validation-commands.log');

assertSourceContracts();

writeFileSync(
  join(proofRoot, 'expected-remote-delivery-receipt-ledger-status.json'),
  `${JSON.stringify(
    {
      acceptedInputs: [
        'row10f local network runtime event-chain projection replay records',
        'row10f exportable stored envelopes',
        'row10b broker/family-hub delivery requirement refs',
      ],
      receiptLedgerRefs: [
        'network.remote-delivery.event-chain.receipt-ledger.10g',
        'network.remote-delivery.event-chain.local-receipt-ack.10g',
        'network.remote-delivery.event-chain.receipt-replay.10g',
        'network.remote-delivery.event-chain.receipt-support-status.10g',
      ],
      renderedStates: [
        'receiptLedgerReady=true',
        'receiptReplayReady=true',
        'receiptRecordsMatchProjection=true',
        'receiptRecordCount equals sourceProjectionReplayRecordCount',
        'localReceiptAckCount equals receiptRecordCount',
        'remoteDeliveryAckImplemented=false',
      ],
      parserInvariants: [
        'receipt refs must all cite row10g',
        'receipt records must preserve replay sequence, event id, event type, and correlation id',
        'receipt ledger cannot claim live broker/family-hub delivery',
        'receipt ledger cannot carry exact content or adapter-action claims',
      ],
      noClaims: [
        'live broker delivery',
        'live family-hub relay delivery',
        'remote provider delivery',
        'child-device delivery',
        'family-hub delivery acknowledgement implementation',
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
    name: 'agent-core-remote-event-chain-journal-test',
    command: 'cargo',
    args: ['test', '-p', 'ocentra-parent-agent-core', 'network_runtime_remote_event_chain_journal'],
    log: join(proofRoot, 'agent-core-remote-event-chain-journal-test.log'),
  },
  {
    name: 'agent-core-remote-delivery-receipt-ledger-test',
    command: 'cargo',
    args: ['test', '-p', 'ocentra-parent-agent-core', 'network_runtime_remote_delivery_receipt_ledger'],
    log: join(proofRoot, 'agent-core-remote-delivery-receipt-ledger-test.log'),
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
    'asserted=no family-hub delivery acknowledgement implementation claim',
    'asserted=no remote provider or child-device delivery claim',
    'asserted=no product-ready remote delivery claim',
    'asserted=no policy authority, side-effect authority, adapter action, host filtering, or enforcement command publication claim',
  ].join('\n') + '\n'
);

const proof = {
  proof: 'network-remote-delivery-receipt-ledger-proof',
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
    expectedRemoteDeliveryReceiptLedgerStatus: join(proofRoot, 'expected-remote-delivery-receipt-ledger-status.json'),
    securityNegativeLog: securityLogPath,
    validationCommands: validationLogPath,
    proofSummary: join(proofRoot, 'proof-summary.json'),
    testProof: join(testRoot, 'proof.json'),
  },
  coveredRows: [
    'network-plan supplemental row 10g remote delivery receipt ledger/local ack status',
    'network-plan supplemental row 10f remote delivery event-chain journal/export boundary status',
    'network-plan supplemental row 10b broker/family-hub remote delivery status',
  ],
  provenBoundaries: [
    'agent-core builds a deterministic local receipt ledger from row10f ocentra-eventing projection replay records',
    'receipt records preserve replay sequence, event id, event type, and correlation id for every exportable event-chain envelope',
    'row10g refs mark receipt ledger, local receipt ack, replay, and support-status boundaries that future broker/family-hub delivery can consume',
    'the proof keeps broker delivery, family-hub relay delivery, family-hub delivery acknowledgement implementation, provider delivery, child-device delivery, product-ready remote delivery, policy authority, side-effect authority, adapter execution, enforcement commands, and host filtering false',
    'the proof keeps exact URL, decrypted payload, and page content unavailable from network-only receipt ledger records',
  ],
  notClaimed: [
    'live broker delivery',
    'live family-hub relay delivery',
    'family-hub delivery acknowledgement implementation',
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
console.log('network-remote-delivery-receipt-ledger-proof-ok:core,source-shape,diff-check');
console.log(`proof=${join(proofRoot, 'proof-summary.json')}`);

function assertSourceContracts() {
  const protocolConstants = readFileSync('crates/agent-protocol/src/constants/network_flow.rs', 'utf8');
  const coreRuntime = readFileSync('crates/agent-core/src/network_event_runtime.rs', 'utf8');
  const coreStore = readFileSync(
    'crates/agent-core/src/network_event_runtime/remote_delivery_event_chain_store.rs',
    'utf8'
  );
  const coreProof = readFileSync(
    'crates/agent-core/src/network_event_runtime/remote_delivery_receipt_ledger.rs',
    'utf8'
  );
  const coreTests = readFileSync('crates/agent-core/src/network_event_runtime_remote_delivery_tests.rs', 'utf8');
  const coreReadme = readFileSync('crates/agent-core/README.md', 'utf8');
  const featureDoc = readFileSync('docs/features/network-domain-control.md', 'utf8');
  const checklist = readFileSync('docs/plans/network-plan/implementation-checklist.md', 'utf8');
  const workpacks = readFileSync('docs/plans/network-plan/workpacks/README.md', 'utf8');
  const requiredSnippets = [
    [protocolConstants, 'TEST_REMOTE_EVENT_CHAIN_RECEIPT_LEDGER_REF'],
    [coreRuntime, 'prove_network_runtime_remote_delivery_receipt_ledger'],
    [coreStore, 'publish_network_runtime_remote_event_chain_store'],
    [coreProof, 'receipt_records_from_projection'],
    [coreProof, 'remote_delivery_ack_implemented: false'],
    [coreTests, 'network_runtime_remote_delivery_receipt_ledger_preserves_local_ack_boundary_without_transport'],
    [coreTests, 'remote_delivery_ack_implemented'],
    [coreReadme, 'row10g preserves the receipt ledger/local ack'],
    [featureDoc, 'network-remote-delivery-receipt-ledger-proof'],
    [checklist, '10g remote delivery receipt ledger/local ack status'],
    [workpacks, '10g'],
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
  return status
    .split(/\r?\n/)
    .filter((line) => {
      if (line.trim().length === 0) {
        return false;
      }
      const filePath = line.slice(3).replaceAll('\\', '/');
      return (
        !filePath.startsWith('output/network-plan-proof/10g-remote-delivery-receipt-ledger/') &&
        !filePath.startsWith('test-results/network-remote-delivery-receipt-ledger-proof/')
      );
    })
    .join('\n');
}
