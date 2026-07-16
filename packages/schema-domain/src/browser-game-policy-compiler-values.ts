import { Schema, withParser, brandedNonEmptyStringSchema, NonEmptyStringSchema } from './effect';
import { ParentEvidenceReferenceIdSchema } from './family-reference-primitives';
import {
  GeneratedBrowserGamePolicyActionCandidateValues,
  GeneratedBrowserGamePolicyCompilerModeValues,
  GeneratedBrowserGamePolicyConfidenceValues,
  GeneratedBrowserGamePolicyReasonCodeValues,
  GeneratedBrowserGamePolicyTargetKindValues,
} from './generated-browser-policy-control-catalog-contracts';

export const BrowserGamePolicyEvidenceRefsSchema = Schema.Array(ParentEvidenceReferenceIdSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected browser game policy evidence refs')
);
export const BrowserGamePolicyAnalysisRefsSchema = Schema.Array(NonEmptyStringSchema);
export const BrowserGamePolicyMobileCapabilityRefsSchema = Schema.Array(NonEmptyStringSchema);
export const BrowserGamePolicyParentRuleRefsSchema = Schema.Array(NonEmptyStringSchema);
export const BrowserGamePolicyScheduleRefsSchema = Schema.Array(NonEmptyStringSchema);

export const BrowserGamePolicyCompileRequestIdSchema = withParser(
  brandedNonEmptyStringSchema('BrowserGamePolicyCompileRequestId')
);
export const BrowserGamePolicyDecisionCandidateIdSchema = withParser(
  brandedNonEmptyStringSchema('BrowserGamePolicyDecisionCandidateId')
);

export const BrowserGamePolicyTargetKindSchema = withParser(
  Schema.Literal(...GeneratedBrowserGamePolicyTargetKindValues)
);

export const BrowserGamePolicyActionCandidateSchema = withParser(
  Schema.Literal(...GeneratedBrowserGamePolicyActionCandidateValues)
);

export const BrowserGamePolicyReasonCodeSchema = withParser(
  Schema.Literal(...GeneratedBrowserGamePolicyReasonCodeValues)
);

export const BrowserGamePolicyCompilerModeSchema = withParser(
  Schema.Literal(...GeneratedBrowserGamePolicyCompilerModeValues)
);
export const BrowserGamePolicyConfidenceSchema = withParser(
  Schema.Literal(...GeneratedBrowserGamePolicyConfidenceValues)
);

export const BrowserGamePolicyReasonCodesSchema = Schema.Array(BrowserGamePolicyReasonCodeSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected browser game policy reason codes')
);
