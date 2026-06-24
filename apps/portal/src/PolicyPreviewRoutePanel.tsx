import type { ReactElement } from 'react';
import { type PortalRoute as PortalRouteValue } from '@ocentra-parent/schema-domain/portal-contracts';
import { type DisplayText as PortalDisplayText } from '@ocentra-parent/schema-domain/text-contracts';
import { PortalDevTextToken, resolvePortalDevText } from '@ocentra-parent/schema-domain/text-portal-dev';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import { PortalDetails } from '@ocentra-parent/portal-domain/details';
import type { PortalShellParentAccessState } from '@ocentra-parent/portal-domain/parent-portal-shell-status';
import {
  createPolicyPreviewPanelIntent,
  type PolicyPreviewPanelCard,
  type PolicyPreviewPanelDetail,
  type PolicyPreviewPanelIntent,
} from '@ocentra-parent/portal-domain/policy-preview-panel';
import { isPortalPolicyPreviewRoute } from '@ocentra-parent/portal-domain/routes';
import type { PortalRenderActions } from './portal-actions';
import type { PortalLiveActivityState } from './live-activity-state';

export function shouldRenderPolicyPreviewRoute(route: PortalRouteValue): boolean {
  return isPortalPolicyPreviewRoute(route);
}

export function PolicyPreviewRoutePanel({
  actions,
  commandEnabled,
  liveActivity,
  parentAccessState,
}: {
  readonly actions: PortalRenderActions;
  readonly commandEnabled: boolean;
  readonly liveActivity: PortalLiveActivityState;
  readonly parentAccessState: PortalShellParentAccessState;
}): ReactElement {
  const intent = createPolicyPreviewPanelIntent(
    liveActivity.policyPreviewEvent,
    liveActivity.policyPreviewReadModel,
    parentAccessState
  );
  return (
    <section
      aria-label={resolvePortalDevText(PortalDevTextToken.PolicyPreview)}
      className={PortalDom.Classes.TrackingStatusOverlay}
    >
      <div className={PortalDom.Classes.TrackingStatusOverlayContent}>
        <header className={PortalDom.Classes.TrackingStatusOverlayHeader}>
          <p className={PortalDom.Classes.ProductEyebrow}>{PortalDetails.PolicyPreview}</p>
          <h2>{intent.title}</h2>
          <p>{intent.body}</p>
          <button
            className={PortalDom.Classes.CommandResultTab}
            disabled={!commandEnabled}
            type={PortalDom.ButtonType.Button}
            onClick={() => void actions.refreshRouteSnapshot?.()}
          >
            {resolvePortalDevText(PortalDevTextToken.GetPolicyPreviewReadModel)}
          </button>
        </header>
        <div
          className={[PortalDom.Classes.ProductDashboard, PortalDom.Classes.TrackingStatusOverlayGrid].join(
            PortalDom.Classes.ClassNameSeparator
          )}
        >
          <PolicyPreviewSummaryCard intent={intent} />
          {intent.cards.length === 0 ? (
            <PolicyPreviewEmptyCard intent={intent} />
          ) : (
            intent.cards.map((card, index) => <PolicyPreviewCard key={`${String(card.title)}:${index}`} card={card} />)
          )}
        </div>
      </div>
    </section>
  );
}

function PolicyPreviewSummaryCard({ intent }: { readonly intent: PolicyPreviewPanelIntent }): ReactElement {
  return (
    <article className={policyPreviewCardClassName()}>
      <h2>{PortalDetails.PolicyPreview}</h2>
      <p>{intent.summary}</p>
      <PolicyPreviewDetails details={intent.summaryDetails} />
    </article>
  );
}

function PolicyPreviewEmptyCard({ intent }: { readonly intent: PolicyPreviewPanelIntent }): ReactElement {
  return (
    <article className={policyPreviewCardClassName()}>
      <h2>{PortalDetails.Status}</h2>
      <p>{intent.emptyMessage}</p>
      <PolicyPreviewDetails
        details={[
          {
            label: PortalDetails.ProductClaim,
            value: intent.productClaim,
          },
        ]}
      />
    </article>
  );
}

function PolicyPreviewCard({ card }: { readonly card: PolicyPreviewPanelCard }): ReactElement {
  return (
    <article className={policyPreviewCardClassName()}>
      <h2>{card.title}</h2>
      <p>{card.summary}</p>
      <PolicyPreviewDetails details={card.details} />
    </article>
  );
}

function PolicyPreviewDetails({ details }: { readonly details: readonly PolicyPreviewPanelDetail[] }): ReactElement {
  return (
    <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
      {details.map((detail, index) => (
        <PolicyPreviewDetail key={`${String(detail.label)}:${index}`} label={detail.label} value={detail.value} />
      ))}
    </dl>
  );
}

function PolicyPreviewDetail({
  label,
  value,
}: {
  readonly label: PortalDisplayText;
  readonly value: PortalDisplayText;
}): ReactElement {
  return (
    <div>
      <dt>{label}</dt>
      <dd>{value}</dd>
    </div>
  );
}

function policyPreviewCardClassName(): string {
  return [PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(PortalDom.Classes.ClassNameSeparator);
}
