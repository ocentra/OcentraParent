import type { ReactElement } from 'react';
import { PortalDevTextToken, resolvePortalDevText } from '@ocentra-parent/portal-domain/display-text';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import {
  createLocalAiRuntimePanelIntent,
  type LocalAiRuntimePanelDetail,
  type LocalAiRuntimePanelIntent,
} from '@ocentra-parent/portal-domain/local-ai-runtime-panel';
import { PortalDetails } from '@ocentra-parent/portal-domain/details';
import { isParentAiRuntimeRoute, type ParentRouteId } from '../generated/parent-ui-bridge';
import type { PortalRenderActions } from './portal-actions';
import { type PortalDisplayText } from '@ocentra-parent/portal-domain/display-text';
import type { PortalLiveActivityState } from './live-activity-state';

export function shouldRenderAiRuntimeRoute(route: ParentRouteId): boolean {
  return isParentAiRuntimeRoute(route);
}

export function AiRuntimeRoutePanel({
  actions,
  commandEnabled,
  liveActivity,
}: {
  readonly actions: PortalRenderActions;
  readonly commandEnabled: boolean;
  readonly liveActivity: PortalLiveActivityState;
}): ReactElement {
  const intent = createLocalAiRuntimePanelIntent(
    liveActivity.localAiRuntimeStatusEvent,
    liveActivity.lanAiJobEvent,
    liveActivity.activityMemoryGraphReadModel,
    liveActivity.parentAssistantBoundaryEvent
  );
  return (
    <section
      aria-label={resolvePortalDevText(PortalDevTextToken.AiRuntime)}
      className={PortalDom.Classes.TrackingStatusOverlay}
    >
      <div className={PortalDom.Classes.TrackingStatusOverlayContent}>
        <header className={PortalDom.Classes.TrackingStatusOverlayHeader}>
          <p className={PortalDom.Classes.ProductEyebrow}>{intent.eyebrow}</p>
          <h2>{intent.title}</h2>
          <p>{intent.body}</p>
          <button
            className={PortalDom.Classes.CommandResultTab}
            disabled={!commandEnabled}
            type={PortalDom.ButtonType.Button}
            onClick={() => void actions.refreshRouteSnapshot?.()}
          >
            {resolvePortalDevText(PortalDevTextToken.GetLocalAiRuntimeStatus)}
          </button>
        </header>
        <AiRuntimeCards intent={intent} />
      </div>
    </section>
  );
}

function AiRuntimeCards({ intent }: { readonly intent: LocalAiRuntimePanelIntent }): ReactElement {
  return (
    <div
      className={[PortalDom.Classes.ProductDashboard, PortalDom.Classes.TrackingStatusOverlayGrid].join(
        PortalDom.Classes.ClassNameSeparator
      )}
    >
      <AiRuntimeCard title={PortalDetails.LocalAiResult} details={intent.summaryDetails} />
      {intent.cards.length === 0 ? (
        <AiRuntimeCard
          title={intent.emptyMessage}
          details={[
            { label: PortalDetails.Status, value: intent.emptyStatus },
            { label: PortalDetails.ProductClaim, value: intent.productClaim },
          ]}
        />
      ) : (
        intent.cards.map((card) => <AiRuntimeCard key={String(card.title)} title={card.title} details={card.details} />)
      )}
    </div>
  );
}

function AiRuntimeCard({
  details,
  title,
}: {
  readonly details: readonly LocalAiRuntimePanelDetail[];
  readonly title: PortalDisplayText;
}): ReactElement {
  return (
    <article className={aiRuntimeCardClassName()}>
      <h2>{title}</h2>
      <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
        {details.map((detail) => (
          <AiRuntimeDetail key={String(detail.label)} label={detail.label} value={detail.value} />
        ))}
      </dl>
    </article>
  );
}

function AiRuntimeDetail({ label, value }: LocalAiRuntimePanelDetail): ReactElement {
  return (
    <div>
      <dt>{label}</dt>
      <dd>{value}</dd>
    </div>
  );
}

function aiRuntimeCardClassName(): string {
  return [PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(PortalDom.Classes.ClassNameSeparator);
}
