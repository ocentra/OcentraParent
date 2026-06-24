import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import {
  createSocialAlertReportPanelIntent,
  type SocialAlertReportPanelDetail,
  type SocialAlertReportPanelIntent,
  type SocialAlertReportPanelRow,
} from '@ocentra-parent/portal-domain/social-alert-report-panel';
import {
  createSocialAlertReportParentSurfacePanelIntent,
  type SocialAlertReportParentSurfacePanelDetail,
  type SocialAlertReportParentSurfacePanelIntent,
} from '@ocentra-parent/portal-domain/social-alert-report-parent-surface-panel';
import {
  createSocialParentNotificationDeliveryPanelIntent,
  type SocialParentNotificationDeliveryPanelDetail,
  type SocialParentNotificationDeliveryPanelIntent,
} from '@ocentra-parent/portal-domain/social-parent-notification-delivery-panel';
import { PortalDetails } from '@ocentra-parent/portal-domain/details';
import { isPortalBrowserParentSurfaceRoute } from '@ocentra-parent/portal-domain/routes';
import { type PortalRoute as PortalRouteValue } from '@ocentra-parent/schema-domain/portal-contracts';
import {
  type BrowserSocialProviderReceiptIngestionReadinessStatusDetail,
  type BrowserSocialProviderReceiptIngestionReadinessStatusIntent,
} from '@ocentra-parent/portal-domain/browser-social-provider-receipt-ingestion-readiness-status';
import {
  type BrowserSocialProviderReceiptStreamStatusDetail,
  type BrowserSocialProviderReceiptStreamStatusIntent,
} from '@ocentra-parent/portal-domain/browser-social-provider-receipt-stream-status';
import { type ReactElement } from 'react';
import type { PortalLiveActivityState } from './live-activity-state';
import type { PortalRenderActions } from './portal-actions';
import {
  createBrowserActionIntentStreamStatusIntent,
  type BrowserActionIntentStreamStatusDetail,
  type BrowserActionIntentStreamStatusIntent,
} from '@ocentra-parent/portal-domain/browser-action-intent-stream-status';

export function shouldRenderSocialAlertReportRoute(route: PortalRouteValue): boolean {
  return isPortalBrowserParentSurfaceRoute(route);
}

export function SocialAlertReportRoutePanel({
  actions,
  commandEnabled,
  liveActivity,
  socialAlertReportSnapshot,
  socialAlertReportParentSurfaceSnapshot,
  socialParentNotificationDeliverySnapshot,
}: {
  readonly actions: PortalRenderActions;
  readonly commandEnabled: boolean;
  readonly liveActivity: PortalLiveActivityState;
  readonly socialAlertReportSnapshot: unknown | null;
  readonly socialAlertReportParentSurfaceSnapshot: unknown | null;
  readonly socialParentNotificationDeliverySnapshot: unknown | null;
}): ReactElement {
  const intent = createSocialAlertReportPanelIntent(socialAlertReportSnapshot);
  const notificationIntent =
    createSocialParentNotificationDeliveryPanelIntent(socialParentNotificationDeliverySnapshot);
  const parentSurfaceIntent =
    createSocialAlertReportParentSurfacePanelIntent(socialAlertReportParentSurfaceSnapshot);
  return (
    <section aria-label={intent.title} className={PortalDom.Classes.TrackingStatusOverlay}>
      <div className={PortalDom.Classes.TrackingStatusOverlayContent}>
        <header className={PortalDom.Classes.TrackingStatusOverlayHeader}>
          <p className={PortalDom.Classes.ProductEyebrow}>{intent.eyebrow}</p>
          <h2>{intent.title}</h2>
          <p>{intent.body}</p>
          <button
            className={PortalDom.Classes.CommandResultTab}
            disabled={!commandEnabled}
            onClick={() => void actions.refreshRouteSnapshot?.()}
            type={PortalDom.ButtonType.Button}
          >
            {intent.title}
          </button>
          <button
            className={PortalDom.Classes.CommandResultTab}
            disabled={!commandEnabled}
            onClick={() => void actions.refreshRouteSnapshot?.()}
            type={PortalDom.ButtonType.Button}
          >
            {notificationIntent.title}
          </button>
          <button
            className={PortalDom.Classes.CommandResultTab}
            disabled={!commandEnabled}
            onClick={() => void actions.refreshRouteSnapshot?.()}
            type={PortalDom.ButtonType.Button}
          >
            {parentSurfaceIntent.title}
          </button>
        </header>
        <div
          className={[PortalDom.Classes.ProductDashboard, PortalDom.Classes.TrackingStatusOverlayGrid].join(
            PortalDom.Classes.ClassNameSeparator
          )}
        >
          <SocialAlertReportSummaryCard intent={intent} />
          {intent.rows.length === 0 ? (
            <SocialAlertReportEmptyCard intent={intent} />
          ) : (
            intent.rows.map((row) => <SocialAlertReportRowCard key={row.key} row={row} />)
          )}
          <SocialParentNotificationDeliveryCards intent={notificationIntent} />
          <SocialAlertReportParentSurfaceCards intent={parentSurfaceIntent} />
          <BrowserReceiptStatusCards liveActivity={liveActivity} />
        </div>
      </div>
    </section>
  );
}

function SocialAlertReportSummaryCard({ intent }: { readonly intent: SocialAlertReportPanelIntent }): ReactElement {
  return (
    <article className={cardClassName()}>
      <h2>{intent.summary}</h2>
      <SocialAlertReportDetails details={intent.metrics} />
    </article>
  );
}

function SocialAlertReportEmptyCard({ intent }: { readonly intent: SocialAlertReportPanelIntent }): ReactElement {
  return (
    <article className={cardClassName()}>
      <h2>{intent.emptyMessage}</h2>
      <SocialAlertReportDetails
        details={[
          { label: PortalDetails.Status, value: intent.state },
          { label: PortalDetails.ProductClaim, value: intent.productClaim },
        ]}
      />
    </article>
  );
}

function SocialAlertReportRowCard({ row }: { readonly row: SocialAlertReportPanelRow }): ReactElement {
  return (
    <article className={cardClassName()}>
      <h2>{row.title}</h2>
      <SocialAlertReportDetails details={row.details} />
    </article>
  );
}

function SocialParentNotificationDeliveryCards({
  intent,
}: {
  readonly intent: SocialParentNotificationDeliveryPanelIntent;
}): ReactElement {
  return (
    <>
      <article className={cardClassName()}>
        <h2>{intent.summary}</h2>
        <SocialAlertReportDetails details={intent.details} />
      </article>
      {intent.rows.map((row) => (
        <article className={cardClassName()} key={row.key}>
          <h2>{row.title}</h2>
          <SocialAlertReportDetails details={row.details} />
        </article>
      ))}
    </>
  );
}

function SocialAlertReportParentSurfaceCards({
  intent,
}: {
  readonly intent: SocialAlertReportParentSurfacePanelIntent;
}): ReactElement {
  return (
    <>
      <article className={cardClassName()}>
        <h2>{intent.summary}</h2>
        <SocialAlertReportDetails details={intent.details} />
      </article>
      {intent.rows.map((row) => (
        <article className={cardClassName()} key={row.key}>
          <h2>{row.title}</h2>
          <SocialAlertReportDetails details={row.details} />
        </article>
      ))}
    </>
  );
}

function BrowserReceiptStatusCards({ liveActivity }: { readonly liveActivity: PortalLiveActivityState }): ReactElement {
  return (
    <>
      {browserReceiptStatusIntents(liveActivity).map((intent) => (
        <BrowserReceiptStatusCard key={intent.title} intent={intent} />
      ))}
    </>
  );
}

function BrowserReceiptStatusCard({ intent }: { readonly intent: BrowserReceiptStatusIntent }): ReactElement {
  return (
    <article className={cardClassName()}>
      <h2>{intent.title}</h2>
      <SocialAlertReportDetails details={intent.details} />
    </article>
  );
}

function browserReceiptStatusIntents(liveActivity: PortalLiveActivityState): readonly BrowserReceiptStatusIntent[] {
  return [
    liveActivity.browserRuntimeEventChainStream === null
      ? null
      : createBrowserActionIntentStreamStatusIntent(liveActivity.browserRuntimeEventChainStream),
    liveActivity.browserSocialProviderReceiptStreamStatusIntent,
    liveActivity.browserSocialProviderReceiptIngestionReadinessStatusIntent,
  ].filter((intent): intent is BrowserReceiptStatusIntent => intent !== null);
}

type BrowserReceiptStatusIntent =
  | BrowserActionIntentStreamStatusIntent
  | BrowserSocialProviderReceiptStreamStatusIntent
  | BrowserSocialProviderReceiptIngestionReadinessStatusIntent;

type SocialAlertReportRenderableDetail =
  | SocialAlertReportPanelDetail
  | BrowserActionIntentStreamStatusDetail
  | BrowserSocialProviderReceiptStreamStatusDetail
  | BrowserSocialProviderReceiptIngestionReadinessStatusDetail
  | SocialParentNotificationDeliveryPanelDetail
  | SocialAlertReportParentSurfacePanelDetail;

function SocialAlertReportDetails({
  details,
}: {
  readonly details: readonly SocialAlertReportRenderableDetail[];
}): ReactElement {
  return (
    <dl>
      {details.map((detail, index) => (
        <div key={`${detail.label}-${index}`}>
          <dt>{detail.label}</dt>
          <dd>{detail.value}</dd>
        </div>
      ))}
    </dl>
  );
}

function cardClassName(): string {
  return [PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(PortalDom.Classes.ClassNameSeparator);
}
