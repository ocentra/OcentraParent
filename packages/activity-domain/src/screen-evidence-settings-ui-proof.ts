import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  ScreenAnalysisParentSettingSchema,
  ScreenEvidenceRemoteBoundarySettingSchema,
} from './screen-evidence-settings';
import { ScreenEvidenceSchemaVersion } from './screen-evidence-primitives';

const ScreenSettingsUiTextSchema = Schema.String.pipe(Schema.minLength(1));

export const ScreenEvidenceSettingsUiIntentKeySchema = withParser(
  Schema.Literal('disabledLocalSummary', 'observeOnlyLocalSummary', 'strictDryRunLocalSummary')
);

export const ScreenEvidenceSettingsUiIntentSchema = withParser(
  Schema.Struct({
    intentKey: ScreenEvidenceSettingsUiIntentKeySchema,
    label: ScreenSettingsUiTextSchema,
    detail: ScreenSettingsUiTextSchema,
    setting: ScreenAnalysisParentSettingSchema,
    remoteBoundarySetting: ScreenEvidenceRemoteBoundarySettingSchema,
  })
);

export const ScreenEvidenceSettingsUiProofSchema = withParser(
  Schema.Struct({
    title: ScreenSettingsUiTextSchema,
    note: ScreenSettingsUiTextSchema,
    intentLegend: ScreenSettingsUiTextSchema,
    draftHeading: ScreenSettingsUiTextSchema,
    draftTriggerHeading: ScreenSettingsUiTextSchema,
    retentionHeading: ScreenSettingsUiTextSchema,
    validationStatusLabel: ScreenSettingsUiTextSchema,
    validationStatusValue: ScreenSettingsUiTextSchema,
    defaultIntentKey: ScreenEvidenceSettingsUiIntentKeySchema,
    intents: Schema.Array(ScreenEvidenceSettingsUiIntentSchema).pipe(
      Schema.filter((value) => value.length === 3 || 'Expected three parent Screen settings UI intents')
    ),
  })
);

export type ScreenEvidenceSettingsUiIntentKey = Infer<typeof ScreenEvidenceSettingsUiIntentKeySchema>;
export type ScreenEvidenceSettingsUiIntent = Infer<typeof ScreenEvidenceSettingsUiIntentSchema>;
export type ScreenEvidenceSettingsUiProof = Infer<typeof ScreenEvidenceSettingsUiProofSchema>;

const ScreenSettingsUiCopy = {
  title: 'Writable screen settings proof',
  note: 'Parent Settings can build a schema-valid local screen-summary intent. This proof does not persist the intent to the child service.',
  intentLegend: 'Intent',
  draftHeading: 'Draft mode',
  draftTriggerHeading: 'Triggers and custody',
  retentionHeading: 'Remote boundary',
  validationStatusLabel: 'Parser status',
  validationStatusValue: 'schema-valid local parent intent',
  disabledLabel: 'Keep screen analysis disabled',
  disabledDetail: 'No cadence capture, trigger capture, strict mode, or policy use can run while disabled.',
  observeLabel: 'Enable observe-only summaries',
  observeDetail: 'Five-minute local summaries can be reviewed by the parent, but policy handoff remains disabled.',
  strictLabel: 'Enable strict dry-run review',
  strictDetail:
    'One-minute cadence, selected triggers, local OCR, redaction, and policy dry-run become explicit parent intent.',
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
      ...DisabledRemoteBoundarySetting,
      parentSettingRef: setting.changedByParentRef,
      settingVersion: setting.settingVersion,
      changedByParentRef: setting.changedByParentRef,
      changedAt: setting.changedAt,
    },
  };
}
