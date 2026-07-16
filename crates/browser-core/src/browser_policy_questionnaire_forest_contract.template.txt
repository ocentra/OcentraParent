/* generated from crates/browser-core/src/browser_policy_questionnaire_forest.rs */

export const BrowserPolicyQuestionIds = [
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

export type BrowserPolicyQuestionId = (typeof BrowserPolicyQuestionIds)[number];

export type BrowserPolicySurface = 'rules' | 'schedule' | 'approvals' | 'audit' | 'ai';
export type BrowserPolicySelectionMode = 'single' | 'multi';

export type BrowserPolicyComputedFlagId =
  | 'policyIsOff'
  | 'policyIsOn'
  | 'policyPaused'
  | 'emergencyOverrideActive'
  | 'askParentExists'
  | 'limitExists'
  | 'downloadsSelected'
  | 'searchSelected'
  | 'videoSelected'
  | 'exactEvidenceSelected'
  | 'managedBrowserRequired'
  | 'reportsEnabled'
  | 'auditEnabled'
  | 'setupRelevant'
  | 'classificationServiceReferenced'
  | 'multiTargetActionMatrixRelevant'
  | 'evidencePrivacyVisible'
  | 'notificationEventsRelevant'
  | 'unsupportedCapabilityRelevant'
  | 'storedBrowserDataExists'
  | 'browserGamesRelevant';

export type BrowserPolicyCondition =
  | {
      readonly kind: 'answer-equals';
      readonly questionId: BrowserPolicyQuestionId;
      readonly optionId: string;
    }
  | {
      readonly kind: 'answer-includes';
      readonly questionId: BrowserPolicyQuestionId;
      readonly optionId: string;
    }
  | {
      readonly kind: 'answer-includes-any';
      readonly questionId: BrowserPolicyQuestionId;
      readonly optionIds: readonly string[];
    }
  | {
      readonly kind: 'answer-has-any-selected';
      readonly questionId: BrowserPolicyQuestionId;
    }
  | {
      readonly kind: 'computed-flag';
      readonly flagId: BrowserPolicyComputedFlagId;
    }
  | {
      readonly kind: 'all';
      readonly conditions: readonly BrowserPolicyCondition[];
    }
  | {
      readonly kind: 'any';
      readonly conditions: readonly BrowserPolicyCondition[];
    }
  | {
      readonly kind: 'not';
      readonly condition: BrowserPolicyCondition;
    };

export type BrowserPolicyOption = {
  readonly id: string;
  readonly label: string;
  readonly covers?: readonly string[];
};

export type BrowserPolicyQuestion = {
  readonly id: BrowserPolicyQuestionId;
  readonly title: string;
  readonly selectionMode: BrowserPolicySelectionMode;
  readonly surface: BrowserPolicySurface;
  readonly options: readonly BrowserPolicyOption[];
  readonly showWhen?: readonly BrowserPolicyCondition[];
  readonly neverShowWhen?: readonly BrowserPolicyCondition[];
  readonly disabledWhen?: readonly BrowserPolicyCondition[];
  readonly readonlyWhen?: readonly BrowserPolicyCondition[];
};

export type BrowserPolicyQuestionState = {
  readonly question: BrowserPolicyQuestion;
  readonly visible: boolean;
  readonly disabled: boolean;
  readonly readonly: boolean;
};

export type BrowserPolicyAnswerMap = Partial<Record<BrowserPolicyQuestionId, readonly string[]>>;

export const BrowserPolicyDefaultAnswers = {
  '1.1': ['on'],
  '1.3': ['no'],
} as const satisfies BrowserPolicyAnswerMap;

export const answerEquals = (questionId: BrowserPolicyQuestionId, optionId: string): BrowserPolicyCondition => ({
  kind: 'answer-equals',
  questionId,
  optionId,
});

export const answerIncludes = (questionId: BrowserPolicyQuestionId, optionId: string): BrowserPolicyCondition => ({
  kind: 'answer-includes',
  questionId,
  optionId,
});

export const answerIncludesAny = (
  questionId: BrowserPolicyQuestionId,
  optionIds: readonly string[]
): BrowserPolicyCondition => ({
  kind: 'answer-includes-any',
  questionId,
  optionIds,
});

export const answerHasAnySelected = (questionId: BrowserPolicyQuestionId): BrowserPolicyCondition => ({
  kind: 'answer-has-any-selected',
  questionId,
});

export const computedFlag = (flagId: BrowserPolicyComputedFlagId): BrowserPolicyCondition => ({
  kind: 'computed-flag',
  flagId,
});

export const all = (conditions: readonly BrowserPolicyCondition[]): BrowserPolicyCondition => ({
  kind: 'all',
  conditions,
});

export const anyCondition = (conditions: readonly BrowserPolicyCondition[]): BrowserPolicyCondition => ({
  kind: 'any',
  conditions,
});

export const not = (condition: BrowserPolicyCondition): BrowserPolicyCondition => ({
  kind: 'not',
  condition,
});
