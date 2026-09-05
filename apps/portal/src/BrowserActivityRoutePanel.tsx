import { type ReactElement } from 'react';
import {
  decodeDisplayText,
  PortalDevTextToken,
  resolvePortalDevText,
} from '@ocentra-parent/portal-domain/display-text';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import { ParentRoute, type ParentRouteId } from '../generated/parent-ui-bridge';
import type { PortalLiveActivityState } from './live-activity-state';
import type { PortalRenderActions } from './portal-actions';

const BROWSER_ROUTE_TEXT = {
  ariaLabel: decodeDisplayText('Browser activity status'),
  eyebrow: decodeDisplayText('Rust service read model'),
  title: decodeDisplayText('Browser activity and managed-session status'),
  body: decodeDisplayText(
    'Current browser activity, managed-session capability, and evidence custody reported by the local service.'
  ),
  unavailableTitle: decodeDisplayText('Browser status unavailable'),
  unavailableBody: decodeDisplayText(
    'No Rust browser read model has been reported for this route. Start or reconnect the local service, then refresh.'
  ),
  unavailableManagedBody: decodeDisplayText(
    'No browser, domain, URL, session, or intervention state is inferred while the service snapshot is unavailable.'
  ),
  unavailableManagedTitle: decodeDisplayText('Managed session'),
  unavailableEvidenceBody: decodeDisplayText(
    'No browser evidence, observation time, custody, or visibility is inferred without a service-owned evidence row.'
  ),
  unavailableEvidenceTitle: decodeDisplayText('Evidence status'),
  unavailableActivityBody: decodeDisplayText(
    'No browser activity rows, summary, or generated time is shown until the Rust read model reports them.'
  ),
  unavailableActivityTitle: decodeDisplayText('Activity rows'),
  refresh: decodeDisplayText('Refresh browser status'),
  notReported: decodeDisplayText('Not reported'),
  managedBrowser: decodeDisplayText('Managed browser'),
  noDegradedReason: decodeDisplayText('No degraded reason reported'),
  capability: decodeDisplayText('Capability'),
  browser: decodeDisplayText('Browser'),
  profileLifecycle: decodeDisplayText('Profile lifecycle'),
  bridgeKind: decodeDisplayText('Bridge kind'),
  custody: decodeDisplayText('Custody'),
  checked: decodeDisplayText('Checked'),
  browserEvidence: decodeDisplayText('Browser evidence'),
  evidenceBody: decodeDisplayText(
    'Only bounded status and custody metadata are shown here; exact URLs are not rendered.'
  ),
  visibility: decodeDisplayText('Visibility'),
  latestObservation: decodeDisplayText('Latest observation'),
  browserActivity: decodeDisplayText('Browser activity'),
  rows: decodeDisplayText('Rows'),
  generated: decodeDisplayText('Generated'),
} as const;

type BrowserStatusDetail = Readonly<{ label: string; value: string }>;

export function shouldRenderBrowserActivityRoute(route: ParentRouteId): boolean {
  return route === ParentRoute.Browser;
}

export function BrowserActivityRoutePanel({
  actions,
  commandEnabled,
  liveActivity,
}: {
  readonly actions: PortalRenderActions;
  readonly commandEnabled: boolean;
  readonly liveActivity: PortalLiveActivityState;
}): ReactElement {
  const reported = browserStatusReported(liveActivity);
  const routeAction = browserRouteAction(actions, commandEnabled);
  return (
    <section
      aria-label={BROWSER_ROUTE_TEXT.ariaLabel}
      className={PortalDom.Classes.TrackingStatusOverlay}
      data-ocentra-browser-surface="product"
      data-ocentra-browser-route-state={reported ? 'reported' : 'unavailable'}
    >
      <div className={PortalDom.Classes.TrackingStatusOverlayContent}>
        <header className={PortalDom.Classes.TrackingStatusOverlayHeader}>
          <p className={PortalDom.Classes.ProductEyebrow}>{BROWSER_ROUTE_TEXT.eyebrow}</p>
          <h2>{reported ? BROWSER_ROUTE_TEXT.title : BROWSER_ROUTE_TEXT.unavailableTitle}</h2>
          <p>{reported ? BROWSER_ROUTE_TEXT.body : BROWSER_ROUTE_TEXT.unavailableBody}</p>
          <button
            className={PortalDom.Classes.CommandResultTab}
            type={PortalDom.ButtonType.Button}
            onClick={routeAction.run}
          >
            {routeAction.label}
          </button>
        </header>
        <div
          className={[PortalDom.Classes.ProductDashboard, PortalDom.Classes.TrackingStatusOverlayGrid].join(
            PortalDom.Classes.ClassNameSeparator
          )}
        >
          {reported ? (
            <>
              <BrowserManagedStatusCard liveActivity={liveActivity} />
              <BrowserEvidenceStatusCard liveActivity={liveActivity} />
              <BrowserActivityStatusCard liveActivity={liveActivity} />
            </>
          ) : (
            <BrowserUnavailableCards />
          )}
        </div>
      </div>
    </section>
  );
}

function BrowserManagedStatusCard({ liveActivity }: { readonly liveActivity: PortalLiveActivityState }): ReactElement {
  const status = liveActivity.browserManagedStatus;
  const browserLabel = [status?.browserFamily, status?.browserChannel, status?.browserVersion]
    .filter((value) => value !== null && value !== undefined && value.length > 0)
    .join(' ');
  return (
    <BrowserStatusCard
      title={BROWSER_ROUTE_TEXT.managedBrowser}
      value={displayValue(status?.managedState)}
      body={displayValue(status?.degradedReason, BROWSER_ROUTE_TEXT.noDegradedReason)}
      details={[
        { label: BROWSER_ROUTE_TEXT.capability, value: displayValue(status?.capabilityStatus) },
        { label: BROWSER_ROUTE_TEXT.browser, value: displayValue(browserLabel) },
        { label: BROWSER_ROUTE_TEXT.profileLifecycle, value: displayValue(status?.profileLifecycleState) },
        { label: BROWSER_ROUTE_TEXT.bridgeKind, value: displayValue(status?.bridgeKind) },
        { label: BROWSER_ROUTE_TEXT.custody, value: displayValue(status?.custodyLabel) },
        { label: BROWSER_ROUTE_TEXT.checked, value: displayValue(status?.checkedAt) },
      ]}
    />
  );
}

function BrowserEvidenceStatusCard({ liveActivity }: { readonly liveActivity: PortalLiveActivityState }): ReactElement {
  const evidence = liveActivity.browserEvidenceReadModel;
  return (
    <BrowserStatusCard
      title={BROWSER_ROUTE_TEXT.browserEvidence}
      value={evidence === null ? BROWSER_ROUTE_TEXT.notReported : `${evidence.returned} reported`}
      body={BROWSER_ROUTE_TEXT.evidenceBody}
      details={[
        { label: BROWSER_ROUTE_TEXT.capability, value: displayValue(evidence?.capabilityStatus) },
        { label: BROWSER_ROUTE_TEXT.custody, value: displayValue(evidence?.custodyLabel) },
        { label: BROWSER_ROUTE_TEXT.visibility, value: displayValue(evidence?.queryVisibility) },
        { label: BROWSER_ROUTE_TEXT.latestObservation, value: displayValue(evidence?.latestObservedAt) },
      ]}
    />
  );
}

function BrowserActivityStatusCard({ liveActivity }: { readonly liveActivity: PortalLiveActivityState }): ReactElement {
  const activity = liveActivity.activityBrowserReadModel;
  const rowCount = activity?.ok === true ? activity.value.rows.length : 0;
  const body = activity?.ok === false ? activity.reason : displayValue(activity?.ok ? activity.value.summary : null);
  return (
    <BrowserStatusCard
      title={BROWSER_ROUTE_TEXT.browserActivity}
      value={activity === null ? BROWSER_ROUTE_TEXT.notReported : activity.state}
      body={body}
      details={[
        { label: BROWSER_ROUTE_TEXT.rows, value: String(rowCount) },
        { label: BROWSER_ROUTE_TEXT.generated, value: displayValue(activity?.ok ? activity.value.generatedAt : null) },
      ]}
    />
  );
}

function BrowserUnavailableCards(): ReactElement {
  return (
    <>
      <BrowserStatusCard
        title={BROWSER_ROUTE_TEXT.unavailableManagedTitle}
        value={BROWSER_ROUTE_TEXT.notReported}
        body={BROWSER_ROUTE_TEXT.unavailableManagedBody}
        details={[
          { label: BROWSER_ROUTE_TEXT.capability, value: BROWSER_ROUTE_TEXT.notReported },
          { label: BROWSER_ROUTE_TEXT.profileLifecycle, value: BROWSER_ROUTE_TEXT.notReported },
          { label: BROWSER_ROUTE_TEXT.bridgeKind, value: BROWSER_ROUTE_TEXT.notReported },
        ]}
      />
      <BrowserStatusCard
        title={BROWSER_ROUTE_TEXT.unavailableEvidenceTitle}
        value={BROWSER_ROUTE_TEXT.notReported}
        body={BROWSER_ROUTE_TEXT.unavailableEvidenceBody}
        details={[
          { label: BROWSER_ROUTE_TEXT.custody, value: BROWSER_ROUTE_TEXT.notReported },
          { label: BROWSER_ROUTE_TEXT.visibility, value: BROWSER_ROUTE_TEXT.notReported },
          { label: BROWSER_ROUTE_TEXT.latestObservation, value: BROWSER_ROUTE_TEXT.notReported },
        ]}
      />
      <BrowserStatusCard
        title={BROWSER_ROUTE_TEXT.unavailableActivityTitle}
        value={BROWSER_ROUTE_TEXT.notReported}
        body={BROWSER_ROUTE_TEXT.unavailableActivityBody}
        details={[
          { label: BROWSER_ROUTE_TEXT.rows, value: '0' },
          { label: BROWSER_ROUTE_TEXT.generated, value: BROWSER_ROUTE_TEXT.notReported },
        ]}
      />
    </>
  );
}

function BrowserStatusCard({
  body,
  details,
  title,
  value,
}: {
  readonly body: string;
  readonly details: readonly BrowserStatusDetail[];
  readonly title: string;
  readonly value: string;
}): ReactElement {
  return (
    <article className={browserStatusCardClassName()}>
      <h2>{title}</h2>
      <p>{value}</p>
      <p>{body}</p>
      <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
        {details.map((detail) => (
          <div key={detail.label}>
            <dt>{detail.label}</dt>
            <dd>{detail.value}</dd>
          </div>
        ))}
      </dl>
    </article>
  );
}

function browserStatusReported(liveActivity: PortalLiveActivityState): boolean {
  return (
    liveActivity.browserManagedStatus !== null ||
    liveActivity.browserEvidenceReadModel !== null ||
    liveActivity.activityBrowserReadModel !== null
  );
}

function browserRouteAction(
  actions: PortalRenderActions,
  commandEnabled: boolean
): { readonly label: string; readonly run: () => void } {
  if (!commandEnabled || actions.refreshRouteSnapshot === undefined) {
    return { label: resolvePortalDevText(PortalDevTextToken.RetryStatus), run: actions.reconnect };
  }
  return {
    label: BROWSER_ROUTE_TEXT.refresh,
    run: () => {
      void actions.refreshRouteSnapshot?.();
    },
  };
}

function displayValue(
  value: string | number | null | undefined,
  fallback: string = BROWSER_ROUTE_TEXT.notReported
): string {
  if (value === null || value === undefined || value === '') return fallback;
  return String(value);
}

function browserStatusCardClassName(): string {
  return [PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(PortalDom.Classes.ClassNameSeparator);
}
