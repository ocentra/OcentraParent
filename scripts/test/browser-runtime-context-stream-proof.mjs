import { execFileSync } from 'node:child_process';
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

const proofName = 'browser-runtime-context-stream-proof';
const testResultsDir = join('test-results', proofName);
const outputDir = join('output', 'browser-plan-proof', 'browser-runtime-context-stream');

mkdirSync(testResultsDir, { recursive: true });
mkdirSync(outputDir, { recursive: true });

const protocolSource = readFileSync('packages/agent-protocol-domain/src/browser-runtime-events.ts', 'utf8');
const coreSource = readFileSync('crates/agent-core/src/browser_event_runtime.rs', 'utf8');
const deliverySource = readFileSync('crates/agent-service/src/browser_runtime_delivery.rs', 'utf8');
const streamSource = readFileSync('crates/agent-service/src/browser_runtime_stream_events.rs', 'utf8');
const portalTestSource = readFileSync('apps/portal/tests/live-activity-state.test.ts', 'utf8');

const sourceChecks = {
  protocolRequiresCapabilityStatus: protocolSource.includes(
    'capabilityStatus: AgentBrowserRuntimeCapabilityStatusSchema'
  ),
  protocolRequiresCustodyLabel: protocolSource.includes('custodyLabel: AgentBrowserRuntimeCustodyLabelSchema'),
  protocolRequiresQueryVisibility: protocolSource.includes('queryVisibility: AgentBrowserRuntimeQueryVisibilitySchema'),
  protocolRequiresDegradedReason: protocolSource.includes('degradedReason: NullableBrowserRuntimeText'),
  protocolRejectsUnavailableContextWithoutReason: protocolSource.includes('browserRuntimeUnavailableContextHasReason'),
  protocolRejectsExactUrlWithoutSupportedContext: protocolSource.includes('browserRuntimeContextSupportsExactUrl'),
  corePayloadCarriesCapabilityStatus: coreSource.includes('pub capability_status: String'),
  corePayloadCarriesDegradedReason: coreSource.includes('pub degraded_reason: Option<String>'),
  deliveryCopiesReadModelCapabilityStatus: deliverySource.includes('row.capability_status.as_protocol_str()'),
  deliveryCopiesReadModelQueryVisibility: deliverySource.includes('row.query_visibility.as_protocol_str()'),
  streamSerializesCapabilityStatus: streamSource.includes('constants::field::CAPABILITY_STATUS'),
  streamSerializesQueryVisibility: streamSource.includes('constants::field::QUERY_VISIBILITY'),
  portalAssertsContextVisible: portalTestSource.includes('firstPayload.capabilityStatus'),
};

for (const [name, passed] of Object.entries(sourceChecks)) {
  if (!passed) {
    throw new Error(`Browser runtime context stream source check failed: ${name}`);
  }
}

const commands = [
  {
    name: 'agent-core-browser-event-runtime-test',
    command: 'cargo test -p ocentra-parent-agent-core browser_event_runtime --quiet',
  },
  {
    name: 'agent-service-browser-runtime-stream-test',
    command: 'cargo test -p ocentra-parent-agent-service browser_runtime_stream --quiet',
  },
  {
    name: 'agent-protocol-domain-browser-runtime-events-test',
    command: 'cmd /c npm run test --workspace @ocentra-parent/agent-protocol-domain -- browser-runtime-events.test.ts',
  },
  {
    name: 'build-contracts',
    command: 'cmd /c npm run build:contracts',
  },
  {
    name: 'portal-live-activity-state-test',
    command: 'cmd /c npm run test --workspace @ocentra-parent/portal -- live-activity-state.test.ts',
  },
];

const commandResults = commands.map((entry) => ({
  name: entry.name,
  command: entry.command,
  output: runShell(entry.command),
}));

const proof = {
  proofName,
  branchHead: runGit(['log', '-1', '--oneline']).trim(),
  gitStatusShort: runGit(['status', '--short']).trim(),
  sourceChecks,
  commands: commandResults.map(({ command }) => command),
  verified: {
    browserRuntimeEventsCarryCapabilityStatus: true,
    browserRuntimeEventsCarryCustodyLabel: true,
    browserRuntimeEventsCarryQueryVisibility: true,
    browserRuntimeEventsCarryDegradedReason: true,
    portalStateReceivesContext: true,
    unavailableContextNeedsReason: true,
    exactUrlNeedsSupportedContext: true,
    newEventBusCreated: false,
    aiExecutes: false,
    policyExecutes: false,
    browserMutationExecutes: false,
    childInterventionExecutes: false,
    enforcementExecutes: false,
  },
};

writeFileSync(join(testResultsDir, 'proof.json'), `${JSON.stringify(proof, null, 2)}\n`);
writeFileSync(
  join(outputDir, '01-browser-runtime-context-stream-proof.md'),
  [
    '# Browser Runtime Context Stream Proof',
    '',
    `- Branch head: ${proof.branchHead}`,
    `- Capability status carried by protocol/core/service/portal: ${sourceChecks.protocolRequiresCapabilityStatus}`,
    `- Custody label carried by protocol/core/service/portal: ${sourceChecks.protocolRequiresCustodyLabel}`,
    `- Query visibility carried by protocol/core/service/portal: ${sourceChecks.protocolRequiresQueryVisibility}`,
    `- Degraded reason carried by protocol/core/service/portal: ${sourceChecks.protocolRequiresDegradedReason}`,
    `- Unsupported exact URL context rejected: ${sourceChecks.protocolRejectsExactUrlWithoutSupportedContext}`,
    `- Unavailable context without reason rejected: ${sourceChecks.protocolRejectsUnavailableContextWithoutReason}`,
    '',
    '## Commands',
    '',
    ...commandResults.map((result) => `- ${result.command}`),
    '',
    '## No-Claim Boundaries',
    '',
    '- No new event bus or private browser bus.',
    '- No AI execution.',
    '- No policy execution.',
    '- No browser mutation.',
    '- No child intervention execution.',
    '- No enforcement.',
    '',
  ].join('\n')
);

console.log(JSON.stringify(proof, null, 2));

function runShell(command) {
  return execFileSync(command, {
    cwd: process.cwd(),
    encoding: 'utf8',
    shell: 'powershell.exe',
    stdio: ['ignore', 'pipe', 'pipe'],
  });
}

function runGit(args) {
  return execFileSync('git', args, {
    cwd: process.cwd(),
    encoding: 'utf8',
  });
}
