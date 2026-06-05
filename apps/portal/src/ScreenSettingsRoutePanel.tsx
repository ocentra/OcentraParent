import type { ReactElement } from 'react';
import {
  screenControlSettingsPortalProof,
  type ScreenControlSettingsPortalGate,
  type ScreenControlSettingsPortalMetric,
} from '@ocentra-parent/parent-domain/screen-control-settings-portal-proof';
import {
  PortalDetails,
  PortalDom,
  PortalRoute,
  type PortalDisplayText,
  type PortalRoute as PortalRouteValue,
} from '@ocentra-parent/portal-domain/contracts';

type ScreenSettingsDetailValue =
  | ScreenControlSettingsPortalMetric[keyof ScreenControlSettingsPortalMetric]
  | ScreenControlSettingsPortalGate[keyof ScreenControlSettingsPortalGate];

export function shouldRenderScreenSettingsRoute(route: PortalRouteValue): boolean {
  return route === PortalRoute.SettingsRules || currentHash() === screenSettingsRouteHash();
}

export function ScreenSettingsRoutePanel(): ReactElement {
  const proof = screenControlSettingsPortalProof();
  return (
    <section aria-label={proof.title} className={PortalDom.Classes.TrackingStatusOverlay}>
      <div className={PortalDom.Classes.TrackingStatusOverlayContent}>
        <header className={PortalDom.Classes.TrackingStatusOverlayHeader}>
          <p className={PortalDom.Classes.ProductEyebrow}>{proof.metrics[0]?.value}</p>
          <h2>{proof.title}</h2>
          <p>{proof.note}</p>
        </header>
        <div
          className={[PortalDom.Classes.ProductDashboard, PortalDom.Classes.TrackingStatusOverlayGrid].join(
            PortalDom.Classes.ClassNameSeparator
          )}
        >
          {proof.metrics.map((metric) => (
            <ScreenSettingsMetricCard key={metric.label} metric={metric} />
          ))}
          {proof.gates.map((gate) => (
            <ScreenSettingsGateCard key={gate.label} gate={gate} />
          ))}
        </div>
      </div>
    </section>
  );
}

function ScreenSettingsMetricCard({ metric }: { readonly metric: ScreenControlSettingsPortalMetric }): ReactElement {
  return (
    <article className={screenSettingsCardClassName()}>
      <h2>{metric.label}</h2>
      <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
        <ScreenSettingsDetail label={PortalDetails.Status} value={metric.value} />
        <ScreenSettingsDetail label={PortalDetails.Reason} value={metric.detail} />
      </dl>
    </article>
  );
}

function ScreenSettingsGateCard({ gate }: { readonly gate: ScreenControlSettingsPortalGate }): ReactElement {
  return (
    <article className={screenSettingsCardClassName()}>
      <h2>{gate.label}</h2>
      <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
        <ScreenSettingsDetail label={PortalDetails.Status} value={gate.statusText} />
        <ScreenSettingsDetail label={PortalDetails.Capability} value={gate.capabilityState} />
        <ScreenSettingsDetail label={PortalDetails.RuntimeReference} value={gate.runtimeOwner} />
        <ScreenSettingsDetail label={PortalDetails.Source} value={gate.sourceDocument} />
        <ScreenSettingsDetail label={PortalDetails.Reason} value={gate.detail} />
      </dl>
    </article>
  );
}

function ScreenSettingsDetail({
  label,
  value,
}: {
  readonly label: PortalDisplayText;
  readonly value: ScreenSettingsDetailValue;
}): ReactElement {
  return (
    <div>
      <dt>{label}</dt>
      <dd>{value}</dd>
    </div>
  );
}

function screenSettingsCardClassName() {
  return [PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(PortalDom.Classes.ClassNameSeparator);
}

function screenSettingsRouteHash() {
  return [PortalDom.HashPrefix, PortalRoute.SettingsRules].join(PortalDom.EmptyHashRoute);
}

function currentHash() {
  if (typeof window === PortalDom.Runtime.Undefined) {
    return PortalDom.EmptyHashRoute;
  }
  return window.location.hash;
}
