import type { ReactElement } from 'react';
import { PortalDom, PortalEnvironment } from '@ocentra-parent/portal-domain/contracts';
import {
  createBrowserParentExplanationPanelIntent,
  type BrowserParentExplanationPanelDetail,
  type BrowserParentExplanationPanelIntent,
  type BrowserParentExplanationPanelRow,
} from '@ocentra-parent/portal-domain/browser-parent-explanation-panel';
import { isPortalBrowserParentSurfaceRoute } from '@ocentra-parent/portal-domain/routes';
import { type PortalRoute as PortalRouteValue } from '@ocentra-parent/schema-domain/portal-contracts';
export function shouldRenderBrowserParentExplanationRoute(route: PortalRouteValue): boolean {
  return isPortalBrowserParentSurfaceRoute(route);
}

export function BrowserParentExplanationRoutePanel(): ReactElement {
  const intent = createBrowserParentExplanationPanelIntent(browserParentExplanationProofInput());
  return (
    <section aria-label={intent.title} className={PortalDom.Classes.TrackingStatusOverlay}>
      <div className={PortalDom.Classes.TrackingStatusOverlayContent}>
        <header className={PortalDom.Classes.TrackingStatusOverlayHeader}>
          <p className={PortalDom.Classes.ProductEyebrow}>{intent.eyebrow}</p>
          <h2>{intent.title}</h2>
          <p>{intent.body}</p>
        </header>
        <div
          className={[PortalDom.Classes.ProductDashboard, PortalDom.Classes.TrackingStatusOverlayGrid].join(
            PortalDom.Classes.ClassNameSeparator
          )}
        >
          <BrowserParentExplanationSummaryCard intent={intent} />
          {intent.rows.length === 0 ? (
            <BrowserParentExplanationEmptyCard intent={intent} />
          ) : (
            intent.rows.map((row) => <BrowserParentExplanationRowCard key={row.key} row={row} />)
          )}
        </div>
      </div>
    </section>
  );
}

function BrowserParentExplanationSummaryCard({
  intent,
}: {
  readonly intent: BrowserParentExplanationPanelIntent;
}): ReactElement {
  return (
    <article className={cardClassName()}>
      <h2>{intent.summary}</h2>
      <BrowserParentExplanationDetails details={intent.metrics} />
    </article>
  );
}

function BrowserParentExplanationEmptyCard({
  intent,
}: {
  readonly intent: BrowserParentExplanationPanelIntent;
}): ReactElement {
  return (
    <article className={cardClassName()}>
      <h2>{intent.emptyMessage}</h2>
      <BrowserParentExplanationDetails details={intent.metrics} />
    </article>
  );
}

function BrowserParentExplanationRowCard({ row }: { readonly row: BrowserParentExplanationPanelRow }): ReactElement {
  return (
    <article className={cardClassName()}>
      <h2>{row.title}</h2>
      <BrowserParentExplanationDetails details={row.details} />
    </article>
  );
}

function BrowserParentExplanationDetails({
  details,
}: {
  readonly details: readonly BrowserParentExplanationPanelDetail[];
}): ReactElement {
  return (
    <dl className={PortalDom.Classes.TrackingStatusOverlayMeta}>
      {details.map((detail) => (
        <div key={`${detail.label}:${detail.value}`}>
          <dt>{detail.label}</dt>
          <dd>{detail.value}</dd>
        </div>
      ))}
    </dl>
  );
}

function browserParentExplanationProofInput(): unknown {
  const proofValue = import.meta.env[PortalEnvironment.BrowserParentExplanationProofBundle];
  if (typeof proofValue !== 'string' || proofValue.trim().length === 0) {
    return null;
  }
  try {
    return JSON.parse(proofValue) as unknown;
  } catch {
    return null;
  }
}

function cardClassName(): string {
  return [PortalDom.Classes.Summary, PortalDom.Classes.ProductStatusCard].join(PortalDom.Classes.ClassNameSeparator);
}
