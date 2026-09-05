import { useMemo, useState, type ReactElement, type ReactNode } from 'react';
import { parentScreenEvidenceSettingsWritableUiProof as screenEvidenceSettingsWritableUiProof } from '../generated/parent-ui-screen-bridge';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import { PortalFormatting } from '@ocentra-parent/portal-domain/formatting';
import { decodeDisplayText, type PortalDisplayText } from '@ocentra-parent/portal-domain/display-text';
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

const SCREEN_SETTINGS_TEXT = {
  mode: decodeDisplayText('Screen analysis mode'),
  selectedSetting: decodeDisplayText('Selected setting'),
  triggersAndPrivacy: decodeDisplayText('Triggers and privacy'),
  rawImagesAndRemoteAccess: decodeDisplayText('Raw images and remote access'),
  enabled: decodeDisplayText('Enabled'),
  analysisMode: decodeDisplayText('Analysis mode'),
  captureInterval: decodeDisplayText('Capture interval'),
  captureScope: decodeDisplayText('Capture scope'),
  redaction: decodeDisplayText('Redaction'),
  triggers: decodeDisplayText('Triggers'),
  temporaryImageLifetime: decodeDisplayText('Temporary image lifetime'),
  policyDryRun: decodeDisplayText('Policy dry run'),
  rawImageRetained: decodeDisplayText('Raw image retained'),
  rawImageRetention: decodeDisplayText('Raw image retention'),
  liveView: decodeDisplayText('Live view'),
  rawImageUpload: decodeDisplayText('Raw image upload'),
  remoteSummary: decodeDisplayText('Remote summary'),
  remoteCustody: decodeDisplayText('Remote custody'),
  none: 'None',
  off: 'Off',
  on: 'On',
} as const;

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
      <ScreenSettingsIntentPickerCard
        commandEnabled={commandEnabled}
        intents={proof.intents}
        selectedIntent={selectedIntent}
        onSelectIntent={setSelectedIntentKey}
      />
      <ScreenSettingsDraftCard selectedIntent={selectedIntent} />
      <ScreenSettingsRetentionCard selectedIntent={selectedIntent} />
      <ScreenSettingsServiceCommandCard
        commandEnabled={commandEnabled}
        onRefresh={sendRefresh}
        onSave={sendReplace}
        pendingRequestId={pendingRequestId}
        proof={proof}
        response={serviceResponse}
        serviceStatus={serviceStatus}
      />
    </>
  );
}

function ScreenSettingsIntentPickerCard({
  commandEnabled,
  intents,
  selectedIntent,
  onSelectIntent,
}: {
  readonly commandEnabled: boolean;
  readonly intents: readonly ScreenEvidenceSettingsUiIntent[];
  readonly selectedIntent: ScreenEvidenceSettingsUiIntent;
  readonly onSelectIntent: (intentKey: ScreenEvidenceSettingsUiIntentKey) => void;
}): ReactElement {
  return (
    <article className={screenSettingsWritableCardClassName()}>
      <h2>{SCREEN_SETTINGS_TEXT.mode}</h2>
      <div>
        <div className={PortalDom.Classes.RouteTabs}>
          {intents.map((intent) => (
            <button
              key={intent.intentKey}
              aria-pressed={intent.intentKey === selectedIntent.intentKey}
              className={screenSettingsIntentClassName(intent.intentKey, selectedIntent.intentKey)}
              disabled={!commandEnabled}
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
  selectedIntent,
}: {
  readonly selectedIntent: ScreenEvidenceSettingsUiIntent;
}): ReactElement {
  const setting = selectedIntent.setting;
  return (
    <>
      <article className={screenSettingsWritableCardClassName()}>
        <h2>{SCREEN_SETTINGS_TEXT.selectedSetting}</h2>
        <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
          <ScreenSettingsWritableDetail
            label={SCREEN_SETTINGS_TEXT.enabled}
            value={readableBoolean(setting.screenAnalysisEnabled)}
          />
          <ScreenSettingsWritableDetail
            label={SCREEN_SETTINGS_TEXT.analysisMode}
            value={readableToken(setting.analysisMode)}
          />
          <ScreenSettingsWritableDetail
            label={SCREEN_SETTINGS_TEXT.captureInterval}
            value={setting.screenAnalysisEnabled ? readableSeconds(setting.cadenceSeconds) : SCREEN_SETTINGS_TEXT.off}
          />
          <ScreenSettingsWritableDetail
            label={SCREEN_SETTINGS_TEXT.captureScope}
            value={readableToken(setting.allowedCaptureScope)}
          />
          <ScreenSettingsWritableDetail
            label={SCREEN_SETTINGS_TEXT.redaction}
            value={readableToken(setting.redactionMode)}
          />
        </dl>
      </article>
      <article className={screenSettingsWritableCardClassName()}>
        <h2>{SCREEN_SETTINGS_TEXT.triggersAndPrivacy}</h2>
        <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
          <ScreenSettingsWritableDetail
            label={SCREEN_SETTINGS_TEXT.triggers}
            value={readableTokens(setting.enabledTriggers)}
          />
          <ScreenSettingsWritableDetail
            label={SCREEN_SETTINGS_TEXT.temporaryImageLifetime}
            value={readableSeconds(setting.temporaryImageTtlSeconds)}
          />
          <ScreenSettingsWritableDetail
            label={SCREEN_SETTINGS_TEXT.policyDryRun}
            value={readableBoolean(setting.policyUseEnabled)}
          />
          <ScreenSettingsWritableDetail
            label={SCREEN_SETTINGS_TEXT.rawImageRetained}
            value={readableBoolean(setting.retainRawImage)}
          />
        </dl>
      </article>
    </>
  );
}

function ScreenSettingsRetentionCard({
  selectedIntent,
}: {
  readonly selectedIntent: ScreenEvidenceSettingsUiIntent;
}): ReactElement {
  const boundary = selectedIntent.remoteBoundarySetting;
  return (
    <article className={screenSettingsWritableCardClassName()}>
      <h2>{SCREEN_SETTINGS_TEXT.rawImagesAndRemoteAccess}</h2>
      <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
        <ScreenSettingsWritableDetail
          label={SCREEN_SETTINGS_TEXT.rawImageRetention}
          value={readableToken(boundary.rawScreenshotRetentionMode)}
        />
        <ScreenSettingsWritableDetail
          label={SCREEN_SETTINGS_TEXT.liveView}
          value={readableToken(boundary.liveViewMode)}
        />
        <ScreenSettingsWritableDetail
          label={SCREEN_SETTINGS_TEXT.rawImageUpload}
          value={readableBoolean(boundary.rawScreenshotRemoteUploadEnabled)}
        />
        <ScreenSettingsWritableDetail
          label={SCREEN_SETTINGS_TEXT.remoteSummary}
          value={readableToken(boundary.remoteSummaryMode)}
        />
        <ScreenSettingsWritableDetail
          label={SCREEN_SETTINGS_TEXT.remoteCustody}
          value={readableToken(boundary.remoteSummaryDestinationCustodyState)}
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
  return value ? SCREEN_SETTINGS_TEXT.on : SCREEN_SETTINGS_TEXT.off;
}

function readableSeconds(value: number): string {
  if (value === 60) return '1 minute';
  if (value % 60 === 0) return `${value / 60} minutes`;
  return `${value} seconds`;
}

function readableTokens(values: readonly string[]): string {
  if (values.length === 0) return SCREEN_SETTINGS_TEXT.none;
  return values.map(readableToken).join(PortalFormatting.EventDetailSeparator);
}

function readableToken(value: string): string {
  const spaced = value
    .replace(/([a-z0-9])([A-Z])/gu, '$1 $2')
    .replace(/[-_]+/gu, ' ')
    .toLowerCase();
  return spaced.length === 0 ? SCREEN_SETTINGS_TEXT.none : `${spaced[0]?.toUpperCase() ?? ''}${spaced.slice(1)}`;
}
