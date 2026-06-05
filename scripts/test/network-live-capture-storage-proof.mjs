import { spawnSync } from 'node:child_process';
import { mkdirSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

const proofRoot = join('output', 'network-plan-proof', '03a-live-capture-storage-proof');
const testRoot = join('test-results', 'network-live-capture-storage-proof');
mkdirSync(proofRoot, { recursive: true });
mkdirSync(testRoot, { recursive: true });

writeFileSync(
  join(proofRoot, '03a-live-capture-storage-proof.json'),
  `${JSON.stringify(
    {
      row: '03a-live-capture-storage-proof',
      storageContract: 'Raw capture artifacts are allowed only with local encrypted custody refs.',
      requiredRefs: [
        'live capture proof-ready ref',
        'raw artifact manifest ref',
        'storage location ref',
        'encryption-at-rest ref',
        'quota rotation ref',
        'retention policy ref',
        'delete/export ref',
        'custody chain ref',
        'private traffic exclusion ref',
      ],
      provenStates: ['custody-ready', 'manual-required', 'unavailable', 'degraded'],
      manualRequiredWhen: [
        'raw artifact manifest is missing',
        'storage location is missing',
        'encryption at rest is not verified',
        'delete/export is not verified',
        'custody chain is not verified',
        'live capture proof is manual-required',
      ],
      notClaimed: [
        'live capture execution',
        'remote upload',
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
    },
    null,
    2
  )}\n`
);

const commands = [
  {
    name: 'network-live-capture-storage-tests',
    command: 'cargo',
    args: ['test', '-p', 'ocentra-network-evidence', 'raw_capture_storage'],
    log: join(proofRoot, 'raw-capture-storage-tests.log'),
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
writeFileSync(join(proofRoot, '12-validation-commands.log'), validationCommandsLog(commandResults));

const proof = {
  proof: 'network-live-capture-storage',
  checkedAt: new Date().toISOString(),
  branch: runText('git', ['branch', '--show-current']).trim(),
  commit: runText('git', ['rev-parse', 'HEAD']).trim(),
  sourceStatusShort: sourceStatusShort(),
  proofRoot,
  testRoot,
  commands: commandResults,
  artifacts: {
    liveCaptureStorageProof: join(proofRoot, '03a-live-capture-storage-proof.json'),
    validationCommands: join(proofRoot, '12-validation-commands.log'),
    proofSummary: join(proofRoot, 'proof-summary.json'),
    testProof: join(testRoot, 'proof.json'),
  },
  provenRows: ['03a Live capture storage custody proof'],
  provenRootGates: [
    'raw capture artifacts require encrypted local custody refs',
    'quota, retention, delete/export, custody, and private-traffic exclusion refs are preserved',
    'manual-required, unavailable, and degraded states remain visible without storage authorization',
    'raw PCAP without custody and remote upload claims are rejected',
  ],
  notClaimed: [
    'live capture execution',
    'remote upload',
    'raw PCAP without custody',
    'exact URL, page content, private message, search query, or decrypted payload',
    'policy or adapter authority',
    'enforcement command publication',
  ],
};
writeFileSync(join(proofRoot, 'proof-summary.json'), `${JSON.stringify(proof, null, 2)}\n`);
writeFileSync(join(testRoot, 'proof.json'), `${JSON.stringify(proof, null, 2)}\n`);
console.log('network-live-capture-storage-proof-ok:tests,clippy,source-shape');
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
    ':(exclude)output/network-plan-proof/03a-live-capture-storage-proof',
    ':(exclude)test-results/network-live-capture-storage-proof',
  ]);
}

function validationCommandsLog(results) {
  const lines = [
    'network-live-capture-storage validation commands',
    '',
    ...results.map((result) => `${result.name}: ${result.command} -> exit ${result.status}; log=${result.log}`),
    '',
    'Additional check expected before commit: git diff --check.',
  ];
  return `${lines.join('\n')}\n`;
}
