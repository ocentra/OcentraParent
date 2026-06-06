import { spawnSync } from 'node:child_process';
import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join, resolve } from 'node:path';

const proofRoot = join('output', 'network-plan-proof', '51d-risk-performance-readiness-portal');
const screenshotDir = join(proofRoot, '08-ui-snapshots');
const testRoot = join('test-results', 'network-risk-performance-readiness-portal-proof');
mkdirSync(screenshotDir, { recursive: true });
mkdirSync(testRoot, { recursive: true });

const screenshotPath = join(screenshotDir, 'network-risk-performance-readiness.png');
const screenshotAbsolutePath = resolve(screenshotPath);
const securityLogPath = join(proofRoot, '09-security-negative-proof.log');
const validationLogPath = join(proofRoot, '12-validation-commands.log');

assertSourceContracts();

writeFileSync(
  join(proofRoot, 'expected-risk-performance-readiness-portal.json'),
  `${JSON.stringify(
    {
      acceptedInputs: [
        'agent.network.product-readiness.status.reported service event',
        'networkProductReadinessStatus JSON payload field',
        'row48 risk-budget details',
        'row49 performance benchmark details',
        'row51a product-readiness status carrier',
      ],
      renderedStates: [
        'network.risk-evaluation.51a',
        'child-profile.network.51a',
        'household-policy.network.51a',
        'network.signal.51a',
        'policy.rule.network-domain.1',
        'network.performance.benchmark.51a',
        'network.performance.fixtures.51a',
        'MeetsBenchmarkGate',
        'DryRun',
        'false production-SLO, adapter-dispatch, and host-filtering rows',
      ],
      uiBoundary:
        'Portal renders service-backed risk-budget and performance benchmark details from the product-readiness status event.',
      noClaims: [
        'exact URL or page content from network-only evidence',
        'decrypted payload',
        'production SLO validation',
        'live adapter execution',
        'host filtering',
        'UI policy authority',
        'enforcement command publication',
      ],
    },
    null,
    2
  )}\n`
);

const commands = [
  {
    name: 'network-evidence-product-readiness-rust-test',
    command: 'cargo',
    args: ['test', '-p', 'ocentra-network-evidence', 'product_readiness_status'],
    log: join(proofRoot, 'network-evidence-product-readiness-rust-test.log'),
  },
  {
    name: 'agent-service-product-readiness-rust-test',
    command: 'cargo',
    args: ['test', '-p', 'ocentra-parent-agent-service', 'network_product_readiness_status'],
    log: join(proofRoot, 'agent-service-product-readiness-rust-test.log'),
  },
  {
    name: 'agent-protocol-domain-lint',
    command: 'cmd',
    args: ['/c', 'npm', 'run', 'lint:exec', '--workspace', '@ocentra-parent/agent-protocol-domain'],
    log: join(proofRoot, 'agent-protocol-domain-lint.log'),
  },
  {
    name: 'agent-protocol-domain-build',
    command: 'cmd',
    args: ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/agent-protocol-domain'],
    log: join(proofRoot, 'agent-protocol-domain-build.log'),
  },
  {
    name: 'agent-protocol-domain-product-readiness-test',
    command: 'cmd',
    args: [
      '/c',
      'npm',
      'run',
      'test',
      '--workspace',
      '@ocentra-parent/agent-protocol-domain',
      '--',
      'network-product-readiness-status.test.ts',
    ],
    log: join(proofRoot, 'agent-protocol-domain-product-readiness-test.log'),
  },
  {
    name: 'portal-domain-lint',
    command: 'cmd',
    args: ['/c', 'npm', 'run', 'lint:exec', '--workspace', '@ocentra-parent/portal-domain'],
    log: join(proofRoot, 'portal-domain-lint.log'),
  },
  {
    name: 'portal-domain-build',
    command: 'cmd',
    args: ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/portal-domain'],
    log: join(proofRoot, 'portal-domain-build.log'),
  },
  {
    name: 'portal-lint',
    command: 'cmd',
    args: ['/c', 'npm', 'run', 'lint:exec', '--workspace', '@ocentra-parent/portal'],
    log: join(proofRoot, 'portal-lint.log'),
  },
  {
    name: 'portal-live-activity-network-flow-test',
    command: 'cmd',
    args: ['/c', 'npm', 'run', 'test', '--workspace', '@ocentra-parent/portal', '--', 'live-activity-network-flow'],
    log: join(proofRoot, 'portal-live-activity-network-flow-test.log'),
  },
  {
    name: 'agent-service-build',
    command: 'cargo',
    args: ['build', '-p', 'ocentra-parent-agent-service'],
    log: join(proofRoot, 'agent-service-build.log'),
  },
  {
    name: 'portal-network-risk-performance-readiness-e2e',
    command: 'cmd',
    args: [
      '/c',
      'npm',
      'run',
      'test:e2e',
      '--workspace',
      '@ocentra-parent/portal',
      '--',
      'e2e/network-risk-performance-readiness-portal-proof.spec.ts',
    ],
    env: {
      NETWORK_RISK_PERFORMANCE_READINESS_SCREENSHOT: screenshotAbsolutePath,
    },
    log: join(proofRoot, 'portal-network-risk-performance-readiness-e2e.log'),
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
if (!existsSync(screenshotPath)) {
  throw new Error(`missing network risk/performance readiness screenshot: ${screenshotPath}`);
}
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
    'asserted=no production SLO validation claim',
    'asserted=no UI-owned policy authority or adapter dispatch',
    'asserted=no live adapter execution, host filtering, or enforcement command publication claim',
  ].join('\n') + '\n'
);

const proof = {
  proof: 'network-risk-performance-readiness-portal-proof',
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
    expectedRiskPerformanceReadinessPortal: join(proofRoot, 'expected-risk-performance-readiness-portal.json'),
    screenshot: screenshotPath,
    securityNegativeLog: securityLogPath,
    validationCommands: validationLogPath,
    proofSummary: join(proofRoot, 'proof-summary.json'),
    testProof: join(testRoot, 'proof.json'),
  },
  coveredRows: [
    'network-plan supplemental row 51d risk/performance readiness portal details',
    'network-plan row48 household risk-budget threshold model',
    'network-plan row49 performance benchmark proof',
    'apps/portal Activity route network drawer',
  ],
  provenBoundaries: [
    'service-backed product-readiness status carries risk-budget details and performance benchmark metrics',
    'agent-protocol-domain parser validates risk/performance detail fields and false authority/execution rows',
    'Activity route renders risk refs, risk score breakdown, performance counts, latency, throughput, resources, and no-claim flags',
    'managed Playwright proof exercises the real Rust service, Vite portal, and WebSocket event path',
  ],
  notClaimed: [
    'exact URL, page content, video, message, search, or decrypted payload from network-only evidence',
    'production SLO validation, UI policy authority, live adapter execution, host filtering, or enforcement command publication',
    'full network-plan completion',
  ],
};

writeFileSync(join(proofRoot, 'proof-summary.json'), `${JSON.stringify(proof, null, 2)}\n`);
writeFileSync(join(testRoot, 'proof.json'), `${JSON.stringify(proof, null, 2)}\n`);
console.log(
  'network-risk-performance-readiness-portal-proof-ok:rust,protocol,portal,e2e,schema-boundaries,source-shape,diff-check'
);
console.log(`proof=${join(proofRoot, 'proof-summary.json')}`);

function assertSourceContracts() {
  const rustStatus = readFileSync('crates/ocentra-network-evidence/src/product_readiness_status.rs', 'utf8');
  const servicePayload = readFileSync('crates/agent-service/src/network_product_readiness_status_payload.rs', 'utf8');
  const parser = readFileSync('packages/agent-protocol-domain/src/network-product-readiness-status.ts', 'utf8');
  const portalSummary = readFileSync('apps/portal/src/network-product-readiness-status.ts', 'utf8');
  const portalCard = readFileSync('apps/portal/src/network-risk-performance-readiness-card.tsx', 'utf8');
  const portalPanel = readFileSync('apps/portal/src/NetworkEvidenceDrawerRoutePanel.tsx', 'utf8');
  const portalSpec = readFileSync('apps/portal/e2e/network-risk-performance-readiness-portal-proof.spec.ts', 'utf8');
  const featureDoc = readFileSync('docs/features/network-domain-control.md', 'utf8');
  const checklist = readFileSync('docs/plans/network-plan/implementation-checklist.md', 'utf8');
  const workpacks = readFileSync('docs/plans/network-plan/workpacks/README.md', 'utf8');
  const requiredSnippets = [
    [rustStatus, 'risk_evaluation_ref'],
    [rustStatus, 'performance_benchmark_run_ref'],
    [servicePayload, 'TEST_RISK_EVALUATION_REF'],
    [servicePayload, 'TEST_PERFORMANCE_BENCHMARK_REF'],
    [parser, 'risk_evaluation_ref'],
    [parser, 'performance_host_filtering_executed'],
    [portalSummary, 'riskPointBreakdown'],
    [portalSummary, 'performanceLatencyMetrics'],
    [portalCard, 'NetworkRiskPerformanceReadinessCard'],
    [portalPanel, 'NetworkRiskPerformanceReadinessCard'],
    [portalSpec, 'network.performance.benchmark.51a'],
    [featureDoc, 'network-risk-performance-readiness-portal-proof'],
    [checklist, '51d risk/performance readiness portal details'],
    [workpacks, '51d'],
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
    env: {
      ...process.env,
      ...(entry.env ?? {}),
    },
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
        !filePath.startsWith('output/network-plan-proof/51d-risk-performance-readiness-portal/') &&
        !filePath.startsWith('test-results/network-risk-performance-readiness-portal-proof/')
      );
    })
    .join('\n');
}
