import type {
  BrowserPolicyAnswerMap,
  BrowserPolicyComputedFlagId,
} from '@ocentra-parent/schema-domain/browser-policy-questionnaire-forest-contract';
import { browserPolicyComputedFlagEvaluatorsBasic } from './browser_policy_questionnaire_forest_computed_basic';
import { browserPolicyComputedFlagEvaluatorsDerived } from './browser_policy_questionnaire_forest_computed_derived';

export function browserPolicyComputedFlagEvaluator(
  flagId: BrowserPolicyComputedFlagId,
  answers: BrowserPolicyAnswerMap
): boolean {
  const evaluator =
    browserPolicyComputedFlagEvaluatorsBasic[flagId] ?? browserPolicyComputedFlagEvaluatorsDerived[flagId];
  return evaluator!(answers);
}
