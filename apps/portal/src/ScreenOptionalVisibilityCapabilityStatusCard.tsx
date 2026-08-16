import { useMemo, type ReactElement, type ReactNode } from 'react';
import {
  ParentScreenOptionalVisibilityCapabilityProofGeneratedAt as ScreenOptionalVisibilityCapabilityProofGeneratedAt,
  parentScreenOptionalVisibilityCapabilityStatusProof as screenOptionalVisibilityCapabilityStatusProof,
  type ParentScreenOptionalVisibilityCapabilityStatus as ScreenOptionalVisibilityCapabilityStatus,
} from '../generated/parent-ui-screen-bridge';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import { PortalDetails, PortalReadableValues } from '@ocentra-parent/portal-domain/details';
import { type PortalDisplayText } from '@ocentra-parent/portal-domain/display-text';

type OptionalVisibilityStatusValue = ReactNode;

export function ScreenOptionalVisibilityCapabilityStatusCard(): ReactElement {
  const proof = useMemo(
    () => screenOptionalVisibilityCapabilityStatusProof(ScreenOptionalVisibilityCapabilityProofGeneratedAt),
    []
  );
  return (
    <>
      <article aria-label={proof.proofId} className={optionalVisibilityCardClassName()}>
        <h2>{proof.proofId}</h2>
        <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
          <OptionalVisibilityDetail label={PortalDetails.Status} value={proof.generatedAt} />
          <OptionalVisibilityDetail label={PortalDetails.Reason} value={proof.nonClaims[0]} />
        </dl>
      </article>
      {proof.rows.map((row) => (
        <OptionalVisibilityCapabilityRow key={row.parentSettingRef} row={row} />
      ))}
    </>
  );
}

function OptionalVisibilityCapabilityRow({
  row,
}: {
  readonly row: ScreenOptionalVisibilityCapabilityStatus;
}): ReactElement {
  return (
    <article className={optionalVisibilityCardClassName()}>
      <h2>{row.parentSettingRef}</h2>
      <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
        <OptionalVisibilityDetail label={PortalDetails.Status} value={row.readinessState} />
        <OptionalVisibilityDetail label={PortalDetails.Capability} value={row.capabilityKind} />
        <OptionalVisibilityDetail label={PortalDetails.Source} value={rowSourceLabel(row)} />
        <OptionalVisibilityDetail label={PortalDetails.Custody} value={rowCustody(row)} />
        <OptionalVisibilityDetail label={PortalDetails.Transport} value={rowTransport(row)} />
        <OptionalVisibilityDetail label={PortalDetails.RuntimeReference} value={rowPermissionEvidence(row)} />
        <OptionalVisibilityDetail
          label={PortalDetails.DeletedEvidence}
          value={readableBoolean(!row.rawFramesRetained)}
        />
        <OptionalVisibilityDetail label={PortalDetails.Reason} value={row.reason} />
      </dl>
    </article>
  );
}

function OptionalVisibilityDetail({
  label,
  value,
}: {
  readonly label: PortalDisplayText;
  readonly value: OptionalVisibilityStatusValue;
}): ReactElement {
  return (
    <div>
      <dt>{label}</dt>
      <dd>{value}</dd>
    </div>
  );
}

function rowSourceLabel(row: ScreenOptionalVisibilityCapabilityStatus): OptionalVisibilityStatusValue {
  return (
    row.rawRetentionSetting?.sourceLabel ?? row.liveViewSetting?.sourceLabel ?? PortalReadableValues['unavailable']
  );
}

function rowCustody(row: ScreenOptionalVisibilityCapabilityStatus): OptionalVisibilityStatusValue {
  return (
    row.rawRetentionSetting?.custodyState ?? row.liveViewSetting?.custodyState ?? PortalReadableValues['unavailable']
  );
}

function rowTransport(row: ScreenOptionalVisibilityCapabilityStatus): OptionalVisibilityStatusValue {
  return (
    row.liveViewSetting?.transportMode ??
    row.rawRetentionSetting?.retentionBehavior ??
    PortalReadableValues['unavailable']
  );
}

function rowPermissionEvidence(row: ScreenOptionalVisibilityCapabilityStatus): OptionalVisibilityStatusValue {
  return row.liveViewPermissionGate?.permissionEvidenceKind ?? PortalReadableValues['unavailable'];
}

function optionalVisibilityCardClassName() {
  return [PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(PortalDom.Classes.ClassNameSeparator);
}

function readableBoolean(value: boolean): OptionalVisibilityStatusValue {
  return value ? PortalDom.Attributes.True : PortalDom.Attributes.False;
}
