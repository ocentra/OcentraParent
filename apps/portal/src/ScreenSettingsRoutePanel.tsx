import type { ReactElement } from 'react';
import {
  parentScreenControlSettingsPortalProof as screenControlSettingsPortalProof,
  type ParentScreenControlSettingsPortalGate as ScreenControlSettingsPortalGate,
  type ParentScreenControlSettingsPortalMetric as ScreenControlSettingsPortalMetric,
} from '../generated/parent-ui-screen-bridge';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import { type PortalDisplayText } from '@ocentra-parent/portal-domain/display-text';
import { PortalDetails } from '@ocentra-parent/portal-domain/details';
import { isParentScreenSettingsRoute, type ParentRouteId } from '../generated/parent-ui-bridge';
import type { PortalRenderActions } from './portal-actions';
import { ScreenOptionalVisibilityCapabilityStatusCard } from './ScreenOptionalVisibilityCapabilityStatusCard';
import { ScreenSettingsWritableControls } from './ScreenSettingsWritableControls';

type ScreenSettingsDetailValue =
  | ScreenControlSettingsPortalMetric[keyof ScreenControlSettingsPortalMetric]
  | ScreenControlSettingsPortalGate[keyof ScreenControlSettingsPortalGate];

export function shouldRenderScreenSettingsRoute(route: ParentRouteId): boolean {
  return isParentScreenSettingsRoute(route);
}

export function ScreenSettingsRoutePanel({
  actions,
  commandEnabled,
  serviceResponseSnapshot,
}: {
  readonly actions: PortalRenderActions;
  readonly commandEnabled: boolean;
  readonly serviceResponseSnapshot: unknown | null;
}): ReactElement {
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
          <ScreenSettingsWritableControls
            actions={actions}
            commandEnabled={commandEnabled}
            serviceResponseSnapshot={serviceResponseSnapshot}
          />
          <ScreenOptionalVisibilityCapabilityStatusCard />
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
