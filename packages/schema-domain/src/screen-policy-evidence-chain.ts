import { type Infer, NonEmptyStringSchema, Schema, withParser } from './effect';

export const ScreenPolicyEvidenceRefSchema = NonEmptyStringSchema;
export const ScreenPolicyEvidenceActionSchema = NonEmptyStringSchema;
export const ScreenPolicyEvidenceReasonSchema = NonEmptyStringSchema;
export const ScreenPolicyEvidenceRefListSchema = Schema.Array(ScreenPolicyEvidenceReasonSchema);

type ScreenPolicyEvidenceChainCandidate = Infer<typeof ScreenPolicyEvidenceChainBaseSchema>;

export function screenPolicyEvidenceChainFields() {
  return {
    policyDecisionRef: Schema.Union(ScreenPolicyEvidenceRefSchema, Schema.Null),
    policyAction: Schema.Union(ScreenPolicyEvidenceActionSchema, Schema.Null),
    policyReasonCodes: ScreenPolicyEvidenceRefListSchema,
    parentRuleRefs: ScreenPolicyEvidenceRefListSchema,
    localModelRuntimeRefs: ScreenPolicyEvidenceRefListSchema,
    parentExplanationRefs: ScreenPolicyEvidenceRefListSchema,
    explanationReasons: ScreenPolicyEvidenceRefListSchema,
    deletionReasons: ScreenPolicyEvidenceRefListSchema,
  };
}

export function screenPolicyEvidenceChainFieldsWithDefaults() {
  return {
    policyDecisionRef: Schema.optionalWith(Schema.Union(ScreenPolicyEvidenceRefSchema, Schema.Null), {
      default: () => null,
    }),
    policyAction: Schema.optionalWith(Schema.Union(ScreenPolicyEvidenceActionSchema, Schema.Null), {
      default: () => null,
    }),
    policyReasonCodes: Schema.optionalWith(ScreenPolicyEvidenceRefListSchema, { default: () => [] }),
    parentRuleRefs: Schema.optionalWith(ScreenPolicyEvidenceRefListSchema, { default: () => [] }),
    localModelRuntimeRefs: Schema.optionalWith(ScreenPolicyEvidenceRefListSchema, {
      default: () => [],
    }),
    parentExplanationRefs: Schema.optionalWith(ScreenPolicyEvidenceRefListSchema, {
      default: () => [],
    }),
    explanationReasons: Schema.optionalWith(ScreenPolicyEvidenceRefListSchema, { default: () => [] }),
    deletionReasons: Schema.optionalWith(ScreenPolicyEvidenceRefListSchema, { default: () => [] }),
  };
}

const ScreenPolicyEvidenceChainBaseSchema = Schema.Struct(screenPolicyEvidenceChainFields());
const ScreenPolicyEvidenceChainWithDefaultsBaseSchema = Schema.Struct(
  screenPolicyEvidenceChainFieldsWithDefaults()
);

export const ScreenPolicyEvidenceChainSchema = withParser(
  ScreenPolicyEvidenceChainBaseSchema.pipe(
    Schema.filter(
      (value) =>
        screenPolicyEvidenceChainIsCoherent(value) ||
        'Expected screen policy decision fields to stay empty until a policy decision reference exists'
    )
  )
);

export const ScreenPolicyEvidenceChainWithDefaultsSchema = withParser(
  ScreenPolicyEvidenceChainWithDefaultsBaseSchema.pipe(
    Schema.filter(
      (value) =>
        screenPolicyEvidenceChainIsCoherent(value) ||
        'Expected screen policy decision fields to stay empty until a policy decision reference exists'
    )
  )
);

export type ScreenPolicyEvidenceChain = Infer<typeof ScreenPolicyEvidenceChainSchema>;

function screenPolicyEvidenceChainIsCoherent(value: ScreenPolicyEvidenceChainCandidate): boolean {
  if (value.policyDecisionRef !== null) {
    return true;
  }

  return (
    value.policyAction === null &&
    value.policyReasonCodes.length === 0 &&
    value.parentRuleRefs.length === 0 &&
    value.parentExplanationRefs.length === 0 &&
    value.explanationReasons.length === 0
  );
}
