import { spawnSync } from 'node:child_process';
import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join, resolve } from 'node:path';

const proofRoot = join('output', 'network-plan-proof', '51c-product-readiness-portal');
const screenshotDir = join(proofRoot, '08-ui-snapshots');
const testRoot = join('test-results', 'network-product-readiness-portal-proof');
mkdirSync(screenshotDir, { recursive: true });
mkdirSync(testRoot, { recursive: true });

const screenshotPath = join(screenshotDir, 'network-product-readiness-status.png');
const screenshotAbsolutePath = resolve(screenshotPath);
const securityLogPath = join(proofRoot, '09-security-negative-proof.log');
const validationLogPath = join(proofRoot, '12-validation-commands.log');

assertSourceContracts();

writeFileSync(
  join(proofRoot, 'expected-product-readiness-portal.json'),
  `${JSON.stringify(
    {
      acceptedInputs: [
        'agent.network.product-readiness.status.reported service event',
        'networkLiveCaptureCustodyStatus JSON payload field',
        'networkProductReadinessStatus JSON payload field',
        'row13a live-capture custody status',
        'row51a product-readiness status',
      ],
      renderedStates: [
        'true parser status for valid service events',
        'invalid-product-readiness-status parser status for malformed service events',
        'network.live-capture.custody-status.13a',
        'CustodyReady',
        'ProofReady',
        'network.product-readiness.status.51a',
        'ManualRequired',
        'AskParentThreshold',
        'AskParent',
        'MeetsBenchmarkGate',
        'DryRun',
        'WindowsWfp | network.platform-claim.manual-followup.51a',
      ],
      uiBoundary:
        'Portal renders service-backed row13a custody and row51a product-readiness status in the Activity route network drawer.',
      noClaims: [
        'exact URL or page content from network-only evidence',
        'decrypted payload',
        'raw PCAP without custody',
        'live capture driver invocation',
        'portal policy authority',
        'portal adapter dispatch',
        'live host adapter execution',
        'production SLO validation',
        'enforcement command publication',
      ],
    },
    null,
    2
  )}\n`
);

const commands = [
  {
    name: 'text-domain-lint',
    command: 'cmd',
    args: ['/c', 'npm', 'run', 'lint:exec', '--workspace', '@ocentra-parent/text-domain'],
    log: join(proofRoot, 'text-domain-lint.log'),
  },
  {
    name: 'text-domain-build',
    command: 'cmd',
    args: ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/text-domain'],
    log: join(proofRoot, 'text-domain-build.log'),
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
    name: 'portal-network-product-readiness-status-e2e',
    command: 'cmd',
    args: [
      '/c',
      'npm',
      'run',
      'test:e2e',
      '--workspace',
      '@ocentra-parent/portal',
      '--',
      'e2e/network-product-readiness-status-proof.spec.ts',
    ],
    env: {
      NETWORK_PRODUCT_READINESS_STATUS_SCREENSHOT: screenshotAbsolutePath,
    },
    log: join(proofRoot, 'portal-network-product-readiness-status-e2e.log'),
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
  throw new Error(`missing network product-readiness screenshot: ${screenshotPath}`);
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
    'asserted=no UI-owned policy authority or adapter dispatch',
    'asserted=no live capture driver invocation, host filtering, or live adapter execution claim',
    'asserted=no production SLO validation claim',
  ].join('\n') + '\n'
);

const proof = {
  proof: 'network-product-readiness-portal-proof',
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
    expectedProductReadinessPortal: join(proofRoot, 'expected-product-readiness-portal.json'),
    screenshot: screenshotPath,
    securityNegativeLog: securityLogPath,
    validationCommands: validationLogPath,
    proofSummary: join(proofRoot, 'proof-summary.json'),
    testProof: join(testRoot, 'proof.json'),
  },
  coveredRows: [
    'network-plan supplemental row 51c product-readiness portal rendering',
    'network feature doc parent portal network evidence drawer gap',
    'apps/portal Activity route network drawer',
  ],
  provenBoundaries: [
    'portal overview refresh requests the service-backed product-readiness status command',
    'portal command controls expose an explicit network readiness refresh button for the same status event',
    'portal parser consumes shared agent-protocol-domain payload field constants',
    'Activity route network drawer renders live-capture custody, raw-storage custody, risk, performance, platform, portal, retention, and no-claim status values',
    'Activity route keeps malformed product-readiness status visible as a typed parser failure instead of hiding it as missing data',
    'managed Playwright proof exercises the real Rust service, Vite portal, and WebSocket event path',
  ],
  notClaimed: [
    'exact URL, page content, video, message, search, or decrypted payload from network-only evidence',
    'raw PCAP without custody or live capture driver invocation',
    'portal policy authority or local policy evaluation',
    'portal adapter dispatch, live adapter execution, host filtering, or enforcement command publication',
    'production SLO validation or full network-plan completion',
  ],
};

writeFileSync(join(proofRoot, 'proof-summary.json'), `${JSON.stringify(proof, null, 2)}\n`);
writeFileSync(join(testRoot, 'proof.json'), `${JSON.stringify(proof, null, 2)}\n`);
console.log('network-product-readiness-portal-proof-ok:lint,build,unit,e2e,schema-boundaries,source-shape,diff-check');
console.log(`proof=${join(proofRoot, 'proof-summary.json')}`);

function assertSourceContracts() {
  const parser = readFileSync('packages/agent-protocol-domain/src/network-product-readiness-status.ts', 'utf8');
  const packageJson = readFileSync('packages/agent-protocol-domain/package.json', 'utf8');
  const commandsSource = readFileSync('packages/portal-domain/src/commands.ts', 'utf8');
  const portalSummary = readFileSync('apps/portal/src/network-product-readiness-status.ts', 'utf8');
  const liveState = readFileSync('apps/portal/src/live-activity-state.ts', 'utf8');
  const portalPanel = readFileSync('apps/portal/src/NetworkEvidenceDrawerRoutePanel.tsx', 'utf8');
  const portalSpec = readFileSync('apps/portal/e2e/network-product-readiness-status-proof.spec.ts', 'utf8');
  const featureDoc = readFileSync('docs/features/network-domain-control.md', 'utf8');
  const checklist = readFileSync('docs/plans/network-plan/implementation-checklist.md', 'utf8');
  const workpacks = readFileSync('docs/plans/network-plan/workpacks/README.md', 'utf8');
  const requiredSnippets = [
    [parser, 'parseAgentNetworkProductReadinessStatusEvent'],
    [packageJson, './network-product-readiness-status'],
    [commandsSource, 'NetworkProductReadinessStatusGet'],
    [commandsSource, 'GetNetworkProductReadinessStatus'],
    [portalSummary, 'noClaimBoundaryUpgraded'],
    [portalSummary, 'failedNetworkProductReadinessStatusSummary'],
    [liveState, 'NetworkProductReadinessStatusReported'],
    [portalPanel, 'NetworkProductReadinessStatusCard'],
    [portalSpec, 'network.product-readiness.status.51a'],
    [featureDoc, 'network-product-readiness-portal-proof'],
    [checklist, '51c network product-readiness portal rendering'],
    [workpacks, '51c'],
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
        !filePath.startsWith('output/network-plan-proof/51c-product-readiness-portal/') &&
        !filePath.startsWith('test-results/network-product-readiness-portal-proof/')
      );
    })
    .join('\n');
}
