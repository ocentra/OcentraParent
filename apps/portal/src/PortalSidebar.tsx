import type { CSSProperties, ReactElement } from 'react';
import { type DisplayText as PortalDisplayText } from '@ocentra-parent/schema-domain/text-contracts';
import { PortalDevTextToken, resolvePortalDevText } from '@ocentra-parent/schema-domain/text-portal-dev';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import { PortalFrameTuner } from '@ocentra-parent/portal-domain/frame-tuner';
import {
  PortalRouteGroup,
  PortalSidebarRouteDescriptors,
  type PortalRouteDescriptor,
} from '@ocentra-parent/portal-domain/routes';
import {
  PortalConnectionState,
  type PortalRoute as PortalRouteValue,
} from '@ocentra-parent/schema-domain/portal-contracts';
import type { PortalRenderActions } from './portal-actions';
import { PortalFrameBackdrop, PortalFrameBoundsOverlay } from './PortalFrameSurface';
import { routeDescriptor } from './portal-route-descriptor';
import { frameContentStyle, frameHostClassName } from './portal-frame-layout-style';
import { frameContentTarget } from './portal-frame-layout-state';
import type { PortalFrameLayout } from './portal-frame-layout-types';
import type { PortalRuntimeState } from './portal-state';

export function PortalSidebar({
  actions,
  frameLayout,
  route,
  state,
}: {
  readonly actions: PortalRenderActions;
  readonly frameLayout: PortalFrameLayout;
  readonly route: PortalRouteValue;
  readonly state: PortalRuntimeState;
}): ReactElement {
  const activeGroup = routeDescriptor(route).group;
  const sideTopContent = frameContentTarget(frameLayout, PortalFrameTuner.FrameTarget.SideTop);
  const sideBottomContent = frameContentTarget(frameLayout, PortalFrameTuner.FrameTarget.SideBottom);
  return (
    <aside className={PortalDom.Classes.AppSidebar}>
      <section
        aria-label={PortalFrameTuner.Text.TargetSideTop}
        className={frameHostClassName(PortalDom.Classes.SidebarNavFrame, sideTopContent)}
        style={frameContentStyle(sideTopContent, frameLayout.sideTop) as CSSProperties}
      >
        <PortalFrameBackdrop ariaLabel={PortalFrameTuner.Text.TargetSideTop} controls={frameLayout.sideTop} />
        <PortalFrameBoundsOverlay content={sideTopContent} />
        <nav className={PortalDom.Classes.Routes} role={PortalDom.Attributes.TabList}>
          <RouteGroup activeGroup={activeGroup} activeRoute={route} group={PortalRouteGroup.Monitor} />
          <RouteGroup activeGroup={activeGroup} activeRoute={route} group={PortalRouteGroup.Guide} />
          <RouteGroup activeGroup={activeGroup} activeRoute={route} group={PortalRouteGroup.Operate} />
        </nav>
      </section>
      <section
        aria-label={PortalFrameTuner.Text.TargetSideBottom}
        className={frameHostClassName(PortalDom.Classes.SidebarDeviceFrame, sideBottomContent)}
        style={frameContentStyle(sideBottomContent, frameLayout.sideBottom) as CSSProperties}
      >
        <PortalFrameBackdrop ariaLabel={PortalFrameTuner.Text.TargetSideBottom} controls={frameLayout.sideBottom} />
        <PortalFrameBoundsOverlay content={sideBottomContent} />
        <SidebarStatus actions={actions} state={state} />
      </section>
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
      {PortalSidebarRouteDescriptors.filter((candidate) => candidate.group === group).map((descriptor) => (
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
      <span aria-hidden={true} className={PortalDom.Classes.RouteLinkFrame} />
      <span aria-hidden={true} className={PortalDom.Classes.RouteLinkIcon} />
      <span className={PortalDom.Classes.RouteLinkCopy}>
        <span className={PortalDom.Classes.RouteLinkLabel}>{descriptor.label}</span>
        <span className={PortalDom.Classes.RouteLinkDescription}>{descriptor.description}</span>
      </span>
      <span aria-hidden={true} className={PortalDom.Classes.RouteLinkArrow} />
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
  return (
    <div className={PortalDom.Classes.ProductSidebarPanel}>
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
          {resolvePortalDevText(PortalDevTextToken.Reconnect)}
        </button>
      </div>
    </div>
  );
}

function connectionStatus(state: PortalRuntimeState): PortalDisplayText {
  if (state.connectionState === PortalConnectionState.Connected) {
    return resolvePortalDevText(PortalDevTextToken.Connected);
  }
  return resolvePortalDevText(PortalDevTextToken.Unavailable);
}
