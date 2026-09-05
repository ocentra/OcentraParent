import { renderToStaticMarkup } from 'react-dom/server';
import { describe, expect, it } from 'vitest';
import { PARENT_PORTAL_NAV_LABELS } from '@ocentra-parent/portal-domain/parent-portal-nav';
import { SERVICE_BACKED_CONTENT } from '@ocentra-parent/portal-domain/parent-portal-service-state';
import {
  ParentPortalSvgSurface,
  type ParentPortalRow,
} from '../../../../vendor/ocentra-parent-core-ui/AppPages/ParentPortal/ParentPortalSvgSurface';
import {
  DEFAULT_PARENT_PORTAL_CONTENT,
  normalizeParentPortalContent,
} from '../../../../vendor/ocentra-parent-core-ui/AppPages/ParentPortal/ParentPortalSvgContent';

describe('parent portal service snapshot boundary', () => {
  it('shows an unavailable state instead of fallback readiness when service rows are absent', () => {
    const markup = parentPortalMarkup([], null);

    expect(markup).toContain('SERVICE SNAPSHOT');
    expect(markup).toContain('UNAVAILABLE');
    expect(markup).toContain('Connect the local service for current device status.');
    expect(markup).toContain('Controls stay read-only.');
    expect(markup).toContain('Current status appears after the local service connects');
    expect(markup).toContain('Controls stay off until the local service confirms');
    expect(markup).not.toContain('Rust-owned route rows');
    expect(markup).not.toContain('unclaimed');
    expect(markup).not.toContain('route snapshot');
    expect(markup).not.toContain('synthesized');
    expect(markup).not.toContain('Overview route');
    expect(markup).not.toContain('24/24');
    expect(markup).not.toContain('>100%</text>');
    expect(markup).not.toContain('wire real service state');
  });

  it('renders the service-owned row when a current snapshot is supplied', () => {
    const row: ParentPortalRow = {
      label: 'Runtime snapshot',
      order: 1,
      signalScore: 1,
      readyCount: 1,
      gapCount: 0,
      primaryArea: 'Local service',
      trend: 'Current',
      tone: 'cyan',
    };

    const markup = parentPortalMarkup([row], row);

    expect(markup).toContain('Runtime snapshot');
    expect(markup).toContain('Current / 100%');
    expect(markup).not.toContain('SERVICE SNAPSHOT');
  });

  it('offers a status retry when no authenticated device-discovery sender is available', () => {
    const markup = renderToStaticMarkup(
      <ParentPortalSvgSurface
        pageMode="parentManage"
        controlCode={1}
        seasonId="LOCAL"
        lastUpdated="not reported"
        parentPortalRows={[]}
        userEntry={null}
        nearbyAbove={[]}
        nearbyBelow={[]}
        content={SERVICE_BACKED_CONTENT}
        initialNavLabel="Devices"
        initialSelectedControlId="lan-pairing"
        onRefreshParentPortal={() => undefined}
        onMatchmaking={() => undefined}
      />
    );

    expect(markup).not.toContain('aria-label="Scan Local Area Network"');
    const retryControl = markup.match(/<g[^>]*aria-label="Retry status"[^>]*>/)?.[0];
    expect(retryControl).toContain('role="button"');
    expect(retryControl).toContain('tabindex="0"');
    expect(retryControl).not.toContain('aria-disabled');
  });

  it('renders the latest host action result as a visible neutral status update', () => {
    const markup = parentPortalMarkup([], null, 'The local service owner is unavailable.');

    expect(markup).toContain('role="status"');
    expect(markup).toContain('aria-live="polite"');
    expect(markup).toContain('STATUS UPDATE');
    expect(markup).toContain('The local service owner is unavailable.');
    expect(markup).not.toContain('role="alert"');
  });

  it('labels the App Use product workspace from its routed app/game control', () => {
    const markup = renderToStaticMarkup(
      <ParentPortalSvgSurface
        pageMode="parentManage"
        controlCode={1}
        seasonId="LOCAL"
        lastUpdated="not reported"
        parentPortalRows={[]}
        userEntry={null}
        nearbyAbove={[]}
        nearbyBelow={[]}
        content={SERVICE_BACKED_CONTENT}
        initialNavLabel={PARENT_PORTAL_NAV_LABELS.AppsGames}
        initialSelectedControlId="app-game-sessions"
        onRefreshParentPortal={() => undefined}
        onMatchmaking={() => undefined}
      />
    );
    const mainBoard = markup.slice(markup.indexOf('parent-portal-study-main-board'));

    expect(mainBoard).toContain('>APP USE</text>');
    expect(mainBoard).not.toContain('>BROWSER</text>');
  });

  it('labels benchmark rows as a demo fixture', () => {
    const fixtureMarkup = parentGuideMarkup(DEFAULT_PARENT_PORTAL_CONTENT);

    expect(fixtureMarkup).toContain('data-ocentra-parent-row-source="fixture"');
    expect(fixtureMarkup).toContain('DEMO FIXTURE · NOT RUNTIME');
    expect(fixtureMarkup).toContain('NO PRODUCT READINESS CLAIM');
  });

  it('keeps product-backed guide routes on the service source', () => {
    expect(SERVICE_BACKED_CONTENT.modes.parentGuide.rowSource).toBe('api');
    expect(normalizeParentPortalContent(SERVICE_BACKED_CONTENT).modes.parentGuide.rowSource).toBe('api');

    const serviceMarkup = parentGuideMarkup(SERVICE_BACKED_CONTENT);

    expect(serviceMarkup).not.toContain('data-ocentra-parent-row-source="fixture"');
    expect(serviceMarkup).not.toContain('DEMO FIXTURE · NOT RUNTIME');
  });
});

function parentGuideMarkup(content: typeof DEFAULT_PARENT_PORTAL_CONTENT): string {
  return renderToStaticMarkup(
    <ParentPortalSvgSurface
      pageMode="parentGuide"
      controlCode={1}
      seasonId="LOCAL"
      lastUpdated="not reported"
      parentPortalRows={[]}
      userEntry={null}
      nearbyAbove={[]}
      nearbyBelow={[]}
      content={content}
      onRefreshParentPortal={() => undefined}
      onMatchmaking={() => undefined}
    />
  );
}

function parentPortalMarkup(
  rows: ParentPortalRow[],
  userEntry: ParentPortalRow | null,
  statusMessage: string | null = null
): string {
  return renderToStaticMarkup(
    <ParentPortalSvgSurface
      pageMode="parentOverview"
      controlCode={1}
      seasonId="LOCAL"
      lastUpdated="not reported"
      parentPortalRows={rows}
      userEntry={userEntry}
      nearbyAbove={[]}
      nearbyBelow={[]}
      content={SERVICE_BACKED_CONTENT}
      statusMessage={statusMessage}
      onRefreshParentPortal={() => undefined}
      onMatchmaking={() => undefined}
    />
  );
}
