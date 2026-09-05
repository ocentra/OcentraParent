import type { ReactElement } from 'react';
import {
  decodeDisplayText,
  PortalDevTextToken,
  resolvePortalDevText,
} from '@ocentra-parent/portal-domain/display-text';
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

const AI_RUNTIME_TEXT = {
  statusTitle: decodeDisplayText('Local AI status'),
  unavailable: decodeDisplayText('Retry status to load local AI runtime and household job status.'),
} as const;

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
  const empty = intent.cards.length === 0;
  const routeAction = aiRuntimeRouteAction(actions, commandEnabled);
  const content = <AiRuntimeContent intent={intent} routeAction={routeAction} showAction={!empty} />;
  return (
    <section
      aria-label={resolvePortalDevText(PortalDevTextToken.AiRuntime)}
      className={empty ? PortalDom.Classes.ParentStatusDock : PortalDom.Classes.TrackingStatusOverlay}
      data-ocentra-ai-runtime-empty={String(empty)}
      data-ocentra-ai-runtime-panel=""
    >
      {empty ? (
        <>
          <div className={PortalDom.Classes.ParentStatusDockToolbar}>
            <div>
              <strong>{AI_RUNTIME_TEXT.statusTitle}</strong>
              <span>{AI_RUNTIME_TEXT.unavailable}</span>
            </div>
            <button
              className={PortalDom.Classes.CommandResultTab}
              onClick={routeAction.run}
              type={PortalDom.ButtonType.Button}
            >
              {routeAction.label}
            </button>
          </div>
          <details className={PortalDom.Classes.ParentStatusDockDisclosure} data-ocentra-ai-runtime-disclosure="">
            <summary>
              <span>{intent.title}</span>
              <span className={PortalDom.Classes.ParentStatusDockState}>{intent.emptyStatus}</span>
            </summary>
            {content}
          </details>
        </>
      ) : (
        content
      )}
    </section>
  );
}

function AiRuntimeContent({
  intent,
  routeAction,
  showAction,
}: {
  readonly intent: LocalAiRuntimePanelIntent;
  readonly routeAction: Readonly<{ label: string; run(): void }>;
  readonly showAction: boolean;
}): ReactElement {
  return (
    <div className={PortalDom.Classes.TrackingStatusOverlayContent}>
      <header className={PortalDom.Classes.TrackingStatusOverlayHeader}>
        <p className={PortalDom.Classes.ProductEyebrow}>{intent.eyebrow}</p>
        <h2>{intent.title}</h2>
        <p>{intent.body}</p>
        {showAction ? (
          <button
            className={PortalDom.Classes.CommandResultTab}
            type={PortalDom.ButtonType.Button}
            onClick={routeAction.run}
          >
            {routeAction.label}
          </button>
        ) : null}
      </header>
      <AiRuntimeCards intent={intent} />
    </div>
  );
}

function aiRuntimeRouteAction(
  actions: PortalRenderActions,
  commandEnabled: boolean
): { readonly label: string; readonly run: () => void } {
  if (!commandEnabled || actions.refreshRouteSnapshot === undefined) {
    return { label: resolvePortalDevText(PortalDevTextToken.RetryStatus), run: actions.reconnect };
  }
  return {
    label: resolvePortalDevText(PortalDevTextToken.GetLocalAiRuntimeStatus),
    run: () => {
      void actions.refreshRouteSnapshot?.();
    },
  };
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
