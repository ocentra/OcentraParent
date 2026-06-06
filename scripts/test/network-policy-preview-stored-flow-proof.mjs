import { spawnSync } from 'node:child_process';
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

const proofRoot = join('output', 'network-plan-proof', '34a-policy-preview-stored-flow');
const testRoot = join('test-results', 'network-policy-preview-stored-flow-proof');
mkdirSync(proofRoot, { recursive: true });
mkdirSync(testRoot, { recursive: true });

assertSourceContracts();

writeFileSync(
  join(proofRoot, 'expected-policy-preview-stored-flow.json'),
  `${JSON.stringify(
    {
      acceptedInputs: [
        'stored ActivityStore network domain observed event',
        'destination domain field',
        'activity event evidence ref',
        'optional parent rule context that cites the stored event',
      ],
      previewStates: ['no matching rule unknown dry-run', 'matching domain rule dry-run block preview'],
      policyBoundary:
        'Stored network flow evidence can feed policy preview, but preview stays dry-run and enforcement handoff stays disabled.',
      noClaims: [
        'exact URL from network-only evidence',
        'decrypted payload',
        'live adapter mutation',
        'enforcement command publication',
        'full policy execution',
        'portal policy authority',
      ],
    },
    null,
    2
  )}\n`
);

const commands = [
  {
    name: 'agent-core-policy-preview-read-model-tests',
    command: 'cargo',
    args: ['test', '-p', 'ocentra-parent-agent-core', 'policy_preview_read_model'],
    log: join(proofRoot, 'agent-core-policy-preview-tests.log'),
  },
  {
    name: 'agent-service-policy-preview-payload-tests',
    command: 'cargo',
    args: ['test', '-p', 'ocentra-parent-agent-service', 'policy_preview_payload'],
    log: join(proofRoot, 'agent-service-policy-preview-payload-tests.log'),
  },
  {
    name: 'agent-core-clippy',
    command: 'cargo',
    args: ['clippy', '-p', 'ocentra-parent-agent-core', '--all-targets', '--', '-D', 'warnings'],
    log: join(proofRoot, 'agent-core-clippy.log'),
  },
  {
    name: 'agent-protocol-domain-policy-preview-contracts',
    command: process.platform === 'win32' ? 'cmd' : 'npm',
    args:
      process.platform === 'win32'
        ? [
            '/c',
            'npm',
            '--workspace',
            '@ocentra-parent/agent-protocol-domain',
            'run',
            'test',
            '--',
            'policy-preview-contracts',
          ]
        : ['--workspace', '@ocentra-parent/agent-protocol-domain', 'run', 'test', '--', 'policy-preview-contracts'],
    log: join(proofRoot, 'agent-protocol-domain-policy-preview-contracts.log'),
  },
  {
    name: 'portal-policy-preview-live-activity-state',
    command: process.platform === 'win32' ? 'cmd' : 'npm',
    args:
      process.platform === 'win32'
        ? [
            '/c',
            'npm',
            '--workspace',
            '@ocentra-parent/portal',
            'run',
            'test',
            '--',
            'policy-preview-live-activity-state',
          ]
        : ['--workspace', '@ocentra-parent/portal', 'run', 'test', '--', 'policy-preview-live-activity-state'],
    log: join(proofRoot, 'portal-policy-preview-live-activity-state.log'),
  },
  {
    name: 'source-shape',
    command: 'node',
    args: ['scripts/check-source-shape.mjs'],
    log: join(proofRoot, 'source-shape.log'),
  },
];
const commandResults = commands.map(runCommand);

const proof = {
  proof: 'network-policy-preview-stored-flow-proof',
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
    expectedPolicyPreviewStoredFlow: join(proofRoot, 'expected-policy-preview-stored-flow.json'),
    proofSummary: join(proofRoot, 'proof-summary.json'),
    testProof: join(testRoot, 'proof.json'),
  },
  coveredRows: [
    'network feature checklist policy preview over stored flow evidence',
    'network-plan row 34a policy preview over stored network flow evidence',
    'network-plan workpack 10 policy and enforcement handoff',
  ],
  provenBoundaries: [
    'stored ActivityStore network domain-observed rows map to domain policy preview targets',
    'network target values come from destinationDomain and not browser exact URLs',
    'preview without a matching parent rule stays unknown dry-run',
    'matching parent domain rules resolve only when they cite stored network evidence refs',
    'matching domain rules can preview a block result while enforcement handoff remains disabled',
    'service and portal policy preview payload parsers remain typed read-model consumers',
  ],
  notClaimed: [
    'exact page, video, message, search, or full URL content from network-only evidence',
    'decrypted payload availability',
    'live adapter mutation or host filtering',
    'enforcement command publication',
    'full policy engine execution',
    'portal policy authority or portal enforcement',
    'notification delivery',
  ],
};

writeFileSync(join(proofRoot, 'proof-summary.json'), `${JSON.stringify(proof, null, 2)}\n`);
writeFileSync(join(testRoot, 'proof.json'), `${JSON.stringify(proof, null, 2)}\n`);
console.log('network-policy-preview-stored-flow-proof-ok:core,service,protocol,portal,source-shape');
console.log(`proof=${join(proofRoot, 'proof-summary.json')}`);

function assertSourceContracts() {
  const fixture = readFileSync('crates/agent-core/src/activity_store_policy_preview_test_fixture.rs', 'utf8');
  const previewTests = readFileSync('crates/agent-core/src/activity_store_policy_preview_tests.rs', 'utf8');
  const parentRuleTests = readFileSync(
    'crates/agent-core/src/activity_store_policy_preview_parent_rule_tests.rs',
    'utf8'
  );
  const targets = readFileSync('crates/agent-core/src/activity_store_policy_preview_targets.rs', 'utf8');
  const featureDoc = readFileSync('docs/features/network-domain-control.md', 'utf8');
  const checklist = readFileSync('docs/plans/network-plan/implementation-checklist.md', 'utf8');
  const workpacks = readFileSync('docs/plans/network-plan/workpacks/README.md', 'utf8');
  const requiredSnippets = [
    [fixture, 'pub(crate) fn network_event() -> ActivityEvent'],
    [fixture, 'parent_rule_context_for_network_event'],
    [previewTests, 'policy_preview_read_model_evaluates_stored_network_flow_without_exact_url_or_enforcement'],
    [parentRuleTests, 'policy_preview_read_model_resolves_network_domain_rule_from_stored_flow'],
    [targets, 'constants::field::DESTINATION_DOMAIN'],
    [featureDoc, 'Policy preview over stored flow evidence.'],
    [featureDoc, 'network-policy-preview-stored-flow-proof'],
    [checklist, '34a policy preview over stored network flow evidence'],
    [checklist, 'output/network-plan-proof/34a-policy-preview-stored-flow/proof-summary.json'],
    [workpacks, '34a'],
    [workpacks, 'Policy preview over stored network flow evidence'],
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
        !path.startsWith('output/network-plan-proof/34a-policy-preview-stored-flow/') &&
        !path.startsWith('test-results/network-policy-preview-stored-flow-proof/')
      );
    })
    .join('\n');
}
