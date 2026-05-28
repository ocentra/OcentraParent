import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';

const BrowserControlKnownWritesToPathLiteralSchema = Schema.Literal(
  '/browserPolicy/enabled',
  '/browserPolicy/defaultPosture',
  '/browserPolicy/managementMode',
  '/browserPolicy/managedBrowser/mode',
  '/browserPolicy/unmanagedBrowser/mode',
  '/browserPolicy/evidence/requiredProof',
  '/browserPolicy/evidence/proofFallback',
  '/browserPolicy/rules/allowedTargetTypes',
  '/browserPolicy/budgets/defaultDailyMinutes',
  '/browserPolicy/downloads/state',
  '/browserPolicy/approvals/state',
  '/browserPolicy/reports/state',
  '/browserPolicy/audit/state',
  '/browserPolicy/retention/state'
);

export const BrowserControlSchemaKnownWritesToPathSchema = withParser(
  BrowserControlKnownWritesToPathLiteralSchema.pipe(Schema.brand('BrowserControlSchemaKnownWritesToPath'))
);

export const BrowserControlFieldValueSchema = withParser(
  Schema.Union(Schema.String, Schema.Number, Schema.Boolean, Schema.Array(Schema.String), Schema.Null)
);

export const BrowserControlKindSchema = withParser(
  Schema.Literal('toggle', 'single-select', 'multi-select', 'number', 'readonly-status')
);
export const BrowserControlConditionKindSchema = withParser(
  Schema.Literal('equals', 'not-equals', 'includes', 'not-includes', 'capability-state', 'default-posture')
);
export const BrowserControlDefaultPostureSchema = withParser(
  Schema.Literal('observe', 'allow', 'limit', 'ask-parent', 'block')
);
export const BrowserControlManagementModeSchema = withParser(
  Schema.Literal('disabled', 'observe-only', 'managed-browser', 'network-assisted')
);
export const BrowserControlManagedBrowserModeSchema = withParser(
  Schema.Literal('not-required', 'preferred', 'required-for-exact-rules')
);
export const BrowserControlUnmanagedBrowserModeSchema = withParser(
  Schema.Literal('observe-only', 'network-domain-only', 'manual-review')
);
export const BrowserControlUrlTargetTypeSchema = withParser(Schema.Literal('domain', 'url-prefix', 'exact-url'));
export const BrowserControlEvidenceProofLevelSchema = withParser(
  Schema.Literal('none', 'network-domain', 'managed-active-tab', 'fresh-managed-active-tab')
);
export const BrowserControlProofFallbackSchema = withParser(
  Schema.Literal('downgrade-to-domain', 'ask-parent', 'block-until-proof', 'observe-only')
);
export const BrowserControlDownloadStateSchema = withParser(
  Schema.Literal('not-configured', 'allow', 'ask-parent', 'block')
);
export const BrowserControlApprovalStateSchema = withParser(
  Schema.Literal('not-required', 'required', 'pending', 'approved', 'denied')
);
export const BrowserControlReportStateSchema = withParser(Schema.Literal('disabled', 'daily', 'weekly', 'on-demand'));
export const BrowserControlAuditStateSchema = withParser(
  Schema.Literal('disabled', 'local-only', 'parent-visible', 'retained')
);
export const BrowserControlRetentionStateSchema = withParser(Schema.Literal('none', 'seven-days', 'thirty-days'));
export const BrowserControlCapabilityStateSchema = withParser(
  Schema.Literal('supported', 'unsupported', 'degraded', 'unavailable', 'unknown')
);
export const BrowserControlRejectionReasonSchema = withParser(
  Schema.Literal(
    'unknown-writes-to',
    'unknown-field',
    'invalid-enum-value',
    'missing-budget-or-fallback',
    'missing-managed-proof-or-fallback',
    'capability-unavailable',
    'scaffold-unavailable',
    'revision-not-found'
  )
);
export const BrowserControlPatchOperationSchema = withParser(Schema.Literal('replace'));
export const BrowserControlUpdateKindSchema = withParser(
  Schema.Literal('get', 'preview', 'patch', 'replace', 'rollback')
);
export const BrowserControlUpdateStatusSchema = withParser(Schema.Literal('accepted', 'rejected'));

export const BrowserControlWritesToPath = {
  Enabled: BrowserControlSchemaKnownWritesToPathSchema.parse('/browserPolicy/enabled'),
  DefaultPosture: BrowserControlSchemaKnownWritesToPathSchema.parse('/browserPolicy/defaultPosture'),
  ManagementMode: BrowserControlSchemaKnownWritesToPathSchema.parse('/browserPolicy/managementMode'),
  ManagedBrowserMode: BrowserControlSchemaKnownWritesToPathSchema.parse('/browserPolicy/managedBrowser/mode'),
  UnmanagedBrowserMode: BrowserControlSchemaKnownWritesToPathSchema.parse('/browserPolicy/unmanagedBrowser/mode'),
  RequiredProof: BrowserControlSchemaKnownWritesToPathSchema.parse('/browserPolicy/evidence/requiredProof'),
  ProofFallback: BrowserControlSchemaKnownWritesToPathSchema.parse('/browserPolicy/evidence/proofFallback'),
  AllowedTargetTypes: BrowserControlSchemaKnownWritesToPathSchema.parse('/browserPolicy/rules/allowedTargetTypes'),
  DailyBudgetMinutes: BrowserControlSchemaKnownWritesToPathSchema.parse('/browserPolicy/budgets/defaultDailyMinutes'),
  DownloadState: BrowserControlSchemaKnownWritesToPathSchema.parse('/browserPolicy/downloads/state'),
  ApprovalState: BrowserControlSchemaKnownWritesToPathSchema.parse('/browserPolicy/approvals/state'),
  ReportState: BrowserControlSchemaKnownWritesToPathSchema.parse('/browserPolicy/reports/state'),
  AuditState: BrowserControlSchemaKnownWritesToPathSchema.parse('/browserPolicy/audit/state'),
  RetentionState: BrowserControlSchemaKnownWritesToPathSchema.parse('/browserPolicy/retention/state'),
} as const;

export type BrowserControlSchemaKnownWritesToPath = typeof BrowserControlSchemaKnownWritesToPathSchema.Type;
export type BrowserControlFieldValue = Infer<typeof BrowserControlFieldValueSchema>;
export type BrowserControlKind = Infer<typeof BrowserControlKindSchema>;
export type BrowserControlConditionKind = Infer<typeof BrowserControlConditionKindSchema>;
export type BrowserControlDefaultPosture = Infer<typeof BrowserControlDefaultPostureSchema>;
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
