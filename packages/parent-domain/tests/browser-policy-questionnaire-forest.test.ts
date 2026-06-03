import { describe, expect, it } from 'vitest';

import { browserControlFullCatalogSettings } from '../src/browser-control-full-catalog';
import {
  BrowserPolicyQuestionIds,
  BrowserPolicyQuestionnaireCompactOrder,
  BrowserPolicyQuestions,
  browserPolicyForestSourceSettingIds,
  browserPolicyVisibleQuestions,
  type BrowserPolicyAnswerMap,
  type BrowserPolicyCondition,
  type BrowserPolicyQuestionId,
  type BrowserPolicySurface,
} from '../src/browser-policy-questionnaire-forest';

describe('browser policy questionnaire forest', () => {
  registerForestShapeCases();
  registerConditionCases();
  registerVisibilityCases();
  registerBranchCases();
  registerSourceCoverageCases();
});

function registerForestShapeCases() {
  it('defines the reduced browser forest and keeps AI questions outside the browser surface', () => {
    expect(BrowserPolicyQuestions.map((question) => question.id)).toEqual(BrowserPolicyQuestionIds);
    expect(
      BrowserPolicyQuestions.filter((question) => question.surface === 'ai').map((question) => question.id)
    ).toEqual(['A1', 'A2', 'A3']);
    expect(
      BrowserPolicyQuestions.filter((question) => question.surface === 'schedule').map((question) => question.id)
    ).toEqual(['10.1', '10.2', '11.1', '11.2']);
    expect(
      BrowserPolicyQuestions.filter((question) => question.surface === 'approvals').map((question) => question.id)
    ).toEqual(['12.1', '12.2', '12.3']);
    expect(BrowserPolicyQuestionnaireCompactOrder).toHaveLength(52);
    expect(
      BrowserPolicyQuestions.find((question) => question.id === '1.1')?.options.map((option) => option.id)
    ).toEqual(['off', 'on', 'paused', 'emergency-allow', 'emergency-block']);
  });
}

function registerConditionCases() {
  it('uses valid condition references and unique option ids per question', () => {
    const questionIds = new Set(BrowserPolicyQuestions.map((question) => question.id));
    const optionIdsByQuestion = new Map(
      BrowserPolicyQuestions.map((question) => [question.id, new Set(question.options.map((option) => option.id))])
    );

    BrowserPolicyQuestions.forEach((question) => {
      expect(question.options.map((option) => option.id)).toEqual([...optionIdsByQuestion.get(question.id)!]);
      collectConditions([
        ...(question.showWhen ?? []),
        ...(question.neverShowWhen ?? []),
        ...(question.disabledWhen ?? []),
        ...(question.readonlyWhen ?? []),
      ]).forEach((condition) => {
        if ('questionId' in condition) {
          expect(questionIds.has(condition.questionId)).toBe(true);
        }
        if (condition.kind === 'answer-equals' || condition.kind === 'answer-includes') {
          expect(optionIdsByQuestion.get(condition.questionId)?.has(condition.optionId)).toBe(true);
        }
        if (condition.kind === 'answer-includes-any') {
          expect(
            condition.optionIds.every((optionId) => optionIdsByQuestion.get(condition.questionId)?.has(optionId))
          ).toBe(true);
        }
      });
    });
  });
}

function registerVisibilityCases() {
  it('matches the root show and hide forest states', () => {
    expect(visibleIds({ '1.1': ['off'] }, 'rules')).toEqual(['1.1']);
    expect(visibleIds({ '1.1': ['off'] }, 'audit')).toEqual(['14.1', '18.1']);
    expect(visibleIds({ '1.1': ['paused'] }, 'schedule')).toEqual(['10.2']);
    expect(visibleIds({ '1.1': ['paused'] }, 'audit')).toEqual(['14.1', '14.3', '18.1']);
    expect(visibleIds({ '1.1': ['emergency-allow'] }, 'schedule')).toEqual(['10.2']);
    expect(visibleIds({ '1.1': ['emergency-block'] }, 'approvals')).toEqual(['12.1']);
    expect(visibleIds({ '1.1': ['on'] }, 'rules')).toEqual(['1.1', '1.2', '1.3']);
    expect(visibleIds({ '1.1': ['on'] }, 'schedule')).toEqual(['10.2']);
    expect(visibleIds({ '1.1': ['on'] }, 'audit')).toEqual(['14.1', '18.1']);
  });
}

function registerBranchCases() {
  it('reveals branch cards from selected parent decisions', () => {
    const base: BrowserPolicyAnswerMap = {
      '1.1': ['on'],
      '1.2': ['ask-parent', 'limit', 'block'],
      '1.3': ['no'],
      '2.1': ['strict'],
      '2.2': ['strict'],
      '3.1': ['managed-all'],
      '4.1': ['block-launch'],
      '5.1': ['exact-url', 'search-terms', 'video', 'downloads', 'browser-games'],
      '6.1': ['ask-parent', 'limit-time', 'block'],
      '8.1': ['limit'],
      '9.1': ['ask-parent'],
      '11.1': ['yes'],
      '12.1': ['blocked-site'],
      '13.1': ['exact-url'],
      '14.1': ['policy-status', 'blocked-events'],
      '17.1': ['standard'],
      '18.1': ['standard'],
    };

    expect(visibleIds(base, 'rules')).toContain('7.1');
    expect(visibleIds(base, 'rules')).toContain('8.1');
    expect(visibleIds(base, 'rules')).toContain('9.1');
    expect(visibleIds(base, 'rules')).toContain('19.1');
    expect(visibleIds(base, 'rules')).toContain('19.7');
    expect(visibleIds({ ...base, '5.1': ['exact-url'] }, 'rules')).not.toContain('19.1');
    expect(visibleIds(base, 'schedule')).toContain('10.1');
    expect(visibleIds(base, 'schedule')).toContain('11.2');
    expect(visibleIds(base, 'approvals')).toContain('12.2');
    expect(visibleIds(base, 'rules')).toContain('13.2');
    expect(visibleIds(base, 'rules')).toContain('15.3');
    expect(visibleIds(base, 'rules')).toContain('16.1');
    expect(visibleIds(base, 'audit')).toContain('17.2');
    expect(visibleIds(base, 'audit')).toContain('18.2');
    expect(browserPolicyVisibleQuestions(base, 'ai').map((question) => question.id)).toEqual([]);
  });
}

function registerSourceCoverageCases() {
  it('keeps source setting coverage complete for the 1057 setting catalog', () => {
    const sourceSettingIds = browserControlFullCatalogSettings()
      .map((setting) => setting.settingId)
      .sort();
    const coveredSettingIds = [...browserPolicyForestSourceSettingIds().values()].flat().sort();

    expect(sourceSettingIds).toHaveLength(1057);
    expect(coveredSettingIds).toEqual(sourceSettingIds);
  });
}

function visibleIds(answers: BrowserPolicyAnswerMap, surface?: BrowserPolicySurface): BrowserPolicyQuestionId[] {
  return browserPolicyVisibleQuestions(answers, surface).map((question) => question.id);
}

function collectConditions(conditions: readonly BrowserPolicyCondition[]): BrowserPolicyCondition[] {
  return conditions.flatMap((condition) => {
    if (condition.kind === 'all' || condition.kind === 'any')
      return [condition, ...collectConditions(condition.conditions)];
    if (condition.kind === 'not') return [condition, ...collectConditions([condition.condition])];
    return [condition];
  });
}
