import type { PortalBackgroundRenderConfig } from './portal-background';

export const PORTAL_BACKGROUND_SVG_WIDTH = 1920;
export const PORTAL_BACKGROUND_SVG_HEIGHT = 1080;

type PortalBackgroundLightBlob = {
  readonly color: string;
  readonly d: string;
  readonly duration: string;
  readonly opacity: number;
  readonly values: string;
};

type PortalBackgroundLightSweep = {
  readonly color: string;
  readonly d: string;
  readonly duration: string;
  readonly opacity: number;
  readonly values: string;
  readonly width: number;
};

export function portalBackgroundSvgContent(renderConfig: PortalBackgroundRenderConfig, idPrefix: string): string {
  const layoutHexRadius = renderConfig.hexRadius;
  const drawHexRadius = Math.max(2, layoutHexRadius - renderConfig.gap / Math.sqrt(3));
  const hexWidth = Math.sqrt(3) * layoutHexRadius;
  const hexRowStep = 1.5 * layoutHexRadius;
  const hexColStep = hexWidth;
  const bleedX = hexColStep * 4;
  const bleedY = hexRowStep * 6;
  const baseId = `${idPrefix}Base`;
  const vignetteId = `${idPrefix}Vignette`;
  const blurLargeId = `${idPrefix}BlurLarge`;
  const blurSmallId = `${idPrefix}BlurSmall`;
  return `
    <defs>
      <linearGradient id="${baseId}" x1="0" x2="1" y1="0" y2="1">
        <stop offset="0%" stop-color="${renderConfig.colors.bgBaseStart}" />
        <stop offset="42%" stop-color="${renderConfig.colors.bgBaseMid}" />
        <stop offset="100%" stop-color="${renderConfig.colors.bgBaseEnd}" />
      </linearGradient>
      <radialGradient cx="50%" cy="45%" id="${vignetteId}" r="75%">
        <stop offset="0%" stop-color="${renderConfig.colors.vignetteCenter}" stop-opacity="0.18" />
        <stop offset="55%" stop-color="${renderConfig.colors.vignetteMid}" stop-opacity="0.08" />
        <stop offset="100%" stop-color="${renderConfig.colors.vignetteEdge}" stop-opacity="0.34" />
      </radialGradient>
      <filter height="200%" id="${blurLargeId}" width="200%" x="-50%" y="-50%">
        <feGaussianBlur stdDeviation="${renderConfig.blobBlur}" />
      </filter>
      <filter height="200%" id="${blurSmallId}" width="200%" x="-50%" y="-50%">
        <feGaussianBlur stdDeviation="${renderConfig.beamBlur}" />
      </filter>
    </defs>
    <rect fill="url(#${baseId})" height="${PORTAL_BACKGROUND_SVG_HEIGHT}" width="${PORTAL_BACKGROUND_SVG_WIDTH}" />
    <rect fill="url(#${vignetteId})" height="${PORTAL_BACKGROUND_SVG_HEIGHT}" width="${PORTAL_BACKGROUND_SVG_WIDTH}" />
    <g filter="url(#${blurLargeId})" style="mix-blend-mode: screen">
      ${portalBackgroundLightBlobs(renderConfig)
        .map(
          (blob, index) => `
        <path d="${blob.d}" fill="${blob.color}" opacity="${blob.opacity}" data-bg-light-blob="${index}">
          <animateTransform attributeName="transform" dur="${blob.duration}" repeatCount="indefinite" type="translate" values="${blob.values}" />
        </path>`
        )
        .join('')}
    </g>
    <g filter="url(#${blurSmallId})" style="mix-blend-mode: screen">
      ${portalBackgroundLightSweeps(renderConfig)
        .map(
          (beam, index) => `
        <path d="${beam.d}" fill="none" stroke="${beam.color}" stroke-linecap="round" stroke-opacity="${beam.opacity}" stroke-width="${beam.width}" data-bg-light-sweep="${index}">
          <animateTransform attributeName="transform" dur="${beam.duration}" repeatCount="indefinite" type="translate" values="${beam.values}" />
        </path>`
        )
        .join('')}
    </g>
    <g>
      ${portalBackgroundHexagons(bleedX, bleedY, drawHexRadius, hexColStep, hexRowStep)
        .map(
          (points) => `
        <polygon fill="none" points="${points}" stroke="${renderConfig.colors.hexStroke}" stroke-opacity="${renderConfig.hexOpacity}" stroke-width="${renderConfig.hexStrokeWidth}" vector-effect="non-scaling-stroke" />`
        )
        .join('')}
    </g>`;
}

export function portalBackgroundSvgMarkup({
  ariaHidden,
  ariaLabel,
  className,
  idPrefix,
  preserveAspectRatio,
  renderConfig,
  style,
}: {
  readonly ariaHidden: boolean;
  readonly ariaLabel: string;
  readonly className?: string;
  readonly idPrefix: string;
  readonly preserveAspectRatio: string;
  readonly renderConfig: PortalBackgroundRenderConfig;
  readonly style: string;
}): string {
  const accessibility = ariaHidden ? 'aria-hidden="true"' : `aria-label="${ariaLabel}"`;
  const classAttribute = className === undefined ? '' : ` class="${className}"`;
  return `<svg ${accessibility}${classAttribute} preserveAspectRatio="${preserveAspectRatio}" style="${style}" viewBox="0 0 ${PORTAL_BACKGROUND_SVG_WIDTH} ${PORTAL_BACKGROUND_SVG_HEIGHT}" xmlns="http://www.w3.org/2000/svg">${portalBackgroundSvgContent(renderConfig, idPrefix)}</svg>`;
}

function portalBackgroundLightBlobs(renderConfig: PortalBackgroundRenderConfig): readonly PortalBackgroundLightBlob[] {
  const { blobColors, lightStrength } = renderConfig;
  return [
    {
      color: blobColors[0],
      d: 'M92 246 C120 148, 232 96, 336 102 C434 108, 552 146, 600 240 C640 318, 630 432, 560 498 C486 568, 364 594, 252 566 C156 542, 76 470, 58 388 C42 318, 56 288, 92 246 Z',
      duration: '26s',
      opacity: 0.06 * lightStrength,
      values: '0 0; 20 -10; 0 0',
    },
    {
      color: blobColors[1],
      d: 'M594 286 C666 176, 826 130, 950 156 C1064 180, 1158 274, 1178 382 C1198 492, 1130 600, 1018 650 C888 710, 720 700, 618 620 C528 548, 506 424, 594 286 Z',
      duration: '31s',
      opacity: 0.055 * lightStrength,
      values: '0 0; -26 12; 0 0',
    },
    {
      color: blobColors[2],
      d: 'M1090 196 C1194 108, 1362 96, 1492 140 C1602 178, 1688 270, 1704 374 C1720 482, 1650 584, 1532 636 C1408 690, 1248 690, 1140 632 C1028 570, 970 468, 982 364 C992 286, 1026 246, 1090 196 Z',
      duration: '28s',
      opacity: 0.05 * lightStrength,
      values: '0 0; 24 14; 0 0',
    },
    {
      color: blobColors[3],
      d: 'M142 808 C228 696, 394 654, 532 690 C650 722, 744 818, 764 930 C782 1038, 726 1142, 620 1194 C510 1248, 356 1240, 228 1182 C102 1124, 34 1016, 44 912 C50 860, 86 844, 142 808 Z',
      duration: '34s',
      opacity: 0.05 * lightStrength,
      values: '0 0; 14 -16; 0 0',
    },
    {
      color: blobColors[4],
      d: 'M902 760 C1012 662, 1186 636, 1332 678 C1458 714, 1566 816, 1592 934 C1620 1056, 1558 1178, 1432 1244 C1304 1310, 1128 1320, 986 1270 C852 1222, 764 1118, 758 1002 C754 900, 810 828, 902 760 Z',
      duration: '29s',
      opacity: 0.045 * lightStrength,
      values: '0 0; -18 18; 0 0',
    },
    {
      color: blobColors[5],
      d: 'M1396 824 C1490 736, 1636 708, 1760 732 C1876 754, 1972 830, 2010 934 C2048 1042, 2026 1160, 1948 1238 C1868 1316, 1736 1344, 1608 1320 C1472 1294, 1362 1212, 1328 1102 C1298 1004, 1318 896, 1396 824 Z',
      duration: '37s',
      opacity: 0.04 * lightStrength,
      values: '0 0; 16 -12; 0 0',
    },
  ];
}

function portalBackgroundLightSweeps(
  renderConfig: PortalBackgroundRenderConfig
): readonly PortalBackgroundLightSweep[] {
  const { beamColors, lightStrength } = renderConfig;
  return [
    {
      color: beamColors[0],
      d: 'M-120 180 C260 120, 640 125, 1060 145 C1450 165, 1740 150, 2040 90',
      duration: '38s',
      opacity: 0.024 * lightStrength,
      values: '-40 0; 35 0; -40 0',
      width: 20,
    },
    {
      color: beamColors[1],
      d: 'M-150 600 C210 565, 620 540, 1020 565 C1410 590, 1710 570, 2050 510',
      duration: '44s',
      opacity: 0.02 * lightStrength,
      values: '28 0; -24 0; 28 0',
      width: 16,
    },
    {
      color: beamColors[2],
      d: 'M-100 940 C330 915, 760 900, 1160 915 C1505 930, 1770 910, 2050 855',
      duration: '41s',
      opacity: 0.018 * lightStrength,
      values: '-22 0; 20 0; -22 0',
      width: 14,
    },
  ];
}

function portalBackgroundHexagons(
  bleedX: number,
  bleedY: number,
  drawHexRadius: number,
  hexColStep: number,
  hexRowStep: number
): readonly string[] {
  const hexagons: string[] = [];
  let row = 0;
  for (let y = -bleedY; y <= PORTAL_BACKGROUND_SVG_HEIGHT + bleedY; y += hexRowStep) {
    const offsetX = row % 2 === 0 ? hexColStep / 2 : hexColStep;
    for (let x = -bleedX + offsetX; x <= PORTAL_BACKGROUND_SVG_WIDTH + bleedX; x += hexColStep) {
      hexagons.push(hexPoints(x, y, drawHexRadius));
    }
    row += 1;
  }
  return hexagons;
}

function hexPoints(cx: number, cy: number, radius: number): string {
  const points: string[] = [];
  for (let index = 0; index < 6; index += 1) {
    const angle = ((60 * index - 30) * Math.PI) / 180;
    const x = cx + radius * Math.cos(angle);
    const y = cy + radius * Math.sin(angle);
    points.push(`${x},${y}`);
  }
  return points.join(' ');
}
