import { useMemo, useState, type ReactElement, type ReactNode } from 'react';
import { parentScreenEvidenceSettingsWritableUiProof as screenEvidenceSettingsWritableUiProof } from '../generated/parent-ui-screen-bridge';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import { PortalDetails } from '@ocentra-parent/portal-domain/details';
import { PortalFormatting } from '@ocentra-parent/portal-domain/formatting';
import { type PortalDisplayText } from '@ocentra-parent/portal-domain/display-text';
import type { PortalRenderActions } from './portal-actions';
import {
  createScreenSettingsGetCommandDraft,
  createScreenSettingsReplaceCommandDraft,
  decodeScreenSettingsServiceResponseSnapshot,
  matchingScreenSettingsServiceResponse,
  screenSettingsBaseVersionForReplace,
  type ScreenSettingsServiceRequestId,
  screenSettingsServiceStatusText,
} from './screen-settings-service-command-state';
import { ScreenSettingsServiceCommandCard } from './ScreenSettingsServiceCommandCard';

type ScreenSettingsWritableDetailValue = ReactNode;
type ScreenEvidenceSettingsWritableProof = ReturnType<typeof screenEvidenceSettingsWritableUiProof>;
type ScreenEvidenceSettingsUiIntent = ScreenEvidenceSettingsWritableProof['intents'][number];
type ScreenEvidenceSettingsUiIntentKey = ScreenEvidenceSettingsWritableProof['defaultIntentKey'];

export function ScreenSettingsWritableControls({
  actions,
  commandEnabled,
  serviceResponseSnapshot,
}: {
  readonly actions: PortalRenderActions;
  readonly commandEnabled: boolean;
  readonly serviceResponseSnapshot: unknown | null;
}): ReactElement {
  const proof = useMemo(() => screenEvidenceSettingsWritableUiProof(), []);
  const [selectedIntentKey, setSelectedIntentKey] = useState(proof.defaultIntentKey);
  const [requestSequence, setRequestSequence] = useState(1);
  const [pendingRequestId, setPendingRequestId] = useState<ScreenSettingsServiceRequestId | null>(null);
  const selectedIntent = screenSettingsIntentByKey(proof.intents, selectedIntentKey);
  const serviceResponse = matchingScreenSettingsServiceResponse(
    decodeScreenSettingsServiceResponseSnapshot(serviceResponseSnapshot),
    pendingRequestId
  );
  const serviceStatus = screenSettingsServiceStatusText({
    commandEnabled,
    pendingRequestId,
    proof,
    response: serviceResponse,
  });
  const submitDraft = (draft: ReturnType<typeof createScreenSettingsReplaceCommandDraft>): void => {
    setPendingRequestId(draft.requestId);
    setRequestSequence(requestSequence + 1);
    if (draft.action === 'screen-settings-get-requested') {
      void actions.requestScreenSettingsGet?.(draft.payload);
      return;
    }
    void actions.requestScreenSettingsReplace?.(draft.payload);
  };
  const sendReplace = (): void => {
    submitDraft(
      createScreenSettingsReplaceCommandDraft({
        baseSettingVersion: screenSettingsBaseVersionForReplace(serviceResponse),
        sequence: requestSequence,
        setting: selectedIntent.setting,
      })
    );
  };
  const sendRefresh = (): void => {
    const draft = createScreenSettingsGetCommandDraft(requestSequence);
    submitDraft(draft);
  };

  return (
    <>
      <ScreenSettingsProofCard proof={proof} />
      <ScreenSettingsIntentPickerCard
        proof={proof}
        selectedIntent={selectedIntent}
        onSelectIntent={setSelectedIntentKey}
      />
      <ScreenSettingsDraftCard
        proofHeading={proof.draftHeading}
        triggerHeading={proof.draftTriggerHeading}
        selectedIntent={selectedIntent}
      />
      <ScreenSettingsRetentionCard proofHeading={proof.retentionHeading} selectedIntent={selectedIntent} />
      <ScreenSettingsServiceCommandCard
        commandEnabled={commandEnabled}
        onRefresh={sendRefresh}
        onSave={sendReplace}
        pendingRequestId={pendingRequestId}
        proof={proof}
        response={serviceResponse}
        serviceStatus={serviceStatus}
      />
      <ScreenSettingsValidationCard proof={proof} selectedIntent={selectedIntent} />
    </>
  );
}

function ScreenSettingsProofCard({ proof }: { readonly proof: ScreenEvidenceSettingsWritableProof }): ReactElement {
  return (
    <article aria-label={proof.title} className={screenSettingsWritableCardClassName()}>
      <h2>{proof.title}</h2>
      <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
        <ScreenSettingsWritableDetail label={PortalDetails.Status} value={proof.validationStatusValue} />
        <ScreenSettingsWritableDetail label={PortalDetails.Reason} value={proof.note} />
      </dl>
    </article>
  );
}

function ScreenSettingsIntentPickerCard({
  proof,
  selectedIntent,
  onSelectIntent,
}: {
  readonly proof: ScreenEvidenceSettingsWritableProof;
  readonly selectedIntent: ScreenEvidenceSettingsUiIntent;
  readonly onSelectIntent: (intentKey: ScreenEvidenceSettingsUiIntentKey) => void;
}): ReactElement {
  return (
    <article className={screenSettingsWritableCardClassName()}>
      <h2>{proof.intentLegend}</h2>
      <div>
        <div className={PortalDom.Classes.RouteTabs}>
          {proof.intents.map((intent) => (
            <button
              key={intent.intentKey}
              aria-pressed={intent.intentKey === selectedIntent.intentKey}
              className={screenSettingsIntentClassName(intent.intentKey, selectedIntent.intentKey)}
              onClick={() => onSelectIntent(intent.intentKey)}
            >
              {intent.label}
            </button>
          ))}
        </div>
        <p>{selectedIntent.detail}</p>
      </div>
    </article>
  );
}

function ScreenSettingsDraftCard({
  proofHeading,
  triggerHeading,
  selectedIntent,
}: {
  readonly proofHeading: ReactNode;
  readonly triggerHeading: ReactNode;
  readonly selectedIntent: ScreenEvidenceSettingsUiIntent;
}): ReactElement {
  const setting = selectedIntent.setting;
  return (
    <>
      <article className={screenSettingsWritableCardClassName()}>
        <h2>{proofHeading}</h2>
        <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
          <ScreenSettingsWritableDetail
            label={PortalDetails.Status}
            value={readableBoolean(setting.screenAnalysisEnabled)}
          />
          <ScreenSettingsWritableDetail label={PortalDetails.DryRun} value={setting.analysisMode} />
          <ScreenSettingsWritableDetail label={PortalDetails.ExecutionState} value={setting.cadenceSeconds} />
          <ScreenSettingsWritableDetail label={PortalDetails.Source} value={setting.allowedCaptureScope} />
          <ScreenSettingsWritableDetail label={PortalDetails.PrivacyMode} value={setting.redactionMode} />
        </dl>
      </article>
      <article className={screenSettingsWritableCardClassName()}>
        <h2>{triggerHeading}</h2>
        <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
          <ScreenSettingsWritableDetail
            label={PortalDetails.Events}
            value={setting.enabledTriggers.join(PortalFormatting.EventDetailSeparator)}
          />
          <ScreenSettingsWritableDetail label={PortalDetails.Custody} value={setting.temporaryImageTtlSeconds} />
          <ScreenSettingsWritableDetail
            label={PortalDetails.PolicyPreview}
            value={readableBoolean(setting.policyUseEnabled)}
          />
          <ScreenSettingsWritableDetail
            label={PortalDetails.DeletedEvidence}
            value={readableBoolean(!setting.retainRawImage)}
          />
        </dl>
      </article>
    </>
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

function ScreenSettingsValidationCard({
  proof,
  selectedIntent,
}: {
  readonly proof: ScreenEvidenceSettingsWritableProof;
  readonly selectedIntent: ScreenEvidenceSettingsUiIntent;
}): ReactElement {
  return (
    <article className={screenSettingsWritableCardClassName()}>
      <h2>{proof.validationStatusLabel}</h2>
      <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
        <ScreenSettingsWritableDetail label={PortalDetails.Status} value={proof.validationStatusValue} />
        <ScreenSettingsWritableDetail label={PortalDetails.Reason} value={selectedIntent.setting.reason} />
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
  if (intentKey !== selectedIntentKey) {
    return PortalDom.Classes.ThemeToggleButton;
  }
  return [PortalDom.Classes.ThemeToggleButton, PortalDom.Classes.ThemeToggleButtonActive].join(
    PortalDom.Classes.ClassNameSeparator
  );
}

function screenSettingsWritableCardClassName() {
  return [PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(PortalDom.Classes.ClassNameSeparator);
}

function readableBoolean(value: boolean): ReactNode {
  return value ? PortalDom.Attributes.True : PortalDom.Attributes.False;
}
