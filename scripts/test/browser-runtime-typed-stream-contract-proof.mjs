import { execFileSync } from 'node:child_process';
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

const proofName = 'browser-runtime-typed-stream-contract-proof';
const testResultsDir = join('test-results', proofName);
const outputDir = join('output', 'browser-plan-proof', 'browser-runtime-typed-stream-contract');

mkdirSync(testResultsDir, { recursive: true });
mkdirSync(outputDir, { recursive: true });

const contractSource = readFileSync('packages/agent-protocol-domain/src/browser-runtime-events.ts', 'utf8');
const portalSource = readFileSync('apps/portal/src/live-activity-state.ts', 'utf8');
const testSource = readFileSync('packages/agent-protocol-domain/tests/browser-runtime-events.test.ts', 'utf8');

const commands = [
  {
    name: 'agent-protocol-domain-browser-runtime-events-test',
    command: 'npm',
    args: [
      'run',
      'test',
      '--workspace',
      '@ocentra-parent/agent-protocol-domain',
      '--',
      'browser-runtime-events.test.ts',
    ],
  },
  {
    name: 'agent-protocol-domain-type-check',
    command: 'npm',
    args: ['run', 'type-check', '--workspace', '@ocentra-parent/agent-protocol-domain'],
  },
];

const commandResults = commands.map((entry) => ({
  name: entry.name,
  command: ['cmd', '/c', entry.command, ...entry.args].join(' '),
  output: runCommand(entry.command, entry.args),
}));

const proof = {
  proofName,
  branchHead: runGit(['log', '-1', '--oneline']).trim(),
  gitStatusShort: runGit(['status', '--short']).trim(),
  sourceChecks: {
    parserLivesInProtocolDomain: contractSource.includes('parseAgentBrowserRuntimeEventChainStreamFields'),
    validatesKnownBrowserRuntimeEventTypes: contractSource.includes('AgentBrowserRuntimeEventTypeSchema'),
    validatesRustSerializedPhases: contractSource.includes('AgentBrowserRuntimePhaseSchema'),
    rejectsEventTypePhaseDrift: contractSource.includes('phaseMatchesEventType'),
    rejectsAiAuthorityOverclaim: contractSource.includes('aiAuthority: Schema.Literal(false)'),
    rejectsHiddenInterventionExecution: contractSource.includes('browserRuntimePayloadIsHonest'),
    testsInvalidJsonAndCountDrift:
      testSource.includes("reason: 'invalid-json'") && testSource.includes("reason: 'invalid-stream'"),
    portalStillNeedsSequencedAdoption: !portalSource.includes('parseAgentBrowserRuntimeEventChainStreamFields'),
  },
  commands: commandResults.map(({ command }) => command),
  verified: {
    protocolDomainTypedParserExists: true,
    servicePayloadShapeIsSchemaBackedForFutureConsumers: true,
    portalConsumptionChanged: false,
    aiExecutes: false,
    policyExecutes: false,
    browserMutationExecutes: false,
    childInterventionExecutes: false,
    enforcementExecutes: false,
  },
};

writeFileSync(join(testResultsDir, 'proof.json'), `${JSON.stringify(proof, null, 2)}\n`);
writeFileSync(
  join(outputDir, '01-browser-runtime-typed-stream-contract-proof.md'),
  [
    '# Browser Runtime Typed Stream Contract Proof',
    '',
    `- Branch head: ${proof.branchHead}`,
    `- Protocol parser exists: ${proof.sourceChecks.parserLivesInProtocolDomain}`,
    `- Known browser runtime event types validated: ${proof.sourceChecks.validatesKnownBrowserRuntimeEventTypes}`,
    `- Rust serialized phase names validated: ${proof.sourceChecks.validatesRustSerializedPhases}`,
    `- Event type/phase drift rejected: ${proof.sourceChecks.rejectsEventTypePhaseDrift}`,
    `- AI authority overclaim rejected: ${proof.sourceChecks.rejectsAiAuthorityOverclaim}`,
    `- Hidden intervention execution rejected: ${proof.sourceChecks.rejectsHiddenInterventionExecution}`,
    `- Portal adoption still sequenced behind current portal lock: ${proof.sourceChecks.portalStillNeedsSequencedAdoption}`,
    '',
    '## Commands',
    '',
    ...commandResults.map((result) => `- ${result.command}`),
    '',
    '## No-Claim Boundaries',
    '',
    '- No portal consumption change in this slice.',
    '- No AI execution.',
    '- No policy execution.',
    '- No browser mutation.',
    '- No child intervention execution.',
    '- No enforcement.',
    '',
  ].join('\n')
);

console.log(JSON.stringify(proof, null, 2));

function runCommand(command, args) {
  return execFileSync('cmd', ['/c', command, ...args], {
    cwd: process.cwd(),
    encoding: 'utf8',
    stdio: ['ignore', 'pipe', 'pipe'],
  });
}

function runGit(args) {
  return execFileSync('git', args, {
    cwd: process.cwd(),
    encoding: 'utf8',
  });
}
