import type { ReactElement, ReactNode } from 'react';
import {
  PortalAssets,
  PortalDom,
  PortalRoute,
  PortalText,
  PortalTextToken,
  PortalUnifiedChrome,
} from '@ocentra-parent/portal-domain/contracts';
import { UnifiedFooter } from '../../../vendor/ocentra-games-core-ui/Footer/UnifiedFooter';
import { UnifiedPageShell } from '../../../vendor/ocentra-games-core-ui/Shell/UnifiedPageShell';
import { PortalHeaderSvgFrame } from './PortalHeaderSvgFrame';

type PortalUnifiedShellProps = {
  readonly children: ReactNode;
  readonly onAuthOpen: () => void;
};

type PortalOutlineHeaderProps = {
  readonly onAuthOpen: () => void;
};

const shellHeaderExtensionAttributes = {
  [PortalUnifiedChrome.Attributes.ShellHeaderExtension]: PortalDom.Attributes.True,
} as const;

function goHome(): void {
  window.location.hash = `${PortalDom.HashPrefix}${PortalRoute.Overview}`;
}

function PortalHeaderConnector(): ReactElement {
  return (
    <span aria-hidden={PortalDom.Attributes.True} className={PortalUnifiedChrome.Classes.OutlineHeaderConnector}>
      <svg
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
    </span>
  );
}

function PortalOutlineHeader({ onAuthOpen }: PortalOutlineHeaderProps): ReactElement {
  return (
    <header {...shellHeaderExtensionAttributes} className={PortalUnifiedChrome.Classes.OutlineHeader}>
      <button
        aria-label={PortalText.Resolve(PortalTextToken.HeaderHome)}
        className={PortalUnifiedChrome.Classes.OutlineHeaderAction}
        onClick={goHome}
        type={PortalDom.ButtonType.Button}
      >
        <PortalHeaderSvgFrame />
        <span aria-hidden={PortalDom.Attributes.True} className={PortalUnifiedChrome.Classes.OutlineHeaderActionIcon}>
          <img
            alt={PortalUnifiedChrome.Alt.DecorativeImage}
            className={PortalUnifiedChrome.Classes.OutlineHeaderActionIconImage}
            src={PortalAssets.HeaderHomeIcon}
          />
        </span>
        <span className={PortalUnifiedChrome.Classes.OutlineHeaderActionLabel}>
          {PortalText.Resolve(PortalTextToken.HeaderHome)}
        </span>
      </button>
      <PortalHeaderConnector />
      <div className={PortalUnifiedChrome.Classes.OutlineHeaderBrand}>
        <PortalHeaderSvgFrame />
        <span className={PortalUnifiedChrome.Classes.OutlineHeaderBrandPart}>
          {PortalText.Resolve(PortalTextToken.HeaderBrandLeft)}
        </span>
        <span
          aria-hidden={PortalDom.Attributes.True}
          className={PortalUnifiedChrome.Classes.OutlineHeaderBrandLogoMount}
        >
          <img
            alt={PortalUnifiedChrome.Alt.DecorativeImage}
            className={PortalUnifiedChrome.Classes.OutlineHeaderBrandLogo}
            src={PortalAssets.HeaderLogo}
          />
        </span>
        <span className={PortalUnifiedChrome.Classes.OutlineHeaderBrandPartMuted}>
          {PortalText.Resolve(PortalTextToken.HeaderBrandRight)}
        </span>
      </div>
      <PortalHeaderConnector />
      <button
        aria-label={PortalText.Resolve(PortalTextToken.HeaderLogin)}
        className={PortalUnifiedChrome.Classes.OutlineHeaderAction}
        onClick={onAuthOpen}
        type={PortalDom.ButtonType.Button}
      >
        <PortalHeaderSvgFrame />
        <span aria-hidden={PortalDom.Attributes.True} className={PortalUnifiedChrome.Classes.OutlineHeaderActionIcon}>
          <img
            alt={PortalUnifiedChrome.Alt.DecorativeImage}
            className={PortalUnifiedChrome.Classes.OutlineHeaderActionIconImage}
            src={PortalAssets.HeaderLoginIcon}
          />
        </span>
        <span className={PortalUnifiedChrome.Classes.OutlineHeaderActionLabel}>
          {PortalText.Resolve(PortalTextToken.HeaderLogin)}
        </span>
      </button>
    </header>
  );
}

export function PortalUnifiedShell({ children, onAuthOpen }: PortalUnifiedShellProps): ReactElement {
  return (
    <UnifiedPageShell
      className={PortalUnifiedChrome.Classes.Shell}
      footer={<UnifiedFooter appVersion={PortalUnifiedChrome.Version.App} />}
      header={<PortalOutlineHeader onAuthOpen={onAuthOpen} />}
      viewportLocked={true}
      workClassName={PortalUnifiedChrome.Classes.ShellWork}
    >
      {children}
    </UnifiedPageShell>
  );
}
