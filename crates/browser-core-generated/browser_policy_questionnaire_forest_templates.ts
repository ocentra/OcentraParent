import type {
  BrowserPolicyAnswerMap,
  BrowserPolicyQuestion,
  BrowserPolicyQuestionId,
  BrowserPolicyQuestionState,
} from '@ocentra-parent/schema-domain/browser-policy-questionnaire-forest-contract';
import { questionStateTemplate } from './browser_policy_questionnaire_forest_question_state';

export function questionById(
  questions: readonly BrowserPolicyQuestion[],
  id: BrowserPolicyQuestionId
): BrowserPolicyQuestion {
  const question = questions.find((candidate) => candidate.id === id);
  if (question === undefined) {
    throw new Error(`Unknown browser policy question ${id}`);
  }
  return question;
}

export function questionStates(
  questions: readonly BrowserPolicyQuestion[],
  answers: BrowserPolicyAnswerMap
): readonly BrowserPolicyQuestionState[] {
  return questions.map((question) => ({
    question,
    ...questionStateTemplate(question, answers),
  }));
}
