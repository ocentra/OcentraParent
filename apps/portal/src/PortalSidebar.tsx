import { useEffect, useState, type CSSProperties, type ReactElement } from 'react';
import { type PortalDisplayText } from '@ocentra-parent/portal-domain/display-text';
import { PortalDevTextToken, resolvePortalDevText } from '@ocentra-parent/portal-domain/display-text';
import { PortalDom } from '@ocentra-parent/portal-domain/contracts';
import { PortalFrameTuner } from '@ocentra-parent/portal-domain/frame-tuner';
import {
  PortalRouteDescriptors as ParentPortalRouteDescriptors,
  PortalRouteGroup as ParentPortalRouteGroup,
} from '@ocentra-parent/portal-domain/routes';
import {
  ParentBridgeConnectionState,
  ParentRouteGroup,
  ParentRouteMetadata,
  ParentSidebarRouteGroups,
  parentRouteHashPath,
  type ParentRouteGroupId,
  type ParentRouteId,
} from '../generated/parent-ui-bridge';
import type { PortalRenderActions } from './portal-actions';
import { PortalFrameBackdrop, PortalFrameBoundsOverlay } from './PortalFrameSurface';
import type { PortalRouteDescriptor } from '@ocentra-parent/portal-domain/routes';
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
  readonly route: ParentRouteId;
  readonly state: PortalRuntimeState;
}): ReactElement {
  const activeGroup = ParentRouteMetadata[route].group;
  const [openGroup, setOpenGroup] = useState<ParentRouteGroupId | null>(activeGroup);
  const sideTopContent = frameContentTarget(frameLayout, PortalFrameTuner.FrameTarget.SideTop);
  const sideBottomContent = frameContentTarget(frameLayout, PortalFrameTuner.FrameTarget.SideBottom);
  useEffect(() => setOpenGroup(activeGroup), [activeGroup]);
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
          {ParentSidebarRouteGroups.map((group) => (
            <RouteGroup
              activeRoute={route}
              group={group}
              key={group}
              onToggle={() => setOpenGroup((current) => (current === group ? null : group))}
              open={group === openGroup}
            />
          ))}
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
  activeRoute,
  group,
  onToggle,
  open,
}: {
  readonly activeRoute: ParentRouteId;
  readonly group: ParentRouteGroupId;
  readonly onToggle: () => void;
  readonly open: boolean;
}): ReactElement {
  const label = parentRouteGroupLabel(group);
  return (
    <details className={PortalDom.Classes.RouteGroup} open={open}>
      <summary
        aria-expanded={open}
        aria-label={`${open ? 'Collapse' : 'Expand'} ${String(label).toUpperCase()}`}
        className={PortalDom.Classes.RouteGroupLabel}
        onClick={(event) => {
          event.preventDefault();
          onToggle();
        }}
        role="button"
      >
        {label}
      </summary>
      {portalRouteDescriptorsForGroup(group).map((descriptor) => (
        <RouteLink activeRoute={activeRoute} descriptor={descriptor} key={descriptor.route} />
      ))}
    </details>
  );
}

export function portalRouteDescriptorsForGroup(group: ParentRouteGroupId): readonly PortalRouteDescriptor[] {
  const label = parentRouteGroupLabel(group);
  return ParentPortalRouteDescriptors.filter(
    (candidate) => candidate.group === label && ParentRouteMetadata[candidate.route].sidebar
  );
}

function parentRouteGroupLabel(group: ParentRouteGroupId): PortalDisplayText {
  switch (group) {
    case ParentRouteGroup.Monitor:
      return ParentPortalRouteGroup.Monitor;
    case ParentRouteGroup.Guide:
      return ParentPortalRouteGroup.Guide;
    case ParentRouteGroup.Operate:
      return ParentPortalRouteGroup.Operate;
    case ParentRouteGroup.DevTools:
      return ParentPortalRouteGroup.DevTools;
  }
}

function RouteLink({
  activeRoute,
  descriptor,
}: {
  readonly activeRoute: ParentRouteId;
  readonly descriptor: PortalRouteDescriptor;
}): ReactElement {
  const isActive = descriptor.route === activeRoute;
  return (
    <a
      aria-current={isActive ? PortalDom.Attributes.Page : undefined}
      aria-selected={isActive ? PortalDom.Attributes.True : PortalDom.Attributes.False}
      className={PortalDom.Classes.RouteLink}
      data-ocentra-parent-route-id={descriptor.route}
      href={parentRouteHashPath(descriptor.route)}
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

export function SidebarStatus({
  actions,
  state,
}: {
  readonly actions: PortalRenderActions;
  readonly state: PortalRuntimeState;
}): ReactElement {
  return (
    <div className={PortalDom.Classes.ProductSidebarPanel}>
      <div className={PortalDom.Classes.SidebarActions}>
        <div className={PortalDom.Classes.SidebarStatusButton} role="status">
          {connectionStatus(state)}
        </div>
        <button
          className={PortalDom.Classes.SidebarReconnectButton}
          onClick={actions.reconnect}
          type={PortalDom.ButtonType.Button}
        >
          {resolvePortalDevText(PortalDevTextToken.RetryStatus)}
        </button>
        {state.lastHostMessage === null ? null : (
          <p aria-live="polite" className={PortalDom.Classes.SidebarHostMessage}>
            {state.lastHostMessage}
          </p>
        )}
      </div>
    </div>
  );
}

function connectionStatus(state: PortalRuntimeState): PortalDisplayText {
  if (state.connectionState === ParentBridgeConnectionState.Connected) {
    return resolvePortalDevText(PortalDevTextToken.Connected);
  }
  return resolvePortalDevText(PortalDevTextToken.PendingServiceReadModel);
}
