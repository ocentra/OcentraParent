import { spawnSync } from 'node:child_process';
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

const proofRoot = join('output', 'network-plan-proof', '51a-product-readiness-status');
const testRoot = join('test-results', 'network-product-readiness-status-proof');
mkdirSync(proofRoot, { recursive: true });
mkdirSync(testRoot, { recursive: true });

assertSourceContracts();

writeFileSync(
  join(proofRoot, 'expected-product-readiness-status.json'),
  `${JSON.stringify(
    {
      acceptedInputs: [
        'row48 household risk-budget evaluation proof',
        'row49 performance benchmark proof',
        'row52 platform-claim manifest proof',
        'portal read-model ref',
        'retention/export ref',
      ],
      readinessStates: ['ready-for-portal', 'manual-required', 'degraded'],
      provenBoundaries: [
        'manual platform follow-ups remain visible',
        'performance regressions degrade readiness instead of claiming production SLO',
        'risk-budget output remains advisory and cannot publish policy, adapter, or enforcement authority',
        'platform manifest cannot claim UI policy authority, live adapter execution, or enforcement commands',
      ],
      unsupportedClaimsRejected: [
        'exact URL',
        'decrypted payload',
        'page content',
        'production SLO',
        'portal policy authority',
        'portal adapter dispatch',
        'live adapter execution',
        'enforcement command',
      ],
    },
    null,
    2
  )}\n`
);

const commands = [
  {
    name: 'network-product-readiness-status-tests',
    command: 'cargo',
    args: ['test', '-p', 'ocentra-network-evidence', 'product_readiness_status'],
    log: join(proofRoot, 'product-readiness-status-tests.log'),
  },
  {
    name: 'network-risk-budget-row48-tests',
    command: 'cargo',
    args: ['test', '-p', 'ocentra-network-evidence', 'risk_budget'],
    log: join(proofRoot, 'risk-budget-tests.log'),
  },
  {
    name: 'network-performance-row49-tests',
    command: 'cargo',
    args: ['test', '-p', 'ocentra-network-evidence', 'performance'],
    log: join(proofRoot, 'performance-tests.log'),
  },
  {
    name: 'network-platform-claims-row52-tests',
    command: 'cargo',
    args: ['test', '-p', 'ocentra-network-evidence', 'platform_claims'],
    log: join(proofRoot, 'platform-claims-tests.log'),
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

const proof = {
  proof: 'network-product-readiness-status-proof',
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
    expectedProductReadinessStatus: join(proofRoot, 'expected-product-readiness-status.json'),
    proofSummary: join(proofRoot, 'proof-summary.json'),
    testProof: join(testRoot, 'proof.json'),
  },
  provenRows: ['51a network product readiness status materializer'],
  composedProofRows: [
    '48 household risk budget and cascade threshold model',
    '49 performance, latency, resource, and high-concurrency benchmark proof',
    '52 platform claim manifest proof',
  ],
  provenBoundaries: [
    'risk-budget, performance, and platform-claim proof objects compose into one portal-safe readiness status',
    'manual platform follow-ups and unavailable states remain visible',
    'performance regressions produce degraded readiness without production SLO claims',
    'portal read-model and retention/export refs are carried without UI policy or adapter authority',
  ],
  notClaimed: [
    'service WebSocket exposure for this status',
    'portal UI rendering of this status',
    'production SLO validation',
    'exact URL, page content, private message, search query, or decrypted payload',
    'policy engine execution',
    'adapter execution, host filtering, or live platform mutation',
    'enforcement command publication',
    'broker or family-hub delivery',
  ],
};

writeFileSync(join(proofRoot, 'proof-summary.json'), `${JSON.stringify(proof, null, 2)}\n`);
writeFileSync(join(testRoot, 'proof.json'), `${JSON.stringify(proof, null, 2)}\n`);
console.log('network-product-readiness-status-proof-ok:tests,clippy,source-shape,diff-check');
console.log(`proof=${join(proofRoot, 'proof-summary.json')}`);

function assertSourceContracts() {
  const lib = readFileSync('crates/ocentra-network-evidence/src/lib.rs', 'utf8');
  const status = readFileSync('crates/ocentra-network-evidence/src/product_readiness_status.rs', 'utf8');
  const tests = readFileSync('crates/ocentra-network-evidence/src/tests/product_readiness_status.rs', 'utf8');
  const requiredSnippets = [
    [lib, 'materialize_network_product_readiness_status'],
    [status, 'NetworkProductReadinessStatusState'],
    [status, 'NetworkPerformanceBenchmarkState::BenchmarkGateExceeded'],
    [status, 'RiskBudgetAuthorityClaimRejected'],
    [status, 'PlatformEnforcementClaimRejected'],
    [tests, 'product_readiness_status_preserves_manual_followups'],
    [tests, 'product_readiness_status_reports_performance_regression_as_degraded'],
  ];
  for (const [haystack, needle] of requiredSnippets) {
    if (!haystack.includes(needle)) {
      throw new Error(`missing source contract snippet: ${needle}`);
    }
  }
}

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
  return runText('git', ['status', '--short'])
    .split(/\r?\n/u)
    .filter((line) => {
      if (line.trim().length === 0) {
        return false;
      }
      const filePath = line.slice(3).replaceAll('\\', '/');
      return (
        !filePath.startsWith('output/network-plan-proof/51a-product-readiness-status/') &&
        !filePath.startsWith('test-results/network-product-readiness-status-proof/')
      );
    })
    .join('\n');
}
