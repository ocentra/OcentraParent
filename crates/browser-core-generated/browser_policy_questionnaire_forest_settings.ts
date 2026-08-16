import type { BrowserPolicyQuestionId } from '@ocentra-parent/schema-domain/browser-policy-questionnaire-forest-contract';

export function browserPolicyQuestionIdForSetting(uiTab: string, sourceText: string): BrowserPolicyQuestionId {
  const text = sourceText.toLowerCase();
  if (uiTab === 'ai' || browserPolicySourceTextIncludesAny(text, [' ai ', 'classification'])) {
    return 'A1';
  }
  if (uiTab === 'schedule') {
    return browserPolicyScheduleQuestionIdForSetting(text);
  }
  return browserPolicyQuestionIdByUiTab.get(uiTab) ?? browserPolicyQuestionIdForSourceText(text) ?? '1.2';
}

const browserPolicyQuestionIdByUiTab = new Map<string, BrowserPolicyQuestionId>([
  ['audit', '18.2'],
  ['data', '17.1'],
  ['reports', '14.1'],
  ['approvals', '12.1'],
  ['setup', '16.1'],
  ['platform', '15.2'],
]);

const browserPolicyQuestionSourceRules: readonly {
  readonly terms: readonly string[];
  readonly questionId: BrowserPolicyQuestionId;
}[] = [
  { terms: ['download'], questionId: '9.1' },
  { terms: ['search'], questionId: '7.1' },
  { terms: ['video', 'channel'], questionId: '8.1' },
  { terms: ['managed browser', 'profile', 'extension'], questionId: '3.1' },
  { terms: ['unmanaged', 'bypass', 'tor', 'portable'], questionId: '4.1' },
  { terms: ['url', 'domain', 'category', 'rule'], questionId: '5.1' },
  { terms: ['evidence', 'proof', 'privacy'], questionId: '13.1' },
  { terms: ['browser', 'discover', 'coverage'], questionId: '2.1' },
];

function browserPolicyScheduleQuestionIdForSetting(text: string): BrowserPolicyQuestionId {
  return browserPolicySourceTextIncludesAny(text, ['budget', 'quota']) ? '11.2' : '10.1';
}

function browserPolicyQuestionIdForSourceText(text: string): BrowserPolicyQuestionId | undefined {
  return browserPolicyQuestionSourceRules.find((rule) => browserPolicySourceTextIncludesAny(text, rule.terms))
    ?.questionId;
}

function browserPolicySourceTextIncludesAny(text: string, terms: readonly string[]): boolean {
  return terms.some((term) => text.includes(term));
}
