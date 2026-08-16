import type {
  BrowserPolicyAnswerMap,
  BrowserPolicyComputedFlagId,
} from '@ocentra-parent/schema-domain/browser-policy-questionnaire-forest-contract';
import {
  browserPolicyBrowserGamesRelevant,
  browserPolicyCount,
  browserPolicyHas,
} from './browser_policy_questionnaire_forest_answers';
import {
  browserPolicyEvidencePrivacyVisible,
  browserPolicyNotificationEventsRelevant,
  browserPolicySetupRelevant,
  browserPolicyStoredBrowserDataExists,
  browserPolicyUnsupportedCapabilityRelevant,
} from './browser_policy_questionnaire_forest_computed';

export const browserPolicyComputedFlagEvaluatorsDerived: Partial<
  Record<BrowserPolicyComputedFlagId, (answers: BrowserPolicyAnswerMap) => boolean>
> = {
  setupRelevant: browserPolicySetupRelevant,
  classificationServiceReferenced: (answers) =>
    browserPolicyHas(answers, '5.1', 'category') || browserPolicyHas(answers, '5.2', 'classification-service'),
  multiTargetActionMatrixRelevant: (answers) =>
    browserPolicyCount(answers, '5.1') >= 2 && browserPolicyCount(answers, '6.1') >= 2,
  evidencePrivacyVisible: browserPolicyEvidencePrivacyVisible,
  notificationEventsRelevant: browserPolicyNotificationEventsRelevant,
  unsupportedCapabilityRelevant: browserPolicyUnsupportedCapabilityRelevant,
  storedBrowserDataExists: browserPolicyStoredBrowserDataExists,
  browserGamesRelevant: browserPolicyBrowserGamesRelevant,
};
