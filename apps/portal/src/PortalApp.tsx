import { useMemo, type CSSProperties, type ReactElement } from 'react';
import { PortalDom, type PortalThemeValue } from '@ocentra-parent/portal-domain/contracts';
import { PortalFrameTuner } from '@ocentra-parent/portal-domain/frame-tuner';
import { PortalRouteDescriptors } from '@ocentra-parent/portal-domain/routes';
import { ParentRoute, type ParentRouteId } from '../generated/parent-ui-bridge';
import { usePortalAppBehavior, type PortalAppBehavior } from './portal-app-behavior';
import type { PortalRenderActions } from './portal-actions';
import type { PortalRuntimeState } from './portal-state';
import { ParentPortalRoute } from './ParentPortalRoute';
import { PortalAuthDialog } from './PortalAuthDialog';
import { PortalFrameBackdrop, PortalFrameBoundsOverlay } from './PortalFrameSurface';
import { PortalFrameTunerRoute } from './PortalFrameTunerRoute';
import { PortalSidebar } from './PortalSidebar';
import { PortalUnifiedShell } from './PortalUnifiedChrome';
import { carouselStyle, frameContentStyle, frameHostClassName, goldenCardStyle } from './portal-frame-layout-style';
import { frameContentTarget } from './portal-frame-layout-state';
import { PortalRouteContentMount } from './portal-route-content';
import type { PortalFrameContentTargetLayout, PortalFrameLayout } from './portal-frame-layout-types';

type PortalAppProps = {
  readonly actions: PortalRenderActions;
  readonly rerender: () => void;
  readonly revision: number;
  readonly route: ParentRouteId;
  readonly state: PortalRuntimeState;
  readonly theme: PortalThemeValue;
  readonly onThemeChange: (theme: PortalThemeValue) => void;
  readonly onProductSurfaceReady: () => void;
};

export function PortalApp(props: PortalAppProps): ReactElement {
  const behavior = usePortalAppBehavior({
    actions: props.actions,
    onProductSurfaceReady: props.onProductSurfaceReady,
    route: props.route,
    state: props.state,
  });

  if (behavior.isFrameTuner) {
    return <PortalFrameTunerRouteShell {...props} behavior={behavior} />;
  }

  if (behavior.isProductRoute) {
    return <PortalProductRouteShell {...props} behavior={behavior} />;
  }

  return <PortalProtocolRouteShell {...props} behavior={behavior} />;
}

type PortalProductRouteShellProps = PortalAppProps & {
  readonly behavior: PortalAppBehavior;
};

type PortalFrameTunerRouteShellProps = PortalAppProps & {
  readonly behavior: PortalAppBehavior;
};

type PortalProtocolRouteShellProps = PortalAppProps & {
  readonly behavior: PortalAppBehavior;
};

function PortalFrameTunerRouteShell({ behavior, onThemeChange, theme }: PortalFrameTunerRouteShellProps): ReactElement {
  return (
    <>
      <PortalUnifiedShell
        onAuthOpen={behavior.openAuthDialog}
        onThemeChange={onThemeChange}
        routeTransitionActive={behavior.headerRouteTransitionActive}
        theme={theme}
      >
        <PortalFrameTunerRoute layout={behavior.frameLayout} onLayoutChange={behavior.setFrameLayout} />
      </PortalUnifiedShell>
      <PortalAuthDialogMount open={behavior.authOpen} onClose={behavior.closeAuthDialog} />
    </>
  );
}

function PortalProductRouteShell({
  actions,
  behavior,
  onProductSurfaceReady,
  onThemeChange,
  route,
  state,
  theme,
}: PortalProductRouteShellProps): ReactElement {
  const controls =
    route === ParentRoute.Assistant
      ? behavior.frameLayout.parentPortal.chatInterface
      : behavior.frameLayout.parentPortal.mainApp;
  return (
    <>
      <PortalUnifiedShell
        onAuthOpen={behavior.openAuthDialog}
        onThemeChange={onThemeChange}
        routeTransitionActive={behavior.headerRouteTransitionActive}
        theme={theme}
      >
        <ParentPortalRoute
          actions={actions}
          controls={controls}
          lanPairingAutoScanSequence={behavior.lanPairingAutoScanSequence}
          onProductSurfaceReady={onProductSurfaceReady}
          route={route}
          screenSummaryPanel={behavior.screenSummaryPanel}
          state={state}
        />
      </PortalUnifiedShell>
      <PortalAuthDialogMount open={behavior.authOpen} onClose={behavior.closeAuthDialog} />
    </>
  );
}

function PortalProtocolRouteShell({
  actions,
  behavior,
  onThemeChange,
  route,
  state,
  rerender,
  revision,
  theme,
}: PortalProtocolRouteShellProps): ReactElement {
  const { appFrameStyle, appMainClassName, appMainStyle, mainContent } = usePortalProtocolFrameState(
    behavior.routeFrameLayout
  );
  return (
    <>
      <PortalUnifiedShell
        onAuthOpen={behavior.openAuthDialog}
        onThemeChange={onThemeChange}
        routeTransitionActive={behavior.headerRouteTransitionActive}
        theme={theme}
      >
        <div className={PortalDom.Classes.AppFrame} style={appFrameStyle}>
          <PortalSidebar actions={actions} frameLayout={behavior.routeFrameLayout} route={route} state={state} />
          <main aria-label={PortalFrameTuner.Text.TargetMain} className={appMainClassName} style={appMainStyle}>
            <PortalFrameBackdrop
              ariaLabel={PortalFrameTuner.Text.PreviewMain}
              controls={behavior.routeFrameLayout.main}
            />
            <PortalFrameBoundsOverlay content={mainContent} />
            <div className={PortalFrameTuner.Classes.FrameContent}>
              <PageHeader route={route} />
              <PortalRouteContentMount
                actions={actions}
                rerender={rerender}
                revision={revision}
                route={route}
                state={state}
                theme={theme}
              />
            </div>
          </main>
        </div>
      </PortalUnifiedShell>
      <PortalAuthDialogMount open={behavior.authOpen} onClose={behavior.closeAuthDialog} />
    </>
  );
}

type PortalProtocolFrameState = {
  readonly appFrameStyle: CSSProperties;
  readonly appMainClassName: string;
  readonly appMainStyle: CSSProperties;
  readonly mainContent: PortalFrameContentTargetLayout;
};

function usePortalProtocolFrameState(routeFrameLayout: PortalFrameLayout): PortalProtocolFrameState {
  const appFrameStyle = useMemo<CSSProperties>(
    () => ({
      columnGap: routeFrameLayout.shell.frameGap,
      gridTemplateColumns: `${routeFrameLayout.shell.sidebarWidth}px minmax(0, 1fr)`,
      padding: routeFrameLayout.shell.shellEdge,
      ...carouselStyle(routeFrameLayout.carousel),
      ...goldenCardStyle(routeFrameLayout.goldenCard),
      [PortalFrameTuner.CssVar.SideBottomHeight]: `${routeFrameLayout.shell.sideBottomHeight}px`,
      [PortalFrameTuner.CssVar.SideStackGap]: `${routeFrameLayout.shell.sideStackGap}px`,
    }),
    [
      routeFrameLayout.shell.frameGap,
      routeFrameLayout.carousel,
      routeFrameLayout.goldenCard,
      routeFrameLayout.shell.shellEdge,
      routeFrameLayout.shell.sidebarWidth,
      routeFrameLayout.shell.sideBottomHeight,
      routeFrameLayout.shell.sideStackGap,
    ]
  );
  const mainContent = frameContentTarget(routeFrameLayout, PortalFrameTuner.FrameTarget.Main);
  const appMainStyle = useMemo<CSSProperties>(
    () => frameContentStyle(mainContent, routeFrameLayout.main) as CSSProperties,
    [routeFrameLayout.main, mainContent]
  );
  const appMainClassName = useMemo(() => frameHostClassName(PortalDom.Classes.AppMain, mainContent), [mainContent]);
  return { appFrameStyle, appMainClassName, appMainStyle, mainContent };
}

function PortalAuthDialogMount({
  onClose,
  open,
}: {
  readonly onClose: () => void;
  readonly open: boolean;
}): ReactElement | null {
  return open ? <PortalAuthDialog onClose={onClose} /> : null;
}

function PageHeader({ route }: { readonly route: ParentRouteId }): ReactElement {
  const descriptor = routeDescriptor(route);
  return (
    <header className={PortalDom.Classes.AppHeader}>
      <div className={PortalDom.Classes.PageHeader}>
        <h2 className={PortalDom.Classes.PageTitle}>{descriptor.label}</h2>
        <p className={PortalDom.Classes.PageDescription}>{descriptor.description}</p>
      </div>
    </header>
  );
}

function routeDescriptor(route: ParentRouteId) {
  const descriptor = PortalRouteDescriptors.find((candidate) => candidate.route === route);
  if (descriptor === undefined) {
    return PortalRouteDescriptors[0]!;
  }
  return descriptor;
}
