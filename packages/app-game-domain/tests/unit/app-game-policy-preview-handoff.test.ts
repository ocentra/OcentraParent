import { describe, expect, it } from 'vitest';
import {
  AppGamePolicyPreviewHandoffReadModelSchema,
  AppGamePolicyPreviewHandoffRowSchema,
  AppGamePolicyPreviewStatus,
  AppGamePolicyPreviewTargetDomain,
  buildAppGamePolicyPreviewHandoffReadModel,
  buildAppGamePolicyPreviewHandoffRow,
} from '../../src/app-game-policy-preview-handoff';
import { AppGamePolicyCompilerRejectionReason } from '../../src/app-game-policy-target-compiler-rules';
import { PolicyAction, PolicyDecisionHandoffState } from '@ocentra-parent/policy-domain/policy';
import {
  EvidenceReference,
  PreviewOptions,
  RuleId,
  appCompiledDecision,
  gameManualCompiledDecision,
} from './app-game-policy-preview-handoff-fixtures';

const assertBuildsNativeAppPreviewRows = () => {
  const row = buildAppGamePolicyPreviewHandoffRow(PreviewOptions, appCompiledDecision);

  expect(row.targetDomain).toBe(AppGamePolicyPreviewTargetDomain.NativeApp);
  expect(row.previewStatus).toBe(AppGamePolicyPreviewStatus.PreviewReady);
  expect(row.policyAction).toBe(PolicyAction.TimeLimit);
  expect(row.sourceCompiledDecisionId).toBe('compiled-decision-preview-app');
  expect(row.evidenceReferences).toEqual([EvidenceReference]);
  expect(row.ruleRefs).toEqual([RuleId]);
  expect(row.capabilityRefs).toEqual(['capability-preview-1']);
  expect(row.auditRefs).toEqual(['audit-preview-1']);
  expect(row.dryRun).toBe(true);
  expect(row.enforcementHandoffState).toBe(PolicyDecisionHandoffState.Disabled);
  expect(row.policyEvaluatorRuntimeClaimed).toBe(false);
  expect(row.timerRuntimeClaimed).toBe(false);
  expect(row.adapterDispatchClaimed).toBe(false);
  expect(row.childDeliveryClaimed).toBe(false);
  expect(row.platformEnforcementClaimed).toBe(false);
};

const assertGameBlockLaunchStaysManualRequired = () => {
  const row = buildAppGamePolicyPreviewHandoffRow(PreviewOptions, gameManualCompiledDecision);

  expect(row.targetDomain).toBe(AppGamePolicyPreviewTargetDomain.NativeGame);
  expect(row.previewStatus).toBe(AppGamePolicyPreviewStatus.ManualRequired);
  expect(row.rejectionReason).toBe(AppGamePolicyCompilerRejectionReason.BlockLaunchManualRequired);
  expect(row.policyAction).toBe(PolicyAction.Block);
  expect(row.adapterDispatchState).toBe('not-dispatched');
  expect(row.adapterDispatchClaimed).toBe(false);
  expect(row.platformEnforcementClaimed).toBe(false);
};

const assertBuildsReadModelCounts = () => {
  const readModel = buildAppGamePolicyPreviewHandoffReadModel(PreviewOptions, [
    appCompiledDecision,
    gameManualCompiledDecision,
  ]);

  expect(readModel.nativeAppRowCount).toBe(1);
  expect(readModel.nativeGameRowCount).toBe(1);
  expect(readModel.previewReadyCount).toBe(1);
  expect(readModel.manualRequiredCount).toBe(1);
  expect(readModel.rejectedCount).toBe(0);
  expect(readModel.rows.map((row) => row.rowId)).toEqual([
    'compiled-decision-preview-app:preview',
    'compiled-decision-preview-game-manual:preview',
  ]);
  expect(readModel.sourceContractRefs).toEqual(PreviewOptions.sourceContractRefs);
  expect(readModel.policyEvaluatorRuntimeClaimed).toBe(false);
  expect(readModel.adapterDispatchClaimed).toBe(false);
};

const assertRejectsRuntimeClaims = () => {
  const row = buildAppGamePolicyPreviewHandoffRow(PreviewOptions, appCompiledDecision);

  expect(AppGamePolicyPreviewHandoffRowSchema.safeParse({ ...row, dryRun: false }).success).toBe(false);
  expect(
    AppGamePolicyPreviewHandoffRowSchema.safeParse({
      ...row,
      enforcementHandoffState: PolicyDecisionHandoffState.Pending,
    }).success
  ).toBe(false);
  expect(AppGamePolicyPreviewHandoffRowSchema.safeParse({ ...row, adapterDispatchClaimed: true }).success).toBe(false);
  expect(
    AppGamePolicyPreviewHandoffRowSchema.safeParse({
      ...row,
      policyEvaluatorRuntimeClaimState: 'not-claimed',
      timerRuntimeClaimed: true,
    }).success
  ).toBe(false);
};

const assertRejectsBadReadModelCounts = () => {
  const readModel = buildAppGamePolicyPreviewHandoffReadModel(PreviewOptions, [
    appCompiledDecision,
    gameManualCompiledDecision,
  ]);

  expect(
    AppGamePolicyPreviewHandoffReadModelSchema.safeParse({
      ...readModel,
      nativeGameRowCount: 0,
    }).success
  ).toBe(false);
  expect(
    AppGamePolicyPreviewHandoffReadModelSchema.safeParse({
      ...readModel,
      rows: [readModel.rows[0], readModel.rows[0]],
      nativeAppRowCount: 2,
      nativeGameRowCount: 0,
      previewReadyCount: 2,
      manualRequiredCount: 0,
    }).success
  ).toBe(false);
};

describe('app/game policy preview handoff contracts', () => {
  it('builds native app preview rows from dry-run compiled decisions', () => {
    assertBuildsNativeAppPreviewRows();
  });

  it('keeps native game block-launch decisions manual-required without adapter dispatch', () => {
    assertGameBlockLaunchStaysManualRequired();
  });

  it('builds read-model counts across app and game rows', () => {
    assertBuildsReadModelCounts();
  });

  it('rejects preview rows that try to execute policy or claim runtime delivery', () => {
    assertRejectsRuntimeClaims();
  });

  it('rejects handoff read models with mismatched counts or duplicate rows', () => {
    assertRejectsBadReadModelCounts();
  });
});
