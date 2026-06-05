import { useMemo, useState, type ReactElement, type ReactNode } from 'react';
import {
  screenEvidenceSettingsWritableUiProof,
  type ScreenEvidenceSettingsUiIntent,
  type ScreenEvidenceSettingsUiIntentKey,
} from '@ocentra-parent/activity-domain/screen-evidence';
import { PortalDetails, PortalDom, type PortalDisplayText } from '@ocentra-parent/portal-domain/contracts';

type ScreenSettingsWritableDetailValue = ReactNode;

export function ScreenSettingsWritableControls(): ReactElement {
  const proof = useMemo(() => screenEvidenceSettingsWritableUiProof(), []);
  const [selectedIntentKey, setSelectedIntentKey] = useState(proof.defaultIntentKey);
  const selectedIntent = screenSettingsIntentByKey(proof.intents, selectedIntentKey);

  return (
    <>
      <article aria-label={proof.title} className={screenSettingsWritableCardClassName()}>
        <h2>{proof.title}</h2>
        <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
          <ScreenSettingsWritableDetail label={PortalDetails.Status} value={proof.validationStatusValue} />
          <ScreenSettingsWritableDetail label={PortalDetails.Reason} value={proof.note} />
        </dl>
      </article>
      <article className={screenSettingsWritableCardClassName()}>
        <h2>{proof.intentLegend}</h2>
        <div className={PortalDom.Classes.RouteTabs}>
          {proof.intents.map((intent) => (
            <button
              key={intent.intentKey}
              aria-pressed={intent.intentKey === selectedIntent.intentKey}
              className={screenSettingsIntentClassName(intent.intentKey, selectedIntent.intentKey)}
              onClick={() => {
                setSelectedIntentKey(intent.intentKey);
              }}
            >
              {intent.label}
            </button>
          ))}
        </div>
        <p>{selectedIntent.detail}</p>
      </article>
      <ScreenSettingsDraftCard proofHeading={proof.draftHeading} selectedIntent={selectedIntent} />
      <ScreenSettingsRetentionCard proofHeading={proof.retentionHeading} selectedIntent={selectedIntent} />
      <article className={screenSettingsWritableCardClassName()}>
        <h2>{proof.validationStatusLabel}</h2>
        <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
          <ScreenSettingsWritableDetail label={PortalDetails.Status} value={proof.validationStatusValue} />
          <ScreenSettingsWritableDetail label={PortalDetails.Reason} value={selectedIntent.setting.reason} />
        </dl>
      </article>
    </>
  );
}

function ScreenSettingsDraftCard({
  proofHeading,
  selectedIntent,
}: {
  readonly proofHeading: ReactNode;
  readonly selectedIntent: ScreenEvidenceSettingsUiIntent;
}): ReactElement {
  const setting = selectedIntent.setting;
  return (
    <article className={screenSettingsWritableCardClassName()}>
      <h2>{proofHeading}</h2>
      <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
        <ScreenSettingsWritableDetail
          label={PortalDetails.Status}
          value={readableBoolean(setting.screenAnalysisEnabled)}
        />
        <ScreenSettingsWritableDetail label={PortalDetails.DryRun} value={setting.analysisMode} />
        <ScreenSettingsWritableDetail label={PortalDetails.ExecutionState} value={setting.cadenceSeconds} />
        <ScreenSettingsWritableDetail
          label={PortalDetails.Events}
          value={setting.enabledTriggers.join(PortalDom.Classes.ClassNameSeparator)}
        />
        <ScreenSettingsWritableDetail label={PortalDetails.Source} value={setting.allowedCaptureScope} />
        <ScreenSettingsWritableDetail label={PortalDetails.Custody} value={setting.temporaryImageTtlSeconds} />
        <ScreenSettingsWritableDetail
          label={PortalDetails.PolicyPreview}
          value={readableBoolean(setting.policyUseEnabled)}
        />
        <ScreenSettingsWritableDetail label={PortalDetails.PrivacyMode} value={setting.redactionMode} />
        <ScreenSettingsWritableDetail
          label={PortalDetails.DeletedEvidence}
          value={readableBoolean(!setting.retainRawImage)}
        />
      </dl>
    </article>
  );
}

function ScreenSettingsRetentionCard({
  proofHeading,
  selectedIntent,
}: {
  readonly proofHeading: ReactNode;
  readonly selectedIntent: ScreenEvidenceSettingsUiIntent;
}): ReactElement {
  const boundary = selectedIntent.remoteBoundarySetting;
  return (
    <article className={screenSettingsWritableCardClassName()}>
      <h2>{proofHeading}</h2>
      <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
        <ScreenSettingsWritableDetail
          label={PortalDetails.DeletedEvidence}
          value={boundary.rawScreenshotRetentionMode}
        />
        <ScreenSettingsWritableDetail label={PortalDetails.ActiveState} value={boundary.liveViewMode} />
        <ScreenSettingsWritableDetail
          label={PortalDetails.Transport}
          value={readableBoolean(boundary.rawScreenshotRemoteUploadEnabled)}
        />
        <ScreenSettingsWritableDetail label={PortalDetails.Destination} value={boundary.remoteSummaryMode} />
        <ScreenSettingsWritableDetail
          label={PortalDetails.Custody}
          value={boundary.remoteSummaryDestinationCustodyState}
        />
      </dl>
    </article>
  );
}

function ScreenSettingsWritableDetail({
  label,
  value,
}: {
  readonly label: PortalDisplayText;
  readonly value: ScreenSettingsWritableDetailValue;
}): ReactElement {
  return (
    <div>
      <dt>{label}</dt>
      <dd>{value}</dd>
    </div>
  );
}

function screenSettingsIntentByKey(
  intents: readonly ScreenEvidenceSettingsUiIntent[],
  intentKey: ScreenEvidenceSettingsUiIntentKey
): ScreenEvidenceSettingsUiIntent {
  const selectedIntent = intents.find((intent) => intent.intentKey === intentKey);
  if (selectedIntent === undefined) {
    throw new Error(intentKey);
  }
  return selectedIntent;
}

function screenSettingsIntentClassName(
  intentKey: ScreenEvidenceSettingsUiIntentKey,
  selectedIntentKey: ScreenEvidenceSettingsUiIntentKey
) {
  return [
    PortalDom.Classes.ThemeToggleButton,
    intentKey === selectedIntentKey ? PortalDom.Classes.ThemeToggleButtonActive : PortalDom.Classes.ThemeToggleButton,
  ].join(PortalDom.Classes.ClassNameSeparator);
}

function screenSettingsWritableCardClassName() {
  return [PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(PortalDom.Classes.ClassNameSeparator);
}

function readableBoolean(value: boolean): ReactNode {
  return value ? PortalDom.Attributes.True : PortalDom.Attributes.False;
}
