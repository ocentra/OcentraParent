import { mkdir, writeFile } from 'node:fs/promises';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { ParentContractSchemaVersion } from '@ocentra-parent/schema-domain/family-reference-primitives';
import { buildAppGameSourceGatedPolicyPreviewReadModel } from '../../src/app-game-source-gated-policy-preview-read-model';
import { buildAppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoff } from '../../src/app-game-source-gated-policy-preview-timer-audit-rollback-handoff';
import { buildAppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntent } from '../../src/app-game-source-gated-policy-preview-timer-audit-rollback-parent-surface-intent';
import { buildAppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModel } from '../../src/app-game-source-gated-policy-preview-timer-audit-rollback-read-model';
import { buildAppGameSourceGatedPolicyPreviewTimerHandoff } from '../../src/app-game-source-gated-policy-preview-timer-handoff';
import { buildAppGameSourceGatedPolicyPreviewTimerRuntimeReadiness } from '../../src/app-game-source-gated-policy-preview-timer-runtime-readiness';
import { buildAppGameSourceGatedPolicyPreviewTimerSchedulerPersistence } from '../../src/app-game-source-gated-policy-preview-timer-scheduler-persistence';
import { buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoff } from '../../src/app-game-source-gated-policy-preview-timer-service-readiness-handoff';
import { buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoff } from '../../src/app-game-source-gated-policy-preview-timer-service-readiness-protocol-command-handoff';
import { buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoff } from '../../src/app-game-source-gated-policy-preview-timer-service-readiness-protocol-handoff';
import { buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModel } from '../../src/app-game-source-gated-policy-preview-timer-service-readiness-protocol-read-model';
import { buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoff } from '../../src/app-game-source-gated-policy-preview-timer-service-readiness-read-api-response-consumer-handoff';
import { buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoff } from '../../src/app-game-source-gated-policy-preview-timer-service-readiness-read-api-response-handoff';
import { buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModel } from '../../src/app-game-source-gated-policy-preview-timer-service-readiness-read-model';
import { buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoff } from '../../src/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-handoff';
import { buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoff } from '../../src/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-read-model-handoff';
import { buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoff } from '../../src/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-handoff';
import { buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoff } from '../../src/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-handoff';
import { buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoff } from '../../src/app-game-source-gated-policy-preview-timer-service-readiness-service-handler-handoff';
import { buildAppGameSourceGatedPolicyPreviewTimerStatus } from '../../src/app-game-source-gated-policy-preview-timer-status';
import { buildAppGameSourceFreshnessPreviewGateReadModel } from '../../src/app-game-source-freshness-preview-gate';
import { AppGameSourceFreshnessPolicyConsumptionMatrix } from '../../src/app-game-source-freshness-policy-consumption-data';
import { buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoff } from '../../src/app-game-source-gated-policy-preview-timer-service-readiness-read-api-handoff';
import { buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModel } from '../../src/app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model';
import { buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoff } from '../../src/app-game-timer-service-event-handoff';
import { buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoff } from '../../src/app-game-timer-service-handoff';
import { buildAppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoff } from '../../src/app-game-timer-service-read-api-response-consumer-parent-surface-handoff';
import { buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoff } from '../../src/app-game-timer-service-read-api-handoff';
import { buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseConsumerHandoff } from '../../src/app-game-timer-service-read-api-response-consumer-handoff';
import { buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseHandoff } from '../../src/app-game-timer-service-read-api-response-handoff';
import { buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoff } from '../../src/app-game-timer-service-read-model-handoff';
import {
  PreviewOptions,
  appCompiledDecision,
  gameManualCompiledDecision,
} from './app-game-policy-preview-handoff-fixtures';
import { buildParentSurfaceReadModelHandoffFixture } from './app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-chain.fixtures';

const repoRoot = fileURLToPath(new URL('../../../../', import.meta.url));
const testResultsRoot = join(repoRoot, 'test-results');

const [readyAppSource, readyGameSource, manualGameSource] = AppGameSourceFreshnessPolicyConsumptionMatrix.readiness;

const GateOptions = {
  schemaVersion: PreviewOptions.schemaVersion,
  gateId: 'source-freshness-preview-gate-proof',
  generatedAt: PreviewOptions.generatedAt,
  sourceContractRefs: ['app-game-source-freshness-policy-consumption', 'app-game-policy-preview-handoff'],
  policyPreviewOptions: PreviewOptions,
} as const;

const ReadModelOptions = {
  schemaVersion: PreviewOptions.schemaVersion,
  readModelId: 'source-gated-policy-preview-read-model-proof',
  generatedAt: PreviewOptions.generatedAt,
  sourceContractRefs: ['app-game-source-freshness-preview-gate'],
} as const;

const TimerHandoffOptions = {
  schemaVersion: PreviewOptions.schemaVersion,
  handoffId: 'source-gated-policy-preview-timer-handoff-proof',
  generatedAt: PreviewOptions.generatedAt,
  sourceContractRefs: ['app-game-source-gated-policy-preview-read-model'],
} as const;

const TimerStatusOptions = {
  schemaVersion: PreviewOptions.schemaVersion,
  statusId: 'source-gated-policy-preview-timer-status-proof',
  generatedAt: PreviewOptions.generatedAt,
  sourceContractRefs: ['app-game-source-gated-policy-preview-timer-handoff'],
  timerRuntimeProofRef: 'future-service-timer-runtime-proof',
  sourceFreshnessProofRef: 'source-freshness-proof-required',
  compilerDecisionProofRef: 'compiler-decision-proof-required',
} as const;

const RuntimeReadinessOptions = {
  schemaVersion: PreviewOptions.schemaVersion,
  readinessId: 'source-gated-policy-preview-timer-runtime-readiness-proof',
  generatedAt: PreviewOptions.generatedAt,
  sourceContractRefs: ['app-game-source-gated-policy-preview-timer-status'],
  timerRuntimeProofRef: 'future-service-timer-runtime-proof',
  schedulerPersistenceProofRef: 'future-scheduler-persistence-proof',
  auditProofRef: 'future-timer-audit-proof',
  rollbackProofRef: 'future-timer-rollback-proof',
} as const;

const SchedulerPersistenceOptions = {
  schemaVersion: PreviewOptions.schemaVersion,
  persistenceId: 'source-gated-policy-preview-timer-scheduler-persistence-proof',
  generatedAt: PreviewOptions.generatedAt,
  sourceContractRefs: ['app-game-source-gated-policy-preview-timer-runtime-readiness'],
  serviceTimerRuntimeProofRef: 'future-service-timer-runtime-proof',
  schedulerPersistenceProofRef: 'future-scheduler-persistence-proof',
  schedulerStateStoreProofRef: 'future-scheduler-state-store-proof',
  auditProofRef: 'future-timer-audit-proof',
  rollbackProofRef: 'future-timer-rollback-proof',
} as const;

const AuditRollbackOptions = {
  schemaVersion: PreviewOptions.schemaVersion,
  handoffId: 'source-gated-policy-preview-timer-audit-rollback-handoff-proof',
  generatedAt: PreviewOptions.generatedAt,
  sourceContractRefs: ['app-game-source-gated-policy-preview-timer-scheduler-persistence'],
  serviceTimerRuntimeProofRef: 'future-service-timer-runtime-proof',
  schedulerPersistenceProofRef: 'future-scheduler-persistence-proof',
  schedulerStateStoreProofRef: 'future-scheduler-state-store-proof',
  auditTrailProofRef: 'future-timer-audit-trail-proof',
  rollbackPlanProofRef: 'future-timer-rollback-plan-proof',
  auditRollbackReadModelProofRef: 'future-timer-audit-rollback-read-model-proof',
} as const;

const AuditRollbackReadModelOptions = {
  schemaVersion: PreviewOptions.schemaVersion,
  readModelId: 'source-gated-policy-preview-timer-audit-rollback-read-model-proof',
  generatedAt: PreviewOptions.generatedAt,
  sourceContractRefs: ['app-game-source-gated-policy-preview-timer-audit-rollback-handoff'],
  parentVisibleSummaryRef: 'future-parent-visible-audit-rollback-summary-proof',
} as const;

const ParentSurfaceIntentOptions = {
  schemaVersion: PreviewOptions.schemaVersion,
  intentId: 'source-gated-policy-preview-timer-audit-rollback-parent-surface-intent-proof',
  generatedAt: PreviewOptions.generatedAt,
  sourceContractRefs: ['app-game-source-gated-policy-preview-timer-audit-rollback-read-model'],
  parentSurfaceDrillInRef: 'future-parent-surface-audit-rollback-drill-in-proof',
  parentSurfaceProofRef: 'future-parent-surface-audit-rollback-intent-proof',
} as const;

const ServiceReadinessOptions = {
  schemaVersion: PreviewOptions.schemaVersion,
  handoffId: 'source-gated-policy-preview-timer-service-readiness-handoff-proof',
  generatedAt: PreviewOptions.generatedAt,
  sourceContractRefs: ['app-game-source-gated-policy-preview-timer-audit-rollback-parent-surface-intent'],
  serviceReadinessProofRef: 'future-service-readiness-proof',
  serviceReadApiProofRef: 'future-service-read-api-proof',
  serviceReadApiRef: 'future-service-read-api-contract-ref',
} as const;

const ServiceReadinessReadModelOptions = {
  schemaVersion: PreviewOptions.schemaVersion,
  readModelId: 'source-gated-policy-preview-timer-service-readiness-read-model-proof',
  generatedAt: PreviewOptions.generatedAt,
  sourceContractRefs: ['app-game-source-gated-policy-preview-timer-service-readiness-handoff'],
  serviceReadinessSummaryRef: 'future-service-readiness-read-model-summary-proof',
} as const;

const ProtocolHandoffOptions = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  handoffId: 'source-gated-policy-preview-timer-service-readiness-protocol-handoff-proof',
  generatedAt: '2026-06-06T07:12:00Z',
  sourceContractRefs: [
    'app-game-source-gated-policy-preview-timer-service-readiness-read-model',
    'docs/expectations/app-game-evidence.md',
    'packages/agent-protocol-domain',
    'crates/agent-protocol',
    'crates/agent-service',
  ],
  protocolCommandContractProofRef: 'future-agent-protocol-command-contract-proof',
  protocolEventContractProofRef: 'future-agent-protocol-event-contract-proof',
  rustProtocolMirrorProofRef: 'future-rust-protocol-mirror-proof',
  serviceHandlerProofRef: 'future-service-handler-proof',
} as const;

const ProtocolReadModelOptions = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  readModelId: 'source-gated-policy-preview-timer-service-readiness-protocol-read-model-proof',
  generatedAt: '2026-06-06T07:23:00Z',
  sourceContractRefs: [
    'app-game-source-gated-policy-preview-timer-service-readiness-protocol-handoff',
    'docs/expectations/app-game-evidence.md',
    'packages/agent-protocol-domain',
    'crates/agent-protocol',
    'crates/agent-service',
  ],
  protocolSummaryRef: 'future-service-readiness-protocol-read-model-summary-proof',
} as const;

const ProtocolCommandHandoffOptions = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  commandHandoffId: 'source-gated-policy-preview-timer-service-readiness-protocol-command-handoff-proof',
  generatedAt: '2026-06-06T07:45:00Z',
  sourceContractRefs: [
    'app-game-source-gated-policy-preview-timer-service-readiness-protocol-read-model',
    'docs/expectations/app-game-evidence.md',
    'packages/agent-protocol-domain',
    'crates/agent-protocol',
    'crates/agent-service',
  ],
  protocolCommandRefs: ['agent.activity.app-game.timer-service-readiness.read-model.get'],
  protocolEventRefs: ['agent.activity.app-game.timer-service-readiness.read-model.reported'],
  serviceHandlerRefs: ['future-app-game-timer-service-readiness-command-handler-proof'],
  commandSummaryRef: 'future-service-readiness-protocol-command-handoff-summary-proof',
} as const;

const ServiceHandlerHandoffOptions = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  serviceHandlerHandoffId: 'source-gated-policy-preview-timer-service-readiness-service-handler-handoff-proof',
  generatedAt: '2026-06-06T07:58:00Z',
  sourceContractRefs: [
    'app-game-source-gated-policy-preview-timer-service-readiness-protocol-command-handoff',
    'docs/expectations/app-game-evidence.md',
    'crates/agent-service',
  ],
  serviceReadApiProofRefs: ['future-app-game-timer-service-readiness-read-api-proof'],
  serviceHandlerSummaryRef: 'future-service-readiness-service-handler-handoff-summary-proof',
} as const;

const ServiceReadApiHandoffOptions = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  serviceReadApiHandoffId: 'source-gated-policy-preview-timer-service-readiness-read-api-handoff-proof',
  generatedAt: '2026-06-06T07:58:00Z',
  sourceContractRefs: [
    'app-game-source-gated-policy-preview-timer-service-readiness-service-handler-handoff',
    'docs/expectations/app-game-evidence.md',
    'crates/agent-service',
  ],
  serviceReadApiProofRefs: ['future-app-game-timer-service-readiness-read-api-proof'],
  serviceReadApiSummaryRef: 'future-service-readiness-read-api-handoff-summary-proof',
} as const;

const ReadApiResponseHandoffOptions = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  readApiResponseHandoffId: 'source-gated-policy-preview-timer-service-readiness-read-api-response-handoff-proof',
  generatedAt: '2026-06-06T08:36:00Z',
  sourceContractRefs: [
    'app-game-source-gated-policy-preview-timer-service-readiness-read-api-handoff',
    'docs/expectations/app-game-evidence.md',
    'crates/agent-service',
  ],
  readApiResponseProofRefs: ['future-app-game-timer-service-readiness-read-api-response-proof'],
  readApiResponseSummaryRef: 'future-service-readiness-read-api-response-handoff-summary-proof',
} as const;

const ResponseConsumerHandoffOptions = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  readApiResponseConsumerHandoffId:
    'source-gated-policy-preview-timer-service-readiness-read-api-response-consumer-handoff-proof',
  generatedAt: '2026-06-06T08:56:00Z',
  sourceContractRefs: [
    'app-game-source-gated-policy-preview-timer-service-readiness-read-api-response-handoff',
    'docs/expectations/app-game-evidence.md',
    'docs/expectations/enforcement.md',
  ],
  readApiResponseConsumerProofRefs: ['future-app-game-timer-service-readiness-read-api-response-consumer-proof'],
  readApiResponseConsumerSummaryRef: 'future-service-readiness-read-api-response-consumer-handoff-summary-proof',
} as const;

const ResponseConsumerParentSurfaceHandoffOptions = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  responseConsumerParentSurfaceHandoffId:
    'source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-handoff-proof',
  generatedAt: '2026-06-06T08:56:00Z',
  sourceContractRefs: [
    'app-game-source-gated-policy-preview-timer-service-readiness-read-api-response-consumer-handoff',
    'docs/expectations/app-game-evidence.md',
    'docs/expectations/enforcement.md',
  ],
  parentSurfaceProofRefs: ['future-app-game-timer-service-readiness-response-consumer-parent-surface-proof'],
  parentSurfaceSummaryRef: 'future-service-readiness-response-consumer-parent-surface-handoff-summary-proof',
} as const;

const ResponseConsumerParentSurfaceReadModelHandoffOptions = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  responseConsumerParentSurfaceReadModelHandoffId:
    'source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-read-model-handoff-proof',
  generatedAt: '2026-06-06T09:45:00Z',
  sourceContractRefs: [
    'app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-handoff',
    'docs/expectations/app-game-evidence.md',
    'docs/expectations/enforcement.md',
  ],
  parentSurfaceReadModelProofRefs: [
    'future-app-game-timer-service-readiness-response-consumer-parent-surface-read-model-proof',
  ],
  parentSurfaceReadModelRef: 'future-service-readiness-response-consumer-parent-surface-read-model-proof',
} as const;

const ResponseConsumerParentSurfaceStatusHandoffOptions = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  responseConsumerParentSurfaceStatusHandoffId:
    'source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-handoff-proof',
  generatedAt: '2026-06-06T10:05:00Z',
  sourceContractRefs: [
    'app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-read-model-handoff',
    'docs/expectations/app-game-evidence.md',
    'docs/expectations/enforcement.md',
  ],
  parentSurfaceStatusProofRefs: [
    'future-app-game-timer-service-readiness-response-consumer-parent-surface-status-proof',
  ],
  parentSurfaceStatusRef: 'future-service-readiness-response-consumer-parent-surface-status-proof',
} as const;

const ResponseConsumerParentSurfaceStatusReadModelHandoffOptions = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  responseConsumerParentSurfaceStatusReadModelHandoffId:
    'source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-handoff-proof',
  generatedAt: '2026-06-06T10:30:00Z',
  sourceContractRefs: [
    'app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-handoff',
    'docs/expectations/app-game-evidence.md',
    'docs/expectations/enforcement.md',
  ],
  parentSurfaceStatusReadModelProofRefs: [
    'future-app-game-timer-service-readiness-response-consumer-parent-surface-status-read-model-proof',
  ],
  parentSurfaceStatusReadModelRef: 'future-service-readiness-response-consumer-parent-surface-status-read-model-proof',
} as const;

const ParentSurfaceReadModelOptions = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  parentSurfaceReadModelId:
    'source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-proof',
  generatedAt: '2026-06-06T11:30:00Z',
  sourceContractRefs: [
    'app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-handoff',
    'docs/expectations/app-game-evidence.md',
    'docs/expectations/enforcement.md',
  ],
} as const;

const ServiceHandoffOptions = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  parentSurfaceReadModelServiceHandoffId:
    'source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-service-handoff-proof',
  generatedAt: '2026-06-06T11:45:00Z',
  sourceContractRefs: [
    'app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model',
    'docs/expectations/app-game-evidence.md',
    'docs/expectations/enforcement.md',
  ],
  serviceProofRefs: [
    'future-app-game-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-service-proof',
  ],
} as const;

const ServiceReadModelHandoffOptions = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  parentSurfaceReadModelServiceReadModelHandoffId:
    'source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-service-read-model-handoff-proof',
  generatedAt: '2026-06-06T12:00:00Z',
  sourceContractRefs: [
    'app-game-timer-service-handoff',
    'docs/expectations/app-game-evidence.md',
    'docs/expectations/enforcement.md',
  ],
  serviceReadModelProofRefs: [
    'future-app-game-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-service-read-model-proof',
  ],
} as const;

const ServiceEventHandoffOptions = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  parentSurfaceReadModelServiceEventHandoffId: 'app-game-timer-service-event-handoff-proof',
  generatedAt: '2026-06-06T12:10:00Z',
  sourceContractRefs: [
    'app-game-timer-service-read-model-handoff',
    'docs/expectations/app-game-evidence.md',
    'docs/expectations/enforcement.md',
  ],
  serviceEventProofRefs: [
    'future-app-game-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-service-event-proof',
  ],
} as const;

const ServiceReadApiHandoffOptions2 = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  parentSurfaceReadModelServiceReadApiHandoffId: 'app-game-timer-service-read-api-handoff-proof',
  generatedAt: '2026-06-06T12:20:00Z',
  sourceContractRefs: [
    'app-game-timer-service-event-handoff',
    'docs/expectations/app-game-evidence.md',
    'docs/expectations/enforcement.md',
  ],
  serviceReadApiProofRefs: [
    'future-app-game-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-service-read-api-proof',
  ],
} as const;

const ServiceReadApiResponseHandoffOptions2 = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  parentSurfaceReadModelServiceReadApiResponseHandoffId: 'app-game-timer-service-read-api-response-handoff-proof',
  generatedAt: '2026-06-06T14:25:00Z',
  sourceContractRefs: [
    'app-game-timer-service-read-api-handoff',
    'docs/expectations/app-game-evidence.md',
    'docs/expectations/enforcement.md',
  ],
  serviceReadApiResponseProofRefs: [
    'future-app-game-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-service-read-api-response-proof',
  ],
} as const;

const ServiceReadApiResponseConsumerHandoffOptions2 = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  parentSurfaceReadModelServiceReadApiResponseConsumerHandoffId:
    'app-game-timer-service-read-api-response-consumer-handoff-proof',
  generatedAt: '2026-06-06T14:35:00Z',
  sourceContractRefs: [
    'app-game-timer-service-read-api-response-handoff',
    'docs/expectations/app-game-evidence.md',
    'docs/expectations/enforcement.md',
  ],
  serviceReadApiResponseConsumerProofRefs: [
    'future-app-game-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-service-read-api-response-consumer-proof',
  ],
} as const;

const TimerParentSurfaceHandoffOptions = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  parentSurfaceHandoffId: 'app-game-timer-service-read-api-response-consumer-parent-surface-handoff-proof',
  generatedAt: '2026-06-07T13:55:00Z',
  sourceContractRefs: [
    'app-game-timer-service-read-api-response-consumer-handoff',
    'docs/expectations/app-game-evidence.md',
    'docs/expectations/enforcement.md',
  ],
  parentSurfaceProofRefs: ['future-app-game-timer-service-read-api-response-consumer-parent-surface-proof'],
} as const;

export default async function generateTimerServiceProofArtifacts() {
  await ensureTimerServiceProofArtifacts();
}

async function ensureTimerServiceProofArtifacts() {
  const protocolArtifacts = buildProtocolArtifacts(buildSourceReadinessArtifacts());
  const followthroughArtifacts = buildFollowthroughArtifacts();
  await writeTimerServiceArtifacts(protocolArtifacts, followthroughArtifacts);
}

function buildSourceReadinessArtifacts() {
  const gateReadModel = buildAppGameSourceFreshnessPreviewGateReadModel(GateOptions, [
    {
      rowId: 'source-gate-row-ready-app',
      sourceReadiness: readyAppSource,
      compiledDecision: appCompiledDecision,
    },
    {
      rowId: 'source-gate-row-manual-game',
      sourceReadiness: manualGameSource,
      compiledDecision: null,
    },
    {
      rowId: 'source-gate-row-compiler-manual-game',
      sourceReadiness: readyGameSource,
      compiledDecision: gameManualCompiledDecision,
    },
  ]);
  const sourceGatedReadModel = buildAppGameSourceGatedPolicyPreviewReadModel(ReadModelOptions, gateReadModel);
  const timerHandoff = buildAppGameSourceGatedPolicyPreviewTimerHandoff(TimerHandoffOptions, sourceGatedReadModel);
  const timerStatus = buildAppGameSourceGatedPolicyPreviewTimerStatus(TimerStatusOptions, timerHandoff);
  const runtimeReadiness = buildAppGameSourceGatedPolicyPreviewTimerRuntimeReadiness(
    RuntimeReadinessOptions,
    timerStatus
  );
  const schedulerPersistence = buildAppGameSourceGatedPolicyPreviewTimerSchedulerPersistence(
    SchedulerPersistenceOptions,
    runtimeReadiness
  );
  const auditRollbackHandoff = buildAppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoff(
    AuditRollbackOptions,
    schedulerPersistence
  );
  const auditRollbackReadModel = buildAppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModel(
    AuditRollbackReadModelOptions,
    auditRollbackHandoff
  );
  const parentSurfaceIntent = buildAppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntent(
    ParentSurfaceIntentOptions,
    auditRollbackReadModel
  );
  const serviceReadinessHandoff = buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoff(
    ServiceReadinessOptions,
    parentSurfaceIntent
  );
  const serviceReadinessReadModel = buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModel(
    ServiceReadinessReadModelOptions,
    serviceReadinessHandoff
  );
  return { serviceReadinessReadModel };
}

function buildProtocolArtifacts(sourceArtifacts: ReturnType<typeof buildSourceReadinessArtifacts>) {
  const protocolHandoff = buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoff(
    ProtocolHandoffOptions,
    sourceArtifacts.serviceReadinessReadModel
  );
  const protocolReadModel = buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModel(
    ProtocolReadModelOptions,
    protocolHandoff
  );
  const protocolCommandHandoff = buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoff(
    ProtocolCommandHandoffOptions,
    protocolReadModel
  );
  const serviceHandlerHandoff = buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoff(
    ServiceHandlerHandoffOptions,
    protocolCommandHandoff
  );
  const serviceReadApiHandoff = buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoff(
    ServiceReadApiHandoffOptions,
    serviceHandlerHandoff
  );
  const readApiResponseHandoff = buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoff(
    ReadApiResponseHandoffOptions,
    serviceReadApiHandoff
  );
  const readApiResponseConsumerHandoff =
    buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoff(
      ResponseConsumerHandoffOptions,
      readApiResponseHandoff
    );
  const responseConsumerParentSurfaceHandoff =
    buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoff(
      ResponseConsumerParentSurfaceHandoffOptions,
      readApiResponseConsumerHandoff
    );
  const responseConsumerParentSurfaceReadModelHandoff =
    buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoff(
      ResponseConsumerParentSurfaceReadModelHandoffOptions,
      responseConsumerParentSurfaceHandoff
    );
  const responseConsumerParentSurfaceStatusHandoff =
    buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoff(
      ResponseConsumerParentSurfaceStatusHandoffOptions,
      responseConsumerParentSurfaceReadModelHandoff
    );
  const responseConsumerParentSurfaceStatusReadModelHandoff =
    buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoff(
      ResponseConsumerParentSurfaceStatusReadModelHandoffOptions,
      responseConsumerParentSurfaceStatusHandoff
    );
  return {
    ...sourceArtifacts,
    protocolHandoff,
    protocolReadModel,
    protocolCommandHandoff,
    serviceHandlerHandoff,
    serviceReadApiHandoff,
    readApiResponseHandoff,
    readApiResponseConsumerHandoff,
    responseConsumerParentSurfaceHandoff,
    responseConsumerParentSurfaceReadModelHandoff,
    responseConsumerParentSurfaceStatusHandoff,
    responseConsumerParentSurfaceStatusReadModelHandoff,
  };
}

function buildFollowthroughArtifacts() {
  const parentSurfaceReadModel =
    buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModel(
      ParentSurfaceReadModelOptions,
      buildParentSurfaceReadModelHandoffFixture()
    );
  const serviceHandoff =
    buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceHandoff(
      ServiceHandoffOptions,
      parentSurfaceReadModel
    );
  const serviceReadModelHandoff =
    buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadModelHandoff(
      ServiceReadModelHandoffOptions,
      serviceHandoff
    );
  const serviceEventHandoff =
    buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceEventHandoff(
      ServiceEventHandoffOptions,
      serviceReadModelHandoff
    );
  const serviceReadApiHandoff2 =
    buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiHandoff(
      ServiceReadApiHandoffOptions2,
      serviceEventHandoff
    );
  const serviceReadApiResponseHandoff2 =
    buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseHandoff(
      ServiceReadApiResponseHandoffOptions2,
      serviceReadApiHandoff2
    );
  const serviceReadApiResponseConsumerHandoff2 =
    buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelServiceReadApiResponseConsumerHandoff(
      ServiceReadApiResponseConsumerHandoffOptions2,
      serviceReadApiResponseHandoff2
    );
  const timerParentSurfaceHandoff = buildAppGameTimerServiceReadApiResponseConsumerParentSurfaceHandoff(
    TimerParentSurfaceHandoffOptions,
    serviceReadApiResponseConsumerHandoff2
  );
  return {
    parentSurfaceReadModel,
    serviceHandoff,
    serviceReadModelHandoff,
    serviceEventHandoff,
    serviceReadApiHandoff2,
    serviceReadApiResponseHandoff2,
    serviceReadApiResponseConsumerHandoff2,
    timerParentSurfaceHandoff,
  };
}

async function writeTimerServiceArtifacts(
  protocolArtifacts: ReturnType<typeof buildProtocolArtifacts>,
  followthroughArtifacts: ReturnType<typeof buildFollowthroughArtifacts>
) {
  await Promise.all([
    ...buildProtocolArtifactWrites(protocolArtifacts),
    ...buildFollowthroughArtifactWrites(protocolArtifacts, followthroughArtifacts),
  ]);
}

function buildProtocolArtifactWrites(protocolArtifacts: ReturnType<typeof buildProtocolArtifacts>) {
  return [
    writeArtifact(
      'app-game-source-gated-policy-preview-timer-service-readiness-read-model-proof',
      'timer-service-readiness-read-model.json',
      protocolArtifacts.serviceReadinessReadModel
    ),
    writeArtifact(
      'app-game-source-gated-policy-preview-timer-service-readiness-protocol-handoff-proof',
      'timer-service-readiness-protocol-handoff.json',
      protocolArtifacts.protocolHandoff
    ),
    writeArtifact(
      'app-game-source-gated-policy-preview-timer-service-readiness-protocol-read-model-proof',
      'timer-service-readiness-protocol-read-model.json',
      protocolArtifacts.protocolReadModel
    ),
    writeArtifact(
      'app-game-source-gated-policy-preview-timer-service-readiness-protocol-command-handoff-proof',
      'timer-service-readiness-protocol-command-handoff.json',
      protocolArtifacts.protocolCommandHandoff
    ),
    writeArtifact(
      'app-game-source-gated-policy-preview-timer-service-readiness-service-handler-handoff-proof',
      'timer-service-readiness-service-handler-handoff.json',
      protocolArtifacts.serviceHandlerHandoff
    ),
    writeArtifact(
      'app-game-source-gated-policy-preview-timer-service-readiness-read-api-handoff-proof',
      'timer-service-readiness-read-api-handoff.json',
      protocolArtifacts.serviceReadApiHandoff
    ),
    writeArtifact(
      'app-game-source-gated-policy-preview-timer-service-readiness-read-api-response-handoff-proof',
      'timer-service-readiness-read-api-response-handoff.json',
      protocolArtifacts.readApiResponseHandoff
    ),
    writeArtifact(
      'app-game-source-gated-policy-preview-timer-service-readiness-read-api-response-consumer-handoff-proof',
      'timer-service-readiness-read-api-response-consumer-handoff.json',
      protocolArtifacts.readApiResponseConsumerHandoff
    ),
    writeArtifact(
      'app-game-timer-parent-surface-proof',
      'handoff.json',
      protocolArtifacts.responseConsumerParentSurfaceHandoff
    ),
    writeArtifact(
      'app-game-timer-parent-rm-proof',
      'handoff.json',
      protocolArtifacts.responseConsumerParentSurfaceReadModelHandoff
    ),
    writeArtifact(
      'app-game-timer-parent-status-proof',
      'handoff.json',
      protocolArtifacts.responseConsumerParentSurfaceStatusHandoff
    ),
  ];
}

function buildFollowthroughArtifactWrites(
  protocolArtifacts: ReturnType<typeof buildProtocolArtifacts>,
  followthroughArtifacts: ReturnType<typeof buildFollowthroughArtifacts>
) {
  return [
    writeArtifact(
      'app-game-timer-parent-read-model-proof',
      'handoff.json',
      followthroughArtifacts.parentSurfaceReadModel
    ),
    writeArtifact('app-game-timer-service-handoff-proof', 'handoff.json', followthroughArtifacts.serviceHandoff),
    writeArtifact(
      'app-game-timer-service-read-model-handoff-proof',
      'handoff.json',
      followthroughArtifacts.serviceReadModelHandoff
    ),
    writeArtifact(
      'app-game-timer-service-event-handoff-proof',
      'handoff.json',
      followthroughArtifacts.serviceEventHandoff
    ),
    writeArtifact(
      'app-game-timer-service-read-api-handoff-proof',
      'handoff.json',
      followthroughArtifacts.serviceReadApiHandoff2
    ),
    writeArtifact(
      'app-game-timer-service-read-api-response-handoff-proof',
      'handoff.json',
      followthroughArtifacts.serviceReadApiResponseHandoff2
    ),
    writeArtifact(
      'app-game-timer-service-read-api-response-consumer-handoff-proof',
      'handoff.json',
      followthroughArtifacts.serviceReadApiResponseConsumerHandoff2
    ),
    writeArtifact(
      'app-game-timer-service-read-api-response-consumer-parent-surface-handoff-proof',
      'handoff.json',
      followthroughArtifacts.timerParentSurfaceHandoff
    ),
    writeArtifact(
      'app-game-timer-parent-status-rm-proof',
      'handoff.json',
      protocolArtifacts.responseConsumerParentSurfaceStatusReadModelHandoff
    ),
  ];
}

async function writeArtifact(directoryName: string, fileName: string, value: unknown) {
  const outputDir = join(testResultsRoot, directoryName);
  await mkdir(outputDir, { recursive: true });
  await writeFile(join(outputDir, fileName), `${JSON.stringify(value, null, 2)}\n`);
}
