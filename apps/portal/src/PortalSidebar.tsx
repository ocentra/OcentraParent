import type { ReactElement } from 'react';
import {
  PortalConnectionState,
  PortalDom,
  PortalFormatting,
  PortalRouteDescriptors,
  PortalRouteGroup,
  PortalText,
  PortalTextToken,
  type PortalDisplayText,
  type PortalRoute as PortalRouteValue,
  type PortalRouteDescriptor,
} from '@ocentra-parent/portal-domain/contracts';
import type { PortalRenderActions } from './portal-actions';
import { routeDescriptor } from './portal-route-descriptor';
import type { PortalRuntimeState } from './portal-state';

export function PortalSidebar({
  actions,
  route,
  state,
}: {
  readonly actions: PortalRenderActions;
  readonly route: PortalRouteValue;
  readonly state: PortalRuntimeState;
}): ReactElement {
  const activeGroup = routeDescriptor(route).group;
  return (
    <aside className={PortalDom.Classes.AppSidebar}>
      <nav className={PortalDom.Classes.Routes} role={PortalDom.Attributes.TabList}>
        <RouteGroup activeGroup={activeGroup} activeRoute={route} group={PortalRouteGroup.Monitor} />
        <RouteGroup activeGroup={activeGroup} activeRoute={route} group={PortalRouteGroup.Guide} />
        <RouteGroup activeGroup={activeGroup} activeRoute={route} group={PortalRouteGroup.Operate} />
      </nav>
      <SidebarStatus actions={actions} state={state} />
    </aside>
  );
}

function RouteGroup({
  activeGroup,
  activeRoute,
  group,
}: {
  readonly activeGroup: PortalDisplayText;
  readonly activeRoute: PortalRouteValue;
  readonly group: PortalDisplayText;
}): ReactElement {
  return (
    <details className={PortalDom.Classes.RouteGroup} open={group === activeGroup}>
      <summary className={PortalDom.Classes.RouteGroupLabel}>{group}</summary>
      {PortalRouteDescriptors.filter((candidate) => candidate.group === group).map((descriptor) => (
        <RouteLink activeRoute={activeRoute} descriptor={descriptor} key={descriptor.route} />
      ))}
    </details>
  );
}

function RouteLink({
  activeRoute,
  descriptor,
}: {
  readonly activeRoute: PortalRouteValue;
  readonly descriptor: PortalRouteDescriptor;
}): ReactElement {
  const isActive = descriptor.route === activeRoute;
  return (
    <a
      aria-current={isActive ? PortalDom.Attributes.Page : undefined}
      aria-selected={isActive ? PortalDom.Attributes.True : PortalDom.Attributes.False}
      className={PortalDom.Classes.RouteLink}
      data-ocentra-parent-route-id={descriptor.route}
      href={`${PortalDom.HashPrefix}${descriptor.route}`}
      role={PortalDom.Attributes.Tab}
    >
      <span className={PortalDom.Classes.RouteLinkLabel}>{descriptor.label}</span>
      <span className={PortalDom.Classes.RouteLinkDescription}>{descriptor.description}</span>
    </a>
  );
}

function SidebarStatus({
  actions,
  state,
}: {
  readonly actions: PortalRenderActions;
  readonly state: PortalRuntimeState;
}): ReactElement {
  const detail = [PortalText.Resolve(PortalTextToken.ProductStatusLocalOnly), String(state.events.length)].join(
    PortalFormatting.EventDetailSeparator
  );
  return (
    <div className={PortalDom.Classes.ProductSidebarPanel}>
      <strong>{PortalText.Resolve(PortalTextToken.FamilyRulesTitle)}</strong>
      <div className={PortalDom.Classes.SidebarActions}>
        <button
          className={PortalDom.Classes.SidebarStatusButton}
          onClick={actions.reconnect}
          type={PortalDom.ButtonType.Button}
        >
          {connectionStatus(state)}
        </button>
        <button
          className={PortalDom.Classes.SidebarReconnectButton}
          onClick={actions.reconnect}
          type={PortalDom.ButtonType.Button}
        >
          {PortalText.Resolve(PortalTextToken.Reconnect)}
        </button>
      </div>
      <p>{detail}</p>
    </div>
  );
}

function connectionStatus(state: PortalRuntimeState): PortalDisplayText {
  if (state.connectionState === PortalConnectionState.Connected) {
    return PortalText.Resolve(PortalTextToken.Connected);
  }
  return PortalText.Resolve(PortalTextToken.Unavailable);
}
