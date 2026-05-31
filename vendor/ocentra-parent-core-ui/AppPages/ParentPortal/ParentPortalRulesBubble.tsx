import {
  useEffect,
  useId,
  useMemo,
  useRef,
  useState,
  type CSSProperties,
  type ReactElement,
  type ReactNode,
} from 'react';

// Forked from ParentPortalChatBubble so Browser Rules can evolve without changing AI chat bubbles.
const defaultChatBubbleText = '';

export const defaultChatBubbleConfig = {
  header: {
    width: 640,
    height: 34,
    radius: 7,
    strokeWidth: 1.15,
    glowStrokeWidth: 2.05,
    innerStrokeWidth: 0.65,
    shineHeightRatio: 0.88,
    buttonShineOpacityIdle: 0.26,
    buttonShineOpacityHover: 0.42,
    bottomShadeOpacity: 0.18,
  },
  body: {
    clampOnLeft: true,
    clampWidth: 32,
    clampGap: 0,
    clampStrokeWidth: 1.15,
    clampGlowStrokeWidth: 2.2,
    clampGlowSpreadX: 4,
    clampGlowSpreadY: 2,
    clampGlowBlurA: 2.8,
    clampGlowBlurB: 7.5,
    clampGlowOpacityIdle: 0.42,
    clampGlowOpacityHover: 0.72,
    minHeight: 52,
    maxHeight: 1024,
    collapsedHeight: 4,
    paddingTop: 11,
    paddingBottom: 13,
    radius: 6,
    topRadius: 0,
    strokeWidth: 1.1,
    glowStrokeWidth: 1.85,
    innerStrokeWidth: 0.55,
    textInsetX: 10,
    viewportInset: 8,
    contentFadeDelayMs: 90,
  },
  controls: {
    buttonSize: 22,
    buttonInsetX: 7,
    buttonAfterClampGap: 5,
    buttonInsetY: 6,
    buttonRadius: 4,
    buttonStrokeWidth: 1.15,
    chevronSize: 11,
    chevronStrokeWidth: 2,
    copyPadding: 1.5,
    copyIconViewBoxX: 220,
    copyIconViewBoxY: 150,
    copyIconViewBoxW: 600,
    copyIconViewBoxH: 720,
  },
  text: {
    fontSize: 12,
    lineHeight: 1.48,
  },
  colors: {
    aiHeaderTop: '#1d5a36',
    aiHeaderBottom: '#0a2917',
    aiBodyTop: 'rgba(4, 28, 48, 0.75)',
    aiBodyBottom: 'rgba(2, 13, 28, 0.75)',
    aiStroke: '#79f7ff',
    aiStrokeHover: '#ffffff',
    aiGlow: '#baffcf',
    aiEdgeGlow: '#52eaff',
    aiClampTop: '#dcff7d',
    aiClampBottom: '#48d86b',
    aiClampEdge: '#f8ffe8',
    aiClampGlow: '#caff4d',
    userHeaderTop: '#15507e',
    userHeaderBottom: '#08243f',
    userBodyTop: '#e5f6ff',
    userBodyBottom: '#acdfff',
    userStroke: '#e6f7ff',
    userStrokeHover: '#ffffff',
    userGlow: '#bdefff',
    userEdgeGlow: '#ffffff',
    userClampTop: '#b9f6ff',
    userClampBottom: '#1fb8ff',
    userClampEdge: '#f1fdff',
    userClampGlow: '#4deaff',
    text: '#0b1f2f',
    codeText: '#eaf7ff',
    codeBackground: 'rgba(3, 18, 34, 0.55)',
    chevron: '#e7f3ff',
    collapseTriangleFill: '#03111f',
    collapseTriangleFillHover: '#06283d',
    collapseTriangleStroke: '#92fbff',
    collapseTriangleGlow: '#27e6ff',
    infoFill: 'rgba(13, 57, 70, 0.72)',
    infoFillHover: 'rgba(37, 214, 255, 0.24)',
    infoStroke: '#8ff7ff',
    infoText: '#dffcff',
    infoGlow: '#31e5ff',
    controlFill: 'rgba(255,255,255,0.035)',
    controlFillHover: 'rgba(255,255,255,0.11)',
    controlStroke: '#7f8fa8',
    controlStrokeHover: '#d7e8ff',
    copyPaper: '#e1f0ff',
    copyMain: '#6d9ee8',
    copyDark: '#446eb1',
    shine: '#ffffff',
    shadow: '#020617',
    scrollbarThumb: 'rgba(216,232,245,0.32)',
    scrollbarTrack: 'rgba(2,6,23,0.18)',
  },
  opacity: {
    disabled: 0.45,
    strokeIdle: 0.78,
    strokeHover: 0.95,
    innerStroke: 0.26,
    shineIdle: 0.22,
    shineHover: 0.38,
    glowIdle: 0.22,
    glowHover: 0.56,
    shadow: 0.32,
    controlIdle: 0.82,
    controlHover: 1,
    copyIdle: 0.88,
    copyHover: 1,
    bodyIdle: 1,
    bodyCollapsed: 0,
    clampIdle: 0.92,
    clampHover: 1,
  },
  hover: {
    liftY: -1.5,
    pressY: 1,
    scaleIdle: 1,
    scaleHover: 1,
    scalePressed: 0.998,
  },
  filters: {
    glow: {
      x: '-14%',
      y: '-45%',
      width: '128%',
      height: '190%',
      blurA: 2.6,
      blurB: 7,
    },
    shadow: {
      x: '-8%',
      y: '-20%',
      width: '116%',
      height: '142%',
      dx: 0,
      dy: 3,
      blur: 3.5,
    },
  },
  typography: {
    fontFamily: 'Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, Segoe UI, sans-serif',
    codeFontFamily: 'ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, Liberation Mono, monospace',
    fontWeight: 500,
    letterSpacing: 0,
  },
  transition: {
    root: 'transform 160ms ease, opacity 160ms ease',
    svg: 'opacity 180ms ease, stroke 180ms ease, fill 180ms ease, filter 180ms ease, transform 180ms ease, height 260ms ease, y 260ms ease',
    body: 'opacity 150ms ease, transform 220ms ease',
  },
};

export type ChatBubbleConfig = typeof defaultChatBubbleConfig;
type BubbleVariant = 'incoming' | 'outgoing';
type DeepPartial<T> = {
  [K in keyof T]?: T[K] extends object ? DeepPartial<T[K]> : T[K];
};

type ChatBubbleSvgProps = {
  variant?: BubbleVariant;
  width?: number;
  body?: ReactNode;
  children?: ReactNode;
  text?: string;
  collapsed?: boolean;
  showCopy?: boolean;
  disabled?: boolean;
  className?: string;
  copyLabel?: string;
  collapseLabel?: string;
  expandLabel?: string;
  messageLabel?: string;
  headerLabel?: string;
  showInfo?: boolean;
  infoLabel?: string;
  onClick?: () => void;
  onCopyClick?: () => void;
  onInfoClick?: () => void;
  onCollapsedChange?: (collapsed: boolean) => void;
  config?: DeepPartial<ChatBubbleConfig>;
};

export type RulesBubbleSvgFrameSlot = {
  bodyX: number;
  bodyY: number;
  bodyW: number;
  bodyH: number;
  bodyContentX: number;
  bodyContentY: number;
  bodyContentW: number;
  bodyContentH: number;
};

type RulesBubbleSvgFrameProps = {
  x: number;
  y: number;
  width: number;
  bodyHeight: number;
  variant?: BubbleVariant;
  collapsed?: boolean;
  visualCollapsed?: boolean;
  headerLabel?: string;
  showInfo?: boolean;
  infoLabel?: string;
  disabled?: boolean;
  collapseDisabled?: boolean;
  collapseLabel?: string;
  expandLabel?: string;
  onInfoClick?: () => void;
  onCollapsedChange?: (collapsed: boolean) => void;
  config?: DeepPartial<ChatBubbleConfig>;
  children?: (slot: RulesBubbleSvgFrameSlot) => ReactNode;
};

type ChatBubbleGeometry = {
  resolvedWidth: number;
  headerH: number;
  headerClampX: number;
  headerClampY: number;
  headerClampW: number;
  headerClampH: number;
  bodyX: number;
  bodyY: number;
  bodyW: number;
  bodyH: number;
  bodyContentW: number;
  bodyContentH: number;
  totalHeight: number;
  viewBoxX: number;
  viewBoxY: number;
  viewBoxW: number;
  viewBoxH: number;
  shouldScroll: boolean;
};

function mergeConfig(base: ChatBubbleConfig, override?: DeepPartial<ChatBubbleConfig>): ChatBubbleConfig {
  if (!override) return base;

  return {
    ...base,
    ...override,
    header: { ...base.header, ...override.header },
    body: { ...base.body, ...override.body },
    controls: { ...base.controls, ...override.controls },
    text: { ...base.text, ...override.text },
    colors: { ...base.colors, ...override.colors },
    opacity: { ...base.opacity, ...override.opacity },
    hover: { ...base.hover, ...override.hover },
    filters: {
      ...base.filters,
      ...override.filters,
      glow: { ...base.filters.glow, ...override.filters?.glow },
      shadow: { ...base.filters.shadow, ...override.filters?.shadow },
    },
    typography: { ...base.typography, ...override.typography },
    transition: { ...base.transition, ...override.transition },
  } as ChatBubbleConfig;
}

function splitBubbleLines(text: string) {
  if (!text) return [];
  return text.split(/\r?\n/);
}

function estimateTextHeight(config: ChatBubbleConfig, text: string) {
  const lineCount = Math.max(1, splitBubbleLines(text).length);
  return lineCount * config.text.fontSize * config.text.lineHeight;
}

function estimateWrappedTextHeight(config: ChatBubbleConfig, text: string, width: number) {
  const bodyContentW = Math.max(1, width - config.body.clampWidth - config.body.textInsetX * 2);
  const averageCharW = config.text.fontSize * 0.54;
  const charsPerLine = Math.max(12, Math.floor(bodyContentW / averageCharW));
  const visualLineCount = splitBubbleLines(text || ' ').reduce((count, line) => {
    return count + Math.max(1, Math.ceil(line.length / charsPerLine));
  }, 0);
  return visualLineCount * config.text.fontSize * config.text.lineHeight;
}

function getStateValue(isPressed: boolean, isHovering: boolean, pressed: number, hover: number, idle: number) {
  if (isPressed) return pressed;
  if (isHovering) return hover;
  return idle;
}

function roundedRectPathByCorner(
  x: number,
  y: number,
  width: number,
  height: number,
  topLeftRadius: number,
  topRightRadius: number,
  bottomRightRadius: number,
  bottomLeftRadius: number
) {
  const safeWidth = Math.max(0, width);
  const safeHeight = Math.max(0, height);
  const maxRadius = Math.min(safeWidth * 0.5, safeHeight * 0.5);
  const tl = Math.min(topLeftRadius, maxRadius);
  const tr = Math.min(topRightRadius, maxRadius);
  const br = Math.min(bottomRightRadius, maxRadius);
  const bl = Math.min(bottomLeftRadius, maxRadius);
  const right = x + safeWidth;
  const bottom = y + safeHeight;

  return `M${x + tl} ${y}H${right - tr}C${right - tr * 0.45} ${y} ${right} ${y + tr * 0.45} ${right} ${y + tr}V${bottom - br}C${right} ${bottom - br * 0.45} ${right - br * 0.45} ${bottom} ${right - br} ${bottom}H${x + bl}C${x + bl * 0.45} ${bottom} ${x} ${bottom - bl * 0.45} ${x} ${bottom - bl}V${y + tl}C${x} ${y + tl * 0.45} ${x + tl * 0.45} ${y} ${x + tl} ${y}Z`;
}

function calculateGeometry(
  config: ChatBubbleConfig,
  width: number | undefined,
  measuredContentHeight: number,
  isCollapsed: boolean
): ChatBubbleGeometry {
  const resolvedWidth = width ?? config.header.width;
  const headerH = config.header.height;
  const bodyY = headerH;
  const headerClampW = config.body.clampWidth;
  const headerClampH = headerH;
  const headerClampY = 0;
  const headerClampX = config.body.clampOnLeft ? 0 : resolvedWidth - headerClampW;
  const bodyX = config.body.clampOnLeft ? headerClampW + config.body.clampGap : 0;
  const bodyW = config.body.clampOnLeft ? resolvedWidth - bodyX : resolvedWidth - headerClampW - config.body.clampGap;
  const bodyContentW = Math.max(0, bodyW - config.body.textInsetX * 2);
  const desiredBodyContentH = measuredContentHeight + config.body.paddingTop + config.body.paddingBottom;
  const expandedBodyH = Math.max(config.body.minHeight, Math.min(config.body.maxHeight, desiredBodyContentH));
  const bodyH = isCollapsed ? config.body.collapsedHeight : expandedBodyH;
  const bodyContentH = Math.max(0, bodyH - config.body.paddingTop - config.body.paddingBottom);
  const totalHeight = bodyY + bodyH;
  const viewBoxX = -config.body.viewportInset;
  const viewBoxY = -config.body.viewportInset;
  const viewBoxW = resolvedWidth + config.body.viewportInset * 2;
  const viewBoxH = totalHeight + config.body.viewportInset * 2;
  const shouldScroll = desiredBodyContentH > config.body.maxHeight;

  return {
    resolvedWidth,
    headerH,
    headerClampX,
    headerClampY,
    headerClampW,
    headerClampH,
    bodyX,
    bodyY,
    bodyW,
    bodyH,
    bodyContentW,
    bodyContentH,
    totalHeight,
    viewBoxX,
    viewBoxY,
    viewBoxW,
    viewBoxH,
    shouldScroll,
  };
}

export function estimateChatBubbleHeight({
  width,
  text,
  collapsed = false,
  config: configOverride,
}: {
  width?: number;
  text?: string;
  collapsed?: boolean;
  config?: DeepPartial<ChatBubbleConfig>;
}) {
  const config = mergeConfig(defaultChatBubbleConfig, configOverride);
  return calculateGeometry(
    config,
    width,
    estimateWrappedTextHeight(config, text ?? defaultChatBubbleText, width ?? config.header.width),
    collapsed
  ).totalHeight;
}

export function RulesBubbleSvgFrame({
  x,
  y,
  width,
  bodyHeight,
  variant = 'incoming',
  collapsed,
  visualCollapsed,
  headerLabel,
  showInfo = false,
  infoLabel = 'More information',
  disabled = false,
  collapseDisabled = false,
  collapseLabel = 'Collapse',
  expandLabel = 'Expand',
  onInfoClick,
  onCollapsedChange,
  config: configOverride,
  children,
}: RulesBubbleSvgFrameProps): ReactElement {
  const rawId = useId();
  const uid = rawId.replace(/[^a-zA-Z0-9_-]/g, '');
  const [isHovering, setIsHovering] = useState(false);
  const [isPressed, setIsPressed] = useState(false);
  const [internalCollapsed, setInternalCollapsed] = useState(false);
  const [hoveredHeaderButton, setHoveredHeaderButton] = useState<'collapse' | 'info' | null>(null);
  const stateCollapsed = collapsed ?? internalCollapsed;
  const isCollapsed = visualCollapsed ?? stateCollapsed;
  const collapseControlDisabled = disabled || collapseDisabled;
  const config = useMemo(() => {
    const merged = mergeConfig(defaultChatBubbleConfig, configOverride);
    return mergeConfig(merged, {
      header: {
        width,
      },
      body: {
        minHeight: bodyHeight,
        maxHeight: bodyHeight,
        paddingTop: 0,
        paddingBottom: 0,
        textInsetX: 0,
      },
    });
  }, [bodyHeight, configOverride, width]);
  const geometry = calculateGeometry(config, width, bodyHeight, isCollapsed);
  const isAiSide = variant === 'incoming';
  const headerTop = isAiSide ? config.colors.aiHeaderTop : config.colors.userHeaderTop;
  const headerBottom = isAiSide ? config.colors.aiHeaderBottom : config.colors.userHeaderBottom;
  const bodyTop = isAiSide ? config.colors.aiBodyTop : config.colors.userBodyTop;
  const bodyBottom = isAiSide ? config.colors.aiBodyBottom : config.colors.userBodyBottom;
  const clampTop = isAiSide ? config.colors.aiClampTop : config.colors.userClampTop;
  const clampBottom = isAiSide ? config.colors.aiClampBottom : config.colors.userClampBottom;
  const clampEdge = isAiSide ? config.colors.aiClampEdge : config.colors.userClampEdge;
  const clampGlow = isAiSide ? config.colors.aiClampGlow : config.colors.userClampGlow;
  const strokeIdle = isAiSide ? config.colors.aiStroke : config.colors.userStroke;
  const strokeHover = isAiSide ? config.colors.aiStrokeHover : config.colors.userStrokeHover;
  const glowColor = isAiSide ? config.colors.aiGlow : config.colors.userGlow;
  const edgeGlowColor = isAiSide ? config.colors.aiEdgeGlow : config.colors.userEdgeGlow;
  const strokeColor = isHovering ? strokeHover : strokeIdle;
  const liftY = disabled ? 0 : getStateValue(isPressed, isHovering, config.hover.pressY, config.hover.liftY, 0);
  const scale = disabled
    ? config.hover.scaleIdle
    : getStateValue(isPressed, isHovering, config.hover.scalePressed, config.hover.scaleHover, config.hover.scaleIdle);
  const rootOpacity = disabled ? config.opacity.disabled : 1;
  const glowOpacity = getStateValue(
    isPressed,
    isHovering,
    config.opacity.glowHover,
    config.opacity.glowHover,
    config.opacity.glowIdle
  );
  const strokeOpacity = isHovering ? config.opacity.strokeHover : config.opacity.strokeIdle;
  const shineOpacity = isHovering ? config.opacity.shineHover : config.opacity.shineIdle;
  const joinedBottomLeftRadius = !isCollapsed && !config.body.clampOnLeft ? 0 : config.header.radius;
  const joinedBottomRightRadius = !isCollapsed && config.body.clampOnLeft ? 0 : config.header.radius;
  const headerPath = roundedRectPathByCorner(
    0,
    0,
    geometry.resolvedWidth,
    geometry.headerH,
    config.header.radius,
    config.header.radius,
    joinedBottomRightRadius,
    joinedBottomLeftRadius
  );
  const headerClampPath = roundedRectPathByCorner(
    geometry.headerClampX,
    geometry.headerClampY,
    geometry.headerClampW,
    geometry.headerClampH,
    config.body.clampOnLeft ? config.header.radius : 0,
    !config.body.clampOnLeft ? config.header.radius : 0,
    !config.body.clampOnLeft ? config.header.radius : 0,
    config.body.clampOnLeft ? config.header.radius : 0
  );
  const clampGlowX = geometry.headerClampX - config.body.clampGlowSpreadX;
  const clampGlowY = geometry.headerClampY - config.body.clampGlowSpreadY;
  const clampGlowW = geometry.headerClampW + config.body.clampGlowSpreadX * 2;
  const clampGlowH = geometry.headerClampH + config.body.clampGlowSpreadY * 2;
  const headerClampGlowPath = roundedRectPathByCorner(
    clampGlowX,
    clampGlowY,
    clampGlowW,
    clampGlowH,
    config.header.radius + config.body.clampGlowSpreadY,
    config.header.radius + config.body.clampGlowSpreadY,
    config.header.radius + config.body.clampGlowSpreadY,
    config.header.radius + config.body.clampGlowSpreadY
  );
  const bodyPath = roundedRectPathByCorner(
    geometry.bodyX,
    geometry.bodyY,
    geometry.bodyW,
    geometry.bodyH,
    config.body.topRadius,
    config.body.topRadius,
    config.body.radius,
    !config.body.clampOnLeft ? 0 : config.body.radius
  );
  const collapseButtonBoxX =
    geometry.headerClampX + Math.max(0, (geometry.headerClampW - config.controls.buttonSize) * 0.5);
  const collapseButtonBoxY =
    geometry.headerClampY + Math.max(0, (geometry.headerClampH - config.controls.buttonSize) * 0.5);
  const copyButtonBoxX = config.body.clampOnLeft
    ? geometry.resolvedWidth - config.controls.buttonInsetX - config.controls.buttonSize
    : geometry.headerClampX - config.controls.buttonAfterClampGap - config.controls.buttonSize;
  const copyButtonBoxY = config.controls.buttonInsetY;
  const collapseButtonCenterX = collapseButtonBoxX + config.controls.buttonSize * 0.5;
  const collapseButtonCenterY = collapseButtonBoxY + config.controls.buttonSize * 0.5;
  const collapseTriangleSize = Math.max(10, Math.min(geometry.headerClampW, geometry.headerClampH) - 14);
  const collapseTrianglePath = stateCollapsed
    ? `M${collapseButtonCenterX - collapseTriangleSize * 0.28} ${collapseButtonCenterY - collapseTriangleSize * 0.48}L${collapseButtonCenterX + collapseTriangleSize * 0.46} ${collapseButtonCenterY}L${collapseButtonCenterX - collapseTriangleSize * 0.28} ${collapseButtonCenterY + collapseTriangleSize * 0.48}Z`
    : `M${collapseButtonCenterX - collapseTriangleSize * 0.48} ${collapseButtonCenterY - collapseTriangleSize * 0.28}L${collapseButtonCenterX + collapseTriangleSize * 0.48} ${collapseButtonCenterY - collapseTriangleSize * 0.28}L${collapseButtonCenterX} ${collapseButtonCenterY + collapseTriangleSize * 0.46}Z`;
  const svgStyle: CSSProperties = { transition: config.transition.svg };
  const slot = {
    bodyX: geometry.bodyX,
    bodyY: geometry.bodyY,
    bodyW: geometry.bodyW,
    bodyH: geometry.bodyH,
    bodyContentX: geometry.bodyX,
    bodyContentY: geometry.bodyY,
    bodyContentW: geometry.bodyContentW,
    bodyContentH: geometry.bodyContentH,
  };

  return (
    <g
      role="group"
      aria-label={headerLabel}
      opacity={rootOpacity}
      transform={`translate(${x} ${y + liftY}) scale(${scale})`}
      onPointerEnter={() => setIsHovering(true)}
      onPointerLeave={() => {
        setIsHovering(false);
        setIsPressed(false);
        setHoveredHeaderButton(null);
      }}
      onPointerDown={() => setIsPressed(true)}
      onPointerUp={() => setIsPressed(false)}
      onPointerCancel={() => setIsPressed(false)}
    >
      <defs>
        <linearGradient
          id={`${uid}-svg-frame-header`}
          x1={0}
          y1={0}
          x2={0}
          y2={geometry.headerH}
          gradientUnits="userSpaceOnUse"
        >
          <stop offset={0} stopColor={headerTop} />
          <stop offset={1} stopColor={headerBottom} />
        </linearGradient>
        <linearGradient
          id={`${uid}-svg-frame-body`}
          x1={0}
          y1={geometry.bodyY}
          x2={0}
          y2={geometry.bodyY + geometry.bodyH}
          gradientUnits="userSpaceOnUse"
        >
          <stop offset={0} stopColor={bodyTop} />
          <stop offset={1} stopColor={bodyBottom} />
        </linearGradient>
        <linearGradient
          id={`${uid}-svg-frame-clamp`}
          x1={0}
          y1={geometry.headerClampY}
          x2={0}
          y2={geometry.headerClampY + geometry.headerClampH}
          gradientUnits="userSpaceOnUse"
        >
          <stop offset={0} stopColor={clampTop} />
          <stop offset={1} stopColor={clampBottom} />
        </linearGradient>
        <linearGradient
          id={`${uid}-svg-frame-shine`}
          x1={0}
          y1={0}
          x2={geometry.resolvedWidth}
          y2={geometry.headerH}
          gradientUnits="userSpaceOnUse"
        >
          <stop offset={0} stopColor={config.colors.shine} stopOpacity={0.62} />
          <stop offset={0.36} stopColor={config.colors.shine} stopOpacity={0.18} />
          <stop offset={1} stopColor={config.colors.shine} stopOpacity={0} />
        </linearGradient>
        <linearGradient
          id={`${uid}-svg-frame-header-shine`}
          x1={0}
          y1={0}
          x2={0}
          y2={geometry.headerH}
          gradientUnits="userSpaceOnUse"
        >
          <stop offset={0} stopColor={config.colors.shine} stopOpacity={0.42} />
          <stop offset={0.55} stopColor={config.colors.shine} stopOpacity={0.1} />
          <stop offset={1} stopColor={config.colors.shine} stopOpacity={0} />
        </linearGradient>
        <linearGradient
          id={`${uid}-svg-frame-header-shade`}
          x1={0}
          y1={0}
          x2={0}
          y2={geometry.headerH}
          gradientUnits="userSpaceOnUse"
        >
          <stop offset={0} stopColor="#000000" stopOpacity={0} />
          <stop offset={0.68} stopColor="#000000" stopOpacity={0.05} />
          <stop offset={1} stopColor="#000000" stopOpacity={0.32} />
        </linearGradient>
        <filter
          id={`${uid}-svg-frame-glow`}
          x={config.filters.glow.x}
          y={config.filters.glow.y}
          width={config.filters.glow.width}
          height={config.filters.glow.height}
        >
          <feDropShadow
            dx={0}
            dy={0}
            stdDeviation={config.filters.glow.blurA}
            floodColor={glowColor}
            floodOpacity={glowOpacity}
          />
          <feDropShadow
            dx={0}
            dy={0}
            stdDeviation={config.filters.glow.blurB}
            floodColor={glowColor}
            floodOpacity={glowOpacity * 0.45}
          />
        </filter>
        <filter id={`${uid}-svg-frame-clamp-glow`} x="-120%" y="-120%" width="340%" height="340%">
          <feDropShadow
            dx={0}
            dy={0}
            stdDeviation={config.body.clampGlowBlurA}
            floodColor={clampGlow}
            floodOpacity={isHovering ? 0.72 : 0.42}
          />
          <feDropShadow
            dx={0}
            dy={0}
            stdDeviation={config.body.clampGlowBlurB}
            floodColor={clampGlow}
            floodOpacity={isHovering ? 0.34 : 0.2}
          />
        </filter>
        <filter id={`${uid}-svg-frame-triangle-glow`} x="-140%" y="-140%" width="380%" height="380%">
          <feDropShadow
            dx={0}
            dy={0}
            stdDeviation={1.4}
            floodColor={config.colors.collapseTriangleGlow}
            floodOpacity={hoveredHeaderButton === 'collapse' ? 0.72 : 0.38}
          />
          <feDropShadow
            dx={0}
            dy={0}
            stdDeviation={3.2}
            floodColor={config.colors.collapseTriangleGlow}
            floodOpacity={hoveredHeaderButton === 'collapse' ? 0.3 : 0.14}
          />
        </filter>
        <filter id={`${uid}-svg-frame-info-glow`} x="-120%" y="-120%" width="340%" height="340%">
          <feDropShadow
            dx={0}
            dy={0}
            stdDeviation={1.3}
            floodColor={config.colors.infoGlow}
            floodOpacity={hoveredHeaderButton === 'info' ? 0.86 : 0.42}
          />
          <feDropShadow
            dx={0}
            dy={0}
            stdDeviation={4.2}
            floodColor={config.colors.infoGlow}
            floodOpacity={hoveredHeaderButton === 'info' ? 0.38 : 0.18}
          />
        </filter>
        <filter
          id={`${uid}-svg-frame-shadow`}
          x={config.filters.shadow.x}
          y={config.filters.shadow.y}
          width={config.filters.shadow.width}
          height={config.filters.shadow.height}
        >
          <feDropShadow
            dx={config.filters.shadow.dx}
            dy={config.filters.shadow.dy}
            stdDeviation={config.filters.shadow.blur}
            floodColor={config.colors.shadow}
            floodOpacity={config.opacity.shadow}
          />
        </filter>
      </defs>
      <path
        d={headerPath}
        fill="none"
        stroke={edgeGlowColor}
        strokeWidth={config.header.glowStrokeWidth}
        strokeOpacity={glowOpacity}
        filter={`url(#${uid}-svg-frame-glow)`}
        style={svgStyle}
      />
      <path
        d={headerPath}
        fill={`url(#${uid}-svg-frame-header)`}
        stroke={strokeColor}
        strokeWidth={config.header.strokeWidth}
        strokeOpacity={strokeOpacity}
        filter={`url(#${uid}-svg-frame-shadow)`}
        style={svgStyle}
      />
      <path
        d={headerClampGlowPath}
        fill={clampGlow}
        opacity={isHovering ? config.body.clampGlowOpacityHover : config.body.clampGlowOpacityIdle}
        filter={`url(#${uid}-svg-frame-clamp-glow)`}
        style={svgStyle}
      />
      <path
        d={headerClampPath}
        fill="none"
        stroke={clampGlow}
        strokeWidth={config.body.clampGlowStrokeWidth}
        strokeOpacity={isHovering ? 0.72 : 0.38}
        filter={`url(#${uid}-svg-frame-clamp-glow)`}
        style={svgStyle}
      />
      <path
        d={headerClampPath}
        fill={`url(#${uid}-svg-frame-clamp)`}
        stroke={clampEdge}
        strokeWidth={config.body.clampStrokeWidth}
        strokeOpacity={isHovering ? 1 : 0.82}
        opacity={isHovering ? config.opacity.clampHover : config.opacity.clampIdle}
        style={svgStyle}
      />
      <path
        d={headerPath}
        fill={`url(#${uid}-svg-frame-header-shine)`}
        opacity={isHovering ? config.header.buttonShineOpacityHover : config.header.buttonShineOpacityIdle}
        style={svgStyle}
      />
      <path
        d={headerPath}
        fill={`url(#${uid}-svg-frame-header-shade)`}
        opacity={config.header.bottomShadeOpacity}
        style={svgStyle}
      />
      <path
        d={headerPath}
        fill="none"
        stroke={config.colors.shine}
        strokeWidth={config.header.innerStrokeWidth}
        strokeOpacity={config.opacity.innerStroke + 0.18}
        style={svgStyle}
      />
      <path
        d={headerClampPath}
        fill="none"
        stroke={config.colors.shine}
        strokeWidth={config.body.innerStrokeWidth}
        strokeOpacity={config.opacity.innerStroke + 0.16}
        style={svgStyle}
      />
      <path
        d={bodyPath}
        fill="none"
        stroke={edgeGlowColor}
        strokeWidth={config.body.glowStrokeWidth}
        strokeOpacity={glowOpacity * 0.72}
        filter={`url(#${uid}-svg-frame-glow)`}
        style={svgStyle}
      />
      <path
        d={bodyPath}
        fill={`url(#${uid}-svg-frame-body)`}
        stroke={strokeColor}
        strokeWidth={config.body.strokeWidth}
        strokeOpacity={strokeOpacity}
        filter={`url(#${uid}-svg-frame-shadow)`}
        style={svgStyle}
      />
      <path
        d={bodyPath}
        fill="none"
        stroke={config.colors.shine}
        strokeWidth={config.body.innerStrokeWidth}
        strokeOpacity={config.opacity.innerStroke}
        style={svgStyle}
      />
      <path
        d={`M0 0H${geometry.resolvedWidth}V${geometry.headerH * config.header.shineHeightRatio}C${geometry.resolvedWidth * 0.66} ${geometry.headerH * 0.2} ${geometry.resolvedWidth * 0.3} ${geometry.headerH * 0.28} 0 ${geometry.headerH * 0.74}Z`}
        fill={`url(#${uid}-svg-frame-shine)`}
        opacity={shineOpacity}
        style={svgStyle}
      />
      <g
        role="button"
        aria-label={stateCollapsed ? expandLabel : collapseLabel}
        tabIndex={collapseControlDisabled ? -1 : 0}
        onPointerEnter={() => setHoveredHeaderButton('collapse')}
        onPointerLeave={() => setHoveredHeaderButton(null)}
        onClick={(event) => {
          if (collapseControlDisabled) return;
          event.stopPropagation();
          const nextCollapsed = !stateCollapsed;
          if (collapsed === undefined) {
            setInternalCollapsed(nextCollapsed);
          }
          onCollapsedChange?.(nextCollapsed);
        }}
        onKeyDown={(event) => {
          if (collapseControlDisabled || (event.key !== 'Enter' && event.key !== ' ')) return;
          event.preventDefault();
          event.stopPropagation();
          const nextCollapsed = !stateCollapsed;
          if (collapsed === undefined) {
            setInternalCollapsed(nextCollapsed);
          }
          onCollapsedChange?.(nextCollapsed);
        }}
        style={{ cursor: collapseControlDisabled ? 'default' : 'pointer', ...svgStyle }}
      >
        <rect
          x={geometry.headerClampX}
          y={geometry.headerClampY}
          width={geometry.headerClampW}
          height={geometry.headerClampH}
          fill="transparent"
          pointerEvents="all"
        />
        <path
          d={collapseTrianglePath}
          fill={
            hoveredHeaderButton === 'collapse'
              ? config.colors.collapseTriangleFillHover
              : config.colors.collapseTriangleFill
          }
          stroke={config.colors.collapseTriangleStroke}
          strokeWidth={config.controls.chevronStrokeWidth * 0.72}
          strokeLinejoin="round"
          opacity={hoveredHeaderButton === 'collapse' ? 1 : 0.9}
          filter={`url(#${uid}-svg-frame-triangle-glow)`}
          style={svgStyle}
        />
      </g>
      {headerLabel ? (
        <text
          x={geometry.headerClampX + geometry.headerClampW + 12}
          y={geometry.headerH * 0.67}
          fill={config.colors.chevron}
          fontFamily={config.typography.fontFamily}
          fontSize={16.5}
          fontWeight={950}
          letterSpacing={0}
          opacity={0.98}
          pointerEvents="none"
          style={svgStyle}
        >
          {headerLabel}
        </text>
      ) : null}
      {showInfo ? (
        <g
          role="button"
          aria-label={infoLabel}
          opacity={hoveredHeaderButton === 'info' ? 1 : 0.92}
          style={{ cursor: 'pointer', ...svgStyle }}
          onPointerEnter={() => setHoveredHeaderButton('info')}
          onPointerLeave={() => setHoveredHeaderButton(null)}
          onClick={(event) => {
            event.stopPropagation();
            onInfoClick?.();
          }}
        >
          <circle
            cx={copyButtonBoxX + config.controls.buttonSize * 0.5}
            cy={copyButtonBoxY + config.controls.buttonSize * 0.5}
            r={config.controls.buttonSize * 0.43}
            fill={hoveredHeaderButton === 'info' ? config.colors.infoFillHover : config.colors.infoFill}
            stroke={config.colors.infoStroke}
            strokeWidth={hoveredHeaderButton === 'info' ? 1.35 : 0.95}
            filter={`url(#${uid}-svg-frame-info-glow)`}
          />
          <text
            x={copyButtonBoxX + config.controls.buttonSize * 0.5}
            y={copyButtonBoxY + config.controls.buttonSize * 0.5 + config.controls.buttonSize * 0.18}
            textAnchor="middle"
            fill={config.colors.infoText}
            fontFamily={config.typography.fontFamily}
            fontSize={config.controls.buttonSize * 0.68}
            fontWeight={950}
            letterSpacing={0}
            pointerEvents="none"
            style={svgStyle}
          >
            i
          </text>
        </g>
      ) : null}
      {!isCollapsed ? children?.(slot) : null}
    </g>
  );
}

const copyDocumentIconPaths = {
  paperBack: 'M589.3 260.9v30H371.4v-30H268.9v513h117.2v-304l109.7-99.1h202.1V260.9z',
  paperFront: 'M516.1 371.1l-122.9 99.8v346.8h370.4V371.1z',
  darkEdges: [
    'M752.7 370.8h21.8v435.8h-21.8z',
    'M495.8 370.8h277.3v21.8H495.8z',
    'M495.8 370.8h21.8v124.3h-21.8z',
    'M397.7 488.7l-15.4-15.4 113.5-102.5 15.4 15.4z',
    'M382.3 473.3h135.3v21.8H382.3z',
    'M382.3 479.7h21.8v348.6h-21.8zM404.1 806.6h370.4v21.8H404.1z',
    'M251.6 763h130.7v21.8H251.6z',
    'M251.6 240.1h21.8v544.7h-21.8zM687.3 240.1h21.8v130.7h-21.8zM273.4 240.1h108.9v21.8H273.4z',
    'M578.4 240.1h130.7v21.8H578.4zM360.5 196.5h21.8v108.9h-21.8zM382.3 283.7h196.1v21.8H382.3zM534.8 196.5h65.4v21.8h-65.4z',
    'M360.5 196.5h65.4v21.8h-65.4zM404.1 174.7h152.5v21.8H404.1zM578.4 196.5h21.8v108.9h-21.8z',
  ],
  softLines: ['M447.7 545.1h261.5v21.8H447.7z', 'M447.7 610.5h261.5v21.8H447.7z', 'M447.7 675.8h261.5v21.8H447.7z'],
};

function CopyDocumentIcon({
  x,
  y,
  size,
  config,
  opacity,
}: {
  x: number;
  y: number;
  size: number;
  config: ChatBubbleConfig;
  opacity: number;
}) {
  return (
    <svg
      x={x}
      y={y}
      width={size}
      height={size}
      viewBox={`${config.controls.copyIconViewBoxX} ${config.controls.copyIconViewBoxY} ${config.controls.copyIconViewBoxW} ${config.controls.copyIconViewBoxH}`}
      aria-hidden="true"
      opacity={opacity}
      preserveAspectRatio="xMidYMid meet"
    >
      <path d={copyDocumentIconPaths.paperBack} fill={config.colors.copyPaper} />
      <path d={copyDocumentIconPaths.paperFront} fill={config.colors.copyPaper} />
      {copyDocumentIconPaths.darkEdges.map((path, index) => (
        <path key={`copy-dark-${index}`} d={path} fill={config.colors.copyDark} />
      ))}
      {copyDocumentIconPaths.softLines.map((path, index) => (
        <path key={`copy-line-${index}`} d={path} fill={config.colors.copyMain} />
      ))}
    </svg>
  );
}

function DefaultTextContent({ text }: { text: string }) {
  return <div style={{ whiteSpace: 'pre-wrap' }}>{text}</div>;
}

export function ChatBubbleSvg({
  variant = 'incoming',
  width,
  body,
  children,
  text = defaultChatBubbleText,
  collapsed,
  showCopy = false,
  disabled = false,
  className,
  copyLabel = 'Copy message',
  collapseLabel = 'Collapse message',
  expandLabel = 'Expand message',
  messageLabel,
  headerLabel,
  showInfo = false,
  infoLabel = 'Open guide',
  onClick,
  onCopyClick,
  onInfoClick,
  onCollapsedChange,
  config: configOverride,
}: ChatBubbleSvgProps) {
  const [isHovering, setIsHovering] = useState(false);
  const [isPressed, setIsPressed] = useState(false);
  const [internalCollapsed, setInternalCollapsed] = useState(false);
  const [hoveredHeaderButton, setHoveredHeaderButton] = useState<'collapse' | 'copy' | 'info' | null>(null);
  const [contentHeight, setContentHeight] = useState(0);
  const [renderBodyContent, setRenderBodyContent] = useState(true);
  const contentMeasureRef = useRef<HTMLDivElement | null>(null);
  const rawId = useId();
  const uid = rawId.replace(/[^a-zA-Z0-9_-]/g, '');
  const config = useMemo(() => mergeConfig(defaultChatBubbleConfig, configOverride), [configOverride]);
  const isCollapsed = collapsed ?? internalCollapsed;
  const bodyNode = body ?? children ?? <DefaultTextContent text={text} />;
  const fallbackContentHeight = estimateTextHeight(config, text);
  const measuredContentHeight = contentHeight || fallbackContentHeight;
  const geometry = calculateGeometry(config, width, measuredContentHeight, isCollapsed);

  useEffect(() => {
    const element = contentMeasureRef.current;
    if (!element) return;

    const updateHeight = () => {
      setContentHeight(element.scrollHeight);
    };

    updateHeight();

    if (typeof ResizeObserver === 'undefined') {
      window.setTimeout(updateHeight, 0);
      return;
    }

    const observer = new ResizeObserver(updateHeight);
    observer.observe(element);
    return () => observer.disconnect();
  }, [bodyNode, config.text.fontSize, config.text.lineHeight, geometry.bodyContentW]);

  useEffect(() => {
    if (isCollapsed) {
      setRenderBodyContent(false);
      return;
    }

    const timeout = window.setTimeout(() => {
      setRenderBodyContent(true);
    }, config.body.contentFadeDelayMs);

    return () => window.clearTimeout(timeout);
  }, [isCollapsed, config.body.contentFadeDelayMs]);

  const isAiSide = config.body.clampOnLeft;
  const headerTop = isAiSide ? config.colors.aiHeaderTop : config.colors.userHeaderTop;
  const headerBottom = isAiSide ? config.colors.aiHeaderBottom : config.colors.userHeaderBottom;
  const bodyTop = isAiSide ? config.colors.aiBodyTop : config.colors.userBodyTop;
  const bodyBottom = isAiSide ? config.colors.aiBodyBottom : config.colors.userBodyBottom;
  const clampTop = isAiSide ? config.colors.aiClampTop : config.colors.userClampTop;
  const clampBottom = isAiSide ? config.colors.aiClampBottom : config.colors.userClampBottom;
  const clampEdge = isAiSide ? config.colors.aiClampEdge : config.colors.userClampEdge;
  const clampGlow = isAiSide ? config.colors.aiClampGlow : config.colors.userClampGlow;
  const strokeIdle = isAiSide ? config.colors.aiStroke : config.colors.userStroke;
  const strokeHover = isAiSide ? config.colors.aiStrokeHover : config.colors.userStrokeHover;
  const glowColor = isAiSide ? config.colors.aiGlow : config.colors.userGlow;
  const edgeGlowColor = isAiSide ? config.colors.aiEdgeGlow : config.colors.userEdgeGlow;
  const strokeColor = isHovering ? strokeHover : strokeIdle;
  const liftY = disabled ? 0 : getStateValue(isPressed, isHovering, config.hover.pressY, config.hover.liftY, 0);
  const scale = disabled
    ? config.hover.scaleIdle
    : getStateValue(isPressed, isHovering, config.hover.scalePressed, config.hover.scaleHover, config.hover.scaleIdle);
  const rootOpacity = disabled ? config.opacity.disabled : 1;
  const glowOpacity = getStateValue(
    isPressed,
    isHovering,
    config.opacity.glowHover,
    config.opacity.glowHover,
    config.opacity.glowIdle
  );
  const strokeOpacity = isHovering ? config.opacity.strokeHover : config.opacity.strokeIdle;
  const shineOpacity = isHovering ? config.opacity.shineHover : config.opacity.shineIdle;
  const copyOpacity = isHovering ? config.opacity.copyHover : config.opacity.copyIdle;
  const bodyOpacity = renderBodyContent && !isCollapsed ? config.opacity.bodyIdle : config.opacity.bodyCollapsed;
  const bodyTextTranslateY = renderBodyContent && !isCollapsed ? 0 : -4;
  const joinedBottomLeftRadius = !isCollapsed && !config.body.clampOnLeft ? 0 : config.header.radius;
  const joinedBottomRightRadius = !isCollapsed && config.body.clampOnLeft ? 0 : config.header.radius;
  const headerPath = roundedRectPathByCorner(
    0,
    0,
    geometry.resolvedWidth,
    geometry.headerH,
    config.header.radius,
    config.header.radius,
    joinedBottomRightRadius,
    joinedBottomLeftRadius
  );
  const headerClampPath = roundedRectPathByCorner(
    geometry.headerClampX,
    geometry.headerClampY,
    geometry.headerClampW,
    geometry.headerClampH,
    config.body.clampOnLeft ? config.header.radius : 0,
    !config.body.clampOnLeft ? config.header.radius : 0,
    !config.body.clampOnLeft ? config.header.radius : 0,
    config.body.clampOnLeft ? config.header.radius : 0
  );
  const clampGlowX = geometry.headerClampX - config.body.clampGlowSpreadX;
  const clampGlowY = geometry.headerClampY - config.body.clampGlowSpreadY;
  const clampGlowW = geometry.headerClampW + config.body.clampGlowSpreadX * 2;
  const clampGlowH = geometry.headerClampH + config.body.clampGlowSpreadY * 2;
  const headerClampGlowPath = roundedRectPathByCorner(
    clampGlowX,
    clampGlowY,
    clampGlowW,
    clampGlowH,
    config.header.radius + config.body.clampGlowSpreadY,
    config.header.radius + config.body.clampGlowSpreadY,
    config.header.radius + config.body.clampGlowSpreadY,
    config.header.radius + config.body.clampGlowSpreadY
  );
  const bodyPath = roundedRectPathByCorner(
    geometry.bodyX,
    geometry.bodyY,
    geometry.bodyW,
    geometry.bodyH,
    config.body.topRadius,
    config.body.topRadius,
    config.body.radius,
    !config.body.clampOnLeft ? 0 : config.body.radius
  );
  const collapseButtonBoxX =
    geometry.headerClampX + Math.max(0, (geometry.headerClampW - config.controls.buttonSize) * 0.5);
  const collapseButtonBoxY =
    geometry.headerClampY + Math.max(0, (geometry.headerClampH - config.controls.buttonSize) * 0.5);
  const copyButtonBoxX = config.body.clampOnLeft
    ? geometry.resolvedWidth - config.controls.buttonInsetX - config.controls.buttonSize
    : geometry.headerClampX - config.controls.buttonAfterClampGap - config.controls.buttonSize;
  const copyButtonBoxY = config.controls.buttonInsetY;
  const collapseButtonCenterX = collapseButtonBoxX + config.controls.buttonSize * 0.5;
  const collapseButtonCenterY = collapseButtonBoxY + config.controls.buttonSize * 0.5;
  const copyX = copyButtonBoxX + config.controls.copyPadding;
  const copyY = copyButtonBoxY + config.controls.copyPadding;
  const copySize = config.controls.buttonSize - config.controls.copyPadding * 2;
  const collapseTriangleSize = Math.max(10, Math.min(geometry.headerClampW, geometry.headerClampH) - 14);
  const collapseTrianglePath = isCollapsed
    ? `M${collapseButtonCenterX - collapseTriangleSize * 0.28} ${collapseButtonCenterY - collapseTriangleSize * 0.48}L${collapseButtonCenterX + collapseTriangleSize * 0.46} ${collapseButtonCenterY}L${collapseButtonCenterX - collapseTriangleSize * 0.28} ${collapseButtonCenterY + collapseTriangleSize * 0.48}Z`
    : `M${collapseButtonCenterX - collapseTriangleSize * 0.48} ${collapseButtonCenterY - collapseTriangleSize * 0.28}L${collapseButtonCenterX + collapseTriangleSize * 0.48} ${collapseButtonCenterY - collapseTriangleSize * 0.28}L${collapseButtonCenterX} ${collapseButtonCenterY + collapseTriangleSize * 0.46}Z`;
  const bodyContentX = geometry.bodyX + config.body.textInsetX;
  const bodyContentY = geometry.bodyY + config.body.paddingTop;
  const rootStyle: CSSProperties = {
    width: geometry.resolvedWidth,
    minWidth: geometry.resolvedWidth,
    maxWidth: geometry.resolvedWidth,
    height: geometry.totalHeight,
    overflow: 'visible',
    opacity: rootOpacity,
    transform: `translateY(${liftY}px) scale(${scale})`,
    transformOrigin: variant === 'outgoing' ? 'right top' : 'left top',
    transition: config.transition.root,
    cursor: disabled ? 'not-allowed' : onClick ? 'pointer' : 'default',
    position: 'relative',
  };
  const svgStyle: CSSProperties = { transition: config.transition.svg };
  const contentStyle: CSSProperties = {
    position: 'absolute',
    left: bodyContentX,
    top: bodyContentY,
    width: geometry.bodyContentW,
    height: geometry.bodyContentH,
    overflowY: geometry.shouldScroll ? 'auto' : 'hidden',
    overflowX: 'hidden',
    color: config.colors.text,
    fontFamily: config.typography.fontFamily,
    fontSize: config.text.fontSize,
    lineHeight: config.text.lineHeight,
    fontWeight: config.typography.fontWeight,
    letterSpacing: config.typography.letterSpacing,
    pointerEvents: isCollapsed ? 'none' : 'auto',
    opacity: bodyOpacity,
    transform: `translateY(${bodyTextTranslateY}px)`,
    transition: config.transition.body,
    whiteSpace: 'normal',
    boxSizing: 'border-box',
    scrollbarColor: `${config.colors.scrollbarThumb} ${config.colors.scrollbarTrack}`,
    scrollbarWidth: 'thin',
  };
  const measureStyle: CSSProperties = {
    position: 'absolute',
    left: -10000,
    top: -10000,
    width: geometry.bodyContentW,
    visibility: 'hidden',
    pointerEvents: 'none',
    color: config.colors.text,
    fontFamily: config.typography.fontFamily,
    fontSize: config.text.fontSize,
    lineHeight: config.text.lineHeight,
    fontWeight: config.typography.fontWeight,
    letterSpacing: config.typography.letterSpacing,
    whiteSpace: 'normal',
    boxSizing: 'border-box',
  };

  return (
    <div
      className={className}
      role="article"
      aria-label={messageLabel ?? text}
      style={rootStyle}
      onPointerEnter={() => setIsHovering(true)}
      onPointerLeave={() => {
        setIsHovering(false);
        setIsPressed(false);
      }}
      onPointerDown={() => setIsPressed(true)}
      onPointerUp={() => setIsPressed(false)}
      onPointerCancel={() => setIsPressed(false)}
      onClick={disabled ? undefined : onClick}
    >
      <div ref={contentMeasureRef} style={measureStyle}>
        {bodyNode}
      </div>
      <svg
        viewBox={`${geometry.viewBoxX} ${geometry.viewBoxY} ${geometry.viewBoxW} ${geometry.viewBoxH}`}
        width={geometry.resolvedWidth}
        height={geometry.totalHeight}
        role="group"
        aria-label={variant === 'outgoing' ? 'Outgoing message' : 'Incoming message'}
        preserveAspectRatio="none"
        style={{ display: 'block', minWidth: geometry.resolvedWidth }}
      >
        <defs>
          <linearGradient
            id={`${uid}-header`}
            x1={0}
            y1={0}
            x2={0}
            y2={geometry.headerH}
            gradientUnits="userSpaceOnUse"
          >
            <stop offset={0} stopColor={headerTop} />
            <stop offset={1} stopColor={headerBottom} />
          </linearGradient>
          <linearGradient
            id={`${uid}-body`}
            x1={0}
            y1={geometry.bodyY}
            x2={0}
            y2={geometry.bodyY + geometry.bodyH}
            gradientUnits="userSpaceOnUse"
          >
            <stop offset={0} stopColor={bodyTop} />
            <stop offset={1} stopColor={bodyBottom} />
          </linearGradient>
          <linearGradient
            id={`${uid}-clamp`}
            x1={0}
            y1={geometry.headerClampY}
            x2={0}
            y2={geometry.headerClampY + geometry.headerClampH}
            gradientUnits="userSpaceOnUse"
          >
            <stop offset={0} stopColor={clampTop} />
            <stop offset={1} stopColor={clampBottom} />
          </linearGradient>
          <linearGradient
            id={`${uid}-shine`}
            x1={0}
            y1={0}
            x2={geometry.resolvedWidth}
            y2={geometry.headerH}
            gradientUnits="userSpaceOnUse"
          >
            <stop offset={0} stopColor={config.colors.shine} stopOpacity={0.62} />
            <stop offset={0.36} stopColor={config.colors.shine} stopOpacity={0.18} />
            <stop offset={1} stopColor={config.colors.shine} stopOpacity={0} />
          </linearGradient>
          <linearGradient
            id={`${uid}-headerButtonShine`}
            x1={0}
            y1={0}
            x2={0}
            y2={geometry.headerH}
            gradientUnits="userSpaceOnUse"
          >
            <stop offset={0} stopColor={config.colors.shine} stopOpacity={0.42} />
            <stop offset={0.55} stopColor={config.colors.shine} stopOpacity={0.1} />
            <stop offset={1} stopColor={config.colors.shine} stopOpacity={0} />
          </linearGradient>
          <linearGradient
            id={`${uid}-headerBottomShade`}
            x1={0}
            y1={0}
            x2={0}
            y2={geometry.headerH}
            gradientUnits="userSpaceOnUse"
          >
            <stop offset={0} stopColor="#000000" stopOpacity={0} />
            <stop offset={0.68} stopColor="#000000" stopOpacity={0.05} />
            <stop offset={1} stopColor="#000000" stopOpacity={0.32} />
          </linearGradient>
          <filter
            id={`${uid}-glow`}
            x={config.filters.glow.x}
            y={config.filters.glow.y}
            width={config.filters.glow.width}
            height={config.filters.glow.height}
          >
            <feDropShadow
              dx={0}
              dy={0}
              stdDeviation={config.filters.glow.blurA}
              floodColor={glowColor}
              floodOpacity={glowOpacity}
            />
            <feDropShadow
              dx={0}
              dy={0}
              stdDeviation={config.filters.glow.blurB}
              floodColor={glowColor}
              floodOpacity={glowOpacity * 0.45}
            />
          </filter>
          <filter id={`${uid}-clampGlow`} x="-120%" y="-120%" width="340%" height="340%">
            <feDropShadow
              dx={0}
              dy={0}
              stdDeviation={config.body.clampGlowBlurA}
              floodColor={clampGlow}
              floodOpacity={isHovering ? 0.72 : 0.42}
            />
            <feDropShadow
              dx={0}
              dy={0}
              stdDeviation={config.body.clampGlowBlurB}
              floodColor={clampGlow}
              floodOpacity={isHovering ? 0.34 : 0.2}
            />
          </filter>
          <filter id={`${uid}-collapseTriangleGlow`} x="-140%" y="-140%" width="380%" height="380%">
            <feDropShadow
              dx={0}
              dy={0}
              stdDeviation={1.4}
              floodColor={config.colors.collapseTriangleGlow}
              floodOpacity={hoveredHeaderButton === 'collapse' ? 0.72 : 0.38}
            />
            <feDropShadow
              dx={0}
              dy={0}
              stdDeviation={3.2}
              floodColor={config.colors.collapseTriangleGlow}
              floodOpacity={hoveredHeaderButton === 'collapse' ? 0.3 : 0.14}
            />
          </filter>
          <filter id={`${uid}-infoGlow`} x="-120%" y="-120%" width="340%" height="340%">
            <feDropShadow
              dx={0}
              dy={0}
              stdDeviation={1.3}
              floodColor={config.colors.infoGlow}
              floodOpacity={hoveredHeaderButton === 'info' ? 0.86 : 0.42}
            />
            <feDropShadow
              dx={0}
              dy={0}
              stdDeviation={4.2}
              floodColor={config.colors.infoGlow}
              floodOpacity={hoveredHeaderButton === 'info' ? 0.38 : 0.18}
            />
          </filter>
          <filter
            id={`${uid}-shadow`}
            x={config.filters.shadow.x}
            y={config.filters.shadow.y}
            width={config.filters.shadow.width}
            height={config.filters.shadow.height}
          >
            <feDropShadow
              dx={config.filters.shadow.dx}
              dy={config.filters.shadow.dy}
              stdDeviation={config.filters.shadow.blur}
              floodColor={config.colors.shadow}
              floodOpacity={config.opacity.shadow}
            />
          </filter>
        </defs>
        <path
          d={headerPath}
          fill="none"
          stroke={edgeGlowColor}
          strokeWidth={config.header.glowStrokeWidth}
          strokeOpacity={glowOpacity}
          filter={`url(#${uid}-glow)`}
          style={svgStyle}
        />
        <path
          d={headerPath}
          fill={`url(#${uid}-header)`}
          stroke={strokeColor}
          strokeWidth={config.header.strokeWidth}
          strokeOpacity={strokeOpacity}
          filter={`url(#${uid}-shadow)`}
          style={svgStyle}
        />
        <path
          d={headerClampGlowPath}
          fill={clampGlow}
          opacity={isHovering ? config.body.clampGlowOpacityHover : config.body.clampGlowOpacityIdle}
          filter={`url(#${uid}-clampGlow)`}
          style={svgStyle}
        />
        <path
          d={headerClampPath}
          fill="none"
          stroke={clampGlow}
          strokeWidth={config.body.clampGlowStrokeWidth}
          strokeOpacity={isHovering ? 0.72 : 0.38}
          filter={`url(#${uid}-clampGlow)`}
          style={svgStyle}
        />
        <path
          d={headerClampPath}
          fill={`url(#${uid}-clamp)`}
          stroke={clampEdge}
          strokeWidth={config.body.clampStrokeWidth}
          strokeOpacity={isHovering ? 1 : 0.82}
          opacity={isHovering ? config.opacity.clampHover : config.opacity.clampIdle}
          style={svgStyle}
        />
        <path
          d={headerPath}
          fill={`url(#${uid}-headerButtonShine)`}
          opacity={isHovering ? config.header.buttonShineOpacityHover : config.header.buttonShineOpacityIdle}
          style={svgStyle}
        />
        <path
          d={headerPath}
          fill={`url(#${uid}-headerBottomShade)`}
          opacity={config.header.bottomShadeOpacity}
          style={svgStyle}
        />
        <path
          d={headerPath}
          fill="none"
          stroke={config.colors.shine}
          strokeWidth={config.header.innerStrokeWidth}
          strokeOpacity={config.opacity.innerStroke + 0.18}
          style={svgStyle}
        />
        <path
          d={headerClampPath}
          fill="none"
          stroke={config.colors.shine}
          strokeWidth={config.body.innerStrokeWidth}
          strokeOpacity={config.opacity.innerStroke + 0.16}
          style={svgStyle}
        />
        <path
          d={bodyPath}
          fill="none"
          stroke={edgeGlowColor}
          strokeWidth={config.body.glowStrokeWidth}
          strokeOpacity={glowOpacity * 0.72}
          filter={`url(#${uid}-glow)`}
          style={svgStyle}
        />
        <path
          d={bodyPath}
          fill={`url(#${uid}-body)`}
          stroke={strokeColor}
          strokeWidth={config.body.strokeWidth}
          strokeOpacity={strokeOpacity}
          filter={`url(#${uid}-shadow)`}
          style={svgStyle}
        />
        <path
          d={bodyPath}
          fill="none"
          stroke={config.colors.shine}
          strokeWidth={config.body.innerStrokeWidth}
          strokeOpacity={config.opacity.innerStroke}
          style={svgStyle}
        />
        <path
          d={`M0 0H${geometry.resolvedWidth}V${geometry.headerH * config.header.shineHeightRatio}C${geometry.resolvedWidth * 0.66} ${geometry.headerH * 0.2} ${geometry.resolvedWidth * 0.3} ${geometry.headerH * 0.28} 0 ${geometry.headerH * 0.74}Z`}
          fill={`url(#${uid}-shine)`}
          opacity={shineOpacity}
          style={svgStyle}
        />
        <g
          role="button"
          aria-label={isCollapsed ? expandLabel : collapseLabel}
          tabIndex={disabled ? -1 : 0}
          onPointerEnter={() => setHoveredHeaderButton('collapse')}
          onPointerLeave={() => setHoveredHeaderButton(null)}
          onClick={(event) => {
            if (disabled) return;
            event.stopPropagation();
            const nextCollapsed = !isCollapsed;
            if (collapsed === undefined) {
              setRenderBodyContent(false);
              window.setTimeout(() => setInternalCollapsed(nextCollapsed), config.body.contentFadeDelayMs);
            }
            onCollapsedChange?.(nextCollapsed);
          }}
          onKeyDown={(event) => {
            if (disabled || (event.key !== 'Enter' && event.key !== ' ')) return;
            event.preventDefault();
            event.stopPropagation();
            const nextCollapsed = !isCollapsed;
            if (collapsed === undefined) {
              setRenderBodyContent(false);
              window.setTimeout(() => setInternalCollapsed(nextCollapsed), config.body.contentFadeDelayMs);
            }
            onCollapsedChange?.(nextCollapsed);
          }}
          style={{ cursor: 'pointer', ...svgStyle }}
        >
          <rect
            x={geometry.headerClampX}
            y={geometry.headerClampY}
            width={geometry.headerClampW}
            height={geometry.headerClampH}
            fill="transparent"
            pointerEvents="all"
          />
          <path
            d={collapseTrianglePath}
            fill={
              hoveredHeaderButton === 'collapse'
                ? config.colors.collapseTriangleFillHover
                : config.colors.collapseTriangleFill
            }
            stroke={config.colors.collapseTriangleStroke}
            strokeWidth={config.controls.chevronStrokeWidth * 0.72}
            strokeLinejoin="round"
            opacity={hoveredHeaderButton === 'collapse' ? 1 : 0.9}
            filter={`url(#${uid}-collapseTriangleGlow)`}
            style={svgStyle}
          />
        </g>
        {headerLabel ? (
          <text
            x={geometry.headerClampX + geometry.headerClampW + 12}
            y={geometry.headerH * 0.67}
            fill={config.colors.chevron}
            fontFamily={config.typography.fontFamily}
            fontSize={16.5}
            fontWeight={950}
            letterSpacing={0}
            opacity={0.98}
            style={svgStyle}
          >
            {headerLabel}
          </text>
        ) : null}
        {showInfo ? (
          <g
            role="button"
            aria-label={infoLabel}
            opacity={hoveredHeaderButton === 'info' ? 1 : 0.92}
            style={{ cursor: 'pointer', ...svgStyle }}
            onPointerEnter={() => setHoveredHeaderButton('info')}
            onPointerLeave={() => setHoveredHeaderButton(null)}
            onClick={(event) => {
              event.stopPropagation();
              onInfoClick?.();
            }}
          >
            <circle
              cx={copyButtonBoxX + config.controls.buttonSize * 0.5}
              cy={copyButtonBoxY + config.controls.buttonSize * 0.5}
              r={config.controls.buttonSize * 0.43}
              fill={hoveredHeaderButton === 'info' ? config.colors.infoFillHover : config.colors.infoFill}
              stroke={config.colors.infoStroke}
              strokeWidth={hoveredHeaderButton === 'info' ? 1.35 : 0.95}
              filter={`url(#${uid}-infoGlow)`}
            />
            <text
              x={copyButtonBoxX + config.controls.buttonSize * 0.5}
              y={copyButtonBoxY + config.controls.buttonSize * 0.5 + config.controls.buttonSize * 0.18}
              textAnchor="middle"
              fill={config.colors.infoText}
              fontFamily={config.typography.fontFamily}
              fontSize={config.controls.buttonSize * 0.68}
              fontWeight={950}
              letterSpacing={0}
              pointerEvents="none"
              style={svgStyle}
            >
              i
            </text>
          </g>
        ) : null}
        {showCopy ? (
          <g
            role="button"
            aria-label={copyLabel}
            opacity={copyOpacity}
            style={{ cursor: 'pointer', ...svgStyle }}
            onPointerEnter={() => setHoveredHeaderButton('copy')}
            onPointerLeave={() => setHoveredHeaderButton(null)}
            onClick={(event) => {
              event.stopPropagation();
              if (onCopyClick) {
                onCopyClick();
                return;
              }
              void navigator.clipboard?.writeText(text);
            }}
          >
            <rect
              x={copyButtonBoxX}
              y={copyButtonBoxY}
              width={config.controls.buttonSize}
              height={config.controls.buttonSize}
              rx={config.controls.buttonRadius}
              fill={hoveredHeaderButton === 'copy' ? config.colors.controlFillHover : config.colors.controlFill}
              stroke={hoveredHeaderButton === 'copy' ? config.colors.controlStrokeHover : config.colors.controlStroke}
              strokeWidth={config.controls.buttonStrokeWidth}
              opacity={hoveredHeaderButton === 'copy' ? config.opacity.controlHover : config.opacity.controlIdle}
            />
            <CopyDocumentIcon
              x={copyX}
              y={copyY}
              size={copySize}
              config={config}
              opacity={hoveredHeaderButton === 'copy' ? 1 : config.opacity.copyIdle}
            />
          </g>
        ) : null}
      </svg>
      <div style={contentStyle}>{renderBodyContent && !isCollapsed ? bodyNode : null}</div>
    </div>
  );
}
