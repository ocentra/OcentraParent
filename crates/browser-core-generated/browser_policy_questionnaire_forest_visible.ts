import type {
  BrowserPolicyAnswerMap,
  BrowserPolicyQuestion,
  BrowserPolicyQuestionId,
  BrowserPolicySurface,
} from '@ocentra-parent/schema-domain/browser-policy-questionnaire-forest-contract';
import { questionStateTemplate } from './browser_policy_questionnaire_forest_question_state';

export function visibleQuestionIds(
  questions: readonly BrowserPolicyQuestion[],
  questionIds: readonly BrowserPolicyQuestionId[],
  compactOrder: readonly BrowserPolicyQuestionId[],
  answers: BrowserPolicyAnswerMap,
  surface: BrowserPolicySurface
): readonly BrowserPolicyQuestionId[] {
  const questionById = new Map(questions.map((question) => [question.id, question]));
  const orderedIds = surface === 'ai' ? questionIds.filter((questionId) => questionId.startsWith('A')) : compactOrder;

  return orderedIds
    .map((questionId) => questionById.get(questionId))
    .filter((question): question is BrowserPolicyQuestion => question !== undefined)
    .filter((question) => question.surface === surface)
    .filter((question) => questionStateTemplate(question, answers).visible)
    .map((question) => question.id);
}

export function visibleQuestions(
  questions: readonly BrowserPolicyQuestion[],
  questionIds: readonly BrowserPolicyQuestionId[],
  compactOrder: readonly BrowserPolicyQuestionId[],
  answers: BrowserPolicyAnswerMap,
  surface: BrowserPolicySurface
): readonly BrowserPolicyQuestion[] {
  const questionById = new Map(questions.map((question) => [question.id, question]));
  return visibleQuestionIds(questions, questionIds, compactOrder, answers, surface)
    .map((questionId) => questionById.get(questionId))
    .filter((question): question is BrowserPolicyQuestion => question !== undefined);
}
