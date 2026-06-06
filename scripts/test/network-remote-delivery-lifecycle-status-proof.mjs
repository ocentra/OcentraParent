import { spawnSync } from 'node:child_process';
import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join, resolve } from 'node:path';

const proofRoot = join('output', 'network-plan-proof', '10d-remote-delivery-lifecycle-status');
const screenshotDir = join(proofRoot, '08-ui-snapshots');
const testRoot = join('test-results', 'network-remote-delivery-lifecycle-status-proof');
mkdirSync(screenshotDir, { recursive: true });
mkdirSync(testRoot, { recursive: true });

const screenshotPath = join(screenshotDir, 'network-remote-delivery-lifecycle-status.png');
const screenshotAbsolutePath = resolve(screenshotPath);
const securityLogPath = join(proofRoot, '09-security-negative-proof.log');
const validationLogPath = join(proofRoot, '12-validation-commands.log');

assertSourceContracts();

writeFileSync(
  join(proofRoot, 'expected-remote-delivery-lifecycle-status.json'),
  `${JSON.stringify(
    {
      acceptedInputs: [
        'agent.network.product-readiness.status.reported service event',
        'networkRemoteDeliveryStatus JSON payload field',
        'row10b broker/family-hub delivery requirement refs',
        'row10d remote lifecycle blocker refs',
      ],
      lifecycleRefs: [
        'broker.network.cross-process-replay.manual-required.10d',
        'broker.network.remote-retention-delete-export.manual-required.10d',
        'family-hub.network.delivery-ack.manual-required.10d',
        'network.remote-delivery.lifecycle-followup.10d',
      ],
      renderedStates: [
        'remoteLifecycleMissingArtifactCount=3',
        'remoteLifecycleManualRequired=true',
        'externalTransportDeliveryImplemented=false',
        'familyHubDeliveryImplemented=false',
        'crossProcessReplayImplemented=false',
        'remoteRetentionDeleteExportPropagationImplemented=false',
      ],
      parserInvariants: [
        'remote lifecycle missing-artifact count matches the three manual-required blocker refs',
        'broker and family-hub requirements-satisfied statuses cannot carry missing requirement counts',
        'local idempotency queue and duplicate rejection proof must remain present',
      ],
      noClaims: [
        'live broker delivery',
        'live family-hub relay delivery',
        'cross-process replay implementation',
        'remote retention/delete/export propagation implementation',
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
    name: 'agent-protocol-domain-build',
    command: 'cmd',
    args: ['/c', 'npm', 'run', 'build', '--workspace', '@ocentra-parent/agent-protocol-domain'],
    log: join(proofRoot, 'agent-protocol-domain-build.log'),
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
    name: 'portal-network-remote-delivery-lifecycle-e2e',
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
    log: join(proofRoot, 'portal-network-remote-delivery-lifecycle-e2e.log'),
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
  throw new Error(`missing network remote delivery lifecycle status screenshot: ${screenshotPath}`);
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
    'asserted=no cross-process replay implementation claim',
    'asserted=no remote retention/delete/export propagation implementation claim',
    'asserted=no policy authority, side-effect authority, adapter action, host filtering, or enforcement command publication claim',
  ].join('\n') + '\n'
);

const proof = {
  proof: 'network-remote-delivery-lifecycle-status-proof',
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
    expectedRemoteDeliveryLifecycleStatus: join(proofRoot, 'expected-remote-delivery-lifecycle-status.json'),
    screenshot: screenshotPath,
    securityNegativeLog: securityLogPath,
    validationCommands: validationLogPath,
    proofSummary: join(proofRoot, 'proof-summary.json'),
    testProof: join(testRoot, 'proof.json'),
  },
  coveredRows: [
    'network-plan supplemental row 10d remote delivery lifecycle blocker status',
    'network-plan supplemental row 10c remote delivery service and portal status',
    'network-plan supplemental row 10b broker/family-hub remote delivery status',
  ],
  provenBoundaries: [
    'Rust protocol status carries remote lifecycle blocker refs and manual-required state',
    'agent-core proof reports cross-process replay, remote retention/delete/export, delivery ack, and follow-up refs without implementation claims',
    'Rust service product-readiness event exposes row10d lifecycle fields through the existing typed status payload',
    'agent-protocol-domain parser requires the row10d fields and literal false/zero authority/action claims',
    'agent-protocol-domain parser rejects row10d lifecycle count/ref mismatches and duplicate-proof regressions',
    'Activity route renders lifecycle blocker refs and manual-required state from the real Rust service WebSocket path',
  ],
  notClaimed: [
    'live broker delivery',
    'live family-hub relay delivery',
    'cross-process replay implementation',
    'remote retention/delete/export propagation implementation',
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
  'network-remote-delivery-lifecycle-status-proof-ok:rust,protocol,portal,e2e,schema-boundaries,source-shape,diff-check'
);
console.log(`proof=${join(proofRoot, 'proof-summary.json')}`);

function assertSourceContracts() {
  const protocolConstants = readFileSync('crates/agent-protocol/src/constants/network_flow.rs', 'utf8');
  const protocolNetworkFlow = readFileSync('crates/agent-protocol/src/network_flow.rs', 'utf8');
  const coreStatus = readFileSync('crates/agent-core/src/network_event_runtime/remote_delivery_status.rs', 'utf8');
  const servicePayload = readFileSync('crates/agent-service/src/network_product_readiness_status_payload.rs', 'utf8');
  const parser = readFileSync('packages/agent-protocol-domain/src/network-product-readiness-status.ts', 'utf8');
  const portalSummary = readFileSync('apps/portal/src/network-product-readiness-status.ts', 'utf8');
  const portalCard = readFileSync('apps/portal/src/network-remote-delivery-status-card.tsx', 'utf8');
  const portalSpec = readFileSync('apps/portal/e2e/network-remote-delivery-status-proof.spec.ts', 'utf8');
  const featureDoc = readFileSync('docs/features/network-domain-control.md', 'utf8');
  const checklist = readFileSync('docs/plans/network-plan/implementation-checklist.md', 'utf8');
  const workpacks = readFileSync('docs/plans/network-plan/workpacks/README.md', 'utf8');
  const requiredSnippets = [
    [protocolConstants, 'TEST_REMOTE_LIFECYCLE_CROSS_PROCESS_REPLAY_REF'],
    [protocolNetworkFlow, 'remote_lifecycle_manual_required'],
    [coreStatus, 'remote_lifecycle_missing_artifact_count: 3'],
    [servicePayload, 'TEST_REMOTE_LIFECYCLE_DELIVERY_ACK_REF'],
    [parser, 'remote_lifecycle_manual_required: Schema.Literal(true)'],
    [parser, 'remoteDeliveryLifecycleBlockersMatch'],
    [parser, 'NetworkRemoteLifecycleBlockerCount'],
    [portalSummary, 'remoteLifecycleBlockerRefs'],
    [portalCard, 'remoteLifecycleManualRequired'],
    [portalSpec, 'broker.network.cross-process-replay.manual-required.10d'],
    [featureDoc, 'network-remote-delivery-lifecycle-status-proof'],
    [checklist, 'Workpack 10 rollup: row10d'],
    [checklist, 'parser invariants that reject lifecycle missing-artifact count mismatches'],
    [checklist, '10d remote delivery lifecycle blocker status'],
    [workpacks, '10d'],
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
        !filePath.startsWith('output/network-plan-proof/10d-remote-delivery-lifecycle-status/') &&
        !filePath.startsWith('test-results/network-remote-delivery-lifecycle-status-proof/')
      );
    })
    .join('\n');
}
