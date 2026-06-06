import { spawnSync } from 'node:child_process';
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

const proofRoot = join('output', 'network-plan-proof', '10e-remote-delivery-durable-envelope-status');
const testRoot = join('test-results', 'network-remote-delivery-durable-envelope-proof');
mkdirSync(proofRoot, { recursive: true });
mkdirSync(testRoot, { recursive: true });

const securityLogPath = join(proofRoot, '09-security-negative-proof.log');
const validationLogPath = join(proofRoot, '12-validation-commands.log');

assertSourceContracts();

writeFileSync(
  join(proofRoot, 'expected-remote-delivery-durable-envelope-status.json'),
  `${JSON.stringify(
    {
      acceptedInputs: [
        'agent.network.product-readiness.status.reported service event',
        'networkRemoteDeliveryStatus JSON payload field',
        'row10b broker/family-hub delivery requirement refs',
        'row10d remote lifecycle blocker refs',
        'row10e durable-envelope readiness refs',
      ],
      durableEnvelopeRefs: [
        'broker.network.durable-envelope.schema.10e',
        'broker.network.durable-envelope.journal-readiness.10e',
        'broker.network.durable-envelope.replay-readiness.10e',
        'broker.network.durable-envelope.delete-export-readiness.10e',
        'network.remote-delivery.durable-envelope.support-status.10e',
      ],
      renderedStates: [
        'durableEnvelopeReady=true',
        'durableEnvelopeMissingArtifactCount=0',
        'providerDeliveryImplemented=false',
        'childDeviceDeliveryImplemented=false',
        'productReadyClaimed=false',
      ],
      parserInvariants: [
        'durable envelope refs must all cite row10e',
        'durable envelope readiness cannot carry missing artifacts',
        'provider, child-device delivery, and product-ready claims remain literal false',
      ],
      noClaims: [
        'live broker delivery',
        'live family-hub relay delivery',
        'remote provider delivery',
        'child-device delivery',
        'product-ready remote delivery',
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
    args: [
      '/c',
      'npm',
      'exec',
      '--workspace',
      '@ocentra-parent/portal',
      '--',
      'vitest',
      'run',
      'tests/live-activity-network-flow.test.ts',
    ],
    log: join(proofRoot, 'portal-live-activity-network-flow-test.log'),
  },
  {
    name: 'agent-service-build',
    command: 'cargo',
    args: ['build', '-p', 'ocentra-parent-agent-service'],
    log: join(proofRoot, 'agent-service-build.log'),
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
    'asserted=no remote provider or child-device delivery claim',
    'asserted=no product-ready remote delivery claim',
    'asserted=no policy authority, side-effect authority, adapter action, host filtering, or enforcement command publication claim',
  ].join('\n') + '\n'
);

const proof = {
  proof: 'network-remote-delivery-durable-envelope-proof',
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
    expectedRemoteDeliveryDurableEnvelopeStatus: join(
      proofRoot,
      'expected-remote-delivery-durable-envelope-status.json'
    ),
    securityNegativeLog: securityLogPath,
    validationCommands: validationLogPath,
    proofSummary: join(proofRoot, 'proof-summary.json'),
    testProof: join(testRoot, 'proof.json'),
  },
  coveredRows: [
    'network-plan supplemental row 10e remote delivery durable-envelope readiness status',
    'network-plan supplemental row 10d remote delivery lifecycle blocker status',
    'network-plan supplemental row 10c remote delivery service and portal status',
    'network-plan supplemental row 10b broker/family-hub remote delivery status',
  ],
  provenBoundaries: [
    'Rust protocol status carries durable-envelope schema, journal, replay, delete/export, and support status refs',
    'agent-core proof reports durable-envelope readiness without live transport or delivery implementation claims',
    'Rust service product-readiness event exposes row10e fields through the existing typed status payload',
    'agent-protocol-domain parser rejects row10e missing artifact and non-row10e ref mismatches',
    'portal summary fixture consumes row10e fields without adding UI authority or adapter dispatch',
  ],
  notClaimed: [
    'live broker delivery',
    'live family-hub relay delivery',
    'remote provider delivery',
    'child-device delivery',
    'product-ready remote delivery',
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
  'network-remote-delivery-durable-envelope-proof-ok:rust,protocol,service,parser,portal-summary,schema-boundaries,source-shape,diff-check'
);
console.log(`proof=${join(proofRoot, 'proof-summary.json')}`);

function assertSourceContracts() {
  const protocolConstants = readFileSync('crates/agent-protocol/src/constants/network_flow.rs', 'utf8');
  const protocolNetworkFlow = readFileSync('crates/agent-protocol/src/network_flow.rs', 'utf8');
  const coreStatus = readFileSync('crates/agent-core/src/network_event_runtime/remote_delivery_status.rs', 'utf8');
  const servicePayload = readFileSync('crates/agent-service/src/network_product_readiness_status_payload.rs', 'utf8');
  const parser = readFileSync('packages/agent-protocol-domain/src/network-product-readiness-status.ts', 'utf8');
  const portalSummary = readFileSync('apps/portal/src/network-product-readiness-status.ts', 'utf8');
  const portalTest = readFileSync('apps/portal/tests/live-activity-network-flow.test.ts', 'utf8');
  const featureDoc = readFileSync('docs/features/network-domain-control.md', 'utf8');
  const checklist = readFileSync('docs/plans/network-plan/implementation-checklist.md', 'utf8');
  const workpacks = readFileSync('docs/plans/network-plan/workpacks/README.md', 'utf8');
  const requiredSnippets = [
    [protocolConstants, 'TEST_REMOTE_DURABLE_ENVELOPE_SCHEMA_REF'],
    [protocolNetworkFlow, 'durable_envelope_ready'],
    [coreStatus, 'durable_envelope_missing_artifact_count: 0'],
    [servicePayload, 'TEST_REMOTE_DURABLE_ENVELOPE_SUPPORT_STATUS_REF'],
    [parser, 'remoteDeliveryDurableEnvelopeShapeMatches'],
    [parser, 'product_ready_claimed: Schema.Literal(false)'],
    [portalSummary, 'remoteDurableEnvelopeRefs'],
    [portalTest, 'broker.network.durable-envelope.schema.10e'],
    [featureDoc, 'network-remote-delivery-durable-envelope-proof'],
    [checklist, '10e remote delivery durable-envelope readiness status'],
    [workpacks, '10e'],
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
        !filePath.startsWith('output/network-plan-proof/10e-remote-delivery-durable-envelope-status/') &&
        !filePath.startsWith('test-results/network-remote-delivery-durable-envelope-proof/')
      );
    })
    .join('\n');
}
