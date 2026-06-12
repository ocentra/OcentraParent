import { Schema, withParser } from '@ocentra-parent/schema-domain/effect';

const NonEmptyBrowserControlId = Schema.String.pipe(Schema.minLength(1));

export const BrowserControlManifestIdSchema = withParser(
  NonEmptyBrowserControlId.pipe(Schema.brand('BrowserControlManifestId'))
);
export const BrowserControlSectionIdSchema = withParser(
  NonEmptyBrowserControlId.pipe(Schema.brand('BrowserControlSectionId'))
);
export const BrowserControlFieldIdSchema = withParser(
  NonEmptyBrowserControlId.pipe(Schema.brand('BrowserControlFieldId'))
);
export const BrowserControlOptionIdSchema = withParser(
  NonEmptyBrowserControlId.pipe(Schema.brand('BrowserControlOptionId'))
);
export const BrowserControlPolicyIdSchema = withParser(
  NonEmptyBrowserControlId.pipe(Schema.brand('BrowserControlPolicyId'))
);
export const BrowserControlRuleIdSchema = withParser(
  NonEmptyBrowserControlId.pipe(Schema.brand('BrowserControlRuleId'))
);
export const BrowserControlScheduleIdSchema = withParser(
  NonEmptyBrowserControlId.pipe(Schema.brand('BrowserControlScheduleId'))
);
export const BrowserControlBudgetIdSchema = withParser(
  NonEmptyBrowserControlId.pipe(Schema.brand('BrowserControlBudgetId'))
);
export const BrowserControlCapabilityIdSchema = withParser(
  NonEmptyBrowserControlId.pipe(Schema.brand('BrowserControlCapabilityId'))
);
export const BrowserControlRevisionIdSchema = withParser(
  NonEmptyBrowserControlId.pipe(Schema.brand('BrowserControlRevisionId'))
);
export const BrowserControlHashIdSchema = withParser(
  NonEmptyBrowserControlId.pipe(Schema.brand('BrowserControlHashId'))
);
export const BrowserControlAuditEventIdSchema = withParser(
  NonEmptyBrowserControlId.pipe(Schema.brand('BrowserControlAuditEventId'))
);
export const BrowserControlRequestIdSchema = withParser(
  NonEmptyBrowserControlId.pipe(Schema.brand('BrowserControlRequestId'))
);

export type BrowserControlManifestId = typeof BrowserControlManifestIdSchema.Type;
export type BrowserControlSectionId = typeof BrowserControlSectionIdSchema.Type;
export type BrowserControlFieldId = typeof BrowserControlFieldIdSchema.Type;
export type BrowserControlOptionId = typeof BrowserControlOptionIdSchema.Type;
export type BrowserControlPolicyId = typeof BrowserControlPolicyIdSchema.Type;
export type BrowserControlRuleId = typeof BrowserControlRuleIdSchema.Type;
export type BrowserControlScheduleId = typeof BrowserControlScheduleIdSchema.Type;
export type BrowserControlBudgetId = typeof BrowserControlBudgetIdSchema.Type;
export type BrowserControlCapabilityId = typeof BrowserControlCapabilityIdSchema.Type;
export type BrowserControlRevisionId = typeof BrowserControlRevisionIdSchema.Type;
export type BrowserControlHashId = typeof BrowserControlHashIdSchema.Type;
export type BrowserControlAuditEventId = typeof BrowserControlAuditEventIdSchema.Type;
export type BrowserControlRequestId = typeof BrowserControlRequestIdSchema.Type;
