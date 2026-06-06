import { spawnSync } from 'node:child_process';
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

const proofRoot = join('output', 'network-plan-proof', '34b-stored-flow-enforcement-result');
const testRoot = join('test-results', 'network-stored-flow-enforcement-result-proof');
mkdirSync(proofRoot, { recursive: true });
mkdirSync(testRoot, { recursive: true });

assertSourceContracts();

writeFileSync(
  join(proofRoot, 'expected-stored-flow-enforcement-result.json'),
  `${JSON.stringify(
    {
      acceptedInputs: [
        'stored ActivityStore network domain observed event',
        'matching parent domain rule context that cites the stored network event',
        'policy preview row target, action, rule, and evidence refs',
        'existing agent-service enforcement command path',
      ],
      resultStates: ['network-control manual-required result', 'network-control unavailable result'],
      policyBoundary:
        'Policy preview remains dry-run; an explicit enforcement command built from the stored evidence refs reaches only the real service boundary and returns manual-required or unavailable without adapter execution.',
      noClaims: [
        'exact URL from network-only evidence',
        'decrypted payload',
        'live DNS mutation',
        'firewall or WFP rule execution',
        'adapter request publication',
        'enforcement-command event publication',
        'live block or terminate behavior',
      ],
    },
    null,
    2
  )}\n`
);

const commands = [
  {
    name: 'agent-service-stored-flow-enforcement-result',
    command: 'cargo',
    args: [
      'test',
      '-p',
      'ocentra-parent-agent-service',
      'enforcement_execute_records_network_domain_result_from_stored_flow_policy_refs',
    ],
    log: join(proofRoot, 'agent-service-stored-flow-enforcement-result-tests.log'),
  },
  {
    name: 'agent-core-stored-flow-policy-preview-parent-rule',
    command: 'cargo',
    args: [
      'test',
      '-p',
      'ocentra-parent-agent-core',
      'policy_preview_read_model_resolves_network_domain_rule_from_stored_flow',
    ],
    log: join(proofRoot, 'agent-core-stored-flow-policy-preview-tests.log'),
  },
  {
    name: 'agent-service-clippy',
    command: 'cargo',
    args: ['clippy', '-p', 'ocentra-parent-agent-service', '--all-targets', '--', '-D', 'warnings'],
    log: join(proofRoot, 'agent-service-clippy.log'),
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
  proof: 'network-stored-flow-enforcement-result-proof',
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
    expectedStoredFlowEnforcementResult: join(proofRoot, 'expected-stored-flow-enforcement-result.json'),
    proofSummary: join(proofRoot, 'proof-summary.json'),
    testProof: join(testRoot, 'proof.json'),
  },
  coveredRows: [
    'network feature checklist stored flow unavailable enforcement result proof',
    'network-plan row 34b stored network flow enforcement unavailable result',
    'network-plan workpack 10 policy and enforcement handoff',
  ],
  provenBoundaries: [
    'stored ActivityStore network domain-observed rows resolve matching parent domain rules by stored evidence refs',
    'the enforcement command payload is built from the policy preview row target, action, rule, and evidence refs',
    'agent-service records pre-action and final audit rows through the real journal/query-store path',
    'network domain block requests resolve to NetworkControl capability',
    'manual-required or unavailable capability returns a typed unavailable result',
    'manual-required or unavailable network-control results do not create adapter requests',
  ],
  notClaimed: [
    'exact page, video, message, search, or full URL content from network-only evidence',
    'decrypted payload availability',
    'live DNS mutation, firewall mutation, WFP execution, or host filtering',
    'live block or terminate result',
    'adapter request publication for manual-required or unavailable capability',
    'enforcement-command event publication',
    'portal policy authority or portal enforcement',
    'notification delivery',
  ],
};

writeFileSync(join(proofRoot, 'proof-summary.json'), `${JSON.stringify(proof, null, 2)}\n`);
writeFileSync(join(testRoot, 'proof.json'), `${JSON.stringify(proof, null, 2)}\n`);
console.log('network-stored-flow-enforcement-result-proof-ok:service,core,clippy,source-shape');
console.log(`proof=${join(proofRoot, 'proof-summary.json')}`);

function assertSourceContracts() {
  const serviceTests = readFileSync('crates/agent-service/src/network_stored_flow_enforcement_result_tests.rs', 'utf8');
  const featureDoc = readFileSync('docs/features/network-domain-control.md', 'utf8');
  const checklist = readFileSync('docs/plans/network-plan/implementation-checklist.md', 'utf8');
  const workpacks = readFileSync('docs/plans/network-plan/workpacks/README.md', 'utf8');
  const requiredSnippets = [
    [serviceTests, 'enforcement_execute_records_network_domain_result_from_stored_flow_policy_refs'],
    [serviceTests, 'command_from_policy_preview_row'],
    [serviceTests, 'network_result_temp_suffix'],
    [serviceTests, 'ADAPTER_KIND_NETWORK_CONTROL'],
    [featureDoc, 'Stored flow unavailable enforcement result proof.'],
    [featureDoc, 'network-stored-flow-enforcement-result-proof'],
    [checklist, '34b stored network flow enforcement unavailable result'],
    [checklist, 'output/network-plan-proof/34b-stored-flow-enforcement-result/proof-summary.json'],
    [workpacks, '34b'],
    [workpacks, 'Stored network flow enforcement unavailable result'],
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
        !path.startsWith('output/network-plan-proof/34b-stored-flow-enforcement-result/') &&
        !path.startsWith('test-results/network-stored-flow-enforcement-result-proof/')
      );
    })
    .join('\n');
}
