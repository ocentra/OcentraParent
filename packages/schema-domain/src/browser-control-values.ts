import { type Infer, NonEmptyStringSchema, Schema, withParser } from './effect';
import {
  GeneratedBrowserControlApprovalStateValues,
  GeneratedBrowserControlAuditStateValues,
  GeneratedBrowserControlCapabilityStateValues,
  GeneratedBrowserControlConditionKindValues,
  GeneratedBrowserControlDefaultPostureValues,
  GeneratedBrowserControlDownloadStateValues,
  GeneratedBrowserControlEvidenceProofLevelValues,
  GeneratedBrowserControlExecutionModeValues,
  GeneratedBrowserControlKindValues,
  GeneratedBrowserControlManagedBrowserModeValues,
  GeneratedBrowserControlManagementModeValues,
  GeneratedBrowserControlPatchOperationValues,
  GeneratedBrowserControlProofFallbackValues,
  GeneratedBrowserControlRejectionReasonValues,
  GeneratedBrowserControlReportStateValues,
  GeneratedBrowserControlRetentionStateValues,
  GeneratedBrowserControlUnmanagedBrowserModeValues,
  GeneratedBrowserControlUpdateKindValues,
  GeneratedBrowserControlUpdateStatusValues,
  GeneratedBrowserControlUrlTargetTypeValues,
  GeneratedBrowserControlWritesToPath,
} from './generated-browser-policy-control-catalog-contracts';

export const BrowserControlSchemaKnownWritesToPathSchema = withParser(
  NonEmptyStringSchema.pipe(Schema.brand('BrowserControlSchemaKnownWritesToPath'))
);

export const BrowserControlFieldValueSchema = withParser(
  Schema.Union(NonEmptyStringSchema, Schema.Number, Schema.Boolean, Schema.Array(NonEmptyStringSchema), Schema.Null)
);

export const BrowserControlKindSchema = withParser(Schema.Literal(...GeneratedBrowserControlKindValues));
export const BrowserControlConditionKindSchema = withParser(
  Schema.Literal(...GeneratedBrowserControlConditionKindValues)
);
export const BrowserControlDefaultPostureSchema = withParser(
  Schema.Literal(...GeneratedBrowserControlDefaultPostureValues)
);
export const BrowserControlExecutionModeSchema = withParser(
  Schema.Literal(...GeneratedBrowserControlExecutionModeValues)
);
export const BrowserControlManagementModeSchema = withParser(
  Schema.Literal(...GeneratedBrowserControlManagementModeValues)
);
export const BrowserControlManagedBrowserModeSchema = withParser(
  Schema.Literal(...GeneratedBrowserControlManagedBrowserModeValues)
);
export const BrowserControlUnmanagedBrowserModeSchema = withParser(
  Schema.Literal(...GeneratedBrowserControlUnmanagedBrowserModeValues)
);
export const BrowserControlUrlTargetTypeSchema = withParser(
  Schema.Literal(...GeneratedBrowserControlUrlTargetTypeValues)
);
export const BrowserControlEvidenceProofLevelSchema = withParser(
  Schema.Literal(...GeneratedBrowserControlEvidenceProofLevelValues)
);
export const BrowserControlProofFallbackSchema = withParser(
  Schema.Literal(...GeneratedBrowserControlProofFallbackValues)
);
export const BrowserControlDownloadStateSchema = withParser(
  Schema.Literal(...GeneratedBrowserControlDownloadStateValues)
);
export const BrowserControlApprovalStateSchema = withParser(
  Schema.Literal(...GeneratedBrowserControlApprovalStateValues)
);
export const BrowserControlReportStateSchema = withParser(Schema.Literal(...GeneratedBrowserControlReportStateValues));
export const BrowserControlAuditStateSchema = withParser(Schema.Literal(...GeneratedBrowserControlAuditStateValues));
export const BrowserControlRetentionStateSchema = withParser(
  Schema.Literal(...GeneratedBrowserControlRetentionStateValues)
);
export const BrowserControlCapabilityStateSchema = withParser(
  Schema.Literal(...GeneratedBrowserControlCapabilityStateValues)
);
export const BrowserControlRejectionReasonSchema = withParser(
  Schema.Literal(...GeneratedBrowserControlRejectionReasonValues)
);
export const BrowserControlPatchOperationSchema = withParser(
  Schema.Literal(...GeneratedBrowserControlPatchOperationValues)
);
export const BrowserControlUpdateKindSchema = withParser(Schema.Literal(...GeneratedBrowserControlUpdateKindValues));
export const BrowserControlUpdateStatusSchema = withParser(
  Schema.Literal(...GeneratedBrowserControlUpdateStatusValues)
);

export type BrowserControlSchemaKnownWritesToPath = typeof BrowserControlSchemaKnownWritesToPathSchema.Type;
export type BrowserControlFieldValue = Infer<typeof BrowserControlFieldValueSchema>;
export type BrowserControlKind = Infer<typeof BrowserControlKindSchema>;
export type BrowserControlConditionKind = Infer<typeof BrowserControlConditionKindSchema>;
export type BrowserControlDefaultPosture = Infer<typeof BrowserControlDefaultPostureSchema>;
export type BrowserControlExecutionMode = Infer<typeof BrowserControlExecutionModeSchema>;
export type BrowserControlManagementMode = Infer<typeof BrowserControlManagementModeSchema>;
export type BrowserControlManagedBrowserMode = Infer<typeof BrowserControlManagedBrowserModeSchema>;
export type BrowserControlUnmanagedBrowserMode = Infer<typeof BrowserControlUnmanagedBrowserModeSchema>;
export type BrowserControlUrlTargetType = Infer<typeof BrowserControlUrlTargetTypeSchema>;
export type BrowserControlEvidenceProofLevel = Infer<typeof BrowserControlEvidenceProofLevelSchema>;
export type BrowserControlProofFallback = Infer<typeof BrowserControlProofFallbackSchema>;
export type BrowserControlDownloadState = Infer<typeof BrowserControlDownloadStateSchema>;
export type BrowserControlApprovalState = Infer<typeof BrowserControlApprovalStateSchema>;
export type BrowserControlReportState = Infer<typeof BrowserControlReportStateSchema>;
export type BrowserControlAuditState = Infer<typeof BrowserControlAuditStateSchema>;
export type BrowserControlRetentionState = Infer<typeof BrowserControlRetentionStateSchema>;
export type BrowserControlCapabilityState = Infer<typeof BrowserControlCapabilityStateSchema>;
export type BrowserControlRejectionReason = Infer<typeof BrowserControlRejectionReasonSchema>;
export type BrowserControlPatchOperation = Infer<typeof BrowserControlPatchOperationSchema>;
export type BrowserControlUpdateKind = Infer<typeof BrowserControlUpdateKindSchema>;
export type BrowserControlUpdateStatus = Infer<typeof BrowserControlUpdateStatusSchema>;

export const BrowserControlWritesToPath = Object.freeze(
  Object.fromEntries(
    Object.entries(GeneratedBrowserControlWritesToPath).map(([key, value]) => [
      key,
      BrowserControlSchemaKnownWritesToPathSchema.parse(value),
    ])
  )
) as {
  readonly [K in keyof typeof GeneratedBrowserControlWritesToPath]: BrowserControlSchemaKnownWritesToPath;
};
