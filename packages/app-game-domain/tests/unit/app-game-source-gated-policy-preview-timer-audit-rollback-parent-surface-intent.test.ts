import { describe, expect, it } from 'vitest';
import { buildAppGameSourceGatedPolicyPreviewReadModel } from '../../src/app-game-source-gated-policy-preview-read-model';
import { buildAppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoff } from '../../src/app-game-source-gated-policy-preview-timer-audit-rollback-handoff';
import { buildAppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntent } from '../../src/app-game-source-gated-policy-preview-timer-audit-rollback-parent-surface-intent';
import { AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentSchema } from '@ocentra-parent/schema-domain/app-game-source-gated-policy-preview-timer-audit-rollback-parent-surface-intent';
import { AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentState } from '@ocentra-parent/schema-domain/app-game-source-gated-policy-preview-timer-audit-rollback-parent-surface-intent-rules';
import { buildAppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModel } from '../../src/app-game-source-gated-policy-preview-timer-audit-rollback-read-model';
import { buildAppGameSourceGatedPolicyPreviewTimerHandoff } from '../../src/app-game-source-gated-policy-preview-timer-handoff';
import { buildAppGameSourceGatedPolicyPreviewTimerRuntimeReadiness } from '../../src/app-game-source-gated-policy-preview-timer-runtime-readiness';
import { buildAppGameSourceGatedPolicyPreviewTimerSchedulerPersistence } from '../../src/app-game-source-gated-policy-preview-timer-scheduler-persistence';
import { buildAppGameSourceGatedPolicyPreviewTimerStatus } from '../../src/app-game-source-gated-policy-preview-timer-status';
import { buildAppGameSourceFreshnessPreviewGateReadModel } from '../../src/app-game-source-freshness-preview-gate';
import { AppGameSourceFreshnessPolicyConsumptionMatrix } from '../../src/app-game-source-freshness-policy-consumption-data';
import {
  PreviewOptions,
  appCompiledDecision,
  gameManualCompiledDecision,
} from './app-game-policy-preview-handoff-fixtures';

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
  sourceContractRefs: [
    'app-game-source-gated-policy-preview-timer-handoff',
    'docs/expectations/app-game-evidence.md',
    'docs/expectations/policy.md',
  ],
  timerRuntimeProofRef: 'future-service-timer-runtime-proof',
  sourceFreshnessProofRef: 'source-freshness-proof-required',
  compilerDecisionProofRef: 'compiler-decision-proof-required',
} as const;

const RuntimeReadinessOptions = {
  schemaVersion: PreviewOptions.schemaVersion,
  readinessId: 'source-gated-policy-preview-timer-runtime-readiness-proof',
  generatedAt: PreviewOptions.generatedAt,
  sourceContractRefs: [
    'app-game-source-gated-policy-preview-timer-status',
    'docs/expectations/app-game-evidence.md',
    'docs/expectations/policy.md',
  ],
  timerRuntimeProofRef: 'future-service-timer-runtime-proof',
  schedulerPersistenceProofRef: 'future-scheduler-persistence-proof',
  auditProofRef: 'future-timer-audit-proof',
  rollbackProofRef: 'future-timer-rollback-proof',
} as const;

const SchedulerPersistenceOptions = {
  schemaVersion: PreviewOptions.schemaVersion,
  persistenceId: 'source-gated-policy-preview-timer-scheduler-persistence-proof',
  generatedAt: PreviewOptions.generatedAt,
  sourceContractRefs: [
    'app-game-source-gated-policy-preview-timer-runtime-readiness',
    'docs/expectations/app-game-evidence.md',
    'docs/expectations/policy.md',
  ],
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
  sourceContractRefs: [
    'app-game-source-gated-policy-preview-timer-scheduler-persistence',
    'docs/expectations/app-game-evidence.md',
    'docs/expectations/policy.md',
  ],
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
  sourceContractRefs: [
    'app-game-source-gated-policy-preview-timer-audit-rollback-handoff',
    'docs/expectations/app-game-evidence.md',
    'docs/expectations/policy.md',
  ],
  parentVisibleSummaryRef: 'future-parent-visible-audit-rollback-summary-proof',
} as const;

const ParentSurfaceIntentOptions = {
  schemaVersion: PreviewOptions.schemaVersion,
  intentId: 'source-gated-policy-preview-timer-audit-rollback-parent-surface-intent-proof',
  generatedAt: PreviewOptions.generatedAt,
  sourceContractRefs: [
    'app-game-source-gated-policy-preview-timer-audit-rollback-read-model',
    'docs/expectations/app-game-evidence.md',
    'docs/expectations/policy.md',
  ],
  parentSurfaceDrillInRef: 'future-parent-surface-audit-rollback-drill-in-proof',
  parentSurfaceProofRef: 'future-parent-surface-audit-rollback-intent-proof',
} as const;

describe('app/game source-gated policy preview timer audit rollback parent-surface intent', () => {
  it('projects audit rollback read-model rows into future parent-surface intent states', () => {
    const intent = buildParentSurfaceIntent();

    expect(intent.auditRollbackParentSurfaceProofRequiredCount).toBe(1);
    expect(intent.blockedBySourceFreshnessCount).toBe(1);
    expect(intent.blockedByCompilerDecisionCount).toBe(1);
    expect(intent.rows.map((row) => row.parentSurfaceIntentState)).toEqual([
      AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentState.AuditRollbackParentSurfaceProofRequired,
      AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentState.BlockedBySourceFreshness,
      AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentState.BlockedByCompilerDecision,
    ]);
  });

  it('adds parent-surface proof and drill-in refs only to eligible rows', () => {
    const intent = buildParentSurfaceIntent();

    expect(intent.rows[0]?.requiredProofRefs).toEqual([
      'future-service-timer-runtime-proof',
      'future-scheduler-persistence-proof',
      'future-scheduler-state-store-proof',
      'future-timer-audit-trail-proof',
      'future-timer-rollback-plan-proof',
      'future-timer-audit-rollback-read-model-proof',
      'future-parent-surface-audit-rollback-intent-proof',
    ]);
    expect(intent.rows[0]?.parentSurfaceProofRequired).toBe(true);
    expect(intent.rows[1]?.parentSurfaceProofRequired).toBe(false);
    expect(intent.rows[0]?.parentSurfaceDrillInRef).toBe('future-parent-surface-audit-rollback-drill-in-proof');
    expect(intent.rows[0]?.timerScheduled).toBe(false);
  });

  it('rejects rendered UI, durable audit logs, rollback execution, and count drift', () => {
    const intent = buildParentSurfaceIntent();

    expect(
      AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentSchema.safeParse({
        ...intent,
        portalUiRendered: true,
      }).success
    ).toBe(false);
    expect(
      AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentSchema.safeParse({
        ...intent,
        durableAuditLogClaimed: true,
      }).success
    ).toBe(false);
    expect(
      AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentSchema.safeParse({
        ...intent,
        rollbackExecutionClaimed: true,
      }).success
    ).toBe(false);
    expect(
      AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentSchema.safeParse({
        ...intent,
        auditRollbackParentSurfaceProofRequiredCount: 0,
      }).success
    ).toBe(false);
  });
});

function buildParentSurfaceIntent() {
  return buildAppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntent(
    ParentSurfaceIntentOptions,
    buildAuditRollbackReadModel()
  );
}

function buildAuditRollbackReadModel() {
  return buildAppGameSourceGatedPolicyPreviewTimerAuditRollbackReadModel(
    AuditRollbackReadModelOptions,
    buildAuditRollbackHandoff()
  );
}

function buildAuditRollbackHandoff() {
  return buildAppGameSourceGatedPolicyPreviewTimerAuditRollbackHandoff(
    AuditRollbackOptions,
    buildSchedulerPersistence()
  );
}

function buildSchedulerPersistence() {
  return buildAppGameSourceGatedPolicyPreviewTimerSchedulerPersistence(
    SchedulerPersistenceOptions,
    buildRuntimeReadiness()
  );
}

function buildRuntimeReadiness() {
  return buildAppGameSourceGatedPolicyPreviewTimerRuntimeReadiness(RuntimeReadinessOptions, buildStatus());
}

function buildStatus() {
  return buildAppGameSourceGatedPolicyPreviewTimerStatus(TimerStatusOptions, buildTimerHandoff());
}

function buildTimerHandoff() {
  return buildAppGameSourceGatedPolicyPreviewTimerHandoff(TimerHandoffOptions, buildReadModel());
}

function buildReadModel() {
  return buildAppGameSourceGatedPolicyPreviewReadModel(ReadModelOptions, buildGateReadModel());
}

function buildGateReadModel() {
  return buildAppSourceFreshnessPreviewGateReadModel([
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
}

function buildAppSourceFreshnessPreviewGateReadModel(
  rows: Parameters<typeof buildAppGameSourceFreshnessPreviewGateReadModel>[1]
) {
  return buildAppGameSourceFreshnessPreviewGateReadModel(GateOptions, rows);
}
