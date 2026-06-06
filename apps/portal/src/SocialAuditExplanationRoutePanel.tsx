import type { ReactElement } from 'react';
import {
  PortalDom,
  PortalEnvironment,
  PortalRoute,
  type PortalRoute as PortalRouteValue,
} from '@ocentra-parent/portal-domain/contracts';
import {
  createSocialAuditExplanationPanelIntent,
  type SocialAuditExplanationPanelDetail,
  type SocialAuditExplanationPanelIntent,
  type SocialAuditExplanationPanelRow,
} from './social-audit-explanation-panel';

export function shouldRenderSocialAuditExplanationRoute(route: PortalRouteValue): boolean {
  return route === PortalRoute.Browser;
}

export function SocialAuditExplanationRoutePanel(): ReactElement {
  const intent = createSocialAuditExplanationPanelIntent(socialAuditExplanationProofInput());
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
          <SocialAuditExplanationSummaryCard intent={intent} />
          {intent.rows.length === 0 ? (
            <SocialAuditExplanationEmptyCard intent={intent} />
          ) : (
            intent.rows.map((row) => <SocialAuditExplanationRowCard key={row.key} row={row} />)
          )}
        </div>
      </div>
    </section>
  );
}

function SocialAuditExplanationSummaryCard({
  intent,
}: {
  readonly intent: SocialAuditExplanationPanelIntent;
}): ReactElement {
  return (
    <article className={cardClassName()}>
      <h2>{intent.summary}</h2>
      <SocialAuditExplanationDetails details={intent.metrics} />
    </article>
  );
}

function SocialAuditExplanationEmptyCard({
  intent,
}: {
  readonly intent: SocialAuditExplanationPanelIntent;
}): ReactElement {
  return (
    <article className={cardClassName()}>
      <h2>{intent.emptyMessage}</h2>
      <SocialAuditExplanationDetails details={intent.metrics} />
    </article>
  );
}

function SocialAuditExplanationRowCard({ row }: { readonly row: SocialAuditExplanationPanelRow }): ReactElement {
  return (
    <article className={cardClassName()}>
      <h2>{row.title}</h2>
      <SocialAuditExplanationDetails details={row.details} />
    </article>
  );
}

function SocialAuditExplanationDetails({
  details,
}: {
  readonly details: readonly SocialAuditExplanationPanelDetail[];
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

function socialAuditExplanationProofInput(): unknown {
  const proofValue = import.meta.env[PortalEnvironment.SocialAuditExplanationProofBundle];
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
