import type { ReactElement } from 'react';
import { PortalAuthChrome } from '@ocentra-parent/portal-domain/contracts';

export function PortalAuthFrame(): ReactElement {
  return (
    <svg
      aria-hidden={true}
      className={PortalAuthChrome.Classes.FrameSvg}
      preserveAspectRatio={PortalAuthChrome.Svg.PreserveAspectRatio}
      viewBox={PortalAuthChrome.Svg.ViewBox}
    >
      <path className={PortalAuthChrome.Classes.FrameOuter} d={PortalAuthChrome.Svg.OuterPath} />
      <path className={PortalAuthChrome.Classes.FrameInset} d={PortalAuthChrome.Svg.InsetPath} />
      <path className={PortalAuthChrome.Classes.FrameRail} d={PortalAuthChrome.Svg.LeftRailPath} />
      <path className={PortalAuthChrome.Classes.FrameRail} d={PortalAuthChrome.Svg.RightRailPath} />
      <circle
        className={PortalAuthChrome.Classes.FrameCorner}
        cx={PortalAuthChrome.Svg.CornerLeft}
        cy={PortalAuthChrome.Svg.CornerTop}
        r={PortalAuthChrome.Svg.CornerRadius}
      />
      <circle
        className={PortalAuthChrome.Classes.FrameCorner}
        cx={PortalAuthChrome.Svg.CornerRight}
        cy={PortalAuthChrome.Svg.CornerTop}
        r={PortalAuthChrome.Svg.CornerRadius}
      />
      <circle
        className={PortalAuthChrome.Classes.FrameCorner}
        cx={PortalAuthChrome.Svg.CornerLeft}
        cy={PortalAuthChrome.Svg.CornerBottom}
        r={PortalAuthChrome.Svg.CornerRadius}
      />
      <circle
        className={PortalAuthChrome.Classes.FrameCorner}
        cx={PortalAuthChrome.Svg.CornerRight}
        cy={PortalAuthChrome.Svg.CornerBottom}
        r={PortalAuthChrome.Svg.CornerRadius}
      />
    </svg>
  );
}
