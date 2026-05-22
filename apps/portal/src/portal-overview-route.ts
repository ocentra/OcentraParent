import {
  PortalConnectionState,
  PortalDetails,
  PortalDom,
  PortalText,
  PortalTextToken,
  decodePortalDetailValue,
} from '@ocentra-parent/portal-domain/contracts';
import { renderBrowserManagedStatus } from './browser-status-panel';
import { appendDetail } from './detail-list';
import { detailFromValue } from './event-detail-values';
import { renderEvidenceStore, renderRecentActivity } from './live-activity-panel';
import { resolveLiveActivityState } from './live-activity-state';
import { productBadge } from './portal-pending-surfaces';
import { renderOverviewGuidance } from './portal-product-guidance';
import { productMetric } from './portal-product-metric';
import type { PortalRuntimeState } from './portal-state';
import { renderDashboard } from './portal-dashboard';
import { renderPolicyPreview } from './policy-preview-panel';

export function renderOverviewRoute(container: HTMLElement, state: PortalRuntimeState): void {
  const liveActivity = resolveLiveActivityState(state.events);
  container.append(createOverviewHero(state, liveActivity));
  renderOverviewGuidance(container);
  renderOverviewDashboard(container, state, liveActivity);
}

function createOverviewHero(
  state: PortalRuntimeState,
  liveActivity: ReturnType<typeof resolveLiveActivityState>
): HTMLElement {
  const hero = document.createElement(PortalDom.Tags.Section);
  hero.className = PortalDom.Classes.ProductShellHero;
  hero.append(
    createOverviewHeroCopy(),
    createOverviewBadges(),
    createOverviewMetrics(state, liveActivity),
    createOverviewMetadata(state)
  );
  return hero;
}

function createOverviewHeroCopy(): HTMLElement {
  const heroCopy = document.createElement(PortalDom.Tags.Division);
  heroCopy.className = PortalDom.Classes.ProductHeroCopy;
  const heroTitle = document.createElement(PortalDom.Tags.HeadingTwo);
  heroTitle.textContent = PortalText.Resolve(PortalTextToken.ProductionShellReady);
  const heroDescription = document.createElement(PortalDom.Tags.Paragraph);
  heroDescription.textContent = PortalText.Resolve(PortalTextToken.LocalDataOnly);
  heroCopy.append(heroTitle, heroDescription);
  return heroCopy;
}

function createOverviewMetadata(state: PortalRuntimeState): HTMLElement {
  const metadata = document.createElement(PortalDom.Tags.DefinitionList);
  appendDetail(metadata, PortalDetails.Status, connectionStatus(state));
  appendDetail(
    metadata,
    PortalDetails.Transport,
    decodePortalDetailValue(PortalText.Resolve(PortalTextToken.LocalOnlyStatus))
  );
  appendDetail(metadata, PortalDetails.Events, decodePortalDetailValue(String(state.events.length)));
  return metadata;
}

function createOverviewBadges(): HTMLElement {
  const badges = document.createElement(PortalDom.Tags.Division);
  badges.className = PortalDom.Classes.AppStatusBar;
  badges.append(
    productBadge(PortalText.Resolve(PortalTextToken.ProductStatusLive)),
    productBadge(PortalText.Resolve(PortalTextToken.ProductStatusLocalOnly)),
    productBadge(PortalText.Resolve(PortalTextToken.ProductStatusPreviewOnly))
  );
  return badges;
}

function createOverviewMetrics(
  state: PortalRuntimeState,
  liveActivity: ReturnType<typeof resolveLiveActivityState>
): HTMLElement {
  const metrics = document.createElement(PortalDom.Tags.Division);
  metrics.className = PortalDom.Classes.ProductDashboard;
  metrics.append(
    productMetric(PortalDetails.Status, connectionStatus(state), PortalText.Resolve(PortalTextToken.ProductStatusLive)),
    productMetric(
      PortalDetails.Events,
      decodePortalDetailValue(String(state.events.length)),
      PortalText.Resolve(PortalTextToken.ActivityTimeline)
    ),
    productMetric(
      PortalDetails.LastObserved,
      detailFromValue(liveActivity.recentSummary?.lastObservedAt),
      PortalText.Resolve(PortalTextToken.RecentActivity)
    ),
    productMetric(
      PortalDetails.BrowserEvidence,
      detailFromValue(liveActivity.browserEvidenceReadModel?.returned),
      PortalText.Resolve(PortalTextToken.BrowserEvidence)
    )
  );
  return metrics;
}

function renderOverviewDashboard(
  container: HTMLElement,
  state: PortalRuntimeState,
  liveActivity: ReturnType<typeof resolveLiveActivityState>
): void {
  renderDashboard(container, (dashboard) => {
    renderEvidenceStore(dashboard, liveActivity);
    renderRecentActivity(dashboard, liveActivity);
    renderBrowserManagedStatus(dashboard, liveActivity);
    renderPolicyPreview(dashboard, state, liveActivity);
  });
}

function connectionStatus(state: PortalRuntimeState) {
  if (state.connectionState === PortalConnectionState.Connected) {
    return decodePortalDetailValue(PortalText.Resolve(PortalTextToken.Connected));
  }
  return decodePortalDetailValue(PortalText.Resolve(PortalTextToken.Unavailable));
}
