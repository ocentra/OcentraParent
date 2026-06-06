import { spawnSync } from 'node:child_process';
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

const proofRoot = join('output', 'network-plan-proof', '22a-risk-target-policy-handoff-proof');
const testRoot = join('test-results', 'network-risk-target-policy-handoff-proof');
mkdirSync(proofRoot, { recursive: true });
mkdirSync(testRoot, { recursive: true });

assertSourceContracts();

writeFileSync(
  join(proofRoot, 'expected-risk-target-policy-handoff.json'),
  `${JSON.stringify(
    {
      acceptedInputs: [
        'risk target ref',
        'domain category lookup',
        'requested parent-rule action',
        'policy decision ref',
        'parent rule ref',
        'evidence refs',
        'optional local-AI result ref',
        'optional adapter capability proof ref',
      ],
      handoffStates: ['policy-dry-run', 'parent-review-required', 'observe-only'],
      gradeMapping: {
        freshHighConfidenceCategory: 'B',
        staleOrLowConfidenceCategory: 'C',
        unknownOrNoMatchCategory: 'D',
      },
      policyBoundary:
        'Risk targets feed the existing evidence-grade policy mapper; they do not become policy authority by themselves.',
      noClaims: [
        'exact URL from network-only evidence',
        'decrypted payload',
        'live adapter mutation',
        'enforcement command publication',
        'broad platform support',
      ],
    },
    null,
    2
  )}\n`
);

const commands = [
  {
    name: 'network-risk-target-policy-handoff-tests',
    command: 'cargo',
    args: ['test', '-p', 'ocentra-network-evidence', 'risk_target_policy_handoff'],
    log: join(proofRoot, 'risk-target-policy-handoff-tests.log'),
  },
  {
    name: 'network-evidence-clippy',
    command: 'cargo',
    args: ['clippy', '-p', 'ocentra-network-evidence', '--all-targets', '--', '-D', 'warnings'],
    log: join(proofRoot, 'clippy.log'),
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
  proof: 'network-risk-target-policy-handoff-proof',
  checkedAt: new Date().toISOString(),
  branch: runText('git', ['branch', '--show-current']).trim(),
  commit: runText('git', ['rev-parse', 'HEAD']).trim(),
  originMain: runText('git', ['rev-parse', 'origin/main']).trim(),
  mergeBase: runText('git', ['merge-base', 'HEAD', 'origin/main']).trim(),
  statusShort: runText('git', ['status', '--short']),
  proofRoot,
  testRoot,
  commands: commandResults,
  artifacts: {
    expectedRiskTargetPolicyHandoff: join(proofRoot, 'expected-risk-target-policy-handoff.json'),
    proofSummary: join(proofRoot, 'proof-summary.json'),
    testProof: join(testRoot, 'proof.json'),
  },
  coveredRows: [
    'network-plan row 22a network risk target policy handoff',
    'network-plan row 34 evidence-grade policy mapping consumer',
  ],
  provenBoundaries: [
    'fresh high-confidence category risk targets map through the existing policy mapper',
    'B-grade block requests route to parent review instead of enforcement',
    'monitor requests stay dry-run without adapter authority',
    'unknown category targets stay observe-only',
    'evidence refs are normalized and preserved',
    'unsupported content, adapter, enforcement, and broad-platform claims are rejected',
  ],
  notClaimed: [
    'exact page, video, message, search, or full URL content from network-only evidence',
    'decrypted payload availability',
    'live adapter mutation or host filtering',
    'enforcement command publication',
    'full policy engine execution',
    'portal UI rendering',
    'broad platform support',
  ],
};

writeFileSync(join(proofRoot, 'proof-summary.json'), `${JSON.stringify(proof, null, 2)}\n`);
writeFileSync(join(testRoot, 'proof.json'), `${JSON.stringify(proof, null, 2)}\n`);
console.log('network-risk-target-policy-handoff-proof-ok:tests,clippy,source-shape');
console.log(`proof=${join(proofRoot, 'proof-summary.json')}`);

function assertSourceContracts() {
  const source = readFileSync('crates/ocentra-network-evidence/src/risk_target_policy_handoff.rs', 'utf8');
  const tests = readFileSync('crates/ocentra-network-evidence/src/tests/risk_target_policy_handoff.rs', 'utf8');
  const lib = readFileSync('crates/ocentra-network-evidence/src/lib.rs', 'utf8');
  const featureDoc = readFileSync('docs/features/network-domain-control.md', 'utf8');
  const checklist = readFileSync('docs/plans/network-plan/implementation-checklist.md', 'utf8');
  const readme = readFileSync('crates/ocentra-network-evidence/README.md', 'utf8');
  const requiredSnippets = [
    [source, 'map_network_evidence_grade_to_policy'],
    [source, 'NetworkRiskTargetPolicyHandoffState::ParentReviewRequired'],
    [source, 'BroadPlatformSupportClaimRejected'],
    [source, 'enforcement_commands_published: 0'],
    [tests, 'risk_target_policy_handoff_routes_signed_video_block_to_parent_review'],
    [tests, 'risk_target_policy_handoff_rejects_network_only_content_and_authority_claims'],
    [lib, 'pub mod risk_target_policy_handoff'],
    [lib, 'map_network_risk_target_to_policy_handoff'],
    [featureDoc, 'Network category/risk targets.'],
    [featureDoc, 'risk-target policy handoff'],
    [checklist, '22a network risk target policy handoff'],
    [checklist, 'output/network-plan-proof/22a-risk-target-policy-handoff-proof/proof-summary.json'],
    [readme, 'risk-target policy handoff'],
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
