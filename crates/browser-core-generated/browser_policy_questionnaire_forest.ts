/* generated from crates/browser-core/src/browser_policy_questionnaire_forest.rs */

import type {
  BrowserPolicyAnswerMap,
  BrowserPolicyComputedFlagId,
  BrowserPolicyCondition,
  BrowserPolicyQuestion,
  BrowserPolicyQuestionId,
  BrowserPolicyQuestionState,
  BrowserPolicySurface,
} from '@ocentra-parent/schema-domain/browser-policy-questionnaire-forest-contract';
import { browserPolicyConditionMatchesTemplate as conditionMatches } from './browser_policy_questionnaire_forest_conditions';
import { browserPolicyComputedFlagTemplate as computedFlag } from './browser_policy_questionnaire_forest_computed';
import { browserPolicyQuestionIdForSetting } from './browser_policy_questionnaire_forest_settings';
import { questionById, questionStates } from './browser_policy_questionnaire_forest_templates';
import { questionState, questionStateTemplate } from './browser_policy_questionnaire_forest_question_state';
import { visibleQuestionIds, visibleQuestions } from './browser_policy_questionnaire_forest_visible';

type BrowserPolicyQuestionStateTemplate = Omit<BrowserPolicyQuestionState, 'question'>;

type BrowserControlFullCatalogSetting = {
  readonly settingId: string;
  readonly uiTab: string;
  readonly sourceText: string;
};

export function browserPolicyVisibleQuestionIdsTemplate(
  questions: readonly BrowserPolicyQuestion[],
  questionIds: readonly BrowserPolicyQuestionId[],
  compactOrder: readonly BrowserPolicyQuestionId[],
  answers: BrowserPolicyAnswerMap,
  surface: BrowserPolicySurface = 'rules'
): readonly BrowserPolicyQuestionId[] {
  return visibleQuestionIds(questions, questionIds, compactOrder, answers, surface);
}

export function browserPolicyQuestionByIdTemplate(
  questions: readonly BrowserPolicyQuestion[],
  id: BrowserPolicyQuestionId
): BrowserPolicyQuestion {
  return questionById(questions, id);
}

export function browserPolicyQuestionStatesTemplate(
  questions: readonly BrowserPolicyQuestion[],
  answers: BrowserPolicyAnswerMap
): readonly BrowserPolicyQuestionState[] {
  return questionStates(questions, answers);
}

export function browserPolicyVisibleQuestionsTemplate(
  questions: readonly BrowserPolicyQuestion[],
  questionIds: readonly BrowserPolicyQuestionId[],
  compactOrder: readonly BrowserPolicyQuestionId[],
  answers: BrowserPolicyAnswerMap,
  surface: BrowserPolicySurface = 'rules'
): readonly BrowserPolicyQuestion[] {
  return visibleQuestions(questions, questionIds, compactOrder, answers, surface);
}

export function browserPolicyQuestionStateTemplate(
  question: BrowserPolicyQuestion,
  answers: BrowserPolicyAnswerMap
): BrowserPolicyQuestionStateTemplate {
  return questionStateTemplate(question, answers);
}

export function browserPolicyQuestionState(
  question: BrowserPolicyQuestion,
  answers: BrowserPolicyAnswerMap
): BrowserPolicyQuestionState {
  return questionState(question, answers);
}

export function browserPolicyConditionMatchesTemplate(
  condition: BrowserPolicyCondition,
  answers: BrowserPolicyAnswerMap
): boolean {
  return conditionMatches(condition, answers);
}

export function browserPolicyComputedFlagTemplate(
  flagId: BrowserPolicyComputedFlagId,
  answers: BrowserPolicyAnswerMap
): boolean {
  return computedFlag(flagId, answers);
}

export function browserPolicyForestSourceSettingIdsTemplate(
  questionIds: readonly BrowserPolicyQuestionId[],
  settings: readonly BrowserControlFullCatalogSetting[]
): ReadonlyMap<BrowserPolicyQuestionId, readonly string[]> {
  const settingsByQuestion = new Map<BrowserPolicyQuestionId, string[]>();
  questionIds.forEach((questionId) => settingsByQuestion.set(questionId, []));
  settings.forEach((setting) => {
    settingsByQuestion
      .get(browserPolicyQuestionIdForSetting(setting.uiTab, setting.sourceText))
      ?.push(setting.settingId);
  });
  return new Map([...settingsByQuestion.entries()].map(([questionId, settingIds]) => [questionId, settingIds.sort()]));
}
