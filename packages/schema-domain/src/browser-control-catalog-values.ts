import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  GeneratedBrowserControlApprovalRequiredForValues,
  GeneratedBrowserControlApprovalUnansweredDefaultValues,
  GeneratedBrowserControlAuditRequiredFieldValues,
  GeneratedBrowserControlBrowserGameApprovalModeValues,
  GeneratedBrowserControlBrowserGamePolicyModeValues,
  GeneratedBrowserControlBudgetCountingModeValues,
  GeneratedBrowserControlCustodyAllowedUseValues,
  GeneratedBrowserControlDownloadBlockedTypeValues,
  GeneratedBrowserControlEvidenceNeverCollectValues,
  GeneratedBrowserControlEvidenceUrlScopeValues,
  GeneratedBrowserControlManagedBrowserBridgeRequirementValues,
  GeneratedBrowserControlManagedBrowserFamilyValues,
  GeneratedBrowserControlManagedBrowserIntegrationMechanismValues,
  GeneratedBrowserControlManagedBrowserLaunchModeValues,
  GeneratedBrowserControlManagedBrowserProfileModeValues,
  GeneratedBrowserControlManagedPolicyWriterControlValues,
  GeneratedBrowserControlManagedPolicyWriterFallbackValues,
  GeneratedBrowserControlReportVisibleFieldValues,
  GeneratedBrowserControlRetentionExactUrlValues,
  GeneratedBrowserControlRuleActionValues,
  GeneratedBrowserControlUnmanagedBrowserClassificationTargetValues,
} from './generated-browser-policy-control-catalog-contracts';

export const BrowserControlManagedBrowserFamilySchema = withParser(
  Schema.Literal(...GeneratedBrowserControlManagedBrowserFamilyValues)
);

export const BrowserControlManagedBrowserLaunchModeSchema = withParser(
  Schema.Literal(...GeneratedBrowserControlManagedBrowserLaunchModeValues)
);

export const BrowserControlManagedBrowserProfileModeSchema = withParser(
  Schema.Literal(...GeneratedBrowserControlManagedBrowserProfileModeValues)
);

export const BrowserControlManagedBrowserBridgeRequirementSchema = withParser(
  Schema.Literal(...GeneratedBrowserControlManagedBrowserBridgeRequirementValues)
);

export const BrowserControlManagedBrowserIntegrationMechanismSchema = withParser(
  Schema.Literal(...GeneratedBrowserControlManagedBrowserIntegrationMechanismValues)
);

export const BrowserControlManagedPolicyWriterControlSchema = withParser(
  Schema.Literal(...GeneratedBrowserControlManagedPolicyWriterControlValues)
);

export const BrowserControlManagedPolicyWriterFallbackSchema = withParser(
  Schema.Literal(...GeneratedBrowserControlManagedPolicyWriterFallbackValues)
);

export const BrowserControlUnmanagedBrowserClassificationTargetSchema = withParser(
  Schema.Literal(...GeneratedBrowserControlUnmanagedBrowserClassificationTargetValues)
);

export const BrowserControlEvidenceUrlScopeSchema = withParser(
  Schema.Literal(...GeneratedBrowserControlEvidenceUrlScopeValues)
);

export const BrowserControlEvidenceNeverCollectSchema = withParser(
  Schema.Literal(...GeneratedBrowserControlEvidenceNeverCollectValues)
);

export const BrowserControlRuleActionSchema = withParser(Schema.Literal(...GeneratedBrowserControlRuleActionValues));

export const BrowserControlBrowserGamePolicyModeSchema = withParser(
  Schema.Literal(...GeneratedBrowserControlBrowserGamePolicyModeValues)
);

export const BrowserControlBrowserGameApprovalModeSchema = withParser(
  Schema.Literal(...GeneratedBrowserControlBrowserGameApprovalModeValues)
);

export const BrowserControlBudgetCountingModeSchema = withParser(
  Schema.Literal(...GeneratedBrowserControlBudgetCountingModeValues)
);

export const BrowserControlDownloadBlockedTypeSchema = withParser(
  Schema.Literal(...GeneratedBrowserControlDownloadBlockedTypeValues)
);

export const BrowserControlApprovalRequiredForSchema = withParser(
  Schema.Literal(...GeneratedBrowserControlApprovalRequiredForValues)
);

export const BrowserControlApprovalUnansweredDefaultSchema = withParser(
  Schema.Literal(...GeneratedBrowserControlApprovalUnansweredDefaultValues)
);

export const BrowserControlReportVisibleFieldSchema = withParser(
  Schema.Literal(...GeneratedBrowserControlReportVisibleFieldValues)
);

export const BrowserControlRetentionExactUrlSchema = withParser(
  Schema.Literal(...GeneratedBrowserControlRetentionExactUrlValues)
);

export const BrowserControlCustodyAllowedUseSchema = withParser(
  Schema.Literal(...GeneratedBrowserControlCustodyAllowedUseValues)
);

export const BrowserControlAuditRequiredFieldSchema = withParser(
  Schema.Literal(...GeneratedBrowserControlAuditRequiredFieldValues)
);

export type BrowserControlManagedBrowserFamily = Infer<typeof BrowserControlManagedBrowserFamilySchema>;
export type BrowserControlManagedBrowserLaunchMode = Infer<typeof BrowserControlManagedBrowserLaunchModeSchema>;
export type BrowserControlManagedBrowserProfileMode = Infer<typeof BrowserControlManagedBrowserProfileModeSchema>;
export type BrowserControlManagedBrowserBridgeRequirement = Infer<
  typeof BrowserControlManagedBrowserBridgeRequirementSchema
>;
export type BrowserControlManagedBrowserIntegrationMechanism = Infer<
  typeof BrowserControlManagedBrowserIntegrationMechanismSchema
>;
export type BrowserControlManagedPolicyWriterControl = Infer<typeof BrowserControlManagedPolicyWriterControlSchema>;
export type BrowserControlManagedPolicyWriterFallback = Infer<typeof BrowserControlManagedPolicyWriterFallbackSchema>;
export type BrowserControlUnmanagedBrowserClassificationTarget = Infer<
  typeof BrowserControlUnmanagedBrowserClassificationTargetSchema
>;
export type BrowserControlEvidenceUrlScope = Infer<typeof BrowserControlEvidenceUrlScopeSchema>;
export type BrowserControlEvidenceNeverCollect = Infer<typeof BrowserControlEvidenceNeverCollectSchema>;
export type BrowserControlRuleAction = Infer<typeof BrowserControlRuleActionSchema>;
export type BrowserControlBrowserGamePolicyMode = Infer<typeof BrowserControlBrowserGamePolicyModeSchema>;
export type BrowserControlBrowserGameApprovalMode = Infer<typeof BrowserControlBrowserGameApprovalModeSchema>;
export type BrowserControlBudgetCountingMode = Infer<typeof BrowserControlBudgetCountingModeSchema>;
export type BrowserControlDownloadBlockedType = Infer<typeof BrowserControlDownloadBlockedTypeSchema>;
export type BrowserControlApprovalRequiredFor = Infer<typeof BrowserControlApprovalRequiredForSchema>;
export type BrowserControlApprovalUnansweredDefault = Infer<typeof BrowserControlApprovalUnansweredDefaultSchema>;
export type BrowserControlReportVisibleField = Infer<typeof BrowserControlReportVisibleFieldSchema>;
export type BrowserControlRetentionExactUrl = Infer<typeof BrowserControlRetentionExactUrlSchema>;
export type BrowserControlCustodyAllowedUse = Infer<typeof BrowserControlCustodyAllowedUseSchema>;
export type BrowserControlAuditRequiredField = Infer<typeof BrowserControlAuditRequiredFieldSchema>;
