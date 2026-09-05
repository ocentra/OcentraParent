import type { ReactElement, ReactNode } from 'react';
import { PortalDevTextToken, resolvePortalDevText } from '@ocentra-parent/portal-domain/display-text';
import { PortalDom, PortalTheme, type PortalThemeValue } from '@ocentra-parent/portal-domain/contracts';
import { PortalAssets, PortalUnifiedChrome } from '@ocentra-parent/portal-domain/unified-chrome';
import { ParentRoute, parentRouteHashPath } from '../generated/parent-ui-bridge';
import { UnifiedFooter } from '../../../vendor/ocentra-parent-core-ui/Footer/UnifiedFooter';
import { BrandedLoadingSpinner } from '../../../vendor/ocentra-parent-core-ui/AppPages/ParentPortal/BrandedLoadingSpinner';
import { ScopeToggle } from '../../../vendor/ocentra-parent-core-ui/AppPages/ParentPortal/ScopeToggle/ScopeToggle';
import { UnifiedPageShell } from '../../../vendor/ocentra-parent-core-ui/Shell/UnifiedPageShell';
import { PortalBackgroundLayer } from './PortalBackgroundLayer';
import { PortalHeaderSvgFrame } from './PortalHeaderSvgFrame';

const headerThemeLightSelectedIcon = svgDataUrl(
  '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><defs><filter id="sun-shadow" x="-45%" y="-45%" width="190%" height="190%"><feDropShadow dx="0" dy="1" stdDeviation="0.5" flood-color="#fff6c0" flood-opacity="0.52"/></filter></defs><g filter="url(#sun-shadow)" fill="none" stroke="#241700" stroke-width="2.15" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="4"/><path d="M12 2v2M12 20v2M4.93 4.93l1.41 1.41M17.66 17.66l1.41 1.41M2 12h2M20 12h2M4.93 19.07l1.41-1.41M17.66 6.34l1.41-1.41"/></g></svg>'
);
const headerThemeLightIdleIcon = svgDataUrl(
  '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><defs><filter id="sun-idle-shadow" x="-45%" y="-45%" width="190%" height="190%"><feDropShadow dx="0" dy="0" stdDeviation="1" flood-color="#1b1000" flood-opacity="0.72"/></filter></defs><g filter="url(#sun-idle-shadow)" fill="none" stroke="#ffd36a" stroke-width="2.1" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="4"/><path d="M12 2v2M12 20v2M4.93 4.93l1.41 1.41M17.66 17.66l1.41 1.41M2 12h2M20 12h2M4.93 19.07l1.41-1.41M17.66 6.34l1.41-1.41"/></g></svg>'
);
const headerThemeDarkIdleIcon = svgDataUrl(
  '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><path d="M20.5 14.5A8.5 8.5 0 0 1 9.5 3.5A7 7 0 1 0 20.5 14.5Z" fill="#dff8ff" stroke="#89e8ff" stroke-width="1.7" stroke-linejoin="round"/></svg>'
);
const headerThemeDarkSelectedIcon = svgDataUrl(
  '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><defs><filter id="moon-selected-shadow" x="-45%" y="-45%" width="190%" height="190%"><feDropShadow dx="0" dy="1" stdDeviation="0.45" flood-color="#fff6c0" flood-opacity="0.45"/></filter></defs><path filter="url(#moon-selected-shadow)" d="M20.5 14.5A8.5 8.5 0 0 1 9.5 3.5A7 7 0 1 0 20.5 14.5Z" fill="#201400" stroke="#2b1a00" stroke-width="1.8" stroke-linejoin="round"/></svg>'
);
const headerThemeToggleConfig = {
  svg: {
    width: 70,
    height: 20,
    viewportInset: 1,
  },
  layout: {
    titleAnchorX: 0,
    titleBoxY: 0,
    titleBoxMinWidth: 0,
    titleBoxPaddingX: 0,
    titleBoxHeight: 0,
    titleBoxRadius: 0,
    titleBoxRightRadius: 0,
    trackY: 1,
    trackMinWidth: 60,
    trackHeight: 18,
    optionPaddingX: 8,
    optionIconSize: 13,
    optionIconGap: 0,
    dividerWidth: 1,
    outerPadX: 1,
    outerPadY: 0,
    outerRadius: 3,
    outerPaddingRight: 1,
  },
  track: {
    radius: 3,
  },
  titleBox: {
    strokeWidth: 0,
    innerStrokeWidth: 0,
    glowStrokeWidth: 0,
  },
  slider: {
    inset: 1,
    gapFromDivider: 2,
    radius: 2,
  },
  text: {
    titleFontSize: 0,
    optionFontSize: 0,
    fontWeight: 800,
    optionFontWeight: 760,
  },
  colors: {
    titleBoxStroke: 'transparent',
    titleBoxStrokeHover: 'transparent',
    titleBoxGlow: 'transparent',
    sliderTop: '#ffe79c',
    sliderBottom: '#e6a823',
    sliderStroke: '#fff4bd',
    sliderGlow: '#ffd36a',
    shine: '#fff8dc',
  },
  opacity: {
    trackGlowIdle: 0,
    trackGlowHover: 0,
    titleGlowIdle: 0,
    titleGlowHover: 0,
    outerGlowIdle: 0,
    outerGlowHover: 0,
    dividerGlowIdle: 0,
    dividerGlowHover: 0,
    sliderGlowIdle: 0.16,
    sliderGlowHover: 0.28,
    shineIdle: 0.22,
    shineHover: 0.28,
  },
  hover: {
    pressScale: 1,
  },
} as const;

const headerThemeToggleVisualStyle = { opacity: 1 } as const;

type PortalUnifiedShellProps = {
  readonly children: ReactNode;
  readonly onAuthOpen: () => void;
  readonly onThemeChange: (theme: PortalThemeValue) => void;
  readonly routeTransitionActive?: boolean;
  readonly theme: PortalThemeValue;
};

type PortalOutlineHeaderProps = {
  readonly onAuthOpen: () => void;
  readonly routeTransitionActive: boolean;
  readonly onThemeChange: (theme: PortalThemeValue) => void;
  readonly theme: PortalThemeValue;
};

const shellHeaderExtensionAttributes = {
  [PortalUnifiedChrome.Attributes.ShellHeaderExtension]: PortalDom.Attributes.True,
} as const;

function goHome(): void {
  window.location.hash = parentRouteHashPath(ParentRoute.Overview);
}

function PortalHeaderConnector({ children }: { readonly children?: ReactNode }): ReactElement {
  const connectorAttributes =
    children === undefined ? { [PortalDom.Attributes.AriaHidden]: PortalDom.Attributes.True } : undefined;
  const connectorClassName =
    children === undefined
      ? PortalUnifiedChrome.Classes.OutlineHeaderConnector
      : [
          PortalUnifiedChrome.Classes.OutlineHeaderConnector,
          PortalUnifiedChrome.Classes.OutlineHeaderConnectorInteractive,
        ].join(PortalDom.Classes.ClassNameSeparator);
  return (
    <span {...connectorAttributes} className={connectorClassName}>
      <svg
        aria-hidden={PortalDom.Attributes.True}
        className={PortalUnifiedChrome.Classes.OutlineHeaderConnectorSvg}
        focusable={PortalDom.Attributes.False}
        preserveAspectRatio={PortalUnifiedChrome.Svg.PreserveAspectRatioNone}
        viewBox={PortalUnifiedChrome.Svg.HeaderConnectorViewBox}
      >
        <rect
          className={PortalUnifiedChrome.Classes.OutlineHeaderConnectorBox}
          fill={PortalUnifiedChrome.Svg.FillNone}
          height={44}
          rx={0}
          stroke={PortalUnifiedChrome.Svg.FrameColorCyan}
          vectorEffect={PortalUnifiedChrome.Svg.VectorEffectNonScalingStroke}
          width={100}
          x={0}
          y={0}
        />
      </svg>
      {children}
    </span>
  );
}

function PortalOutlineHeader({
  onAuthOpen,
  onThemeChange,
  routeTransitionActive,
  theme,
}: PortalOutlineHeaderProps): ReactElement {
  return (
    <header {...shellHeaderExtensionAttributes} className={PortalUnifiedChrome.Classes.OutlineHeader}>
      <PortalHeaderHomeAction />
      <PortalHeaderConnector />
      <PortalHeaderBrand routeTransitionActive={routeTransitionActive} />
      <PortalHeaderConnector>
        <PortalHeaderThemeToggle onThemeChange={onThemeChange} theme={theme} />
      </PortalHeaderConnector>
      <PortalHeaderLoginAction onAuthOpen={onAuthOpen} />
    </header>
  );
}

function PortalHeaderHomeAction(): ReactElement {
  return (
    <button
      aria-label={resolvePortalDevText(PortalDevTextToken.HeaderHome)}
      className={PortalUnifiedChrome.Classes.OutlineHeaderAction}
      onClick={goHome}
      type={PortalDom.ButtonType.Button}
    >
      <PortalHeaderSvgFrame>
        <span className={PortalUnifiedChrome.Classes.OutlineHeaderActionContent}>
          <PortalHeaderActionIcon src={PortalAssets.HeaderHomeIcon} />
          <span className={PortalUnifiedChrome.Classes.OutlineHeaderActionLabel}>
            {resolvePortalDevText(PortalDevTextToken.HeaderHome)}
          </span>
        </span>
      </PortalHeaderSvgFrame>
    </button>
  );
}

function PortalHeaderBrand({ routeTransitionActive }: { readonly routeTransitionActive: boolean }): ReactElement {
  const logoMountAttributes = routeTransitionActive
    ? { [PortalUnifiedChrome.Attributes.HeaderLogoLoading]: PortalDom.Attributes.True }
    : undefined;
  return (
    <div className={PortalUnifiedChrome.Classes.OutlineHeaderBrand}>
      <PortalHeaderSvgFrame />
      <span className={PortalUnifiedChrome.Classes.OutlineHeaderBrandPart}>
        {resolvePortalDevText(PortalDevTextToken.HeaderBrandLeft)}
      </span>
      <span
        aria-hidden={PortalDom.Attributes.True}
        className={PortalUnifiedChrome.Classes.OutlineHeaderBrandLogoMount}
        {...logoMountAttributes}
      >
        <img
          alt={PortalUnifiedChrome.Alt.DecorativeImage}
          className={PortalUnifiedChrome.Classes.OutlineHeaderBrandLogo}
          src={PortalAssets.HeaderLogo}
        />
        {routeTransitionActive ? (
          <span className={PortalUnifiedChrome.Classes.OutlineHeaderBrandLogoSpinner}>
            <BrandedLoadingSpinner size="small" />
          </span>
        ) : null}
      </span>
      <span className={PortalUnifiedChrome.Classes.OutlineHeaderBrandPartMuted}>
        {resolvePortalDevText(PortalDevTextToken.HeaderBrandRight)}
      </span>
    </div>
  );
}

function PortalHeaderLoginAction({ onAuthOpen }: { readonly onAuthOpen: () => void }): ReactElement {
  return (
    <button
      aria-label={resolvePortalDevText(PortalDevTextToken.HeaderLogin)}
      className={PortalUnifiedChrome.Classes.OutlineHeaderAction}
      onClick={onAuthOpen}
      type={PortalDom.ButtonType.Button}
    >
      <PortalHeaderSvgFrame>
        <span className={PortalUnifiedChrome.Classes.OutlineHeaderActionContent}>
          <PortalHeaderActionIcon src={PortalAssets.HeaderLoginIcon} />
          <span className={PortalUnifiedChrome.Classes.OutlineHeaderActionLabel}>
            {resolvePortalDevText(PortalDevTextToken.HeaderLogin)}
          </span>
        </span>
      </PortalHeaderSvgFrame>
    </button>
  );
}

function PortalHeaderActionIcon({ src }: { readonly src: string }): ReactElement {
  return (
    <span aria-hidden={PortalDom.Attributes.True} className={PortalUnifiedChrome.Classes.OutlineHeaderActionIcon}>
      <img
        alt={PortalUnifiedChrome.Alt.DecorativeImage}
        className={PortalUnifiedChrome.Classes.OutlineHeaderActionIconImage}
        src={src}
      />
    </span>
  );
}

function PortalHeaderThemeToggle({
  onThemeChange,
  theme,
}: {
  readonly onThemeChange: (theme: PortalThemeValue) => void;
  readonly theme: PortalThemeValue;
}): ReactElement {
  const lightActive = theme === PortalTheme.Light;
  const darkActive = theme === PortalTheme.Dark;
  const lightIcon = darkActive ? headerThemeLightIdleIcon : headerThemeLightSelectedIcon;
  const darkIcon = darkActive ? headerThemeDarkSelectedIcon : headerThemeDarkIdleIcon;
  return (
    <span
      aria-label={resolvePortalDevText(PortalDevTextToken.DisplayTheme)}
      className={PortalUnifiedChrome.Classes.OutlineHeaderTheme}
      role="group"
    >
      <span aria-hidden={PortalDom.Attributes.True} className={PortalUnifiedChrome.Classes.OutlineHeaderThemeVisual}>
        <ScopeToggle
          config={headerThemeToggleConfig}
          disabled
          options={[
            {
              value: PortalTheme.Light,
              label: resolvePortalDevText(PortalDevTextToken.ThemeLight),
              iconHref: lightIcon,
            },
            {
              value: PortalTheme.Dark,
              label: resolvePortalDevText(PortalDevTextToken.ThemeDark),
              iconHref: darkIcon,
            },
          ]}
          style={headerThemeToggleVisualStyle}
          title={resolvePortalDevText(PortalDevTextToken.DisplayTheme)}
          value={theme}
        />
      </span>
      <button
        aria-label={resolvePortalDevText(PortalDevTextToken.ThemeLight)}
        aria-pressed={lightActive ? PortalDom.Attributes.True : PortalDom.Attributes.False}
        className={[
          PortalUnifiedChrome.Classes.OutlineHeaderThemeButton,
          PortalUnifiedChrome.Classes.OutlineHeaderThemeButtonLeft,
        ].join(PortalDom.Classes.ClassNameSeparator)}
        onClick={() => onThemeChange(PortalTheme.Light)}
        type={PortalDom.ButtonType.Button}
      />
      <button
        aria-label={resolvePortalDevText(PortalDevTextToken.ThemeDark)}
        aria-pressed={darkActive ? PortalDom.Attributes.True : PortalDom.Attributes.False}
        className={[
          PortalUnifiedChrome.Classes.OutlineHeaderThemeButton,
          PortalUnifiedChrome.Classes.OutlineHeaderThemeButtonRight,
        ].join(PortalDom.Classes.ClassNameSeparator)}
        onClick={() => onThemeChange(PortalTheme.Dark)}
        type={PortalDom.ButtonType.Button}
      />
    </span>
  );
}

function svgDataUrl(svg: string): string {
  return `data:image/svg+xml,${encodeURIComponent(svg)}`;
}

export function PortalUnifiedShell({
  children,
  onAuthOpen,
  onThemeChange,
  routeTransitionActive = false,
  theme,
}: PortalUnifiedShellProps): ReactElement {
  return (
    <UnifiedPageShell
      background={<PortalBackgroundLayer theme={theme} />}
      className={PortalUnifiedChrome.Classes.Shell}
      footer={<UnifiedFooter appVersion={PortalUnifiedChrome.Version.App} />}
      header={
        <PortalOutlineHeader
          onAuthOpen={onAuthOpen}
          onThemeChange={onThemeChange}
          routeTransitionActive={routeTransitionActive}
          theme={theme}
        />
      }
      viewportLocked={true}
      workClassName={PortalUnifiedChrome.Classes.ShellWork}
    >
      {children}
    </UnifiedPageShell>
  );
}
