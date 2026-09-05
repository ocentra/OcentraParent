import type { ReactElement } from 'react';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import { ParentRoute, parentRouteHashPath } from '../generated/parent-ui-bridge';

type ScheduleRouteUnavailablePanelProps = {
  readonly onNavigate: (routePath: string) => boolean;
};

const SCHEDULE_UNAVAILABLE_DESTINATIONS = [
  { label: 'Open rules', route: ParentRoute.RuleManagement },
  { label: 'Open approvals', route: ParentRoute.Approvals },
  { label: 'Open enforcement', route: ParentRoute.Enforcement },
] as const;

export function ScheduleRouteUnavailablePanel({ onNavigate }: ScheduleRouteUnavailablePanelProps): ReactElement {
  return (
    <section
      aria-label="Schedules unavailable"
      className={PortalDom.Classes.TrackingStatusOverlay}
      data-ocentra-schedule-authority="manual-required"
      data-ocentra-schedule-state="unavailable"
    >
      <div className={PortalDom.Classes.TrackingStatusOverlayContent}>
        <header className={PortalDom.Classes.TrackingStatusOverlayHeader}>
          <div className={PortalDom.Classes.ScheduleUnavailableCopy}>
            <p className={PortalDom.Classes.ProductEyebrow}>Schedules</p>
            <h1>Schedules unavailable</h1>
            <p>
              Ocentra has not received a current schedule or time-budget status from the local service, so schedule
              controls stay off instead of guessing.
            </p>
          </div>
          <ScheduleUnavailableNavigation onNavigate={onNavigate} />
        </header>
        <ScheduleUnavailableDetails />
      </div>
    </section>
  );
}

function ScheduleUnavailableNavigation({ onNavigate }: ScheduleRouteUnavailablePanelProps): ReactElement {
  return (
    <nav aria-label="Available control areas" className={PortalDom.Classes.RouteTabs}>
      {SCHEDULE_UNAVAILABLE_DESTINATIONS.map((destination) => (
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
  );
}

function ScheduleUnavailableDetails(): ReactElement {
  return (
    <div
      aria-label="Schedule availability details"
      className={[PortalDom.Classes.ProductDashboard, PortalDom.Classes.TrackingStatusOverlayGrid].join(
        PortalDom.Classes.ClassNameSeparator
      )}
      role="list"
    >
      <ScheduleUnavailableCard
        body="No effective schedule, template selection, timezone, or daylight-saving calculation is inferred."
        details={[
          ['Current/effective state', 'Not reported'],
          ['Templates', 'Not available'],
          ['Timezone/DST', 'Not reported'],
        ]}
        label="Current schedule"
        value="Not reported"
      />
      <ScheduleUnavailableCard
        body="A current local schedule service is required before timers or durable schedule state can be trusted."
        details={[
          ['Timer owner', 'Local schedule service required'],
          ['Durability', 'Not reported'],
        ]}
        label="Timer ownership"
        value="Manual required"
      />
      <ScheduleUnavailableCard
        body="Rules, approvals, and enforcement remain available for review without creating a schedule command."
        details={[['Actions', 'Manual required']]}
        label="Available now"
        value="Review only"
      />
    </div>
  );
}

function ScheduleUnavailableCard({
  body,
  details,
  label,
  value,
}: {
  readonly body: string;
  readonly details: readonly (readonly [string, string])[];
  readonly label: string;
  readonly value: string;
}): ReactElement {
  return (
    <article
      className={[PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(
        PortalDom.Classes.ClassNameSeparator
      )}
      role="listitem"
    >
      <p className={PortalDom.Classes.ProductEyebrow}>{label}</p>
      <h2>{value}</h2>
      <p>{body}</p>
      <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
        {details.map(([detailLabel, detailValue]) => (
          <div key={detailLabel}>
            <dt>{detailLabel}</dt>
            <dd>{detailValue}</dd>
          </div>
        ))}
      </dl>
    </article>
  );
}
