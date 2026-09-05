import type { ReactElement } from 'react';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import { ParentRoute, parentRouteHashPath, type ParentRouteId } from '../generated/parent-ui-bridge';

type PolicyCategoryRoute =
  | typeof ParentRoute.BrowserSettings
  | typeof ParentRoute.PolicyApps
  | typeof ParentRoute.PolicyGames
  | typeof ParentRoute.PolicyRemoteScreen;

type PolicyCategoryDestination = {
  readonly label: string;
  readonly route: ParentRouteId;
};

type PolicyCategoryConfig = {
  readonly body: string;
  readonly destinations: readonly PolicyCategoryDestination[];
  readonly eyebrow: string;
  readonly reviewBody: string;
  readonly title: string;
};

const POLICY_CATEGORY_CONFIG: Readonly<Record<PolicyCategoryRoute, PolicyCategoryConfig>> = {
  [ParentRoute.BrowserSettings]: {
    eyebrow: 'Browser policy',
    title: 'Browser policy controls unavailable',
    body: 'No service-reported browser policy editor is connected. Review current browser activity or open the parent-owned rules and approvals without inventing an active browser rule.',
    reviewBody:
      'Browser activity remains available for reported evidence while policy editing waits for the local service.',
    destinations: [
      { label: 'Open browser activity', route: ParentRoute.Browser },
      { label: 'Open rules', route: ParentRoute.RuleManagement },
      { label: 'Open approvals', route: ParentRoute.Approvals },
    ],
  },
  [ParentRoute.PolicyApps]: {
    eyebrow: 'App policy',
    title: 'App policy controls unavailable',
    body: 'No service-reported app policy editor is connected. Review current app activity or open the parent-owned policy and approval routes without inventing an active rule.',
    reviewBody:
      'App activity remains available for reported evidence while policy editing waits for the local service.',
    destinations: [
      { label: 'Open app activity', route: ParentRoute.AppGameSessions },
      { label: 'Open rules', route: ParentRoute.RuleManagement },
      { label: 'Open approvals', route: ParentRoute.Approvals },
    ],
  },
  [ParentRoute.PolicyGames]: {
    eyebrow: 'Game policy',
    title: 'Game policy controls unavailable',
    body: 'No service-reported game policy editor is connected. Review current game activity or open the parent-owned policy and approval routes without inventing an active rule.',
    reviewBody:
      'Game activity remains available for reported evidence while policy editing waits for the local service.',
    destinations: [
      { label: 'Open game activity', route: ParentRoute.AppGameSessions },
      { label: 'Open rules', route: ParentRoute.RuleManagement },
      { label: 'Open approvals', route: ParentRoute.Approvals },
    ],
  },
  [ParentRoute.PolicyRemoteScreen]: {
    eyebrow: 'Remote screen policy',
    title: 'Remote screen controls unavailable',
    body: 'No owner-backed live-view session, child capability, permission, route, custody, or current authority is connected. Review current screen status or reconnect before requesting remote access.',
    reviewBody: 'Screen status and device selection remain available while remote-session authority is unavailable.',
    destinations: [
      { label: 'Open screen analysis', route: ParentRoute.ScreenAnalysis },
      { label: 'Open devices', route: ParentRoute.Devices },
      { label: 'Open Start Here', route: ParentRoute.Start },
    ],
  },
};

export function shouldRenderPolicyCategoryRoute(route: ParentRouteId): route is PolicyCategoryRoute {
  return (
    route === ParentRoute.BrowserSettings ||
    route === ParentRoute.PolicyApps ||
    route === ParentRoute.PolicyGames ||
    route === ParentRoute.PolicyRemoteScreen
  );
}

export function PolicyCategoryRoutePanel({
  onNavigate,
  route,
}: {
  readonly onNavigate: (routePath: string) => boolean | void;
  readonly route: PolicyCategoryRoute;
}): ReactElement {
  const config = POLICY_CATEGORY_CONFIG[route];
  return (
    <section
      aria-label={config.title}
      className={PortalDom.Classes.TrackingStatusOverlay}
      data-ocentra-policy-category-route={route}
      data-ocentra-policy-category-state="manual-required"
    >
      <div className={PortalDom.Classes.TrackingStatusOverlayContent}>
        <header className={PortalDom.Classes.TrackingStatusOverlayHeader}>
          <div className={PortalDom.Classes.PolicyCategoryRouteCopy}>
            <p className={PortalDom.Classes.ProductEyebrow}>{config.eyebrow}</p>
            <h2>{config.title}</h2>
            <p>{config.body}</p>
          </div>
          <nav aria-label={`${config.eyebrow} available destinations`} className={PortalDom.Classes.RouteTabs}>
            {config.destinations.map((destination) => (
              <button
                className={PortalDom.Classes.CommandResultTab}
                key={destination.route}
                onClick={() => {
                  onNavigate(parentRouteHashPath(destination.route));
                }}
                type={PortalDom.ButtonType.Button}
              >
                {destination.label}
              </button>
            ))}
          </nav>
        </header>
        <PolicyCategoryStatusGrid config={config} />
      </div>
    </section>
  );
}

function PolicyCategoryStatusGrid({ config }: { readonly config: PolicyCategoryConfig }): ReactElement {
  return (
    <div
      aria-label={`${config.eyebrow} status`}
      className={[PortalDom.Classes.ProductDashboard, PortalDom.Classes.TrackingStatusOverlayGrid].join(
        PortalDom.Classes.ClassNameSeparator
      )}
      role="list"
    >
      <PolicyCategoryStatusCard
        body="No current or effective policy is displayed until the local service reports one for this exact area."
        label="Current policy"
        value="Not reported"
      />
      <PolicyCategoryStatusCard
        body="Changes stay locked until both a current policy snapshot and owner-backed editing authority are reported."
        label="Editing authority"
        value="Manual required"
      />
      <PolicyCategoryStatusCard body={config.reviewBody} label="Available now" value="Review only" />
    </div>
  );
}

function PolicyCategoryStatusCard({
  body,
  label,
  value,
}: {
  readonly body: string;
  readonly label: string;
  readonly value: string;
}): ReactElement {
  return (
    <article
      className={[PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(
        PortalDom.Classes.ClassNameSeparator
      )}
      data-ocentra-policy-category-card={label.toLowerCase().replaceAll(' ', '-')}
      role="listitem"
    >
      <p className={PortalDom.Classes.ProductEyebrow}>{label}</p>
      <h3>{value}</h3>
      <p>{body}</p>
    </article>
  );
}
