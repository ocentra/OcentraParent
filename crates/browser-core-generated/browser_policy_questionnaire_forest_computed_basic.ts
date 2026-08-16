import type {
  BrowserPolicyAnswerMap,
  BrowserPolicyComputedFlagId,
} from '@ocentra-parent/schema-domain/browser-policy-questionnaire-forest-contract';
import {
  browserPolicyAskParentExists,
  browserPolicyExactEvidenceSelected,
  browserPolicyHas,
  browserPolicyHasAny,
  browserPolicyHasAnyRoot,
  browserPolicyManagedBrowserRequired,
  browserPolicyRootAnswer,
} from './browser_policy_questionnaire_forest_answers';

export const browserPolicyComputedFlagEvaluatorsBasic: Partial<
  Record<BrowserPolicyComputedFlagId, (answers: BrowserPolicyAnswerMap) => boolean>
> = {
  policyIsOff: (answers) => browserPolicyRootAnswer(answers) === 'off',
  policyIsOn: (answers) => browserPolicyRootAnswer(answers) === 'on',
  policyPaused: (answers) => browserPolicyRootAnswer(answers) === 'paused',
  emergencyOverrideActive: (answers) => browserPolicyHasAnyRoot(answers, ['emergency-allow', 'emergency-block']),
  askParentExists: browserPolicyAskParentExists,
  limitExists: (answers) =>
    browserPolicyHas(answers, '1.2', 'limit') ||
    browserPolicyHas(answers, '6.1', 'limit-time') ||
    browserPolicyHas(answers, '8.1', 'limit'),
  downloadsSelected: (answers) => browserPolicyHas(answers, '5.1', 'downloads'),
  searchSelected: (answers) => browserPolicyHasAny(answers, '5.1', ['search-terms', 'safe-search']),
  videoSelected: (answers) => browserPolicyHas(answers, '5.1', 'video'),
  exactEvidenceSelected: browserPolicyExactEvidenceSelected,
  managedBrowserRequired: browserPolicyManagedBrowserRequired,
  reportsEnabled: (answers) => (answers['14.1'] ?? []).some((optionId) => optionId !== 'policy-status'),
  auditEnabled: (answers) => browserPolicyHasAny(answers, '18.1', ['minimal', 'standard', 'detailed', 'custom']),
};
