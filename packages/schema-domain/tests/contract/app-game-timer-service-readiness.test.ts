import { describe, expect, it } from 'vitest';
import {
  buildAppGameSourceGatedPolicyPreviewTimerHandoff,
  AppGameSourceGatedPolicyPreviewTimerHandoffState,
} from '../../src/app-game-source-gated-policy-preview-timer-handoff';
import {
  buildAppGameSourceGatedPolicyPreviewTimerStatus,
  AppGameSourceGatedPolicyPreviewTimerStatusState,
} from '../../src/app-game-source-gated-policy-preview-timer-status';
import {
  buildAppGameSourceGatedPolicyPreviewTimerRuntimeReadiness,
  AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessState,
} from '../../src/app-game-source-gated-policy-preview-timer-runtime-readiness';
import {
  buildAppGameSourceGatedPolicyPreviewTimerSchedulerPersistence,
  AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceState,
} from '../../src/app-game-source-gated-policy-preview-timer-scheduler-persistence';
import {
  buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModel,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelState,
} from '../../src/app-game-source-gated-policy-preview-timer-service-readiness-read-model';
import { buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoff } from '../../src/app-game-source-gated-policy-preview-timer-service-readiness-protocol-handoff';
import {
  buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModel,
  AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelState,
} from '../../src/app-game-source-gated-policy-preview-timer-service-readiness-protocol-read-model';
import {
  serviceReadinessHandoff,
  serviceReadinessProtocolHandoffMetadata,
  serviceReadinessProtocolReadModelMetadata,
  serviceReadinessReadModelMetadata,
  timerHandoffMetadata,
  timerRuntimeReadinessMetadata,
  timerSchedulerPersistenceMetadata,
  timerSourceGatedReadModel,
  timerStatusMetadata,
} from './app-game-timer-service-readiness.fixtures';

describe('schema-domain app-game timer/service-readiness contracts', () => {
  buildsTimerHandoffThroughSchedulerPersistenceFromRustGeneratedRules();
  buildsServiceReadinessAndProtocolReadModelsThroughRustGeneratedRules();
});

function buildsTimerHandoffThroughSchedulerPersistenceFromRustGeneratedRules(): void {
  it('builds timer handoff through scheduler persistence from the Rust-generated rules', () => {
    const timerHandoff = buildAppGameSourceGatedPolicyPreviewTimerHandoff(
      timerHandoffMetadata,
      timerSourceGatedReadModel
    );

    expect(timerHandoff.timerSequenceCandidateCount).toBe(1);
    expect(timerHandoff.sourceManualBlockedCount).toBe(1);
    expect(timerHandoff.rows[0]?.timerHandoffState).toBe(
      AppGameSourceGatedPolicyPreviewTimerHandoffState.ReadyForTimerSequencing
    );
    expect(timerHandoff.rows[1]?.timerHandoffState).toBe(
      AppGameSourceGatedPolicyPreviewTimerHandoffState.SourceManualRequiredBeforeTimer
    );

    const timerStatus = buildAppGameSourceGatedPolicyPreviewTimerStatus(
      timerStatusMetadata,
      timerHandoff
    );

    expect(timerStatus.timerRuntimeProofRequiredCount).toBe(1);
    expect(timerStatus.sourceFreshnessProofRequiredCount).toBe(1);
    expect(timerStatus.rows[0]?.timerStatusState).toBe(
      AppGameSourceGatedPolicyPreviewTimerStatusState.TimerRuntimeProofRequired
    );
    expect(timerStatus.rows[0]?.requiredProofRefs).toEqual(['proof-timer-runtime']);
    expect(timerStatus.rows[1]?.requiredProofRefs).toEqual(['proof-source-freshness']);

    const runtimeReadiness = buildAppGameSourceGatedPolicyPreviewTimerRuntimeReadiness(
      timerRuntimeReadinessMetadata,
      timerStatus
    );

    expect(runtimeReadiness.runtimeProofRequiredCount).toBe(1);
    expect(runtimeReadiness.rows[0]?.runtimeReadinessState).toBe(
      AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessState.RuntimeProofRequired
    );
    expect(runtimeReadiness.rows[0]?.requiredProofRefs).toEqual([
      'proof-timer-runtime',
      'proof-scheduler-persistence',
      'proof-audit',
      'proof-rollback',
    ]);
    expect(runtimeReadiness.rows[1]?.requiredProofRefs).toEqual(['proof-source-freshness']);

    const schedulerPersistence = buildAppGameSourceGatedPolicyPreviewTimerSchedulerPersistence(
      timerSchedulerPersistenceMetadata,
      runtimeReadiness
    );

    expect(schedulerPersistence.schedulerPersistenceProofRequiredCount).toBe(1);
    expect(schedulerPersistence.rows[0]?.schedulerPersistenceState).toBe(
      AppGameSourceGatedPolicyPreviewTimerSchedulerPersistenceState.SchedulerPersistenceProofRequired
    );
    expect(schedulerPersistence.rows[0]?.requiredProofRefs).toEqual([
      'proof-service-timer-runtime',
      'proof-scheduler-persistence',
      'proof-scheduler-store',
      'proof-audit',
      'proof-rollback',
    ]);
    expect(schedulerPersistence.rows[1]?.requiredProofRefs).toEqual(['proof-source-freshness']);
  });
}

function buildsServiceReadinessAndProtocolReadModelsThroughRustGeneratedRules(): void {
  it('builds service-readiness and protocol read models through the Rust-generated rules', () => {
    const serviceReadinessReadModel = buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModel(
      serviceReadinessReadModelMetadata,
      serviceReadinessHandoff
    );

    expect(serviceReadinessReadModel.serviceReadModelProofRequiredCount).toBe(1);
    expect(serviceReadinessReadModel.rows[0]?.serviceReadinessReadModelState).toBe(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelState.ServiceReadModelProofRequired
    );
    expect(serviceReadinessReadModel.rows[1]?.serviceReadinessReadModelState).toBe(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelState.BlockedBySourceFreshness
    );

    const protocolHandoff = buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoff(
      serviceReadinessProtocolHandoffMetadata,
      serviceReadinessReadModel
    );

    const protocolReadModel = buildAppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModel(
      serviceReadinessProtocolReadModelMetadata,
      protocolHandoff
    );

    expect(protocolReadModel.protocolReadModelProofRequiredCount).toBe(1);
    expect(protocolReadModel.rows[0]?.protocolReadModelState).toBe(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelState.ProtocolReadModelProofRequired
    );
    expect(protocolReadModel.rows[0]?.requiredProtocolProofRefs).toEqual([
      'proof-protocol-command',
      'proof-protocol-event',
      'proof-rust-protocol-mirror',
      'proof-service-handler',
    ]);
    expect(protocolReadModel.rows[1]?.protocolReadModelState).toBe(
      AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelState.BlockedBySourceFreshness
    );
  });
}
