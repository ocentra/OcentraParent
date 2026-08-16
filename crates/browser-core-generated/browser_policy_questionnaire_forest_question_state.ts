import type {
  BrowserPolicyAnswerMap,
  BrowserPolicyCondition,
  BrowserPolicyQuestion,
  BrowserPolicyQuestionState,
} from '@ocentra-parent/schema-domain/browser-policy-questionnaire-forest-contract';
import { browserPolicyConditionMatchesTemplate } from './browser_policy_questionnaire_forest_conditions';

type BrowserPolicyQuestionStateTemplate = Omit<BrowserPolicyQuestionState, 'question'>;

export function questionStateTemplate(
  question: BrowserPolicyQuestion,
  answers: BrowserPolicyAnswerMap
): BrowserPolicyQuestionStateTemplate {
  const neverShow = conditionListMatches(question.neverShowWhen, answers);
  const show = (question.showWhen ?? []).length === 0 || conditionsMatch(question.showWhen ?? [], answers);
  const visible = show && !neverShow;

  return {
    visible,
    disabled: visible && conditionListMatches(question.disabledWhen, answers),
    readonly: visible && conditionListMatches(question.readonlyWhen, answers),
  };
}

export function questionState(
  question: BrowserPolicyQuestion,
  answers: BrowserPolicyAnswerMap
): BrowserPolicyQuestionState {
  return {
    question,
    ...questionStateTemplate(question, answers),
  };
}

function conditionsMatch(conditions: readonly BrowserPolicyCondition[], answers: BrowserPolicyAnswerMap): boolean {
  return conditions.every((condition) => browserPolicyConditionMatchesTemplate(condition, answers));
}

function conditionListMatches(
  conditions: readonly BrowserPolicyCondition[] | undefined,
  answers: BrowserPolicyAnswerMap
): boolean {
  return conditions !== undefined && conditions.length > 0 && conditionsMatch(conditions, answers);
}
