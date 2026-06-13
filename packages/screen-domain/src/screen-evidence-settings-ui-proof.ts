import {
  type Infer,
  Schema,
  withParser,
  NonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import {
  ScreenAnalysisParentSettingSchema,
  ScreenEvidenceRemoteBoundarySettingSchema,
} from './screen-evidence-settings';
import { ScreenEvidenceSchemaVersion } from './screen-evidence-primitives';

export const ScreenEvidenceSettingsUiIntentKeySchema = withParser(
  Schema.Literal(
    'disabledLocalSummary',
    'observeOnlyLocalSummary',
    'strictDryRunLocalSummary',
    'approvedRawRetentionLocalTtl'
  )
);

export const ScreenEvidenceSettingsUiIntentSchema = withParser(
  Schema.Struct({
    intentKey: ScreenEvidenceSettingsUiIntentKeySchema,
    label: NonEmptyStringSchema,
    detail: NonEmptyStringSchema,
    setting: ScreenAnalysisParentSettingSchema,
    remoteBoundarySetting: ScreenEvidenceRemoteBoundarySettingSchema,
  })
);

export const ScreenEvidenceSettingsUiProofSchema = withParser(
  Schema.Struct({
    title: NonEmptyStringSchema,
    note: NonEmptyStringSchema,
    intentLegend: NonEmptyStringSchema,
    draftHeading: NonEmptyStringSchema,
    draftTriggerHeading: NonEmptyStringSchema,
    retentionHeading: NonEmptyStringSchema,
    serviceCommandHeading: NonEmptyStringSchema,
    serviceApplyActionLabel: NonEmptyStringSchema,
    serviceRefreshActionLabel: NonEmptyStringSchema,
    servicePendingStatus: NonEmptyStringSchema,
    serviceAcceptedStatus: NonEmptyStringSchema,
    serviceRejectedStatus: NonEmptyStringSchema,
    serviceDisconnectedStatus: NonEmptyStringSchema,
    serviceNoResponseStatus: NonEmptyStringSchema,
    validationStatusLabel: NonEmptyStringSchema,
    validationStatusValue: NonEmptyStringSchema,
    defaultIntentKey: ScreenEvidenceSettingsUiIntentKeySchema,
    intents: Schema.Array(ScreenEvidenceSettingsUiIntentSchema).pipe(
      Schema.filter((value) => value.length === 4 || 'Expected four parent Screen settings UI intents')
    ),
  })
);

export type ScreenEvidenceSettingsUiIntentKey = Infer<typeof ScreenEvidenceSettingsUiIntentKeySchema>;
export type ScreenEvidenceSettingsUiIntent = Infer<typeof ScreenEvidenceSettingsUiIntentSchema>;
export type ScreenEvidenceSettingsUiProof = Infer<typeof ScreenEvidenceSettingsUiProofSchema>;

const ScreenSettingsUiCopy = {
  title: 'Writable screen settings proof',
  note: 'Parent Settings can build a schema-valid local screen-summary intent and submit it to the child service command path.',
  intentLegend: 'Intent',
  draftHeading: 'Draft mode',
  draftTriggerHeading: 'Triggers and custody',
  retentionHeading: 'Remote boundary',
  serviceCommandHeading: 'Service command',
  serviceApplyActionLabel: 'Save selected screen setting',
  serviceRefreshActionLabel: 'Refresh persisted screen setting',
  servicePendingStatus: 'waiting for service response',
  serviceAcceptedStatus: 'service accepted persisted setting',
  serviceRejectedStatus: 'service rejected setting',
  serviceDisconnectedStatus: 'service command unavailable while disconnected',
  serviceNoResponseStatus: 'no service settings response yet',
  validationStatusLabel: 'Parser status',
  validationStatusValue: 'schema-valid local parent intent',
  disabledLabel: 'Keep screen analysis disabled',
  disabledDetail: 'No cadence capture, trigger capture, strict mode, or policy use can run while disabled.',
  observeLabel: 'Enable observe-only summaries',
  observeDetail: 'Five-minute local summaries can be reviewed by the parent, but policy handoff remains disabled.',
  strictLabel: 'Enable strict dry-run review',
  strictDetail:
    'One-minute cadence, selected triggers, local OCR, redaction, and policy dry-run become explicit parent intent.',
  rawRetentionLabel: 'Approve local short-TTL retention',
  rawRetentionDetail:
    'Parent-approved local raw screenshot retention uses a short TTL and keeps delete-after-success and delete-after-expiry required.',
} as const;

const DisabledSetting = ScreenAnalysisParentSettingSchema.parse({
  schemaVersion: ScreenEvidenceSchemaVersion,
  screenAnalysisEnabled: false,
  analysisMode: 'observeOnly',
  cadenceCaptureEnabled: false,
  cadenceSeconds: 300,
  strictModeEnabled: false,
  triggerCaptureEnabled: false,
  enabledTriggers: [],
  allowedCaptureScope: 'unsupported',
  ocrTextEnabled: false,
  ocrTextSnippetLimit: 0,
  redactionMode: 'disabled',
  ocrTextRetentionMode: 'disabled',
  credentialSuppressionEnabled: true,
  piiRedactionEnabled: false,
  temporaryImageTtlSeconds: 300,
  maxRetryCount: 0,
  deleteAfterSuccess: true,
  deleteAfterExpiry: true,
  retainRawImage: false,
  policyUseEnabled: false,
  changedByParentRef: 'screen-settings-ui-parent-disabled',
  changedAt: '2026-06-04T23:50:00Z',
  settingVersion: 1,
  reason: 'parent kept local screen summaries disabled',
});

const ObserveOnlySetting = ScreenAnalysisParentSettingSchema.parse({
  ...DisabledSetting,
  screenAnalysisEnabled: true,
  cadenceCaptureEnabled: true,
  cadenceSeconds: 300,
  triggerCaptureEnabled: true,
  enabledTriggers: ['foregroundAppChange', 'policyAmbiguity'],
  allowedCaptureScope: 'activeWindow',
  ocrTextEnabled: true,
  ocrTextSnippetLimit: 3,
  redactionMode: 'localSensitiveText',
  ocrTextRetentionMode: 'redactedSnippets',
  piiRedactionEnabled: true,
  maxRetryCount: 2,
  changedByParentRef: 'screen-settings-ui-parent-observe',
  settingVersion: 2,
  reason: 'parent enabled observe-only local screen summaries',
});

const StrictDryRunSetting = ScreenAnalysisParentSettingSchema.parse({
  ...ObserveOnlySetting,
  analysisMode: 'policyDryRun',
  cadenceSeconds: 60,
  strictModeEnabled: true,
  enabledTriggers: ['foregroundAppChange', 'managedBrowserUrlChange', 'appGameForegroundStart', 'policyAmbiguity'],
  ocrTextSnippetLimit: 5,
  policyUseEnabled: true,
  changedByParentRef: 'screen-settings-ui-parent-strict',
  settingVersion: 3,
  reason: 'parent enabled strict local screen summary dry run',
});

const ApprovedRawRetentionSetting = ScreenAnalysisParentSettingSchema.parse({
  ...StrictDryRunSetting,
  temporaryImageTtlSeconds: 120,
  retainRawImage: true,
  changedByParentRef: 'screen-settings-ui-parent-raw-retention-local-ttl',
  settingVersion: 4,
  reason: 'parent approved local short TTL raw screenshot retention',
});

const DisabledRemoteBoundarySetting = ScreenEvidenceRemoteBoundarySettingSchema.parse({
  schemaVersion: ScreenEvidenceSchemaVersion,
  parentSettingRef: 'screen-settings-ui-remote-boundary',
  settingVersion: 1,
  rawScreenshotRetentionMode: 'disabled',
  liveViewMode: 'disabled',
  rawScreenshotRemoteUploadEnabled: false,
  remoteSummaryMode: 'disabled',
  remoteSummaryRedactedOnly: true,
  parentApprovedRemoteSummary: false,
  remoteSummaryApprovalRef: null,
  remoteSummaryDestinationCustodyState: 'unavailable',
  changedByParentRef: 'screen-settings-ui-parent-disabled',
  changedAt: '2026-06-04T23:50:00Z',
  reason: 'local screen summary settings do not enable raw retention or live view',
});

export function screenEvidenceSettingsWritableUiProof(): ScreenEvidenceSettingsUiProof {
  return ScreenEvidenceSettingsUiProofSchema.parse({
    title: ScreenSettingsUiCopy.title,
    note: ScreenSettingsUiCopy.note,
    intentLegend: ScreenSettingsUiCopy.intentLegend,
    draftHeading: ScreenSettingsUiCopy.draftHeading,
    draftTriggerHeading: ScreenSettingsUiCopy.draftTriggerHeading,
    retentionHeading: ScreenSettingsUiCopy.retentionHeading,
    serviceCommandHeading: ScreenSettingsUiCopy.serviceCommandHeading,
    serviceApplyActionLabel: ScreenSettingsUiCopy.serviceApplyActionLabel,
    serviceRefreshActionLabel: ScreenSettingsUiCopy.serviceRefreshActionLabel,
    servicePendingStatus: ScreenSettingsUiCopy.servicePendingStatus,
    serviceAcceptedStatus: ScreenSettingsUiCopy.serviceAcceptedStatus,
    serviceRejectedStatus: ScreenSettingsUiCopy.serviceRejectedStatus,
    serviceDisconnectedStatus: ScreenSettingsUiCopy.serviceDisconnectedStatus,
    serviceNoResponseStatus: ScreenSettingsUiCopy.serviceNoResponseStatus,
    validationStatusLabel: ScreenSettingsUiCopy.validationStatusLabel,
    validationStatusValue: ScreenSettingsUiCopy.validationStatusValue,
    defaultIntentKey: 'disabledLocalSummary',
    intents: [
      intent(
        'disabledLocalSummary',
        ScreenSettingsUiCopy.disabledLabel,
        ScreenSettingsUiCopy.disabledDetail,
        DisabledSetting
      ),
      intent(
        'observeOnlyLocalSummary',
        ScreenSettingsUiCopy.observeLabel,
        ScreenSettingsUiCopy.observeDetail,
        ObserveOnlySetting
      ),
      intent(
        'strictDryRunLocalSummary',
        ScreenSettingsUiCopy.strictLabel,
        ScreenSettingsUiCopy.strictDetail,
        StrictDryRunSetting
      ),
      intent(
        'approvedRawRetentionLocalTtl',
        ScreenSettingsUiCopy.rawRetentionLabel,
        ScreenSettingsUiCopy.rawRetentionDetail,
        ApprovedRawRetentionSetting
      ),
    ],
  });
}

function intent(
  intentKey: ScreenEvidenceSettingsUiIntentKey,
  label: string,
  detail: string,
  setting: typeof DisabledSetting
): ScreenEvidenceSettingsUiIntent {
  return {
    intentKey,
    label,
    detail,
    setting,
    remoteBoundarySetting: {
      ...remoteBoundaryForSetting(setting),
      parentSettingRef: setting.changedByParentRef,
      settingVersion: setting.settingVersion,
      changedByParentRef: setting.changedByParentRef,
      changedAt: setting.changedAt,
    },
  };
}

function remoteBoundaryForSetting(setting: typeof DisabledSetting) {
  if (setting.retainRawImage) {
    return ScreenEvidenceRemoteBoundarySettingSchema.parse({
      ...DisabledRemoteBoundarySetting,
      rawScreenshotRetentionMode: 'parentApprovedLocalShortTtl',
      reason: 'parent approved local short TTL raw screenshot retention without raw remote upload',
    });
  }
  return DisabledRemoteBoundarySetting;
}

