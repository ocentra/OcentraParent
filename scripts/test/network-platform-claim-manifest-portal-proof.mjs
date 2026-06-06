import { spawnSync } from 'node:child_process';
import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join, resolve } from 'node:path';

const proofRoot = join('output', 'network-plan-proof', '52a-platform-claim-manifest-portal');
const screenshotDir = join(proofRoot, '08-ui-snapshots');
const testRoot = join('test-results', 'network-platform-claim-manifest-portal-proof');
mkdirSync(screenshotDir, { recursive: true });
mkdirSync(testRoot, { recursive: true });

const screenshotPath = join(screenshotDir, 'network-platform-claim-manifest.png');
const screenshotAbsolutePath = resolve(screenshotPath);
const securityLogPath = join(proofRoot, '09-security-negative-proof.log');
const validationLogPath = join(proofRoot, '12-validation-commands.log');

assertSourceContracts();

writeFileSync(
  join(proofRoot, 'expected-platform-claim-manifest-portal.json'),
  `${JSON.stringify(
    {
      acceptedInputs: [
        'agent.network.product-readiness.status.reported service event',
        'networkProductReadinessStatus JSON payload field',
        'row52 platform claim manifest entries',
        'row51a product-readiness status carrier',
      ],
      renderedStates: [
        'WindowsFirewall',
        'WindowsWfp',
        'Ready',
        'ManualRequired',
        'event.policy.decision.completed.1',
        'policy.rule.network-domain.1',
        'evidence.network.flow.1',
        'device.child.windows-1',
        'network.live-capture.permission-proof.13',
        'adapter.capability.network.dry-run.1',
        'network.platform-claim.manual-followup.51a',
        'event.audit.entry.committed.1',
        'false enforcement-command-published rows',
      ],
      uiBoundary:
        'Portal renders Row52 per-platform claim manifest details from the service-backed product-readiness status event.',
      noClaims: [
        'exact URL or page content from network-only evidence',
        'decrypted payload',
        'live adapter execution',
        'host filtering',
        'UI policy authority',
        'production SLO',
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
    name: 'portal-network-platform-claim-manifest-e2e',
    command: 'cmd',
    args: [
      '/c',
      'npm',
      'run',
      'test:e2e',
      '--workspace',
      '@ocentra-parent/portal',
      '--',
      'e2e/network-platform-claim-manifest-portal-proof.spec.ts',
    ],
    env: {
      NETWORK_PLATFORM_CLAIM_MANIFEST_SCREENSHOT: screenshotAbsolutePath,
    },
    log: join(proofRoot, 'portal-network-platform-claim-manifest-e2e.log'),
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
  throw new Error(`missing network platform claim manifest screenshot: ${screenshotPath}`);
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
    'asserted=no live adapter execution, host filtering, or enforcement command publication claim',
    'asserted=no production SLO validation claim',
  ].join('\n') + '\n'
);

const proof = {
  proof: 'network-platform-claim-manifest-portal-proof',
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
    expectedPlatformClaimManifestPortal: join(proofRoot, 'expected-platform-claim-manifest-portal.json'),
    screenshot: screenshotPath,
    securityNegativeLog: securityLogPath,
    validationCommands: validationLogPath,
    proofSummary: join(proofRoot, 'proof-summary.json'),
    testProof: join(testRoot, 'proof.json'),
  },
  coveredRows: [
    'network-plan supplemental row 52a platform-claim manifest portal matrix',
    'network-plan row52 platform claim manifest proof',
    'apps/portal Activity route network drawer',
  ],
  provenBoundaries: [
    'service-backed product-readiness status carries Row52 platform entries',
    'agent-protocol-domain parser validates platform target, proof state, refs, and false enforcement-command rows',
    'Activity route renders per-platform device/OS, permission/entitlement, adapter capability, missing artifact, and audit refs',
    'managed Playwright proof exercises the real Rust service, Vite portal, and WebSocket event path',
  ],
  notClaimed: [
    'exact URL, page content, video, message, search, or decrypted payload from network-only evidence',
    'UI policy authority, live adapter execution, host filtering, or enforcement command publication',
    'production SLO validation or full network-plan completion',
  ],
};

writeFileSync(join(proofRoot, 'proof-summary.json'), `${JSON.stringify(proof, null, 2)}\n`);
writeFileSync(join(testRoot, 'proof.json'), `${JSON.stringify(proof, null, 2)}\n`);
console.log(
  'network-platform-claim-manifest-portal-proof-ok:rust,protocol,portal,e2e,schema-boundaries,source-shape,diff-check'
);
console.log(`proof=${join(proofRoot, 'proof-summary.json')}`);

function assertSourceContracts() {
  const rustStatus = readFileSync('crates/ocentra-network-evidence/src/product_readiness_status.rs', 'utf8');
  const servicePayload = readFileSync('crates/agent-service/src/network_product_readiness_status_payload.rs', 'utf8');
  const parser = readFileSync('packages/agent-protocol-domain/src/network-product-readiness-status.ts', 'utf8');
  const portalSummary = readFileSync('apps/portal/src/network-product-readiness-status.ts', 'utf8');
  const portalCard = readFileSync('apps/portal/src/network-platform-claim-manifest-card.tsx', 'utf8');
  const portalPanel = readFileSync('apps/portal/src/NetworkEvidenceDrawerRoutePanel.tsx', 'utf8');
  const portalSpec = readFileSync('apps/portal/e2e/network-platform-claim-manifest-portal-proof.spec.ts', 'utf8');
  const featureDoc = readFileSync('docs/features/network-domain-control.md', 'utf8');
  const checklist = readFileSync('docs/plans/network-plan/implementation-checklist.md', 'utf8');
  const workpacks = readFileSync('docs/plans/network-plan/workpacks/README.md', 'utf8');
  const requiredSnippets = [
    [rustStatus, 'platform_entries'],
    [servicePayload, 'platform_entry('],
    [parser, 'NetworkPlatformClaimEntrySchema'],
    [parser, 'platform_entries'],
    [portalSummary, 'NetworkPlatformClaimManifestEntrySummary'],
    [portalCard, 'NetworkPlatformClaimManifestCard'],
    [portalPanel, 'NetworkPlatformClaimManifestCard'],
    [portalSpec, 'network.platform-claim.manual-followup.51a'],
    [featureDoc, 'network-platform-claim-manifest-portal-proof'],
    [checklist, '52a platform-claim manifest portal matrix'],
    [workpacks, '52a'],
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
        !filePath.startsWith('output/network-plan-proof/52a-platform-claim-manifest-portal/') &&
        !filePath.startsWith('test-results/network-platform-claim-manifest-portal-proof/')
      );
    })
    .join('\n');
}
