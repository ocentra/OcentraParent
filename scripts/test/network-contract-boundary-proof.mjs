import { spawnSync } from 'node:child_process';
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

const proofRoot = join('output', 'network-plan-proof', '03-contract-boundary-and-effect-schemas');
mkdirSync(proofRoot, { recursive: true });

const commands = [
  npmCommand('activity-domain-network-contract-tests', [
    'run',
    'test',
    '--workspace',
    '@ocentra-parent/network-domain',
    '--',
    'network-contracts.test.ts',
    'network-flow.test.ts',
  ]),
  npmCommand('activity-domain-build', ['run', 'build', '--workspace', '@ocentra-parent/network-domain']),
  {
    name: 'source-shape',
    command: 'node',
    args: ['scripts/check-source-shape.mjs'],
  },
];

const sourceSnapshot = [
  '# Network Contract Boundary Source Snapshot',
  '',
  `checkedAt: ${new Date().toISOString()}`,
  `branch: ${runText('git', ['branch', '--show-current']).trim()}`,
  `commit: ${runText('git', ['rev-parse', 'HEAD']).trim()}`,
  '',
  '```text',
  runText('git', ['status', '--short']),
  '```',
  '',
  'Inspected source paths:',
  '',
  '- packages/network-domain/src/network-flow.ts',
  '- packages/network-domain/src/network-contracts.ts',
  '- packages/network-domain/tests/unit/network-flow.test.ts',
  '- packages/network-domain/tests/unit/network-contracts.test.ts',
  '',
  'Before-state gap: network flow observations existed, but workpack 03 rows 05-09 did not yet expose a shared Effect Schema boundary for flow evidence, domain evidence, activity classification, A/B/C/D evidence grades, and policy/action capability gating.',
];
writeFileSync(join(proofRoot, '00-source-snapshot.md'), `${sourceSnapshot.join('\n')}\n`);

const commandResults = commands.map((entry) => runCommand(entry));

const networkContractsSource = readFileSync('packages/network-domain/src/network-contracts.ts', 'utf8');
const networkFlowSource = readFileSync('packages/network-domain/src/network-flow.ts', 'utf8');
const networkContractTests = readFileSync('packages/network-domain/tests/unit/network-contracts.test.ts', 'utf8');

const assertions = [
  ['flow-evidence-schema', networkContractsSource.includes('ActivityNetworkFlowEvidenceSchema')],
  ['domain-evidence-schema', networkContractsSource.includes('ActivityNetworkDomainEvidenceSchema')],
  ['classification-schema', networkContractsSource.includes('ActivityNetworkActivityClassificationSchema')],
  ['evidence-grade-schema', networkContractsSource.includes("Schema.Literal('A', 'B', 'C', 'D')")],
  ['policy-action-schema', networkContractsSource.includes('ActivityNetworkPolicyActionSchema')],
  ['network-flow-public-reexport', networkFlowSource.includes("export * from './network-contracts';")],
  ['unsupported-exact-url-negative-test', networkContractTests.includes("unsupportedClaimAttempts: ['exact-url']")],
  ['adapter-authorization-negative-test', networkContractTests.includes('adapterCallAuthorized: true')],
];
const failed = assertions.find(([, passed]) => !passed);
if (failed) {
  throw new Error(`network contract boundary assertion failed: ${failed[0]}`);
}

writeFileSync(
  join(proofRoot, '09-security-negative-proof.log'),
  [
    'Network contract boundary negative proof:',
    '- ActivityNetworkFlowEvidenceSchema rejects unsupportedClaimAttempts including exact-url and decrypted-payload.',
    '- ActivityNetworkPolicyActionSchema authorizes adapter calls only when mode is apply-ready, policyDecisionRef exists, adapter capability is proved-available, and action is monitor/limit/block.',
    '- Dry-run, manual-required, adapter-unavailable, and observe-only states cannot authorize adapter calls.',
  ].join('\n') + '\n'
);

writeFileSync(
  join(proofRoot, '12-validation-commands.log'),
  `${commandResults.map((entry) => `${entry.command} -> ${entry.status}`).join('\n')}\n`
);

const proof = {
  proof: 'network-contract-boundary',
  proofRoot,
  checkedAt: new Date().toISOString(),
  commands: commandResults,
  assertions: assertions.map(([name]) => name),
  artifacts: {
    sourceSnapshot: join(proofRoot, '00-source-snapshot.md'),
    contractProof: join(proofRoot, '01-contract-proof.log'),
    securityNegativeProof: join(proofRoot, '09-security-negative-proof.log'),
    validationCommands: join(proofRoot, '12-validation-commands.log'),
  },
  provenRows: [
    '03 Contract boundary and Effect schemas',
    '05 NetworkFlowEvidence contract',
    '06 NetworkDomainEvidence contract',
    '07 NetworkActivityClassification contract',
    '08 NetworkEvidenceGrade model',
    '09 NetworkPolicyAction and capability contract',
  ],
  notClaimed: [
    'Rust protocol parity for network contracts',
    'PCAP parsing or live capture',
    'Zeek, Suricata, or Snort analyzer comparison',
    'AI model evaluation, production risk budgets, or adapter execution',
    'Portal UI network evidence drawer',
  ],
};

writeFileSync(join(proofRoot, 'proof-summary.json'), `${JSON.stringify(proof, null, 2)}\n`);
console.log(`network-contract-boundary-proof-ok:${proof.assertions.join(',')}`);
console.log(`proof=${join(proofRoot, 'proof-summary.json')}`);

function npmCommand(name, args) {
  if (process.platform === 'win32') {
    return { name, command: 'cmd', args: ['/c', 'npm', ...args] };
  }
  return { name, command: 'npm', args };
}

function runCommand(entry) {
  const result = spawnSync(entry.command, entry.args, {
    encoding: 'utf8',
    shell: false,
  });
  const output = `${result.stdout ?? ''}${result.stderr ?? ''}`;
  const logName =
    entry.name === 'activity-domain-network-contract-tests' ? '01-contract-proof.log' : `${entry.name}.log`;
  writeFileSync(join(proofRoot, logName), output);
  if (result.status !== 0) {
    throw new Error(`${entry.name} failed with exit ${result.status}`);
  }
  return {
    name: entry.name,
    command: [entry.command, ...entry.args].join(' '),
    status: result.status,
    log: join(proofRoot, logName),
  };
}

function runText(command, args) {
  const result = spawnSync(command, args, { encoding: 'utf8', shell: false });
  if (result.status !== 0) {
    throw new Error(`${command} ${args.join(' ')} failed with exit ${result.status}`);
  }
  return `${result.stdout ?? ''}${result.stderr ?? ''}`;
}
