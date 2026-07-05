import type { ReactElement, ReactNode } from 'react';
import { type ParentScreenEvidenceSettingsUiProof } from '../generated/parent-ui-screen-bridge';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import { PortalDetails } from '@ocentra-parent/portal-domain/details';
import { type PortalDisplayText } from '@ocentra-parent/portal-domain/display-text';
import {
  type ScreenSettingsServiceResponse,
  type ScreenSettingsServiceRequestId,
} from './screen-settings-service-command-state';

export function ScreenSettingsServiceCommandCard({
  commandEnabled,
  onRefresh,
  onSave,
  pendingRequestId,
  proof,
  response,
  serviceStatus,
}: {
  readonly commandEnabled: boolean;
  readonly onRefresh: () => void;
  readonly onSave: () => void;
  readonly pendingRequestId: ScreenSettingsServiceRequestId | null;
  readonly proof: ParentScreenEvidenceSettingsUiProof;
  readonly response: ScreenSettingsServiceResponse;
  readonly serviceStatus: ReactNode;
}): ReactElement {
  return (
    <article className={screenSettingsWritableCardClassName()}>
      <h2>{proof.serviceCommandHeading}</h2>
      <div className={PortalDom.Classes.RouteTabs}>
        <button
          className={PortalDom.Classes.ThemeToggleButton}
          disabled={!commandEnabled}
          onClick={onSave}
          type={PortalDom.ButtonType.Button}
        >
          {proof.serviceApplyActionLabel}
        </button>
        <button
          className={PortalDom.Classes.ThemeToggleButton}
          disabled={!commandEnabled}
          onClick={onRefresh}
          type={PortalDom.ButtonType.Button}
        >
          {proof.serviceRefreshActionLabel}
        </button>
      </div>
      <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
        <ScreenSettingsServiceCommandDetail label={PortalDetails.Status} value={serviceStatus} />
        <ScreenSettingsServiceCommandDetail
          label={PortalDetails.RequestId}
          value={response?.requestId ?? pendingRequestId ?? proof.serviceNoResponseStatus}
        />
        <ScreenSettingsServiceCommandDetail
          label={PortalDetails.Version}
          value={response?.setting?.settingVersion ?? null}
        />
        <ScreenSettingsServiceCommandDetail label={PortalDetails.EventId} value={response?.auditEventId ?? null} />
        <ScreenSettingsServiceCommandDetail
          label={PortalDetails.Reason}
          value={response?.message ?? response?.rejectionReason}
        />
      </dl>
    </article>
  );
}

function ScreenSettingsServiceCommandDetail({
  label,
  value,
}: {
  readonly label: PortalDisplayText;
  readonly value: ReactNode;
}): ReactElement {
  return (
    <div>
      <dt>{label}</dt>
      <dd>{value}</dd>
    </div>
  );
}

function screenSettingsWritableCardClassName() {
  return [PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(PortalDom.Classes.ClassNameSeparator);
}
