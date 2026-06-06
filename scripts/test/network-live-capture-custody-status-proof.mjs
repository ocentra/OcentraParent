import { spawnSync } from 'node:child_process';
import { mkdirSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

const proofRoot = join('output', 'network-plan-proof', '13a-live-capture-custody-status');
const testRoot = join('test-results', 'network-live-capture-custody-status-proof');
const validationLogPath = join(proofRoot, '12-validation-commands.log');
const securityLogPath = join(proofRoot, '09-security-negative-proof.log');
const boundaryPath = join(proofRoot, '13a-live-capture-custody-status.json');
const proofSummaryPath = join(proofRoot, 'proof-summary.json');
const testProofPath = join(testRoot, 'proof.json');

mkdirSync(proofRoot, { recursive: true });
mkdirSync(testRoot, { recursive: true });

const boundary = {
  reportRef: 'network-live-capture-custody-status-row13a',
  composedProofs: ['row13 live-capture readiness gate', 'row03a raw-capture storage custody gate'],
  supportedStates: ['custody-ready', 'manual-required', 'unavailable', 'degraded'],
  requiredBeforeCustodyReady: [
    'proof-ready live capture gate',
    'raw artifact manifest ref',
    'local encrypted storage location ref',
    'encryption-at-rest verification ref',
    'quota rotation ref',
    'retention policy ref',
    'delete/export ref',
    'custody chain ref',
    'private-family-traffic exclusion ref',
  ],
  rejectedClaims: [
    'live driver execution by status materializer',
    'raw artifact creation by status materializer',
    'remote upload',
    'raw PCAP without custody',
    'exact URL',
    'page content',
    'private message',
    'search query',
    'decrypted payload',
    'policy authority',
    'adapter authority',
    'enforcement command publication',
  ],
  authorityBoundary:
    'Row13a materializes the existing capture and storage gates into one auditable custody status. It does not invoke drivers, create raw artifacts, upload evidence, inspect content, or authorize policy, adapter, or enforcement actions.',
};

writeFileSync(boundaryPath, `${JSON.stringify(boundary, null, 2)}\n`);

const commands = [
  {
    name: 'network-live-capture-custody-status-tests',
    command: 'cargo',
    args: ['test', '-p', 'ocentra-network-evidence', 'live_capture_custody_status'],
    log: join(proofRoot, 'live-capture-custody-status-tests.log'),
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
  {
    name: 'diff-check',
    command: 'git',
    args: ['diff', '--check'],
    log: join(proofRoot, 'diff-check.log'),
  },
];

const commandResults = commands.map(runCommand);
writeFileSync(
  validationLogPath,
  `${commandResults.map((entry) => `${entry.command}\nlog=${entry.log}`).join('\n\n')}\n`
);

const checkedAt = new Date().toISOString();
const proof = {
  proof: 'network-live-capture-custody-status',
  checkedAt,
  branch: runText('git', ['branch', '--show-current']).trim(),
  commit: runText('git', ['rev-parse', 'HEAD']).trim(),
  originMain: runText('git', ['rev-parse', 'origin/main']).trim(),
  mergeBase: runText('git', ['merge-base', 'HEAD', 'origin/main']).trim(),
  sourceStatusShort: sourceStatusShort(),
  proofRoot,
  testRoot,
  commands: commandResults,
  artifacts: {
    boundary: boundaryPath,
    securityNegativeLog: securityLogPath,
    validationCommands: validationLogPath,
    proofSummary: proofSummaryPath,
    testProof: testProofPath,
  },
  provenRows: ['13a live capture custody status materializer'],
  provenBoundaries: [
    'live-capture and raw-capture storage proof refs are composed into one auditable custody status',
    'manual-required, unavailable, and degraded capture/storage states remain visible',
    'custody-ready status still reports no driver invocation, no raw artifact creation, no remote upload, and zero enforcement commands',
    'mismatched live-capture proof refs are rejected before status materialization',
  ],
  notClaimed: [
    'service WebSocket command wiring while codex-d owns shared protocol/service files',
    'live capture driver invocation',
    'raw artifact creation',
    'remote upload',
    'raw PCAP without custody',
    'exact URL, page content, private message, search query, or decrypted payload',
    'policy or adapter authority',
    'enforcement command publication',
  ],
};

writeFileSync(
  securityLogPath,
  [
    `checkedAt=${checkedAt}`,
    'asserted=no live driver invocation by custody status materializer',
    'asserted=no raw artifact creation by custody status materializer',
    'asserted=no remote upload and no raw PCAP without custody',
    'asserted=no exact URL, decrypted payload, page content, private message, or search query claim',
    'asserted=no policy authority, adapter authority, or enforcement command publication',
  ].join('\n') + '\n'
);
writeFileSync(proofSummaryPath, `${JSON.stringify(proof, null, 2)}\n`);
writeFileSync(testProofPath, `${JSON.stringify(proof, null, 2)}\n`);

console.log('network-live-capture-custody-status-proof-ok:tests,clippy,source-shape,diff-check');
console.log(`proof=${proofSummaryPath}`);

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
  const status = runText('git', ['status', '--short']);
  return status
    .split(/\r?\n/u)
    .filter((line) => {
      if (line.trim().length === 0) {
        return false;
      }
      const filePath = line.slice(3).replaceAll('\\', '/');
      return (
        !filePath.startsWith('output/network-plan-proof/13a-live-capture-custody-status/') &&
        !filePath.startsWith('test-results/network-live-capture-custody-status-proof/')
      );
    })
    .join('\n');
}
