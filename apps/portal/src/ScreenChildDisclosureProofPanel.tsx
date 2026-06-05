import { useMemo, type ReactElement, type ReactNode } from 'react';
import {
  screenChildDisclosureUxProof,
  type ScreenChildDisclosureStatus,
} from '@ocentra-parent/activity-domain/screen-evidence';
import { resolveScreenChildDisclosureUxText } from '@ocentra-parent/text-domain/screen-child-disclosure-ux-text';
import {
  PortalDetails,
  PortalDom,
  PortalFormatting,
  type PortalDisplayText,
} from '@ocentra-parent/portal-domain/contracts';

type ScreenChildDisclosureDetailValue = ReactNode;

export function ScreenChildDisclosureProofPanel(): ReactElement {
  const proof = useMemo(() => screenChildDisclosureUxProof(), []);
  const title = resolveScreenChildDisclosureUxText(proof.titleTokenRef);
  const intro = resolveScreenChildDisclosureUxText(proof.introTokenRef);

  return (
    <>
      <article aria-label={title} className={screenChildDisclosureCardClassName()}>
        <h2>{title}</h2>
        <p>{intro}</p>
        <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
          <ScreenChildDisclosureDetail label={PortalDetails.Status} value={readableBoolean(proof.hiddenCaptureAllowed)} />
          <ScreenChildDisclosureDetail
            label={PortalDetails.Custody}
            value={readableBoolean(proof.localOnlyDisclosure)}
          />
          <ScreenChildDisclosureDetail
            label={PortalDetails.ProductClaim}
            value={readableBoolean(proof.productionChildAppClaimed)}
          />
        </dl>
      </article>
      {proof.statuses.map((status) => (
        <ScreenChildDisclosureStatusCard key={status.statusId} status={status} />
      ))}
    </>
  );
}

function ScreenChildDisclosureStatusCard({
  status,
}: {
  readonly status: ScreenChildDisclosureStatus;
}): ReactElement {
  const title = resolveScreenChildDisclosureUxText(status.copyRefs.titleTokenRef);
  const body = resolveScreenChildDisclosureUxText(status.copyRefs.bodyTokenRef);
  const statusText = resolveScreenChildDisclosureUxText(status.copyRefs.statusTokenRef);
  const actionText = resolveScreenChildDisclosureUxText(status.copyRefs.actionTokenRef);

  return (
    <article className={screenChildDisclosureCardClassName()}>
      <h2>{title}</h2>
      <p>{body}</p>
      <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
        <ScreenChildDisclosureDetail label={PortalDetails.Status} value={statusText} />
        <ScreenChildDisclosureDetail label={PortalDetails.ActiveState} value={status.indicator} />
        <ScreenChildDisclosureDetail label={PortalDetails.Capability} value={status.capabilityStatus} />
        <ScreenChildDisclosureDetail label={PortalDetails.Custody} value={status.custodyState} />
        <ScreenChildDisclosureDetail label={PortalDetails.Observer} value={status.delivery} />
        <ScreenChildDisclosureDetail label={PortalDetails.ChildSafeAction} value={actionText} />
        <ScreenChildDisclosureDetail
          label={PortalDetails.DeletedEvidence}
          value={[
            readableBoolean(status.rawScreenshotPathVisible),
            readableBoolean(status.rawScreenshotRemoteUploadEnabled),
          ].join(PortalFormatting.EventDetailSeparator)}
        />
        <ScreenChildDisclosureDetail label={PortalDetails.Reason} value={status.reason} />
      </dl>
    </article>
  );
}

function ScreenChildDisclosureDetail({
  label,
  value,
}: {
  readonly label: PortalDisplayText;
  readonly value: ScreenChildDisclosureDetailValue;
}): ReactElement {
  return (
    <div>
      <dt>{label}</dt>
      <dd>{value}</dd>
    </div>
  );
}

function screenChildDisclosureCardClassName() {
  return [PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(PortalDom.Classes.ClassNameSeparator);
}

function readableBoolean(value: boolean): ReactNode {
  return value ? PortalDom.Attributes.True : PortalDom.Attributes.False;
}
