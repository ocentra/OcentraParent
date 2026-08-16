import { describe, expect, it } from 'vitest';

import {
  BrowserPolicyQuestionIds,
  BrowserPolicyDefaultAnswers,
  all,
  answerEquals,
  answerHasAnySelected,
  answerIncludes,
  answerIncludesAny,
  anyCondition,
  computedFlag,
  not,
} from '../../src/browser-policy-questionnaire-forest-contract';

const ExpectedBrowserPolicyQuestionIds = [
  '1.1',
  '1.2',
  '1.3',
  '2.1',
  '2.2',
  '2.3',
  '3.1',
  '3.2',
  '3.3',
  '4.1',
  '4.2',
  '4.3',
  '5.1',
  '5.2',
  '5.3',
  '6.1',
  '6.2',
  '7.1',
  '7.2',
  '8.1',
  '8.2',
  '9.1',
  '9.2',
  '9.3',
  '10.1',
  '10.2',
  '11.1',
  '11.2',
  '12.1',
  '12.2',
  '12.3',
  '13.1',
  '13.2',
  '14.1',
  '14.2',
  '14.3',
  '15.1',
  '15.2',
  '15.3',
  '16.1',
  '16.2',
  '17.1',
  '17.2',
  '18.1',
  '18.2',
  '19.1',
  '19.2',
  '19.3',
  '19.4',
  '19.5',
  '19.6',
  '19.7',
  'A1',
  'A2',
  'A3',
] as const;

const ExpectedBrowserPolicyDefaultAnswers = {
  '1.1': ['on'],
  '1.3': ['no'],
};

describe('browser policy questionnaire forest contract', () => {
  it('keeps the browser policy question ids stable', () => {
    expect(BrowserPolicyQuestionIds).toEqual(ExpectedBrowserPolicyQuestionIds);
    expect(BrowserPolicyDefaultAnswers).toEqual(ExpectedBrowserPolicyDefaultAnswers);
  });

  it('builds browser policy condition discriminants without hidden logic', () => {
    expect(answerEquals('1.1', 'on')).toEqual({
      kind: 'answer-equals',
      questionId: '1.1',
      optionId: 'on',
    });
    expect(answerIncludes('1.2', 'parent-review')).toEqual({
      kind: 'answer-includes',
      questionId: '1.2',
      optionId: 'parent-review',
    });
    expect(answerIncludesAny('5.1', ['video', 'search-terms'])).toEqual({
      kind: 'answer-includes-any',
      questionId: '5.1',
      optionIds: ['video', 'search-terms'],
    });
    expect(answerHasAnySelected('9.1')).toEqual({
      kind: 'answer-has-any-selected',
      questionId: '9.1',
    });
    expect(computedFlag('policyIsOn')).toEqual({
      kind: 'computed-flag',
      flagId: 'policyIsOn',
    });
    expect(all([answerEquals('1.1', 'on')])).toEqual({
      kind: 'all',
      conditions: [
        {
          kind: 'answer-equals',
          questionId: '1.1',
          optionId: 'on',
        },
      ],
    });
    expect(anyCondition([answerEquals('1.3', 'no')])).toEqual({
      kind: 'any',
      conditions: [
        {
          kind: 'answer-equals',
          questionId: '1.3',
          optionId: 'no',
        },
      ],
    });
    expect(not(answerEquals('1.3', 'no'))).toEqual({
      kind: 'not',
      condition: {
        kind: 'answer-equals',
        questionId: '1.3',
        optionId: 'no',
      },
    });
  });
});
