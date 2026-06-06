import { spawnSync } from 'node:child_process';
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

const proofRoot = join('output', 'network-plan-proof', '36b-adapter-capability-status');
const testRoot = join('test-results', 'network-adapter-capability-status-proof');
mkdirSync(proofRoot, { recursive: true });
mkdirSync(testRoot, { recursive: true });

assertSourceContracts();

writeFileSync(
  join(proofRoot, 'expected-adapter-capability-status.json'),
  `${JSON.stringify(
    {
      acceptedInputs: [
        'service-backed supported-adapter runtime proof event',
        'network observe-only policy handoff row',
        'host network/domain manual-required row',
        'host network/domain artifact-status manual row',
        'degraded, unavailable, unsupported, and not-claimed capability rows',
      ],
      renderedStates: [
        'implemented-boundary',
        'manual-required',
        'degraded',
        'unavailable',
        'unsupported',
        'not-claimed',
      ],
      uiBoundary:
        'Portal renders existing service capability rows in the Network activity drawer and does not publish policy, enforcement, adapter, or host-filter commands.',
      noClaims: [
        'exact URL from network-only evidence',
        'decrypted payload',
        'portal policy authority',
        'portal enforcement action',
        'live DNS mutation',
        'firewall or WFP mutation',
        'host filtering',
        'production platform support',
      ],
    },
    null,
    2
  )}\n`
);

const commands = [
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
    name: 'portal-network-adapter-capability-status-e2e',
    command: 'cmd',
    args: [
      '/c',
      'npm',
      'run',
      'test:e2e',
      '--workspace',
      '@ocentra-parent/portal',
      '--',
      'e2e/network-adapter-capability-status-proof.spec.ts',
    ],
    log: join(proofRoot, 'portal-network-adapter-capability-status-e2e.log'),
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

const proof = {
  proof: 'network-adapter-capability-status-proof',
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
    expectedAdapterCapabilityStatus: join(proofRoot, 'expected-adapter-capability-status.json'),
    proofSummary: join(proofRoot, 'proof-summary.json'),
    testProof: join(testRoot, 'proof.json'),
  },
  coveredRows: [
    'network-plan row 36 parent UI network evidence drawer',
    'network-plan row 36b adapter capability status drawer proof',
    'network feature doc adapter capability status gap',
  ],
  provenBoundaries: [
    'portal overview refresh requests the existing supported-adapter runtime proof from the Rust service',
    'portal parses the shared agent-protocol-domain adapter event instead of owning parser discriminants',
    'network drawer renders observe-only policy handoff, manual-required host-domain gates, degraded dependency, unavailable Linux, unsupported macOS, and exact active-tab not-claimed rows',
    'managed Playwright proof exercises the real Rust service, Vite portal, and WebSocket event path',
    'focused runner forwards Playwright spec args and still defaults to the full suite when no spec is supplied',
  ],
  notClaimed: [
    'exact page, video, message, search, or full URL content from network-only evidence',
    'decrypted payload availability',
    'portal policy authority',
    'portal enforcement or adapter dispatch',
    'live DNS mutation, firewall mutation, WFP execution, packet blocking, or host filtering',
    'production platform support or cross-platform enforcement readiness',
    'risk-budget UI completion or full network-plan completion',
  ],
};

writeFileSync(join(proofRoot, 'proof-summary.json'), `${JSON.stringify(proof, null, 2)}\n`);
writeFileSync(join(testRoot, 'proof.json'), `${JSON.stringify(proof, null, 2)}\n`);
console.log('network-adapter-capability-status-proof-ok:lint,build,e2e,source-shape,diff-check');
console.log(`proof=${join(proofRoot, 'proof-summary.json')}`);

function assertSourceContracts() {
  const parser = readFileSync(
    'packages/agent-protocol-domain/src/enforcement-supported-adapter-runtime-proof-adapter.ts',
    'utf8'
  );
  const commandsSource = readFileSync('packages/portal-domain/src/commands.ts', 'utf8');
  const portalSummary = readFileSync('apps/portal/src/network-adapter-capability-status.ts', 'utf8');
  const portalPanel = readFileSync('apps/portal/src/NetworkEvidenceDrawerRoutePanel.tsx', 'utf8');
  const portalSpec = readFileSync('apps/portal/e2e/network-adapter-capability-status-proof.spec.ts', 'utf8');
  const runner = readFileSync('scripts/test/portal-playwright-runner.mjs', 'utf8');
  const featureDoc = readFileSync('docs/features/network-domain-control.md', 'utf8');
  const checklist = readFileSync('docs/plans/network-plan/implementation-checklist.md', 'utf8');
  const workpacks = readFileSync('docs/plans/network-plan/workpacks/README.md', 'utf8');
  const requiredSnippets = [
    [parser, 'parseEnforcementSupportedAdapterRuntimeProofReadModel'],
    [commandsSource, 'EnforcementSupportedAdapterRuntimeProofGet'],
    [portalSummary, 'WindowsNetworkFlowObservePolicyHandoff'],
    [portalPanel, 'NetworkAdapterCapabilityStatusCard'],
    [portalSpec, 'windows-network-flow-observe-policy-handoff'],
    [runner, 'forwardedPlaywrightArgs'],
    [featureDoc, 'network-adapter-capability-status-proof'],
    [checklist, '36b parent UI adapter capability status'],
    [workpacks, '36b'],
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
  const status = runText('git', ['status', '--short']);
  return status
    .split(/\r?\n/)
    .filter((line) => {
      if (line.trim().length === 0) {
        return false;
      }
      const path = line.slice(3).replaceAll('\\', '/');
      return (
        !path.startsWith('output/network-plan-proof/36b-adapter-capability-status/') &&
        !path.startsWith('test-results/network-adapter-capability-status-proof/')
      );
    })
    .join('\n');
}
