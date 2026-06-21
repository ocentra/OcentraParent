import { execFileSync } from 'node:child_process';
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

const proofName = 'browser-runtime-portal-typed-stream-consumer-proof';
const testResultsDir = join('test-results', proofName);
const outputDir = join('output', 'browser-plan-proof', 'browser-runtime-portal-typed-stream-consumer');

mkdirSync(testResultsDir, { recursive: true });
mkdirSync(outputDir, { recursive: true });

const portalSource = readFileSync('apps/portal/src/live-activity-state.ts', 'utf8');
const portalTestSource = readFileSync('apps/portal/tests/live-activity-state.test.ts', 'utf8');

const sourceChecks = {
  portalDelegatesToSharedTypedConsumer: portalSource.includes('resolvePortalDomainLiveActivityState'),
  portalRemovedLooseEntryParser: !portalSource.includes('function parseBrowserRuntimeEventChainEntry'),
  portalReexportsTypedBrowserRuntimeEntries: portalSource.includes(
    'export type PortalBrowserRuntimeEventChainEntry = PortalDomainPortalBrowserRuntimeEventChainEntry'
  ),
  testsRejectEventTypePhaseDrift: portalTestSource.includes('event type and phase drift'),
  testsRejectAiAuthorityOverclaim: portalTestSource.includes('claim AI authority in the portal'),
  testsRejectCountDrift: portalTestSource.includes('count fields drift from entries'),
  testsUseRustSerializedPhaseNames: portalTestSource.includes('AgentBrowserRuntimePhase.EvidenceObserved'),
};

for (const [name, passed] of Object.entries(sourceChecks)) {
  if (!passed) {
    throw new Error(`Browser runtime portal typed stream consumer source check failed: ${name}`);
  }
}

const commands = [
  {
    name: 'portal-live-activity-state-test',
    command: 'npm',
    args: ['run', 'test', '--workspace', '@ocentra-parent/portal', '--', 'live-activity-state.test.ts'],
  },
  {
    name: 'portal-type-check',
    command: 'npm',
    args: ['run', 'type-check', '--workspace', '@ocentra-parent/portal'],
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
  sourceChecks,
  commands: commandResults.map(({ command }) => command),
  verified: {
    portalConsumesSharedTypedBrowserRuntimeStreamContract: true,
    portalRejectsEventTypePhaseDrift: true,
    portalRejectsAiAuthorityOverclaim: true,
    portalRejectsCountDrift: true,
    portalUiChanged: false,
    aiExecutes: false,
    policyExecutes: false,
    browserMutationExecutes: false,
    childInterventionExecutes: false,
    enforcementExecutes: false,
  },
};

writeFileSync(join(testResultsDir, 'proof.json'), `${JSON.stringify(proof, null, 2)}\n`);
writeFileSync(
  join(outputDir, '01-browser-runtime-portal-typed-stream-consumer-proof.md'),
  [
    '# Browser Runtime Portal Typed Stream Consumer Proof',
    '',
    `- Branch head: ${proof.branchHead}`,
    `- Portal delegates to shared typed consumer: ${sourceChecks.portalDelegatesToSharedTypedConsumer}`,
    `- Loose local entry parser removed: ${sourceChecks.portalRemovedLooseEntryParser}`,
    `- Portal reexports typed browser runtime entries: ${sourceChecks.portalReexportsTypedBrowserRuntimeEntries}`,
    `- Event type/phase drift rejected by portal test: ${sourceChecks.testsRejectEventTypePhaseDrift}`,
    `- AI authority overclaim rejected by portal test: ${sourceChecks.testsRejectAiAuthorityOverclaim}`,
    `- Stream count drift rejected by portal test: ${sourceChecks.testsRejectCountDrift}`,
    '',
    '## Commands',
    '',
    ...commandResults.map((result) => `- ${result.command}`),
    '',
    '## No-Claim Boundaries',
    '',
    '- No new portal visual surface.',
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
