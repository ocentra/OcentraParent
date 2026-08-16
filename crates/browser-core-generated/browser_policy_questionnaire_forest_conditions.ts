import type {
  BrowserPolicyAnswerMap,
  BrowserPolicyCondition,
  BrowserPolicyQuestionId,
} from '@ocentra-parent/schema-domain/browser-policy-questionnaire-forest-contract';
import { browserPolicyComputedFlagTemplate } from './browser_policy_questionnaire_forest_computed';

function chosen(answers: BrowserPolicyAnswerMap, questionId: BrowserPolicyQuestionId): readonly string[] {
  return answers[questionId] ?? [];
}

export function browserPolicyConditionMatchesTemplate(
  condition: BrowserPolicyCondition,
  answers: BrowserPolicyAnswerMap
): boolean {
  switch (condition.kind) {
    case 'answer-equals':
      return chosen(answers, condition.questionId)[0] === condition.optionId;
    case 'answer-includes':
      return chosen(answers, condition.questionId).includes(condition.optionId);
    case 'answer-includes-any':
      return condition.optionIds.some((optionId) => chosen(answers, condition.questionId).includes(optionId));
    case 'answer-has-any-selected':
      return chosen(answers, condition.questionId).length > 0;
    case 'computed-flag':
      return browserPolicyComputedFlagTemplate(condition.flagId, answers);
    case 'all':
      return conditionsMatch(condition.conditions, answers);
    case 'any':
      return condition.conditions.some((candidate) => browserPolicyConditionMatchesTemplate(candidate, answers));
    case 'not':
      return !browserPolicyConditionMatchesTemplate(condition.condition, answers);
  }
}

export function conditionsMatch(
  conditions: readonly BrowserPolicyCondition[],
  answers: BrowserPolicyAnswerMap
): boolean {
  return conditions.every((condition) => browserPolicyConditionMatchesTemplate(condition, answers));
}

export function conditionListMatches(
  conditions: readonly BrowserPolicyCondition[] | undefined,
  answers: BrowserPolicyAnswerMap
): boolean {
  return conditions !== undefined && conditions.length > 0 && conditionsMatch(conditions, answers);
}
