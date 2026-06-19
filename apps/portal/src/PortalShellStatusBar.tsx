import { useMemo, type ReactElement } from 'react';
import {
  PortalDom,
  type PortalRoute as PortalRouteValue,
} from '@ocentra-parent/portal-domain/contracts';
import {
  PARENT_PORTAL_SHELL_STATUS_COPY,
  PARENT_PORTAL_SHELL_STATUS_DOM,
  resolveParentPortalShellStatus,
  type ParentPortalShellStatusCard,
} from '@ocentra-parent/portal-domain/parent-portal-shell-status';
import type { PortalRuntimeState } from './portal-state';

export function PortalShellStatusBar({
  route,
  state,
}: {
  readonly route: PortalRouteValue;
  readonly state: PortalRuntimeState;
}): ReactElement {
  const latestEventId = state.events[0]?.eventId ?? '';
  const shellStatus = useMemo(
    () =>
      resolveParentPortalShellStatus({
        route,
        connectionState: state.connectionState,
        events: state.events,
      }),
    [latestEventId, route, state.connectionState, state.events.length]
  );

  return (
    <section
      aria-label={PARENT_PORTAL_SHELL_STATUS_COPY.Summary}
      className={PARENT_PORTAL_SHELL_STATUS_DOM.Panel}
    >
      <div className={PortalDom.Classes.PageHeader}>
        <h2>{PARENT_PORTAL_SHELL_STATUS_COPY.Summary}</h2>
        <div className={PortalDom.Classes.AppStatusBar}>
          <span className={PortalDom.Classes.ProductBadge}>{shellStatus.globalConnectionState}</span>
          <span className={PortalDom.Classes.ProductBadge}>{shellStatus.dataSourceLabel}</span>
        </div>
      </div>
      <div className={PARENT_PORTAL_SHELL_STATUS_DOM.Grid}>
        {shellStatus.cards.map((card) => (
          <article className={shellStatusCardClassName(card)} key={card.id}>
            <span className={PortalDom.Classes.ProductMetricLabel}>{card.label}</span>
            <strong className={PortalDom.Classes.ProductMetricValue}>{card.value}</strong>
            <span className={PARENT_PORTAL_SHELL_STATUS_DOM.Detail}>{card.detail}</span>
          </article>
        ))}
      </div>
    </section>
  );
}

function shellStatusCardClassName(card: ParentPortalShellStatusCard): string {
  return [
    PortalDom.Classes.ProductMetric,
    PARENT_PORTAL_SHELL_STATUS_DOM.Card,
    shellStatusToneClassName(card),
  ].join(PortalDom.Classes.ClassNameSeparator);
}

function shellStatusToneClassName(card: ParentPortalShellStatusCard): string {
  switch (card.tone) {
    case 'cyan':
      return PARENT_PORTAL_SHELL_STATUS_DOM.ToneCyan;
    case 'gold':
      return PARENT_PORTAL_SHELL_STATUS_DOM.ToneGold;
    case 'purple':
      return PARENT_PORTAL_SHELL_STATUS_DOM.TonePurple;
    case 'red':
      return PARENT_PORTAL_SHELL_STATUS_DOM.ToneRed;
    case 'muted':
      return PARENT_PORTAL_SHELL_STATUS_DOM.ToneMuted;
  }
}
