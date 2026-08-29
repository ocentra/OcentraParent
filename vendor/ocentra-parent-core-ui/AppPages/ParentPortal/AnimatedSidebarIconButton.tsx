import { useId, useMemo, useState, type CSSProperties, type ReactElement } from 'react';
import './AnimatedSidebarIconButton.css';

export const defaultSidebarIconConfig = {
  viewBox: { value: '0 0 64 64' },
  preview: {
    size: 260,
    background: '#111827',
    minHeight: '100vh',
  },
  colors: {
    frame: '#8fa1b2',
    frameHover: '#aab9c7',
    frameOutline: '#d4e0ea',
    panelTop: '#bff7ff',
    panelMid: '#2797d8',
    panelBottom: '#0a4d85',
    panelHoverTop: '#e4fdff',
    panelHoverMid: '#38c8ff',
    panelHoverBottom: '#126aa8',
    panelPressedTop: '#e9ffff',
    panelPressedMid: '#68e4ff',
    panelPressedBottom: '#1684c8',
    border: '#38dfff',
    borderHover: '#9af7ff',
    outerGlow: '#38dfff',
    outerGlowSoft: '#0ea5e9',
    panelGlow: '#67e8f9',
    darkShadow: '#020617',
    panelHighlight: '#ffffff',
  },
  button: {
    idleScale: 1,
    pressedScale: 0.985,
    disabledOpacity: 0.45,
    enabledOpacity: 1,
  },
  outerBorder: {
    x: 1.55,
    y: 1.55,
    width: 60.9,
    height: 60.9,
    radius: 1.8,
    strokeWidth: 0.6,
    opacityIdle: 0.55,
    opacityOpen: 0.82,
    opacityHover: 0.98,
    opacityPressed: 1,
  },
  doc: {
    x: 7,
    y: 7.8,
    width: 34,
    height: 48.4,
    cornerInset: 4,
    cornerRadius: 1.8,
    boxLeftOffset: 12,
    boxRightOffset: 0,
    boxTopOffset: 13,
    boxBottomOffset: 21.4,
    boxRadius: 1,
    lineLeftOffset: 14,
    lineTopOffset: 38,
    strokeIdle: 2.35,
    strokeHover: 2.8,
    outlineExtra: 0.7,
    outlineOpacityIdle: 0.36,
    outlineOpacityHover: 0.68,
  },
  panel: {
    right: 58.6,
    openWidth: 12,
    closedWidth: 32,
    y: 3.6,
    height: 56.8,
    radius: 3.2,
    outlinePad: 0.65,
    outlineRadius: 3.8,
    outlineStrokeWidth: 1.05,
    outlineOpacityIdle: 0.46,
    outlineOpacityOpen: 0.46,
    outlineOpacityHover: 0.82,
    outlineOpacityPressed: 0.92,
    topShineInsetX: 4,
    topShineInsetY: 5.2,
    topShineRightInset: 5,
    topShineStrokeWidth: 1.35,
    topShineOpacityIdle: 0.52,
    topShineOpacityOpen: 0.52,
    topShineOpacityHover: 0.94,
    topShineOpacityPressed: 1,
    shineLeftInset: 2.4,
    shineTopInset: 5,
    shineTopCurveA: 0.34,
    shineTopCurveB: 0.72,
    shineTopY1: 2.8,
    shineTopY2: 3,
    shineRightInset: 2.6,
    shineRightY: 5.1,
    shineHeight: 23,
    shineBottomCurveA: 0.66,
    shineBottomCurveB: 0.3,
    shineBottomY1: 20.7,
    shineBottomY2: 21.2,
    shineBottomY3: 24.2,
    shineOpacityIdle: 0.42,
    shineOpacityOpen: 0.42,
    shineOpacityHover: 0.78,
    shineOpacityPressed: 0.9,
    edgeInsetX: 2.4,
    edgeInsetY: 5.8,
    edgeStrokeWidth: 1.1,
    edgeOpacityIdle: 0.34,
    edgeOpacityOpen: 0.34,
    edgeOpacityHover: 0.72,
    edgeOpacityPressed: 0.82,
  },
  filters: {
    outerGlow: {
      blurA: 1.1,
      blurB: 3,
      opacityAIdle: 0.42,
      opacityAOpen: 0.72,
      opacityAHover: 0.95,
      opacityAPressed: 1,
      opacityBIdle: 0.12,
      opacityBOpen: 0.26,
      opacityBHover: 0.46,
      opacityBPressed: 0.6,
    },
    docShadow: {
      dx: 0,
      dy: 2,
      blur: 1.2,
      opacity: 1,
    },
    panelGlow: {
      glowDx: 0,
      glowDy: 0,
      glowBlur: 2.3,
      glowOpacityIdle: 0.24,
      glowOpacityOpen: 0.42,
      glowOpacityHover: 0.66,
      glowOpacityPressed: 0.78,
      shadowDx: 0,
      shadowDy: 3,
      shadowBlur: 1.6,
      shadowOpacity: 0.42,
    },
  },
} as const;

type SidebarIconConfig = typeof defaultSidebarIconConfig;
type ConfigLeaf<T> = T extends string ? string : T extends number ? number : T extends boolean ? boolean : T;
type DeepPartial<T> = {
  [K in keyof T]?: T[K] extends object ? DeepPartial<T[K]> : ConfigLeaf<T[K]>;
};

type AnimatedSidebarIconButtonProps = {
  readonly isOpen?: boolean;
  readonly size?: number;
  readonly centerPreview?: boolean;
  readonly title?: string;
  readonly disabled?: boolean;
  readonly onClick?: () => void;
  readonly className?: string;
  readonly config?: DeepPartial<SidebarIconConfig>;
};

function mergeConfig(base: SidebarIconConfig, override?: DeepPartial<SidebarIconConfig>): SidebarIconConfig {
  if (!override) return base;
  return {
    ...base,
    ...override,
    viewBox: { ...base.viewBox, ...override.viewBox },
    preview: { ...base.preview, ...override.preview },
    colors: { ...base.colors, ...override.colors },
    button: { ...base.button, ...override.button },
    outerBorder: { ...base.outerBorder, ...override.outerBorder },
    doc: { ...base.doc, ...override.doc },
    panel: { ...base.panel, ...override.panel },
    filters: {
      ...base.filters,
      ...override.filters,
      outerGlow: { ...base.filters.outerGlow, ...override.filters?.outerGlow },
      docShadow: { ...base.filters.docShadow, ...override.filters?.docShadow },
      panelGlow: { ...base.filters.panelGlow, ...override.filters?.panelGlow },
    },
  } as SidebarIconConfig;
}

function pathOuterDoc(doc: SidebarIconConfig['doc']): string {
  const right = doc.x + doc.width;
  const bottom = doc.y + doc.height;
  const leftCorner = doc.x + doc.cornerInset;
  const topCorner = doc.y + doc.cornerInset;
  const bottomCorner = bottom - doc.cornerInset;
  return `M${right} ${doc.y}H${leftCorner}C${doc.x + doc.cornerRadius} ${doc.y} ${doc.x} ${doc.y + doc.cornerRadius} ${doc.x} ${topCorner}V${bottomCorner}C${doc.x} ${bottom - doc.cornerRadius} ${doc.x + doc.cornerRadius} ${bottom} ${leftCorner} ${bottom}H${right}`;
}

function pathInnerDocBox(doc: SidebarIconConfig['doc']): string {
  const right = doc.x + doc.width - doc.boxRightOffset;
  const left = doc.x + doc.boxLeftOffset;
  const top = doc.y + doc.boxTopOffset;
  const bottom = doc.y + doc.height - doc.boxBottomOffset;
  const innerLeft = left + 2;
  return `M${right} ${top}H${innerLeft}C${left + doc.boxRadius} ${top} ${left} ${top + doc.boxRadius} ${left} ${top + 2}V${bottom - 2}C${left} ${bottom - doc.boxRadius} ${left + doc.boxRadius} ${bottom} ${innerLeft} ${bottom}H${right}`;
}

function pathDocLine(doc: SidebarIconConfig['doc']): string {
  const right = doc.x + doc.width;
  const left = doc.x + doc.lineLeftOffset;
  const y = doc.y + doc.lineTopOffset;
  return `M${left} ${y}H${right}`;
}

function stateValue(
  pressedState: boolean,
  hoverState: boolean,
  openState: boolean,
  pressed: number,
  hover: number,
  opened: number,
  idle: number
): number {
  if (pressedState) return pressed;
  if (hoverState) return hover;
  if (openState) return opened;
  return idle;
}

function classNames(...values: readonly (string | undefined)[]): string {
  return values.filter(Boolean).join(' ');
}

export function AnimatedSidebarIconButton({
  isOpen,
  size,
  centerPreview = true,
  title = 'Toggle sidebar',
  disabled = false,
  onClick,
  className,
  config: configOverride,
}: AnimatedSidebarIconButtonProps): ReactElement {
  const [isHovering, setIsHovering] = useState(false);
  const [isPressed, setIsPressed] = useState(false);
  const [previewOpen, setPreviewOpen] = useState(true);
  const uid = useId().replace(/[^a-zA-Z0-9_-]/g, '');
  const config = useMemo(() => mergeConfig(defaultSidebarIconConfig, configOverride), [configOverride]);
  const open = isOpen ?? previewOpen;
  const resolvedSize = size ?? config.preview.size;
  const buttonScale = disabled
    ? config.button.idleScale
    : isPressed
      ? config.button.pressedScale
      : config.button.idleScale;
  const buttonOpacity = disabled ? config.button.disabledOpacity : config.button.enabledOpacity;
  const borderColor = isHovering || open ? config.colors.borderHover : config.colors.border;
  const frameColor = isHovering ? config.colors.frameHover : config.colors.frame;
  const panelGradientId = isPressed ? `${uid}-panelPressed` : isHovering ? `${uid}-panelHover` : `${uid}-panel`;
  const strokeWidth = isHovering ? config.doc.strokeHover : config.doc.strokeIdle;
  const outlineStrokeWidth = strokeWidth + config.doc.outlineExtra;
  const docOuterPath = pathOuterDoc(config.doc);
  const docBoxPath = pathInnerDocBox(config.doc);
  const docLinePath = pathDocLine(config.doc);
  const panelW = open ? config.panel.openWidth : config.panel.closedWidth;
  const panelX = config.panel.right - panelW;
  const panelY = config.panel.y;
  const panelH = config.panel.height;
  const panelOutlineX = panelX - config.panel.outlinePad;
  const panelOutlineY = panelY - config.panel.outlinePad;
  const panelOutlineW = panelW + config.panel.outlinePad * 2;
  const panelOutlineH = panelH + config.panel.outlinePad * 2;
  const glowA = stateValue(
    isPressed,
    isHovering,
    open,
    config.filters.outerGlow.opacityAPressed,
    config.filters.outerGlow.opacityAHover,
    config.filters.outerGlow.opacityAOpen,
    config.filters.outerGlow.opacityAIdle
  );
  const glowB = stateValue(
    isPressed,
    isHovering,
    open,
    config.filters.outerGlow.opacityBPressed,
    config.filters.outerGlow.opacityBHover,
    config.filters.outerGlow.opacityBOpen,
    config.filters.outerGlow.opacityBIdle
  );
  const borderOpacity = stateValue(
    isPressed,
    isHovering,
    open,
    config.outerBorder.opacityPressed,
    config.outerBorder.opacityHover,
    config.outerBorder.opacityOpen,
    config.outerBorder.opacityIdle
  );
  const panelOutlineOpacity = stateValue(
    isPressed,
    isHovering,
    open,
    config.panel.outlineOpacityPressed,
    config.panel.outlineOpacityHover,
    config.panel.outlineOpacityOpen,
    config.panel.outlineOpacityIdle
  );
  const panelGlowOpacity = stateValue(
    isPressed,
    isHovering,
    open,
    config.filters.panelGlow.glowOpacityPressed,
    config.filters.panelGlow.glowOpacityHover,
    config.filters.panelGlow.glowOpacityOpen,
    config.filters.panelGlow.glowOpacityIdle
  );
  const panelTopLineOpacity = stateValue(
    isPressed,
    isHovering,
    open,
    config.panel.topShineOpacityPressed,
    config.panel.topShineOpacityHover,
    config.panel.topShineOpacityOpen,
    config.panel.topShineOpacityIdle
  );
  const panelShineOpacity = stateValue(
    isPressed,
    isHovering,
    open,
    config.panel.shineOpacityPressed,
    config.panel.shineOpacityHover,
    config.panel.shineOpacityOpen,
    config.panel.shineOpacityIdle
  );
  const panelEdgeOpacity = stateValue(
    isPressed,
    isHovering,
    open,
    config.panel.edgeOpacityPressed,
    config.panel.edgeOpacityHover,
    config.panel.edgeOpacityOpen,
    config.panel.edgeOpacityIdle
  );
  const panelShinePath = `M${panelX + config.panel.shineLeftInset} ${panelY + config.panel.shineTopInset}C${panelX + panelW * config.panel.shineTopCurveA} ${panelY + config.panel.shineTopY1} ${panelX + panelW * config.panel.shineTopCurveB} ${panelY + config.panel.shineTopY2} ${panelX + panelW - config.panel.shineRightInset} ${panelY + config.panel.shineRightY}V${panelY + config.panel.shineHeight}C${panelX + panelW * config.panel.shineBottomCurveA} ${panelY + config.panel.shineBottomY1} ${panelX + panelW * config.panel.shineBottomCurveB} ${panelY + config.panel.shineBottomY2} ${panelX + config.panel.shineLeftInset} ${panelY + config.panel.shineBottomY3}Z`;
  const button = (
    <button
      type="button"
      aria-label={title}
      aria-pressed={open}
      disabled={disabled}
      title={title}
      onClick={
        disabled
          ? undefined
          : () => {
              if (isOpen === undefined) {
                setPreviewOpen((value) => !value);
              }
              onClick?.();
            }
      }
      onPointerEnter={() => setIsHovering(true)}
      onPointerLeave={() => {
        setIsHovering(false);
        setIsPressed(false);
      }}
      onPointerDown={() => setIsPressed(true)}
      onPointerUp={() => setIsPressed(false)}
      onPointerCancel={() => setIsPressed(false)}
      className={classNames('animated-sidebar-icon-button', className)}
      style={
        {
          '--animated-sidebar-icon-size': `${resolvedSize}px`,
          '--animated-sidebar-icon-opacity': buttonOpacity,
          '--animated-sidebar-icon-scale': buttonScale,
        } as CSSProperties
      }
    >
      <svg className="animated-sidebar-icon-button__svg" viewBox={config.viewBox.value} role="img" aria-hidden="true">
        <defs>
          <linearGradient id={`${uid}-panel`} x1="34" y1="3" x2="58" y2="61" gradientUnits="userSpaceOnUse">
            <stop offset="0" stopColor={config.colors.panelTop} />
            <stop offset="0.18" stopColor={config.colors.panelMid} />
            <stop offset="1" stopColor={config.colors.panelBottom} />
          </linearGradient>
          <linearGradient id={`${uid}-panelHover`} x1="28" y1="3" x2="60" y2="61" gradientUnits="userSpaceOnUse">
            <stop offset="0" stopColor={config.colors.panelHoverTop} />
            <stop offset="0.2" stopColor={config.colors.panelHoverMid} />
            <stop offset="1" stopColor={config.colors.panelHoverBottom} />
          </linearGradient>
          <linearGradient id={`${uid}-panelPressed`} x1="28" y1="3" x2="60" y2="61" gradientUnits="userSpaceOnUse">
            <stop offset="0" stopColor={config.colors.panelPressedTop} />
            <stop offset="0.24" stopColor={config.colors.panelPressedMid} />
            <stop offset="1" stopColor={config.colors.panelPressedBottom} />
          </linearGradient>
          <linearGradient id={`${uid}-panelShine`} x1="34" y1="4" x2="59" y2="60" gradientUnits="userSpaceOnUse">
            <stop offset="0" stopColor={config.colors.panelHighlight} stopOpacity="0.92" />
            <stop offset="0.22" stopColor={config.colors.panelHighlight} stopOpacity="0.3" />
            <stop offset="0.5" stopColor={config.colors.panelHighlight} stopOpacity="0.06" />
            <stop offset="1" stopColor={config.colors.panelHighlight} stopOpacity="0" />
          </linearGradient>
          <filter id={`${uid}-outerGlow`} x="-30%" y="-30%" width="160%" height="160%">
            <feDropShadow
              dx="0"
              dy="0"
              stdDeviation={config.filters.outerGlow.blurA}
              floodColor={config.colors.outerGlow}
              floodOpacity={glowA}
            />
            <feDropShadow
              dx="0"
              dy="0"
              stdDeviation={config.filters.outerGlow.blurB}
              floodColor={config.colors.outerGlowSoft}
              floodOpacity={glowB}
            />
          </filter>
          <filter id={`${uid}-docShadow`} x="-25%" y="-25%" width="150%" height="150%">
            <feDropShadow
              dx={config.filters.docShadow.dx}
              dy={config.filters.docShadow.dy}
              stdDeviation={config.filters.docShadow.blur}
              floodColor={config.colors.darkShadow}
              floodOpacity={config.filters.docShadow.opacity}
            />
          </filter>
          <filter id={`${uid}-panelGlow`} x="-45%" y="-45%" width="190%" height="190%">
            <feDropShadow
              dx={config.filters.panelGlow.glowDx}
              dy={config.filters.panelGlow.glowDy}
              stdDeviation={config.filters.panelGlow.glowBlur}
              floodColor={config.colors.panelGlow}
              floodOpacity={panelGlowOpacity}
            />
            <feDropShadow
              dx={config.filters.panelGlow.shadowDx}
              dy={config.filters.panelGlow.shadowDy}
              stdDeviation={config.filters.panelGlow.shadowBlur}
              floodColor={config.colors.darkShadow}
              floodOpacity={config.filters.panelGlow.shadowOpacity}
            />
          </filter>
        </defs>
        <rect
          x={config.outerBorder.x}
          y={config.outerBorder.y}
          width={config.outerBorder.width}
          height={config.outerBorder.height}
          rx={config.outerBorder.radius}
          fill="none"
          stroke={borderColor}
          strokeWidth={config.outerBorder.strokeWidth}
          opacity={borderOpacity}
          filter={`url(#${uid}-outerGlow)`}
        />
        <g filter={`url(#${uid}-docShadow)`}>
          <path
            d={docOuterPath}
            fill="none"
            stroke={config.colors.frameOutline}
            strokeWidth={outlineStrokeWidth}
            strokeLinecap="butt"
            strokeLinejoin="round"
            opacity={isHovering ? config.doc.outlineOpacityHover : config.doc.outlineOpacityIdle}
          />
          <path
            d={docBoxPath}
            fill="none"
            stroke={config.colors.frameOutline}
            strokeWidth={outlineStrokeWidth}
            strokeLinecap="butt"
            strokeLinejoin="round"
            opacity={isHovering ? config.doc.outlineOpacityHover : config.doc.outlineOpacityIdle}
          />
          <path
            d={docLinePath}
            fill="none"
            stroke={config.colors.frameOutline}
            strokeWidth={outlineStrokeWidth}
            strokeLinecap="round"
            opacity={isHovering ? config.doc.outlineOpacityHover : config.doc.outlineOpacityIdle}
          />
          <path
            d={docOuterPath}
            fill="none"
            stroke={frameColor}
            strokeWidth={strokeWidth}
            strokeLinecap="butt"
            strokeLinejoin="round"
          />
          <path
            d={docBoxPath}
            fill="none"
            stroke={frameColor}
            strokeWidth={strokeWidth}
            strokeLinecap="butt"
            strokeLinejoin="round"
          />
          <path d={docLinePath} fill="none" stroke={frameColor} strokeWidth={strokeWidth} strokeLinecap="round" />
        </g>
        <rect
          x={panelOutlineX}
          y={panelOutlineY}
          width={panelOutlineW}
          height={panelOutlineH}
          rx={config.panel.outlineRadius}
          fill="none"
          stroke={borderColor}
          strokeWidth={config.panel.outlineStrokeWidth}
          opacity={panelOutlineOpacity}
        />
        <rect
          x={panelX}
          y={panelY}
          width={panelW}
          height={panelH}
          rx={config.panel.radius}
          fill={`url(#${panelGradientId})`}
          filter={`url(#${uid}-panelGlow)`}
        />
        <path
          d={`M${panelX + config.panel.topShineInsetX} ${panelY + config.panel.topShineInsetY}H${panelX + panelW - config.panel.topShineRightInset}`}
          stroke={config.colors.panelHighlight}
          strokeWidth={config.panel.topShineStrokeWidth}
          strokeLinecap="round"
          opacity={panelTopLineOpacity}
        />
        <path d={panelShinePath} fill={`url(#${uid}-panelShine)`} opacity={panelShineOpacity} />
        <path
          d={`M${panelX + panelW - config.panel.edgeInsetX} ${panelY + config.panel.edgeInsetY}V${panelY + panelH - config.panel.edgeInsetY}`}
          stroke={config.colors.panelHighlight}
          strokeWidth={config.panel.edgeStrokeWidth}
          strokeLinecap="round"
          opacity={panelEdgeOpacity}
        />
      </svg>
    </button>
  );
  if (!centerPreview) return button;
  return (
    <div
      className="animated-sidebar-icon-preview"
      style={
        {
          '--animated-sidebar-icon-preview-min-height': config.preview.minHeight,
          background: config.preview.background,
        } as CSSProperties
      }
    >
      {button}
    </div>
  );
}
