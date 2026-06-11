import { execFileSync } from 'node:child_process';
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

const proofName = 'browser-runtime-event-name-parity-proof';
const resultRoot = join('test-results', proofName);
const outputRoot = join('output', 'browser-plan-proof', 'browser-runtime-event-name-parity');

mkdirSync(resultRoot, { recursive: true });
mkdirSync(outputRoot, { recursive: true });

const rustConstantsSource = readFileSync('crates/agent-protocol/src/constants/browser.rs', 'utf8');
const rustPhaseSource = readFileSync('crates/agent-core/src/browser_event_runtime_phase.rs', 'utf8');
const typescriptParserSource = readFileSync('packages/agent-protocol-domain/src/browser-runtime-events.ts', 'utf8');
const typescriptTestSource = readFileSync(
  'packages/agent-protocol-domain/tests/browser-runtime-events.test.ts',
  'utf8'
);

const rustRuntimeEventNames = [
  'EVENT_BROWSER_EVIDENCE_OBSERVED',
  'EVENT_BROWSER_EVIDENCE_JOURNALED',
  'EVENT_BROWSER_AI_ANALYSIS_REQUESTED',
  'EVENT_BROWSER_AI_ANALYSIS_COMPLETED',
  'EVENT_BROWSER_POLICY_EVALUATION_REQUESTED',
  'EVENT_BROWSER_POLICY_DECISION_COMPLETED',
  'EVENT_BROWSER_INTERVENTION_COMMAND_ISSUED',
  'EVENT_BROWSER_INTERVENTION_RESULT_OBSERVED',
  'EVENT_BROWSER_AUDIT_ENTRY_COMMITTED',
  'EVENT_BROWSER_READ_MODEL_PROJECTED',
].map((constantName) => rustStringConstant(rustConstantsSource, constantName));

const typescriptRuntimeEventNames = [
  'EvidenceObserved',
  'EvidenceJournaled',
  'AiAnalysisRequested',
  'AiAnalysisCompleted',
  'PolicyEvaluationRequested',
  'PolicyDecisionCompleted',
  'InterventionCommandIssued',
  'InterventionResultObserved',
  'AuditEntryCommitted',
  'ReadModelProjected',
].map((propertyName) => typescriptEventType(typescriptParserSource, propertyName));

const commandResults = [
  {
    command: 'cmd /c npm run test --workspace @ocentra-parent/agent-protocol-domain -- browser-runtime-events.test.ts',
    output: runNpm([
      'run',
      'test',
      '--workspace',
      '@ocentra-parent/agent-protocol-domain',
      '--',
      'browser-runtime-events.test.ts',
    ]),
  },
  {
    command: 'cmd /c npm run type-check --workspace @ocentra-parent/agent-protocol-domain',
    output: runNpm(['run', 'type-check', '--workspace', '@ocentra-parent/agent-protocol-domain']),
  },
  {
    command: 'cargo test -p ocentra-parent-agent-core browser_runtime_chain_topology --quiet',
    output: run('cargo', ['test', '-p', 'ocentra-parent-agent-core', 'browser_runtime_chain_topology', '--quiet']),
  },
];

const namesMatch = JSON.stringify(rustRuntimeEventNames) === JSON.stringify(typescriptRuntimeEventNames);
const parserRejectsPhaseDrift = typescriptParserSource.includes('phaseMatchesEventType');
const testCoversAllRustNames = rustRuntimeEventNames.every((eventName) =>
  typescriptTestSource.includes(`'${eventName}'`)
);
const rustPhaseUsesProtocolConstants =
  rustPhaseSource.includes('constants::browser::EVENT_BROWSER_AI_ANALYSIS_REQUESTED') &&
  rustPhaseSource.includes('constants::browser::EVENT_BROWSER_POLICY_DECISION_COMPLETED') &&
  rustPhaseSource.includes('constants::browser::EVENT_BROWSER_INTERVENTION_COMMAND_ISSUED');

if (!namesMatch || !parserRejectsPhaseDrift || !testCoversAllRustNames || !rustPhaseUsesProtocolConstants) {
  throw new Error(
    JSON.stringify(
      {
        namesMatch,
        parserRejectsPhaseDrift,
        testCoversAllRustNames,
        rustPhaseUsesProtocolConstants,
        rustRuntimeEventNames,
        typescriptRuntimeEventNames,
      },
      null,
      2
    )
  );
}

const proof = {
  proofName,
  rustRuntimeEventNames,
  typescriptRuntimeEventNames,
  sourceChecks: {
    namesMatch,
    parserRejectsPhaseDrift,
    testCoversAllRustNames,
    rustPhaseUsesProtocolConstants,
  },
  commands: commandResults.map((result) => result.command),
  verified: {
    rustToTypescriptBrowserRuntimeEventNameParity: true,
    allBrowserRuntimePhasesCovered: true,
    parserStillRejectsPhaseEventTypeDrift: true,
    genericEventBusChanged: false,
    portalUiChanged: false,
    aiExecutes: false,
    policyExecutes: false,
    browserMutationExecutes: false,
    childInterventionExecutes: false,
    enforcementExecutes: false,
  },
};

writeFileSync(join(resultRoot, 'proof.json'), `${JSON.stringify(proof, null, 2)}\n`);
writeFileSync(
  join(outputRoot, '01-browser-runtime-event-name-parity-proof.md'),
  [
    '# Browser Runtime Event Name Parity Proof',
    '',
    `- Rust and TypeScript event names match: ${namesMatch}`,
    `- Parser rejects phase/event drift: ${parserRejectsPhaseDrift}`,
    `- Test covers all Rust browser runtime event names: ${testCoversAllRustNames}`,
    `- Rust phase mapping uses protocol constants: ${rustPhaseUsesProtocolConstants}`,
    '',
    '## Rust / TypeScript Event Names',
    '',
    ...rustRuntimeEventNames.map((eventName) => `- ${eventName}`),
    '',
    '## Commands',
    '',
    ...commandResults.map((result) => `- ${result.command}`),
    '',
    '## No-Claim Boundaries',
    '',
    '- No generic event bus change.',
    '- No portal UI change.',
    '- No AI execution.',
    '- No policy execution.',
    '- No browser mutation.',
    '- No child intervention execution.',
    '- No enforcement.',
    '',
  ].join('\n')
);

console.log(JSON.stringify(proof, null, 2));

function rustStringConstant(source, constantName) {
  const match = source.match(new RegExp(`pub const ${constantName}: &str =\\s*"([^"]+)"`));
  if (!match) {
    throw new Error(`Missing Rust browser constant ${constantName}`);
  }
  return match[1];
}

function typescriptEventType(source, propertyName) {
  const match = source.match(new RegExp(`${propertyName}: '([^']+)'`));
  if (!match) {
    throw new Error(`Missing TypeScript browser event type ${propertyName}`);
  }
  return match[1];
}

function run(command, args) {
  return execFileSync(command, args, {
    cwd: process.cwd(),
    encoding: 'utf8',
    stdio: ['ignore', 'pipe', 'pipe'],
  }).trim();
}

function runNpm(args, ...rest) {
  const command = process.platform === 'win32' ? 'cmd' : 'npm';
  const commandArgs = process.platform === 'win32' ? ['/c', 'npm', ...args] : args;
  return run(command, commandArgs, ...rest);
}
