import { useMemo, type CSSProperties, type ReactElement } from 'react';
import type { PortalBackgroundRenderConfig } from './portal-background-config';
import {
  PORTAL_BACKGROUND_SVG_HEIGHT,
  PORTAL_BACKGROUND_SVG_WIDTH,
  portalBackgroundSvgContent,
} from './portal-background-svg-markup';

export type PortalBackgroundSvgProps = PortalBackgroundRenderConfig & {
  readonly ariaHidden?: boolean;
  readonly ariaLabel?: string;
  readonly className?: string;
  readonly preserveAspectRatio?: string;
  readonly style?: CSSProperties;
};

export function PortalBackgroundSvg({
  ariaHidden = false,
  ariaLabel = 'Portal background',
  beamBlur,
  beamColors,
  blobBlur,
  blobColors,
  className,
  colors,
  gap,
  hexOpacity,
  hexRadius,
  hexStrokeWidth,
  lightStrength,
  preserveAspectRatio = 'xMidYMid meet',
  style,
}: PortalBackgroundSvgProps): ReactElement {
  const accessibilityProps = ariaHidden ? { 'aria-hidden': true } : { 'aria-label': ariaLabel };
  const svgContent = useMemo(
    () =>
      portalBackgroundSvgContent(
        {
          beamBlur,
          beamColors,
          blobBlur,
          blobColors,
          colors,
          gap,
          hexOpacity,
          hexRadius,
          hexStrokeWidth,
          lightStrength,
        },
        'portalBackgroundReact'
      ),
    [beamBlur, beamColors, blobBlur, blobColors, colors, gap, hexOpacity, hexRadius, hexStrokeWidth, lightStrength]
  );

  return (
    <svg
      {...accessibilityProps}
      className={className}
      dangerouslySetInnerHTML={{ __html: svgContent }}
      preserveAspectRatio={preserveAspectRatio}
      style={{ display: 'block', height: 'auto', pointerEvents: 'none', width: '100%', ...style }}
      viewBox={`0 0 ${PORTAL_BACKGROUND_SVG_WIDTH} ${PORTAL_BACKGROUND_SVG_HEIGHT}`}
      xmlns="http://www.w3.org/2000/svg"
    />
  );
}
