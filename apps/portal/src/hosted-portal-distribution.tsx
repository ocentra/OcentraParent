import type { CSSProperties, ReactElement } from 'react';
import {
  resolveHostedPortalDistributionState as resolveHostedPortalDistributionStateFromDomain,
  type HostedPortalEnv,
  type HostedPortalDistributionState,
  type HostedPortalDistributionState as HostedPortalDistributionStateValue,
  type HostedPortalLocation,
} from '@ocentra-parent/portal-domain/hosted-portal-distribution';

type HostedPortalEnvironment = HostedPortalDistributionStateValue['environment'];

const ENVIRONMENT_LABELS: Readonly<Record<HostedPortalEnvironment, string>> = {
  preview: 'Preview verification route',
  staging: 'Staging verification route',
  production: 'Production release route',
};

const RELEASE_LABELS: Readonly<Record<HostedPortalEnvironment, string>> = {
  preview: 'Preview candidate',
  staging: 'Staging candidate',
  production: 'Production release',
};

const SURFACE_STYLE: CSSProperties = {
  boxSizing: 'border-box',
  display: 'grid',
  gap: '1.25rem',
  margin: '0 auto',
  maxWidth: '72rem',
  minHeight: '100vh',
  padding: '2rem 1.5rem 3rem',
};

const CARD_GRID_STYLE: CSSProperties = {
  display: 'grid',
  gap: '1rem',
  gridTemplateColumns: 'repeat(auto-fit, minmax(16rem, 1fr))',
};

const CARD_STYLE: CSSProperties = {
  background: 'rgba(9, 18, 33, 0.78)',
  border: '1px solid rgba(120, 152, 199, 0.28)',
  borderRadius: '1rem',
  padding: '1rem',
};

const HEADER_STYLE: CSSProperties = {
  display: 'grid',
  gap: '0.75rem',
};

const HEADER_COPY_STYLE: CSSProperties = {
  display: 'grid',
  gap: '0.35rem',
};

const TITLE_STYLE: CSSProperties = {
  fontSize: 'clamp(2rem, 4vw, 3.4rem)',
  margin: 0,
};

const LEDE_STYLE: CSSProperties = {
  fontSize: '1.05rem',
  lineHeight: 1.5,
  margin: 0,
  maxWidth: '44rem',
};

const CARD_HEADING_STYLE: CSSProperties = {
  marginTop: 0,
};

const ROUTE_REQUEST_STYLE: CSSProperties = {
  marginBottom: '0.5rem',
};

const PARAGRAPH_TOP_FLUSH_STYLE: CSSProperties = {
  marginTop: 0,
};

const PARAGRAPH_BOTTOM_FLUSH_STYLE: CSSProperties = {
  marginBottom: 0,
};

const PARAGRAPH_FLUSH_STYLE: CSSProperties = {
  margin: 0,
};

const ACTION_COPY_STYLE: CSSProperties = {
  marginTop: 0,
  maxWidth: '42rem',
};

const BADGE_STYLE: CSSProperties = {
  border: '1px solid rgba(160, 197, 255, 0.35)',
  borderRadius: '999px',
  display: 'inline-flex',
  fontSize: '0.8rem',
  fontWeight: 700,
  letterSpacing: '0.06em',
  padding: '0.35rem 0.75rem',
  textTransform: 'uppercase',
};

const PRIMARY_ACTION_STYLE: CSSProperties = {
  background: 'linear-gradient(135deg, rgba(119, 212, 172, 0.96), rgba(95, 174, 255, 0.92))',
  border: 'none',
  borderRadius: '999px',
  color: '#071421',
  cursor: 'pointer',
  fontSize: '1rem',
  fontWeight: 700,
  minWidth: '15rem',
  padding: '0.8rem 1.2rem',
};

const DISABLED_ACTION_STYLE: CSSProperties = {
  ...PRIMARY_ACTION_STYLE,
  background: 'rgba(86, 110, 139, 0.42)',
  color: 'rgba(226, 235, 247, 0.88)',
  cursor: 'not-allowed',
};

export function resolveHostedPortalDistributionState(
  location: HostedPortalLocation,
  env: HostedPortalEnv,
  defaultNowMinutes?: number
): HostedPortalDistributionState | null {
  return resolveHostedPortalDistributionStateFromDomain(location, env, defaultNowMinutes);
}

export function HostedPortalDistribution({ state }: { readonly state: HostedPortalDistributionState }): ReactElement {
  return (
    <main data-testid="hosted-portal-distribution" style={SURFACE_STYLE}>
      <HostedPortalHeader environment={state.environment} />

      <section style={CARD_GRID_STYLE}>
        <HostedPortalRouteBoundaryCard state={state} />
        <HostedPortalAuthBoundaryCard authState={state.authState} />
        <HostedPortalCacheBoundaryCard cacheAgeMinutes={state.cacheAgeMinutes} cacheState={state.cacheState} />
      </section>

      <HostedPortalReleaseCard environment={state.environment} requestedRelease={state.requestedRelease} />
      <HostedPortalActionCard controlsEnabled={state.controlsEnabled} openActionLabel={state.openActionLabel} />
    </main>
  );
}

function HostedPortalHeader({ environment }: { readonly environment: HostedPortalEnvironment }): ReactElement {
  return (
    <header style={HEADER_STYLE}>
      <span data-testid="hosted-environment-badge" style={BADGE_STYLE}>
        {ENVIRONMENT_LABELS[environment]}
      </span>
      <div style={HEADER_COPY_STYLE}>
        <h1 style={TITLE_STYLE}>Ocentra Parent Web Portal Distribution</h1>
        <p style={LEDE_STYLE}>
          Hosted parent-only portal route for distribution status, sign-in gating, cache honesty, and environment
          separation. Child runtime execution, setup completion, desktop parity, and mobile parity remain outside this
          surface.
        </p>
      </div>
    </header>
  );
}

function HostedPortalRouteBoundaryCard({ state }: { readonly state: HostedPortalDistributionState }): ReactElement {
  const routeMismatch = state.routeState === 'wrong-route';

  return (
    <article data-testid="hosted-route-card" style={CARD_STYLE}>
      <h2 style={CARD_HEADING_STYLE}>Route boundary</h2>
      <p style={ROUTE_REQUEST_STYLE}>
        Requested path: <code>{state.requestedPath}</code>
      </p>
      {routeMismatch ? (
        <>
          <p data-testid="hosted-route-blocker" style={PARAGRAPH_TOP_FLUSH_STYLE}>
            Unsupported hosted parent portal route. This page refuses the path instead of falling through to a child or
            setup surface.
          </p>
          <p style={PARAGRAPH_BOTTOM_FLUSH_STYLE}>
            Allowed hosted paths: <code>{state.expectedPaths.join(', ')}</code>
          </p>
        </>
      ) : (
        <p data-testid="hosted-route-allow" style={PARAGRAPH_BOTTOM_FLUSH_STYLE}>
          Parent-only hosted route accepted. Release label: <strong>{RELEASE_LABELS[state.environment]}</strong>
        </p>
      )}
    </article>
  );
}

function HostedPortalAuthBoundaryCard({
  authState,
}: {
  readonly authState: HostedPortalDistributionState['authState'];
}): ReactElement {
  const missingAuth = authState === 'missing';

  return (
    <article data-testid="hosted-auth-card" style={CARD_STYLE}>
      <h2 style={CARD_HEADING_STYLE}>Auth boundary</h2>
      {missingAuth ? (
        <>
          <p data-testid="hosted-auth-required" style={PARAGRAPH_TOP_FLUSH_STYLE}>
            Parent sign-in is required before any parent-only distribution control is shown.
          </p>
          <p style={PARAGRAPH_BOTTOM_FLUSH_STYLE}>No release controls are exposed while the auth gate is unresolved.</p>
        </>
      ) : (
        <p data-testid="hosted-auth-ok" style={PARAGRAPH_FLUSH_STYLE}>
          Parent-authenticated distribution state is present. The surface stays within parent web portal scope.
        </p>
      )}
    </article>
  );
}

function HostedPortalCacheBoundaryCard({
  cacheAgeMinutes,
  cacheState,
}: {
  readonly cacheAgeMinutes: number;
  readonly cacheState: HostedPortalDistributionState['cacheState'];
}): ReactElement {
  const staleCache = cacheState === 'stale';

  return (
    <article data-testid="hosted-cache-card" style={CARD_STYLE}>
      <h2 style={CARD_HEADING_STYLE}>Cache boundary</h2>
      {staleCache ? (
        <>
          <p data-testid="hosted-cache-stale" style={PARAGRAPH_TOP_FLUSH_STYLE}>
            Cached shell age is {cacheAgeMinutes} minutes. Fresh install or release claims are blocked until the portal
            is refreshed from the hosted source.
          </p>
          <p style={PARAGRAPH_BOTTOM_FLUSH_STYLE}>
            This page stays honest about stale parent cache instead of presenting a fresh state.
          </p>
        </>
      ) : (
        <p data-testid="hosted-cache-fresh" style={PARAGRAPH_FLUSH_STYLE}>
          Cached shell age is {cacheAgeMinutes} minutes. Route metadata is still treated as fresh.
        </p>
      )}
    </article>
  );
}

function HostedPortalReleaseCard({
  environment,
  requestedRelease,
}: {
  readonly environment: HostedPortalEnvironment;
  readonly requestedRelease: HostedPortalDistributionState['requestedRelease'];
}): ReactElement {
  return (
    <section data-testid="hosted-release-card" style={CARD_STYLE}>
      <h2 style={CARD_HEADING_STYLE}>Environment separation</h2>
      <p style={PARAGRAPH_TOP_FLUSH_STYLE}>
        Current hosted environment: <strong>{environment}</strong>. Requested release claim:{' '}
        <strong>{requestedRelease}</strong>.
      </p>
      {requestedRelease === 'production' && environment !== 'production' ? (
        <p data-testid="hosted-production-claim-blocked" style={PARAGRAPH_BOTTOM_FLUSH_STYLE}>
          Production release claim blocked. Preview and staging routes cannot present themselves as production.
        </p>
      ) : (
        <p data-testid="hosted-production-claim-allowed" style={PARAGRAPH_BOTTOM_FLUSH_STYLE}>
          Environment and release label remain aligned for this hosted parent portal route.
        </p>
      )}
    </section>
  );
}

function HostedPortalActionCard({
  controlsEnabled,
  openActionLabel,
}: {
  readonly controlsEnabled: boolean;
  readonly openActionLabel: string;
}): ReactElement {
  return (
    <section data-testid="hosted-action-card" style={CARD_STYLE}>
      <h2 style={CARD_HEADING_STYLE}>Parent portal action state</h2>
      <p style={ACTION_COPY_STYLE}>
        This action only covers the hosted parent web portal. Setup handoff, child runtime control, desktop package
        launch, and mobile package support remain manual-required outside this workpack.
      </p>
      <button
        aria-disabled={!controlsEnabled}
        data-testid="hosted-primary-action"
        style={controlsEnabled ? PRIMARY_ACTION_STYLE : DISABLED_ACTION_STYLE}
        type="button"
      >
        {controlsEnabled ? openActionLabel : 'Parent release action blocked'}
      </button>
    </section>
  );
}
