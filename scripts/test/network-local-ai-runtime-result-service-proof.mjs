import { spawnSync } from 'node:child_process';
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

const proofRoot = join('output', 'network-plan-proof', '33b-network-local-ai-runtime-result-service-status');
const testRoot = join('test-results', 'network-local-ai-runtime-result-service-proof');
const expectedPath = join(proofRoot, 'expected-local-ai-runtime-result-service-status.json');
const securityLogPath = join(proofRoot, '09-security-negative-proof.log');
const validationLogPath = join(proofRoot, '12-validation-commands.log');
const proofSummaryPath = join(proofRoot, 'proof-summary.json');
const testProofPath = join(testRoot, 'proof.json');

mkdirSync(proofRoot, { recursive: true });
mkdirSync(testRoot, { recursive: true });

assertSourceContracts();

writeFileSync(
  expectedPath,
  `${JSON.stringify(
    {
      proof: 'network-local-ai-runtime-result-service-proof',
      command: 'agent.network.product-readiness.status.get',
      event: 'agent.network.product-readiness.status.reported',
      payloadField: 'networkLocalAiRuntimeResultStatus',
      acceptedInputs: [
        'row33a NetworkLocalAiRuntimeResultBridge',
        'row33 refs-only local-AI queue refs',
        'local runtime result refs',
        'output-summary refs',
        'prompt template, policy context, parent rule, evidence, and summary refs',
      ],
      statusStates: ['ResultReady', 'RuntimeUnavailable', 'RuntimeFailed', 'RuntimeTimedOut', 'QueueNotReady'],
      queueStates: ['Queued', 'NotRecommended', 'DisabledByParent', 'ModelUnavailable', 'QueueUnavailable'],
      noClaims: [
        'live local model execution proof',
        'raw model output text',
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
        'portal rendering',
      ],
    },
    null,
    2
  )}\n`
);

const commands = [
  {
    name: 'agent-protocol-local-ai-runtime-result-status-test',
    command: 'cargo',
    args: ['test', '-p', 'ocentra-parent-agent-protocol', 'network_local_ai_runtime_result_status'],
    log: join(proofRoot, 'agent-protocol-local-ai-runtime-result-status-test.log'),
  },
  {
    name: 'agent-service-product-readiness-status-test',
    command: 'cargo',
    args: ['test', '-p', 'ocentra-parent-agent-service', 'network_product_readiness_status'],
    log: join(proofRoot, 'agent-service-product-readiness-status-test.log'),
  },
  {
    name: 'network-evidence-local-ai-runtime-result-test',
    command: 'cargo',
    args: ['test', '-p', 'ocentra-network-evidence', 'local_ai_runtime_result'],
    log: join(proofRoot, 'network-evidence-local-ai-runtime-result-test.log'),
  },
  {
    name: 'network-evidence-local-ai-queue-test',
    command: 'cargo',
    args: ['test', '-p', 'ocentra-network-evidence', 'local_ai_queue'],
    log: join(proofRoot, 'network-evidence-local-ai-queue-test.log'),
  },
  {
    name: 'agent-protocol-domain-lint',
    command: npmCommand(),
    args: npmArgs(['run', 'lint:exec', '--workspace', '@ocentra-parent/agent-protocol-domain']),
    log: join(proofRoot, 'agent-protocol-domain-lint.log'),
  },
  {
    name: 'agent-protocol-domain-build',
    command: npmCommand(),
    args: npmArgs(['run', 'build', '--workspace', '@ocentra-parent/agent-protocol-domain']),
    log: join(proofRoot, 'agent-protocol-domain-build.log'),
  },
  {
    name: 'agent-protocol-domain-product-readiness-test',
    command: npmCommand(),
    args: npmArgs([
      'run',
      'test',
      '--workspace',
      '@ocentra-parent/agent-protocol-domain',
      '--',
      'network-product-readiness-status.test.ts',
    ]),
    log: join(proofRoot, 'agent-protocol-domain-product-readiness-test.log'),
  },
  {
    name: 'portal-live-activity-network-flow-test',
    command: npmCommand(),
    args: npmArgs(['run', 'test', '--workspace', '@ocentra-parent/portal', '--', 'live-activity-network-flow.test.ts']),
    log: join(proofRoot, 'portal-live-activity-network-flow-test.log'),
  },
  {
    name: 'agent-service-clippy',
    command: 'cargo',
    args: ['clippy', '-p', 'ocentra-parent-agent-service', '--all-targets', '--', '-D', 'warnings'],
    log: join(proofRoot, 'agent-service-clippy.log'),
  },
  {
    name: 'schema-boundaries',
    command: npmCommand(),
    args: npmArgs(['run', 'lint:schema-boundaries']),
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
    'asserted=no raw model output text is exposed',
    'asserted=no live local model execution proof claim',
    'asserted=no raw PCAP, exact URL from network-only evidence, page content, private message, search query, or decrypted payload claim',
    'asserted=no remote AI provider claim',
    'asserted=no policy authority, adapter authority, host filtering, or enforcement command publication claim',
    'asserted=no portal rendering claim',
  ].join('\n') + '\n'
);

const proof = {
  proof: 'network-local-ai-runtime-result-service-proof',
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
    expected: expectedPath,
    securityNegativeLog: securityLogPath,
    validationCommands: validationLogPath,
    proofSummary: proofSummaryPath,
    testProof: testProofPath,
  },
  coveredRows: [
    'network-plan row 33 network-triggered local AI queue',
    'network-plan supplemental row 33a network local-AI runtime result bridge',
    'network-plan supplemental row 33b network local-AI runtime result service status',
    'agent.network.product-readiness.status.reported service status event',
  ],
  provenBoundaries: [
    'Rust protocol owns the networkLocalAiRuntimeResultStatus field and status shape',
    'Rust service builds the status from the existing row33a ocentra-network-evidence bridge',
    'shared TypeScript parser requires the status field and rejects model-execution, raw-content, remote-AI, authority, adapter, and enforcement regressions',
    'status carries refs only, including queue, local runtime result, output summary, prompt, policy context, parent rule, evidence, and summary refs',
  ],
  notClaimed: [
    'live local model execution',
    'raw model output text',
    'remote AI provider execution',
    'portal rendering',
    'full policy engine execution',
    'adapter execution',
    'broker or family-hub delivery',
    'host filtering',
    'full network-plan completion',
  ],
};

writeFileSync(proofSummaryPath, `${JSON.stringify(proof, null, 2)}\n`);
writeFileSync(testProofPath, `${JSON.stringify(proof, null, 2)}\n`);
console.log(
  'network-local-ai-runtime-result-service-proof-ok:rust-protocol,service,evidence,ts-parser,portal-parser,clippy,schema-boundaries,source-shape,diff-check'
);
console.log(`proof=${proofSummaryPath}`);

function assertSourceContracts() {
  const fieldConstants = readText('crates/agent-protocol/src/constants/field.rs');
  const rustProtocol = readText('crates/agent-protocol/src/network_flow.rs');
  const rustProtocolTests = readText('crates/agent-protocol/src/network_flow_tests.rs');
  const servicePayload = readText('crates/agent-service/src/network_product_readiness_status_payload.rs');
  const localAiStatusPayload = readText('crates/agent-service/src/network_local_ai_runtime_result_status_payload.rs');
  const serviceTests = readText('crates/agent-service/src/network_product_readiness_status_service_tests.rs');
  const defaults = readText('packages/agent-protocol-domain/src/defaults.ts');
  const parser = readText('packages/agent-protocol-domain/src/network-product-readiness-status.ts');
  const parserTests = readText('packages/agent-protocol-domain/tests/network-product-readiness-status.test.ts');
  const portalNetworkFlowTests = readText('apps/portal/tests/live-activity-network-flow.test.ts');
  const featureDoc = readText('docs/features/network-domain-control.md');
  const checklist = readText('docs/plans/network-plan/implementation-checklist.md');
  const workpacks = readText('docs/plans/network-plan/workpacks/README.md');

  assertIncludes(fieldConstants, 'NETWORK_LOCAL_AI_RUNTIME_RESULT_STATUS', 'Rust field constant');
  assertIncludes(rustProtocol, 'NetworkLocalAiRuntimeResultStatus', 'Rust protocol status');
  assertIncludes(
    rustProtocolTests,
    'network_local_ai_runtime_result_status_serializes_no_claim_boundary',
    'Rust parity test'
  );
  assertIncludes(servicePayload, 'NETWORK_LOCAL_AI_RUNTIME_RESULT_STATUS', 'service event field');
  assertIncludes(localAiStatusPayload, 'bridge_network_local_ai_runtime_result', 'service consumes row33a bridge');
  assertIncludes(localAiStatusPayload, 'TEST_LOCAL_AI_RUNTIME_RESULT_STATUS_REF', 'service status ref');
  assertIncludes(serviceTests, 'assert_local_ai_runtime_result_status', 'service tests status');
  assertIncludes(defaults, 'networkLocalAiRuntimeResultStatus', 'TypeScript field default');
  assertIncludes(parser, 'AgentNetworkLocalAiRuntimeResultStatusSchema', 'TypeScript parser schema');
  assertIncludes(parser, 'localAiRuntimeResultShapeMatches', 'TypeScript parser invariant guard');
  assertIncludes(parser, 'model_execution_proved: Schema.Literal(false)', 'model execution false guard');
  assertIncludes(parserTests, 'localAiRuntimeResultClaimRegressionEvent', 'TypeScript regression test');
  assertIncludes(
    parserTests,
    'rejects local AI runtime result state/ref mismatches',
    'TypeScript invariant regression test'
  );
  assertIncludes(
    portalNetworkFlowTests,
    'NetworkLocalAiRuntimeResultStatus',
    'portal network-flow fixture carries local AI status'
  );
  assertIncludes(featureDoc, 'row33b service/protocol visibility', 'network feature row33b');
  assertIncludes(checklist, '33b network local-AI runtime result service status', 'network checklist row33b');
  assertIncludes(workpacks, '33b', 'network workpack row33b');
}

function npmCommand() {
  return process.platform === 'win32' ? 'cmd' : 'npm';
}

function npmArgs(args) {
  return process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
}

function readText(path) {
  return readFileSync(path, 'utf8');
}

function assertIncludes(text, expected, label) {
  if (!text.includes(expected)) {
    throw new Error(`${label}: missing ${expected}`);
  }
}

function runCommand(entry) {
  const result = spawnSync(entry.command, entry.args, { encoding: 'utf8', shell: false });
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
  return runText('git', ['status', '--short'])
    .split(/\r?\n/u)
    .filter((line) => {
      if (line.trim().length === 0) {
        return false;
      }
      const filePath = line.slice(3).replaceAll('\\', '/');
      return (
        !filePath.startsWith('output/network-plan-proof/33b-network-local-ai-runtime-result-service-status/') &&
        !filePath.startsWith('test-results/network-local-ai-runtime-result-service-proof/')
      );
    })
    .join('\n');
}
