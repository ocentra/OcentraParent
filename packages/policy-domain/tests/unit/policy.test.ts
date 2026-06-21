import { describe, expect, it } from 'vitest';
import {
  PolicyAction,
  PolicyDecisionHandoffState,
  PolicyPreviewConfirmationState,
  PolicyPreviewOrigin,
  PolicyScheduleBudgetCarryoverMode,
  PolicyPreviewBudgetBoundaryState,
  PolicyScheduleBudgetResetKind,
  PolicyRuleSchema,
  PolicyScheduleClockSource,
  PolicyScheduleDay,
  PolicyScheduleOfflineRecovery,
  PolicyTargetType,
  parseFamilyPolicySet,
  parsePolicyPreview,
  parsePolicySchedule,
  resolvePolicyPreviewBudgetBoundaryState,
} from '@ocentra-parent/schema-domain/policy';

function previewDecision() {
  return {
    schemaVersion: 'v0.6',
    decisionId: 'preview-decision-1',
    action: PolicyAction.Block,
    reasonCodes: ['school-night-video'],
    evidenceReferences: [],
    ruleIds: ['rule-1'],
    localAiResultId: null,
    dryRun: true,
    enforcementHandoffState: PolicyDecisionHandoffState.Disabled,
    expiresAt: null,
  } as const;
}

describe('parent policy contracts', () => {
  it('parseFamilyPolicySet: parses parent-authored rules, schedules, children, and devices with explicit time-budget semantics', () => {
    const parsed = parseFamilyPolicySet({
      schemaVersion: 'v0.6',
      family: { familyId: 'family-main' },
      childProfiles: [{ childProfileId: 'child-1', displayName: 'Sam' }],
      devices: [
        {
          deviceId: 'device-1',
          childProfileId: 'child-1',
          label: 'Sam Windows PC',
          platform: 'windows',
        },
      ],
      policyVersion: 'policy-v1',
      rules: [
        {
          ruleId: 'rule-1',
          target: { targetId: 'target-1', targetType: PolicyTargetType.Domain, targetValue: 'video.example' },
          action: PolicyAction.AskParent,
          scheduleId: 'school-night',
          priority: 10,
          reasonCode: 'school-night-video',
          createdBy: { actorId: 'parent-1', role: 'parent' },
          enabled: true,
          effectiveFrom: '2026-05-20T00:00:00.000Z',
          effectiveUntil: null,
        },
      ],
      schedules: [
        {
          scheduleId: 'school-night',
          timeZone: 'America/Toronto',
          windows: [
            {
              days: [PolicyScheduleDay.Monday, PolicyScheduleDay.Tuesday],
              startLocalTime: '18:00',
              endLocalTime: '21:00',
            },
          ],
          timeBudget: {
            budgetWindowMinutes: 120,
            reset: {
              kind: PolicyScheduleBudgetResetKind.Weekly,
              localTime: '00:00',
              day: PolicyScheduleDay.Monday,
            },
            carryover: {
              mode: PolicyScheduleBudgetCarryoverMode.CapCarryover,
              maxMinutes: 30,
            },
            gracePeriodMinutes: 10,
            effectiveFrom: '2026-05-20T00:00:00.000Z',
            effectiveUntil: null,
            clockSource: PolicyScheduleClockSource.ChildDevice,
            offlineRecovery: PolicyScheduleOfflineRecovery.RecomputeFromJournal,
          },
        },
      ],
    });

    expect(parsed.rules[0]?.action).toBe(PolicyAction.AskParent);
    expect(parsed.schedules[0]?.windows[0]?.days).toEqual([PolicyScheduleDay.Monday, PolicyScheduleDay.Tuesday]);
    expect(parsed.schedules[0]?.timeBudget.carryover.mode).toBe(PolicyScheduleBudgetCarryoverMode.CapCarryover);
  });

  it('parsePolicySchedule: rejects capped carryover without an explicit minute cap', () => {
    expect(() =>
      parsePolicySchedule({
        scheduleId: 'school-night',
        timeZone: 'America/Toronto',
        windows: [
          {
            days: [PolicyScheduleDay.Monday],
            startLocalTime: '18:00',
            endLocalTime: '21:00',
          },
        ],
        timeBudget: {
          budgetWindowMinutes: 120,
          reset: {
            kind: PolicyScheduleBudgetResetKind.Daily,
            localTime: '00:00',
            day: null,
          },
          carryover: {
            mode: PolicyScheduleBudgetCarryoverMode.CapCarryover,
            maxMinutes: null,
          },
          gracePeriodMinutes: 10,
          effectiveFrom: '2026-05-20T00:00:00.000Z',
          effectiveUntil: null,
          clockSource: PolicyScheduleClockSource.ChildDevice,
          offlineRecovery: PolicyScheduleOfflineRecovery.ResumeRemaining,
        },
      })
    ).toThrow('cap-carryover requires timeBudget.carryover.maxMinutes');
  });

  it('parsePolicySchedule: rejects weekly resets without an explicit reset day', () => {
    expect(() =>
      parsePolicySchedule({
        scheduleId: 'school-night',
        timeZone: 'America/Toronto',
        windows: [
          {
            days: [PolicyScheduleDay.Monday],
            startLocalTime: '18:00',
            endLocalTime: '21:00',
          },
        ],
        timeBudget: {
          budgetWindowMinutes: 120,
          reset: {
            kind: PolicyScheduleBudgetResetKind.Weekly,
            localTime: '00:00',
            day: null,
          },
          carryover: {
            mode: PolicyScheduleBudgetCarryoverMode.DiscardUnused,
            maxMinutes: null,
          },
          gracePeriodMinutes: 10,
          effectiveFrom: '2026-05-20T00:00:00.000Z',
          effectiveUntil: null,
          clockSource: PolicyScheduleClockSource.ChildDevice,
          offlineRecovery: PolicyScheduleOfflineRecovery.ManualRequired,
        },
      })
    ).toThrow('weekly reset rules require timeBudget.reset.day');
  });

  it('PolicyRuleSchema: rejects actions outside the local policy decision set', () => {
    const result = PolicyRuleSchema.safeParse({
      ruleId: 'rule-1',
      target: { targetId: 'target-1', targetType: PolicyTargetType.Domain, targetValue: 'video.example' },
      action: 'auto-escalate',
      scheduleId: null,
      priority: 10,
      reasonCode: 'bad-action',
      createdBy: { actorId: 'parent-1', role: 'parent' },
      enabled: true,
      effectiveFrom: null,
      effectiveUntil: null,
    });

    expect(result.success).toBe(false);
    if (!result.success) {
      expect([...new Set(result.error.issues.map((issue) => issue.path.join('.')))]).toEqual(['action']);
    }
  });

  it('parsePolicyPreview: keeps assistant-authored previews confirmation-required until a parent confirms', () => {
    const preview = parsePolicyPreview({
      previewId: 'policy-preview-1',
      origin: PolicyPreviewOrigin.AssistantPreview,
      confirmationState: PolicyPreviewConfirmationState.ConfirmationRequired,
      confirmedBy: null,
      confirmedAt: null,
      target: { targetId: 'target-1', targetType: PolicyTargetType.Domain, targetValue: 'video.example' },
      requestedAction: PolicyAction.Block,
      scheduleBoundary: null,
      decision: previewDecision(),
    });

    expect(preview.origin).toBe(PolicyPreviewOrigin.AssistantPreview);
    expect(preview.confirmationState).toBe(PolicyPreviewConfirmationState.ConfirmationRequired);
  });

  it('parsePolicyPreview: accepts parent-authored previews once a parent confirmation is recorded', () => {
    const preview = parsePolicyPreview({
      previewId: 'policy-preview-2',
      origin: PolicyPreviewOrigin.ParentPreview,
      confirmationState: PolicyPreviewConfirmationState.Confirmed,
      confirmedBy: { actorId: 'parent-1', role: 'parent' },
      confirmedAt: '2026-06-13T20:12:00.000Z',
      target: { targetId: 'target-1', targetType: PolicyTargetType.Domain, targetValue: 'video.example' },
      requestedAction: PolicyAction.Block,
      scheduleBoundary: null,
      decision: previewDecision(),
    });

    expect(preview.confirmationState).toBe(PolicyPreviewConfirmationState.Confirmed);
    expect(preview.confirmedBy?.actorId).toBe('parent-1');
  });

  it('parsePolicyPreview: rejects preview decisions that attempt runtime handoff', () => {
    expect(() =>
      parsePolicyPreview({
        previewId: 'policy-preview-3',
        origin: PolicyPreviewOrigin.ParentPreview,
        confirmationState: PolicyPreviewConfirmationState.ConfirmationRequired,
        confirmedBy: null,
        confirmedAt: null,
        target: { targetId: 'target-1', targetType: PolicyTargetType.Domain, targetValue: 'video.example' },
        requestedAction: PolicyAction.Block,
        scheduleBoundary: null,
        decision: {
          ...previewDecision(),
          dryRun: false,
          enforcementHandoffState: PolicyDecisionHandoffState.Pending,
        },
      })
    ).toThrow('preview decisions must remain dry-run');
  });

  it('resolvePolicyPreviewBudgetBoundaryState: treats non-scheduled previews as within-budget by default', () => {
    expect(resolvePolicyPreviewBudgetBoundaryState(null)).toBe(PolicyPreviewBudgetBoundaryState.WithinBudget);
  });
});
