import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = fileURLToPath(new URL('../..', import.meta.url));
const testOutputDir = join(repoRoot, 'test-results', 'app-game-source-status-state-proof');
const appGameProofDir = join(repoRoot, 'output', 'app-game-plan-proof', '47-backend-source-freshness-read-model');
const appProofDir = join(repoRoot, 'output', 'app-plan-proof', '47-backend-source-freshness-read-model');

const sourceFiles = {
  activityDomain: 'packages/activity-domain/src/activity-surface.ts',
  activityDomainTest: 'packages/activity-domain/tests/activity-surface.test.ts',
  agentProtocol: 'crates/agent-protocol/src/activity_surface.rs',
  agentProtocolConstants: 'crates/agent-protocol/src/constants/activity_surface.rs',
  agentProtocolTest: 'crates/agent-protocol/src/activity_surface_tests.rs',
  agentServiceSource: 'crates/agent-service/src/activity_surface_read_models/shared.rs',
  agentServicePayload: 'crates/agent-service/src/activity_surface_payload.rs',
  agentServiceTest: 'crates/agent-service/src/activity_surface_read_models/app_game_source_status_tests.rs',
  protocolAdapterTest: 'packages/agent-protocol-domain/tests/activity-surface-adapter.test.ts',
};

const sources = Object.fromEntries(
  await Promise.all(
    Object.entries(sourceFiles).map(async ([key, path]) => [key, await readFile(join(repoRoot, path), 'utf8')])
  )
);

const checks = {
  activityDomainStates:
    sources.activityDomain.includes("'manual-required'") && sources.activityDomain.includes("'degraded'"),
  rustProtocolStates:
    sources.agentProtocol.includes('ManualRequired') &&
    sources.agentProtocol.includes('#[serde(rename = "manual-required")]') &&
    sources.agentProtocol.includes('Degraded') &&
    sources.agentProtocol.includes('#[serde(rename = "degraded")]'),
  serviceManualRequiredMapping: sources.agentServiceSource.includes(
    'APP_GAME_CAPABILITY_STATUS_MANUAL_REQUIRED => ActivityReadModelState::ManualRequired'
  ),
  serviceDegradedMapping: sources.agentServiceSource.includes(
    'APP_GAME_CAPABILITY_STATUS_DEGRADED => ActivityReadModelState::Degraded'
  ),
  serviceNotClaimedMapping: sources.agentServiceSource.includes(
    'APP_GAME_CAPABILITY_STATUS_NOT_CLAIMED => ActivityReadModelState::ScaffoldOnly'
  ),
  protocolStateConstants:
    sources.agentProtocolConstants.includes('STATE_MANUAL_REQUIRED: &str = "manual-required"') &&
    sources.agentProtocolConstants.includes('STATE_DEGRADED: &str = "degraded"'),
  servicePayloadMapping:
    sources.agentServicePayload.includes('ActivityReadModelState::ManualRequired') &&
    sources.agentServicePayload.includes('STATE_MANUAL_REQUIRED') &&
    sources.agentServicePayload.includes('ActivityReadModelState::Degraded') &&
    sources.agentServicePayload.includes('STATE_DEGRADED'),
  rustServiceBehaviorTest: sources.agentServiceTest.includes(
    'app_use_read_model_preserves_non_ready_source_status_state'
  ),
  rustProtocolBehaviorTest: sources.agentProtocolTest.includes(
    'activity_app_game_source_status_serializes_manual_required_and_degraded_states'
  ),
  typeScriptContractTest: sources.activityDomainTest.includes(
    'accepts manual and degraded source status rows without ready source claims'
  ),
  protocolAdapterTest: sources.protocolAdapterTest.includes(
    'parses app-use source status rows that preserve manual-required and degraded states'
  ),
};

const failed = Object.entries(checks)
  .filter(([, ok]) => !ok)
  .map(([name]) => name);

if (failed.length > 0) {
  throw new Error(`app-game-source-status-state-proof failed: ${failed.join(', ')}`);
}

const proof = {
  proofMode: 'app-game-source-status-state',
  generatedAt: new Date().toISOString(),
  scope:
    'WP47 follow-up: backend app/game source status rows preserve manual-required, degraded, and not-claimed source states instead of reporting them as ready.',
  sourceFiles,
  checks,
  noClaimBoundaries: [
    'source status rows summarize existing backend evidence only',
    'manualRequired capability maps to manual-required read-model state and does not become ready',
    'degraded capability maps to degraded read-model state and does not become ready',
    'notClaimed capability maps to scaffold-only read-model state and does not become ready',
    'no portal rendering, policy consumption, provider delivery, adapter execution, broad blocking, or platform support is added',
  ],
};

const summary = [
  '# App/Game Source Status State Proof',
  '',
  'WP47 follow-up proof for backend source freshness rows.',
  '',
  '- `manualRequired` source capability now maps to `manual-required` read-model state.',
  '- `degraded` source capability now maps to `degraded` read-model state.',
  '- `notClaimed` source capability now maps to `scaffold-only` read-model state.',
  '- Activity-domain, protocol-domain adapter, Rust protocol, and Rust service tests cover the boundary.',
  '',
  'No portal rendering, policy consumption, adapter execution, broad blocking, provider delivery, or platform support is claimed.',
  '',
];

await Promise.all([
  mkdir(testOutputDir, { recursive: true }),
  mkdir(appGameProofDir, { recursive: true }),
  mkdir(appProofDir, { recursive: true }),
]);
await Promise.all([
  writeFile(join(testOutputDir, 'proof.json'), JSON.stringify(proof, null, 2)),
  writeFile(join(appGameProofDir, '13-source-status-state-proof.json'), JSON.stringify(proof, null, 2)),
  writeFile(join(appGameProofDir, '13-source-status-state-proof.md'), summary.join('\n')),
  writeFile(join(appProofDir, '13-source-status-state-proof.json'), JSON.stringify(proof, null, 2)),
  writeFile(join(appProofDir, '13-source-status-state-proof.md'), summary.join('\n')),
]);

console.log('app-game-source-status-state-proof-ok');
