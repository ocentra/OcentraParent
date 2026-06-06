import { spawnSync } from 'node:child_process';
import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join, resolve } from 'node:path';

const proofRoot = join('output', 'network-plan-proof', '10c-remote-delivery-service-portal-status');
const screenshotDir = join(proofRoot, '08-ui-snapshots');
const testRoot = join('test-results', 'network-remote-delivery-status-portal-proof');
mkdirSync(screenshotDir, { recursive: true });
mkdirSync(testRoot, { recursive: true });

const screenshotPath = join(screenshotDir, 'network-remote-delivery-status.png');
const screenshotAbsolutePath = resolve(screenshotPath);
const securityLogPath = join(proofRoot, '09-security-negative-proof.log');
const validationLogPath = join(proofRoot, '12-validation-commands.log');

assertSourceContracts();

writeFileSync(
  join(proofRoot, 'expected-remote-delivery-status-portal.json'),
  `${JSON.stringify(
    {
      acceptedInputs: [
        'agent.network.product-readiness.status.reported service event',
        'networkRemoteDeliveryStatus JSON payload field',
        'row10b broker/family-hub delivery requirement refs',
        'row10a local idempotency/dead-letter evidence counters',
      ],
      renderedStates: [
        'network.remote-delivery.status.10c',
        'RequirementsSatisfiedButNotImplemented',
        'broker.network.custody-proof.1',
        'broker.network.publisher-auth.1',
        'broker.network.subscriber-auth.1',
        'family-hub.network.identity.1',
        'family-hub.network.relay-policy.1',
        'false broker/family-hub delivery, replay, retention propagation, authority, and action rows',
      ],
      uiBoundary:
        'Portal renders service-backed broker/family-hub remote delivery status from the product-readiness event.',
      noClaims: [
        'live broker delivery',
        'live family-hub relay delivery',
        'cross-process replay',
        'remote retention/delete/export propagation',
        'policy authority',
        'side-effect authority',
        'enforcement command publication',
        'adapter action execution',
        'host filtering',
      ],
    },
    null,
    2
  )}\n`
);

const commands = [
  {
    name: 'agent-protocol-remote-delivery-rust-test',
    command: 'cargo',
    args: ['test', '-p', 'ocentra-parent-agent-protocol', 'network_remote_delivery_status'],
    log: join(proofRoot, 'agent-protocol-remote-delivery-rust-test.log'),
  },
  {
    name: 'agent-core-remote-delivery-rust-test',
    command: 'cargo',
    args: ['test', '-p', 'ocentra-parent-agent-core', 'network_runtime_remote_delivery'],
    log: join(proofRoot, 'agent-core-remote-delivery-rust-test.log'),
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
    name: 'text-domain-build',
    command: 'cmd',
    args: ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/text-domain'],
    log: join(proofRoot, 'text-domain-build.log'),
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
    name: 'portal-network-remote-delivery-status-e2e',
    command: 'cmd',
    args: [
      '/c',
      'npm',
      'run',
      'test:e2e',
      '--workspace',
      '@ocentra-parent/portal',
      '--',
      'e2e/network-remote-delivery-status-proof.spec.ts',
    ],
    env: {
      NETWORK_REMOTE_DELIVERY_STATUS_SCREENSHOT: screenshotAbsolutePath,
    },
    log: join(proofRoot, 'portal-network-remote-delivery-status-e2e.log'),
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
  throw new Error(`missing network remote delivery status screenshot: ${screenshotPath}`);
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
    'asserted=no live broker/family-hub delivery claim',
    'asserted=no cross-process replay or remote retention/delete/export propagation claim',
    'asserted=no policy authority, side-effect authority, adapter action, host filtering, or enforcement command publication claim',
  ].join('\n') + '\n'
);

const proof = {
  proof: 'network-remote-delivery-status-portal-proof',
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
    expectedRemoteDeliveryStatusPortal: join(proofRoot, 'expected-remote-delivery-status-portal.json'),
    screenshot: screenshotPath,
    securityNegativeLog: securityLogPath,
    validationCommands: validationLogPath,
    proofSummary: join(proofRoot, 'proof-summary.json'),
    testProof: join(testRoot, 'proof.json'),
  },
  coveredRows: [
    'network-plan supplemental row 10c remote delivery service and portal status',
    'network-plan supplemental row 10b broker/family-hub remote delivery status',
    'network-plan row45 broker/family-hub delivery requirements',
    'apps/portal Activity route network drawer',
  ],
  provenBoundaries: [
    'service product-readiness event carries networkRemoteDeliveryStatus as a typed protocol field',
    'agent-protocol-domain parser requires and validates remote delivery status false live-transport/authority/action claims',
    'Activity route renders broker, family-hub, custody, auth, encryption, retention, replay, delete, offset, dedupe, transport, relay, idempotency, dead-letter, and false authority/action rows',
    'managed Playwright proof exercises the real Rust service, Vite portal, and WebSocket event path',
  ],
  notClaimed: [
    'live broker delivery',
    'live family-hub relay delivery',
    'cross-process replay',
    'remote retention/delete/export propagation',
    'policy authority',
    'side-effect authority',
    'adapter execution',
    'host filtering',
    'full network-plan completion',
  ],
};

writeFileSync(join(proofRoot, 'proof-summary.json'), `${JSON.stringify(proof, null, 2)}\n`);
writeFileSync(join(testRoot, 'proof.json'), `${JSON.stringify(proof, null, 2)}\n`);
console.log(
  'network-remote-delivery-status-portal-proof-ok:rust,protocol,portal,e2e,schema-boundaries,source-shape,diff-check'
);
console.log(`proof=${join(proofRoot, 'proof-summary.json')}`);

function assertSourceContracts() {
  const protocolConstants = readFileSync('crates/agent-protocol/src/constants/field.rs', 'utf8');
  const protocolNetworkFlow = readFileSync('crates/agent-protocol/src/network_flow.rs', 'utf8');
  const servicePayload = readFileSync('crates/agent-service/src/network_product_readiness_status_payload.rs', 'utf8');
  const parser = readFileSync('packages/agent-protocol-domain/src/network-product-readiness-status.ts', 'utf8');
  const portalSummary = readFileSync('apps/portal/src/network-product-readiness-status.ts', 'utf8');
  const portalCard = readFileSync('apps/portal/src/network-remote-delivery-status-card.tsx', 'utf8');
  const portalPanel = readFileSync('apps/portal/src/NetworkEvidenceDrawerRoutePanel.tsx', 'utf8');
  const portalSpec = readFileSync('apps/portal/e2e/network-remote-delivery-status-proof.spec.ts', 'utf8');
  const featureDoc = readFileSync('docs/features/network-domain-control.md', 'utf8');
  const checklist = readFileSync('docs/plans/network-plan/implementation-checklist.md', 'utf8');
  const workpacks = readFileSync('docs/plans/network-plan/workpacks/README.md', 'utf8');
  const requiredSnippets = [
    [protocolConstants, 'NETWORK_REMOTE_DELIVERY_STATUS'],
    [protocolNetworkFlow, 'NetworkRemoteDeliveryStatus'],
    [servicePayload, 'TEST_REMOTE_DELIVERY_STATUS_REF'],
    [servicePayload, 'TEST_FAMILY_HUB_IDENTITY_REF'],
    [parser, 'AgentNetworkRemoteDeliveryStatusSchema'],
    [parser, 'family_hub_delivery_implemented: Schema.Literal(false)'],
    [portalSummary, 'remoteFamilyHubDeliveryImplemented'],
    [portalSummary, 'unsupportedRemoteDeliveryClaims'],
    [portalCard, 'NetworkRemoteDeliveryStatusCard'],
    [portalPanel, 'NetworkRemoteDeliveryStatusCard'],
    [portalSpec, 'network.remote-delivery.status.10c'],
    [featureDoc, 'network-remote-delivery-status-portal-proof'],
    [checklist, '10c remote delivery service/portal status'],
    [workpacks, '10c'],
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
        !filePath.startsWith('output/network-plan-proof/10c-remote-delivery-service-portal-status/') &&
        !filePath.startsWith('test-results/network-remote-delivery-status-portal-proof/')
      );
    })
    .join('\n');
}
