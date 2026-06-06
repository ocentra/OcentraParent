import { spawnSync } from 'node:child_process';
import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join, resolve } from 'node:path';

const proofRoot = join('output', 'network-plan-proof', '33c-network-local-ai-runtime-result-portal');
const screenshotDir = join(proofRoot, '08-ui-snapshots');
const testRoot = join('test-results', 'network-local-ai-runtime-result-portal-proof');
mkdirSync(screenshotDir, { recursive: true });
mkdirSync(testRoot, { recursive: true });

const screenshotPath = join(screenshotDir, 'network-local-ai-runtime-result-status.png');
const screenshotAbsolutePath = resolve(screenshotPath);
const securityLogPath = join(proofRoot, '09-security-negative-proof.log');
const validationLogPath = join(proofRoot, '12-validation-commands.log');

assertSourceContracts();

writeFileSync(
  join(proofRoot, 'expected-local-ai-runtime-result-portal.json'),
  `${JSON.stringify(
    {
      acceptedInputs: [
        'agent.network.product-readiness.status.reported service event',
        'networkLocalAiRuntimeResultStatus JSON payload field',
        'row33b service-backed local-AI runtime result status',
      ],
      renderedStates: [
        'network.local-ai.runtime-result.status.33b',
        'ResultReady',
        'Queued',
        'network.local-ai.runtime-ref.33b',
        'network.local-ai.model-version.33b',
        'network.local-ai.prompt-template.33b',
        'network.local-ai.policy-context.33b',
        'policy.rule.network-domain.1',
        'network.local-ai.managed-browser-exact-url-evidence.33b',
        'network.local-ai.result.33b',
        'network.local-ai.output-summary.33b',
      ],
      uiBoundary:
        'Portal renders service-backed row33b local-AI runtime result status in the Activity route network drawer.',
      noClaims: [
        'live model execution proof',
        'raw model output',
        'raw PCAP',
        'network-only exact URL',
        'decrypted payload',
        'remote AI use',
        'policy authority',
        'adapter authority',
        'enforcement command publication',
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
    name: 'portal-network-local-ai-runtime-result-status-e2e',
    command: 'cmd',
    args: [
      '/c',
      'npm',
      'run',
      'test:e2e',
      '--workspace',
      '@ocentra-parent/portal',
      '--',
      'e2e/network-local-ai-runtime-result-status-proof.spec.ts',
    ],
    env: {
      NETWORK_LOCAL_AI_RUNTIME_RESULT_STATUS_SCREENSHOT: screenshotAbsolutePath,
    },
    log: join(proofRoot, 'portal-network-local-ai-runtime-result-status-e2e.log'),
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
  throw new Error(`missing network local-AI runtime result screenshot: ${screenshotPath}`);
}
writeFileSync(
  validationLogPath,
  commandResults.map((entry) => `${entry.command} -> ${entry.status}`).join('\n') + '\n'
);
writeFileSync(
  securityLogPath,
  [
    `checkedAt=${new Date().toISOString()}`,
    'asserted=no raw model output rendering',
    'asserted=no live local model execution proof claim',
    'asserted=no exact URL/page/message/search claim from network-only evidence',
    'asserted=no decrypted payload or raw PCAP claim',
    'asserted=no remote AI, policy authority, adapter authority, or enforcement command claim',
  ].join('\n') + '\n'
);

const proof = {
  proof: 'network-local-ai-runtime-result-portal-proof',
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
    expectedLocalAiRuntimeResultPortal: join(proofRoot, 'expected-local-ai-runtime-result-portal.json'),
    screenshot: screenshotPath,
    securityNegativeLog: securityLogPath,
    validationCommands: validationLogPath,
    proofSummary: join(proofRoot, 'proof-summary.json'),
    testProof: join(testRoot, 'proof.json'),
  },
  coveredRows: [
    'network-plan supplemental row 33c local-AI runtime result portal rendering',
    'network feature doc Activity route network drawer gap',
    'apps/portal Activity route network drawer',
  ],
  provenBoundaries: [
    'portal parser consumes the service-backed networkLocalAiRuntimeResultStatus payload field',
    'Activity route network drawer renders bridge, queue, runtime, model, prompt, policy, parent-rule, evidence, summary, and output-summary refs',
    'Activity route renders false local model execution, raw PCAP, exact URL, remote AI, policy, adapter, and enforcement claims',
    'managed Playwright proof exercises the real Rust service, Vite portal, and WebSocket event path',
  ],
  notClaimed: [
    'live local model execution or raw model output',
    'exact URL, page content, video, message, search, or decrypted payload from network-only evidence',
    'raw PCAP without custody or live capture driver invocation',
    'remote AI execution',
    'portal policy authority or local policy evaluation',
    'portal adapter dispatch, live adapter execution, host filtering, or enforcement command publication',
    'broker/family-hub delivery or full network-plan completion',
  ],
};

writeFileSync(join(proofRoot, 'proof-summary.json'), `${JSON.stringify(proof, null, 2)}\n`);
writeFileSync(join(testRoot, 'proof.json'), `${JSON.stringify(proof, null, 2)}\n`);
console.log(
  'network-local-ai-runtime-result-portal-proof-ok:lint,build,unit,e2e,schema-boundaries,source-shape,diff-check'
);
console.log(`proof=${join(proofRoot, 'proof-summary.json')}`);

function assertSourceContracts() {
  const parser = readFileSync('packages/agent-protocol-domain/src/network-product-readiness-status.ts', 'utf8');
  const portalSummary = readFileSync('apps/portal/src/network-product-readiness-status.ts', 'utf8');
  const liveState = readFileSync('apps/portal/src/live-activity-state.ts', 'utf8');
  const portalPanel = readFileSync('apps/portal/src/NetworkEvidenceDrawerRoutePanel.tsx', 'utf8');
  const portalSpec = readFileSync('apps/portal/e2e/network-local-ai-runtime-result-status-proof.spec.ts', 'utf8');
  const featureDoc = readFileSync('docs/features/network-domain-control.md', 'utf8');
  const checklist = readFileSync('docs/plans/network-plan/implementation-checklist.md', 'utf8');
  const workpacks = readFileSync('docs/plans/network-plan/workpacks/README.md', 'utf8');
  const requiredSnippets = [
    [parser, 'parseLocalAiRuntimeResultStatus'],
    [portalSummary, 'localAiRuntimeResultSummary'],
    [portalSummary, 'unsupportedLocalAiClaims'],
    [liveState, 'NetworkProductReadinessStatusReported'],
    [portalPanel, 'NetworkLocalAiRuntimeResultStatusCard'],
    [portalSpec, 'network.local-ai.runtime-result.status.33b'],
    [featureDoc, 'network-local-ai-runtime-result-portal-proof'],
    [checklist, '33c network local-AI runtime result portal status'],
    [workpacks, '33c'],
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
        !filePath.startsWith('output/network-plan-proof/33c-network-local-ai-runtime-result-portal/') &&
        !filePath.startsWith('test-results/network-local-ai-runtime-result-portal-proof/')
      );
    })
    .join('\n');
}
