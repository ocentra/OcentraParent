import type {
  BrowserPolicyAnswerMap,
  BrowserPolicyComputedFlagId,
} from '@ocentra-parent/schema-domain/browser-policy-questionnaire-forest-contract';
import {
  browserPolicyCount,
  browserPolicyHasAny,
  browserPolicyHasAnyRoot,
  browserPolicyRootAnswer,
} from './browser_policy_questionnaire_forest_answers';
import { browserPolicyComputedFlagEvaluator } from './browser_policy_questionnaire_forest_computed_map';

export function browserPolicySetupRelevant(answers: BrowserPolicyAnswerMap): boolean {
  return (
    browserPolicyHasAny(answers, '3.1', ['prefer-managed', 'managed-exact', 'managed-all']) ||
    browserPolicyHasAny(answers, '2.2', ['standard', 'strict', 'custom']) ||
    browserPolicyHasAny(answers, '2.3', ['parent-review', 'block-until-approved']) ||
    browserPolicyComputedFlagTemplate('unsupportedCapabilityRelevant', answers)
  );
}

export function browserPolicyEvidencePrivacyVisible(answers: BrowserPolicyAnswerMap): boolean {
  return browserPolicyRootAnswer(answers) === 'on' && browserPolicyHasAnyEvidenceTrigger(answers);
}

function browserPolicyHasAnyEvidenceTrigger(answers: BrowserPolicyAnswerMap): boolean {
  return (
    browserPolicyCount(answers, '2.1') > 0 ||
    browserPolicyCount(answers, '5.1') > 0 ||
    browserPolicyCount(answers, '7.1') > 0 ||
    browserPolicyCount(answers, '8.1') > 0 ||
    browserPolicyCount(answers, '9.1') > 0 ||
    browserPolicyComputedFlagTemplate('reportsEnabled', answers)
  );
}

export function browserPolicyNotificationEventsRelevant(answers: BrowserPolicyAnswerMap): boolean {
  return browserPolicyRootAnswer(answers) !== 'off' && browserPolicyHasAnyNotificationTrigger(answers);
}

function browserPolicyHasAnyNotificationTrigger(answers: BrowserPolicyAnswerMap): boolean {
  return (
    browserPolicyHasAny(answers, '2.3', ['notify-parent', 'parent-review', 'block-until-approved']) ||
    browserPolicyHasAny(answers, '4.1', [
      'warn',
      'notify-parent',
      'parent-review',
      'close',
      'close-open-managed',
      'block-launch',
    ]) ||
    browserPolicyHasAny(answers, '6.1', ['warn', 'parent-review', 'block', 'close-browser']) ||
    browserPolicyHasAny(answers, '9.1', ['notify-parent', 'parent-review', 'block-risky', 'block-all-approved']) ||
    browserPolicyComputedFlagTemplate('limitExists', answers) ||
    browserPolicyHasAnyRoot(answers, ['paused', 'emergency-allow', 'emergency-block'])
  );
}

export function browserPolicyUnsupportedCapabilityRelevant(answers: BrowserPolicyAnswerMap): boolean {
  return (
    browserPolicyHasAny(answers, '2.1', ['strict', 'custom']) ||
    browserPolicyComputedFlagTemplate('managedBrowserRequired', answers) ||
    browserPolicyHasAny(answers, '5.1', ['exact-url', 'search-terms', 'downloads', 'video']) ||
    browserPolicyHasAny(answers, '6.1', ['block', 'close-browser', 'require-managed'])
  );
}

export function browserPolicyStoredBrowserDataExists(answers: BrowserPolicyAnswerMap): boolean {
  return (
    browserPolicyComputedFlagTemplate('reportsEnabled', answers) ||
    browserPolicyComputedFlagTemplate('auditEnabled', answers) ||
    browserPolicyCount(answers, '13.1') > 0 ||
    browserPolicyCount(answers, '12.1') > 0
  );
}

export function browserPolicyComputedFlagTemplate(
  flagId: BrowserPolicyComputedFlagId,
  answers: BrowserPolicyAnswerMap
): boolean {
  return browserPolicyComputedFlagEvaluator(flagId, answers);
}
