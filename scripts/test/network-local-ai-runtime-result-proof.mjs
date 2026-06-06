import { spawnSync } from 'node:child_process';
import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

const proofRoot = join('output', 'network-plan-proof', '33a-network-local-ai-runtime-result');
const testRoot = join('test-results', 'network-local-ai-runtime-result-proof');
mkdirSync(proofRoot, { recursive: true });
mkdirSync(testRoot, { recursive: true });

assertSourceContracts();

writeFileSync(
  join(proofRoot, 'expected-network-local-ai-runtime-result.json'),
  `${JSON.stringify(
    {
      proof: 'network-local-ai-runtime-result-proof',
      acceptedInputs: [
        'NetworkLocalAiQueuePlan',
        'local-AI runtime result refs',
        'prompt template refs',
        'policy context refs',
        'parent rule refs',
      ],
      resultStates: ['ResultReady', 'RuntimeUnavailable', 'RuntimeFailed', 'RuntimeTimedOut', 'QueueNotReady'],
      noClaims: [
        'raw PCAP input',
        'exact URL from network-only evidence',
        'page content',
        'private message',
        'search query',
        'decrypted payload',
        'remote AI provider',
        'policy authority',
        'adapter authority',
        'enforcement command publication',
        'model execution proof',
      ],
    },
    null,
    2
  )}\n`
);

const commands = [
  {
    name: 'network-local-ai-runtime-result-rust-test',
    command: 'cargo',
    args: ['test', '-p', 'ocentra-network-evidence', 'local_ai_runtime_result'],
    log: join(proofRoot, 'network-local-ai-runtime-result-rust-test.log'),
  },
  {
    name: 'network-local-ai-queue-rust-test',
    command: 'cargo',
    args: ['test', '-p', 'ocentra-network-evidence', 'local_ai_queue'],
    log: join(proofRoot, 'network-local-ai-queue-rust-test.log'),
  },
  {
    name: 'network-ai-detection-rust-test',
    command: 'cargo',
    args: ['test', '-p', 'ocentra-network-evidence', 'ai_detection'],
    log: join(proofRoot, 'network-ai-detection-rust-test.log'),
  },
  {
    name: 'network-ai-audit-rust-test',
    command: 'cargo',
    args: ['test', '-p', 'ocentra-network-evidence', 'ai_audit'],
    log: join(proofRoot, 'network-ai-audit-rust-test.log'),
  },
  {
    name: 'network-evidence-clippy',
    command: 'cargo',
    args: ['clippy', '-p', 'ocentra-network-evidence', '--all-targets', '--', '-D', 'warnings'],
    log: join(proofRoot, 'network-evidence-clippy.log'),
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

const results = commands.map(runCommand);
const proofSummary = {
  proof: 'network-local-ai-runtime-result-proof',
  checkedAt: new Date().toISOString(),
  branch: gitValue(['branch', '--show-current']),
  commit: gitValue(['rev-parse', 'HEAD']),
  originMain: gitValue(['rev-parse', 'origin/main']),
  mergeBase: gitValue(['merge-base', 'HEAD', 'origin/main']),
  sourceStatusShort: gitValue(['status', '--short']),
  proofRoot,
  testRoot,
  commands: results,
  artifacts: {
    expected: join(proofRoot, 'expected-network-local-ai-runtime-result.json'),
    proofSummary: join(proofRoot, 'proof-summary.json'),
    testProof: join(testRoot, 'proof.json'),
  },
  docs: {
    featureDoc: 'docs/features/network-domain-control.md',
    checklist: 'docs/plans/network-plan/implementation-checklist.md',
    workpacks: 'docs/plans/network-plan/workpacks/README.md',
    crateReadme: 'crates/ocentra-network-evidence/README.md',
  },
  noClaims: {
    rawPcapInput: false,
    exactUrlFromNetworkOnlyEvidence: false,
    pageContent: false,
    privateMessage: false,
    searchQuery: false,
    decryptedPayload: false,
    remoteAiProvider: false,
    policyAuthority: false,
    adapterAuthority: false,
    enforcementCommandPublished: false,
    modelExecutionProved: false,
  },
};

writeFileSync(join(proofRoot, 'proof-summary.json'), `${JSON.stringify(proofSummary, null, 2)}\n`);
writeFileSync(join(testRoot, 'proof.json'), `${JSON.stringify(proofSummary, null, 2)}\n`);

console.log('network-local-ai-runtime-result-proof-ok:rust,clippy,schema-boundaries,source-shape,diff-check');
console.log(`proof=${join(proofRoot, 'proof-summary.json')}`);

function runCommand(entry) {
  const result = spawnSync(entry.command, entry.args, {
    encoding: 'utf8',
    shell: false,
    env: { ...process.env },
  });
  const output = `${result.stdout ?? ''}${result.stderr ?? ''}`;
  writeFileSync(entry.log, output);
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

function gitValue(args) {
  const result = spawnSync('git', args, { encoding: 'utf8', shell: false });
  if (result.status !== 0) {
    return '';
  }
  return result.stdout.trim();
}

function assertSourceContracts() {
  assertFileIncludes('docs/features/network-domain-control.md', 'network-local-ai-runtime-result-proof');
  assertFileIncludes(
    'docs/plans/network-plan/implementation-checklist.md',
    '33a network local-AI runtime result bridge'
  );
  assertFileIncludes('docs/plans/network-plan/workpacks/README.md', '33a');
  assertFileIncludes('crates/ocentra-network-evidence/README.md', 'runtime result bridge');
}

function assertFileIncludes(path, expected) {
  if (!existsSync(path)) {
    throw new Error(`${path} missing`);
  }
  const text = readFileSync(path, 'utf8');
  if (!text.includes(expected)) {
    throw new Error(`${path} missing ${expected}`);
  }
}
