import { spawnSync } from 'node:child_process';
import { mkdirSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

const proofRoot = join('output', 'network-plan-proof', '54-policy-preview-flow-proof');
const testRoot = join('test-results', 'network-policy-preview-flow-proof');
mkdirSync(proofRoot, { recursive: true });
mkdirSync(testRoot, { recursive: true });

const generatedPaths = [
  'output/network-plan-proof/54-policy-preview-flow-proof',
  'test-results/network-policy-preview-flow-proof',
];

const commands = [
  {
    name: 'agent-core-policy-preview-tests',
    command: 'cargo',
    args: ['test', '-p', 'ocentra-parent-agent-core', 'policy_preview_read_model'],
    log: join(proofRoot, 'agent-core-policy-preview-tests.log'),
  },
  {
    name: 'agent-service-policy-preview-tests',
    command: 'cargo',
    args: ['test', '-p', 'ocentra-parent-agent-service', 'policy_preview'],
    log: join(proofRoot, 'agent-service-policy-preview-tests.log'),
  },
  {
    name: 'agent-core-clippy',
    command: 'cargo',
    args: ['clippy', '-p', 'ocentra-parent-agent-core', '--all-targets', '--', '-D', 'warnings'],
    log: join(proofRoot, 'agent-core-clippy.log'),
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

writeFileSync(
  join(proofRoot, 'expected-policy-preview-flow-proof.json'),
  `${JSON.stringify(
    {
      storedEvidenceSource: 'ActivityStore activity.domain.observed rows from the Windows network observer',
      policyPreviewPath: [
        'network_observation_event',
        'ActivityStore.ingest_events',
        'ActivityStore.policy_preview_read_model',
        'evaluate_policy_dry_run',
        'policy_preview_read_model_payload',
      ],
      authorityBoundary: 'dry-run policy preview with disabled enforcement handoff',
      supportedTargets: ['domain', 'process alias from network flow metadata'],
      requiredRefs: ['source event id', 'parent rule ref', 'policy decision ref', 'network evidence ref'],
      unsupportedClaimsRejected: [
        'exact URL from network-only evidence',
        'page content',
        'decrypted payload',
        'adapter execution',
        'enforcement command publication',
      ],
    },
    null,
    2
  )}\n`
);

const commandResults = commands.map(runCommand);
const proof = {
  schemaVersion: 1,
  proof: 'network-policy-preview-flow-proof',
  checkedAt: new Date().toISOString(),
  branch: runText('git', ['branch', '--show-current']).trim(),
  sourceCommit: runText('git', ['rev-parse', 'HEAD']).trim(),
  artifactCommit: 'see the enclosing git commit for generated proof artifacts',
  originMain: runText('git', ['rev-parse', 'origin/main']).trim(),
  mergeBase: runText('git', ['merge-base', 'HEAD', 'origin/main']).trim(),
  sourceStatusShort: sourceStatusShort(),
  proofRoot,
  testRoot,
  commands: commandResults,
  artifacts: {
    expectedPolicyPreviewFlowProof: join(proofRoot, 'expected-policy-preview-flow-proof.json'),
    proofSummary: join(proofRoot, 'proof-summary.json'),
    testProof: join(testRoot, 'proof.json'),
  },
  evidence: {
    corePolicyPreviewFixture: 'crates/agent-core/src/activity_store_policy_preview_test_fixture.rs',
    corePolicyPreviewTests: [
      'crates/agent-core/src/activity_store_policy_preview_tests.rs',
      'crates/agent-core/src/activity_store_policy_preview_parent_rule_tests.rs',
    ],
    corePolicyPreviewBuilder: 'crates/agent-core/src/activity_store_policy_preview.rs',
    corePolicyPreviewTargets: 'crates/agent-core/src/activity_store_policy_preview_targets.rs',
    servicePolicyPreviewApi: 'crates/agent-service/src/policy_preview_api.rs',
    servicePolicyPreviewPayload: 'crates/agent-service/src/policy_preview_payload.rs',
    portalPolicyPreviewParser: 'apps/portal/src/policy-preview-read-model.ts',
  },
  claimsProved: [
    'stored network flow evidence feeds the existing ActivityStore policy preview read model',
    'network domain evidence can match a parent domain rule and produce a dry-run block decision',
    'network process metadata can match a parent process rule alias and produce a dry-run ask-parent decision',
    'network policy preview decisions preserve source event and parent-rule evidence refs',
    'network policy preview keeps enforcement handoff disabled and does not execute adapters',
    'network policy preview does not create an exact URL, page-content, or decrypted-payload claim',
  ],
  notClaimed: [
    'full policy engine runtime execution',
    'portal rule authoring UX',
    'adapter execution, host filtering, process termination, or command invocation',
    'published enforcement commands',
    'live packet capture driver invocation',
    'broker or family-hub delivery',
    'exact URL, page content, private message, search query, or decrypted payload availability from network-only evidence',
  ],
};

const serialized = `${JSON.stringify(proof, null, 2)}\n`;
writeFileSync(join(proofRoot, 'proof-summary.json'), serialized);
writeFileSync(join(testRoot, 'proof.json'), serialized);
console.log(
  'network-policy-preview-flow-proof-ok:agent-core-policy-preview,agent-service-policy-preview,clippy,source-shape,diff-check'
);
console.log(`proof=${join(proofRoot, 'proof-summary.json')}`);

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
    .split(/\r?\n/u)
    .filter((line) => {
      const path = line.slice(3);
      return line.trim().length > 0 && !generatedPaths.some((generatedPath) => path.startsWith(generatedPath));
    })
    .join('\n');
}
