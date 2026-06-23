import {
  useEffect,
  useLayoutEffect,
  useMemo,
  useRef,
  useState,
  type CSSProperties,
  type Dispatch,
  type ReactElement,
  type SetStateAction,
  type TransitionEvent,
} from 'react';

import { Action } from '../../Common/NavSvgIcons/Action';
import { EnforcementOfficerIcon } from '../../Common/NavSvgIcons/ParentNavSvgIcons';
import { defaultChatBubbleConfig, RulesBubbleSvgFrame } from './ParentPortalRulesBubble';
import { calculateScopeMultiChoiceMetrics } from './ScopeMultiChoice/ScopeMultiChoiceMetrics';
import { defaultScopeMultiChoiceConfig, mergeScopeMultiChoiceConfig } from './ScopeMultiChoice/ScopeMultiChoiceConfig';
import { ScopeMultiChoice } from './ScopeMultiChoice/ScopeMultiChoice';
import { calculateScopeToggleMetrics } from './ScopeToggle/ScopeToggleMetrics';
import { defaultScopeToggleConfig, mergeScopeToggleConfig } from './ScopeToggle/ScopeToggleConfig';
import { ScopeToggle } from './ScopeToggle/ScopeToggle';
import './BrowserRulesQuestionnaire.css';

export type BrowserRulesChoiceOption = {
  readonly value: string;
  readonly label: string;
  readonly disabled?: boolean;
};

export type BrowserRulesQuestionKind = 'single' | 'multi' | 'action';

export type BrowserRulesQuestion = {
  readonly id: string;
  readonly header: string;
  readonly title: string;
  readonly kind: BrowserRulesQuestionKind;
  readonly disabled?: boolean;
  readonly enforcementDisabled?: boolean;
  readonly enforcementHidden?: boolean;
  readonly multiSelect?: boolean;
  readonly value?: string;
  readonly selected?: readonly string[];
  readonly options: readonly BrowserRulesChoiceOption[];
  readonly collapsed: boolean;
  readonly onCollapsedChange: (collapsed: boolean) => void;
  readonly onSingleChange?: (value: string) => void;
  readonly onMultiChange?: (selected: readonly string[]) => void;
  readonly enforcementValue: string;
  readonly onEnforcementChange: (value: string) => void;
};

type BrowserRulesQuestionnaireProps = {
  readonly questions: readonly BrowserRulesQuestion[];
  readonly availableWidth?: number;
  readonly disabled?: boolean;
  readonly onInfoClick?: () => void;
};

const BROWSER_RULES_CONTROL_TITLE_SLOT_WIDTH = 52;

const BROWSER_RULES_CONTROL_FRAME_GLOW = {
  titleBoxGlowStrokeWidth: 1.7,
  titleGlowIdle: 0.78,
  titleGlowHover: 1,
  outerEdgeGlowStrokeWidth: 1.8,
  outerGlowIdle: 0.3,
  outerGlowHover: 0.58,
  trackGlowStrokeWidth: 2.3,
  trackGlowIdle: 0.3,
  trackGlowHover: 0.58,
  titleBoxGlow: '#ffd36a',
  outerEdgeGlow: '#70dfff',
  trackGlow: '#53c7ff',
} as const;

const BROWSER_RULES_CONTROL_LAYOUT = {
  actionTitleWidth: BROWSER_RULES_CONTROL_TITLE_SLOT_WIDTH,
  actionControlMinWidth: 284,
  actionDefaultWidth: 480,
  actionInlineShare: 0.78,
  actionSingleRowMinWidth: 450,
  actionThreeColumnMinWidth: 360,
  actionBottomPadding: 11,
  actionOptionGapY: 4,
  multiControlMinWidth: 284,
  multiDefaultWidth: 520,
  enforcementControlMinWidth: 224,
  enforcementDefaultWidth: 224,
  enforcementControlMaxWidth: 238,
  enforcementControlFluidRatio: 0.22,
  enforcementOptionMinWidth: 78,
  enforcementOptionMaxWidth: 104,
  enforcementOptionPaddingX: 8,
  actionHeight: 88,
  actionViewportInset: 6,
  actionTitleY: 23,
  actionTitleHeight: 40,
  actionTrackY: 17,
  actionTrackHeight: 52,
  actionTrackMinWidth: 180,
  actionTrackInset: 0,
  actionOptionPaddingX: 17,
  actionOptionFontSize: 15.8,
  enforcementTitleWidth: BROWSER_RULES_CONTROL_TITLE_SLOT_WIDTH,
  enforcementTrackMinWidth: 168,
  enforcementTrackInset: 0,
  multiMinHeight: 66,
  multiViewportInset: 7,
  multiTitleY: 11,
  multiTitleHeight: 36,
  titleTrackOverlap: 8,
  multiTrackMinWidth: 260,
  multiTrackInset: 0,
  multiOptionMinWidth: 94,
  multiOptionMaxWidth: 190,
  multiOptionHeight: 44,
  multiOptionGap: 4,
  multiOptionInsetX: 5,
  multiOptionInsetY: 2,
  multiIndicatorRadius: 5.5,
  multiIndicatorStrokeWidth: 1.7,
  multiIndicatorOuterRingOffset: 3,
  multiIndicatorOuterRingStrokeWidth: 1.2,
  multiOptionPaddingX: 13,
  optionFillExtraWidth: 999,
  multiOuterPaddingRight: 4,
  multiOuterPaddingBottom: 4,
  multiOptionFontSize: 13,
  hiddenTitleFontSize: 1,
  optionFontWeight: 850,
  actionIconWidthRatio: 0.94,
  actionIconHeightRatio: 0.9,
  enforcementIconWidthRatio: 0.78,
  enforcementIconHeightRatio: 0.88,
  enforcementIconBottomInset: 2,
} as const;

const BROWSER_RULES_QUESTIONNAIRE_LAYOUT = {
  maximumColumnCount: 2,
  readableColumnMinWidth: 650,
  columnGap: 8,
  rowGap: 8,
  rootInlineReservedWidth: 12,
  bubbleDefaultWidth: 640,
  bubbleMinWidth: 320,
  frameHeaderHeight: defaultChatBubbleConfig.header.height,
  frameCollapsedBodyHeight: defaultChatBubbleConfig.body.collapsedHeight,
  frameVerticalOverflowInset: 10,
  bodyClampWidth: defaultChatBubbleConfig.body.clampWidth,
  bodyContentInsetX: 8,
  bodyContentInsetY: 7,
  controlColumnGap: 8,
  collapseAnimationMs: 220,
  collapseAnimationSettleMs: 40,
} as const;

const BROWSER_RULES_COPY = {
  collapseQuestion: 'Collapse rule question',
  expandQuestion: 'Expand rule question',
  openGuide: 'Open browser rules guide',
  actionTitle: 'A',
  actionIconTitle: 'Action',
  enforcementTitle: 'E',
} as const;

const BROWSER_RULES_RENDER_INNER_SVG_CONTROLS = true;
const BROWSER_RULES_COLLAPSED_HEIGHT_CSS_VAR = '--browser-rules-bubble-collapsed-height';

const BROWSER_RULES_CLASS_NAMES = {
  root: 'browser-rules-questionnaire',
  columns: 'browser-rules-questionnaire__columns',
  column: 'browser-rules-questionnaire__column',
  bubble: 'browser-rules-bubble',
  bubbleCollapsed: 'browser-rules-bubble--collapsed',
  bubbleAnimating: 'browser-rules-bubble--animating',
  bubbleClosing: 'browser-rules-bubble--closing',
  bubbleOpening: 'browser-rules-bubble--opening',
  bubbleSvg: 'browser-rules-bubble__svg',
  bubbleBody: 'browser-rules-bubble__body',
  bubbleBodyClosing: 'browser-rules-bubble__body--closing',
  bubbleBodyOpening: 'browser-rules-bubble__body--opening',
  choicePanel: 'browser-rules-choice-panel',
  choicePanelSingle: 'browser-rules-choice-panel--single',
  choicePanelMulti: 'browser-rules-choice-panel--multi',
  choicePanelEnforcement: 'browser-rules-choice-panel--enforcement',
  svgControl: 'browser-rules-svg-control',
} as const;

const BROWSER_RULES_ENFORCEMENT_OPTIONS = [
  { value: 'observe', label: 'Observe' },
  { value: 'enforce', label: 'Enforce' },
];

type BrowserRulesBubbleMetrics = {
  readonly frameWidth: number;
  readonly frameHeight: number;
  readonly bodyHeight: number;
  readonly controlsInline: boolean;
  readonly choiceControlWidth: number;
  readonly enforcementControlWidth: number;
  readonly enforcementAlignSelf: 'center' | 'flex-start';
};

type BrowserRulesBubbleTransitionPhase = 'idle' | 'expanding' | 'collapsing';

type BrowserRulesQuestionnaireLayout = {
  readonly columnCount: number;
  readonly columnWidth: number;
};

function clampNumber(value: number, min: number, max: number) {
  return Math.max(min, Math.min(max, value));
}

function calculateQuestionnaireLayout(availableWidth: number): BrowserRulesQuestionnaireLayout {
  const safeAvailableWidth = Math.max(1, Math.floor(availableWidth));
  const columnCount = clampNumber(
    Math.floor(
      (safeAvailableWidth + BROWSER_RULES_QUESTIONNAIRE_LAYOUT.columnGap) /
        (BROWSER_RULES_QUESTIONNAIRE_LAYOUT.readableColumnMinWidth + BROWSER_RULES_QUESTIONNAIRE_LAYOUT.columnGap)
    ),
    1,
    BROWSER_RULES_QUESTIONNAIRE_LAYOUT.maximumColumnCount
  );
  const totalGap = BROWSER_RULES_QUESTIONNAIRE_LAYOUT.columnGap * (columnCount - 1);
  const columnWidth = Math.max(1, Math.floor((safeAvailableWidth - totalGap) / columnCount));

  return { columnCount, columnWidth };
}

function stableLayoutWidth(width: number | undefined) {
  const safeWidth = Math.floor(width ?? BROWSER_RULES_QUESTIONNAIRE_LAYOUT.bubbleDefaultWidth);
  return Number.isFinite(safeWidth) && safeWidth > 0
    ? safeWidth
    : BROWSER_RULES_QUESTIONNAIRE_LAYOUT.bubbleDefaultWidth;
}

function stableQuestionnaireContentWidth(availableWidth: number | undefined) {
  return Math.max(1, stableLayoutWidth(availableWidth) - BROWSER_RULES_QUESTIONNAIRE_LAYOUT.rootInlineReservedWidth);
}

function setQuestionnaireLayoutFromWidth(
  setLayout: Dispatch<SetStateAction<BrowserRulesQuestionnaireLayout>>,
  width: number | undefined
) {
  const nextLayout = calculateQuestionnaireLayout(stableQuestionnaireContentWidth(width));
  setLayout((current) =>
    current.columnCount === nextLayout.columnCount && current.columnWidth === nextLayout.columnWidth
      ? current
      : nextLayout
  );
}

function getBrowserRulesControlMinWidth(question: BrowserRulesQuestion) {
  return question.kind === 'multi'
    ? BROWSER_RULES_CONTROL_LAYOUT.multiControlMinWidth
    : BROWSER_RULES_CONTROL_LAYOUT.actionControlMinWidth;
}

function getBrowserRulesActionInitialOptionsPerRow(optionCount: number, width: number) {
  const safeOptionCount = Math.max(1, optionCount);
  if (safeOptionCount <= 2) {
    return safeOptionCount;
  }
  if (width < BROWSER_RULES_CONTROL_LAYOUT.actionThreeColumnMinWidth) {
    return 2;
  }
  if (width < BROWSER_RULES_CONTROL_LAYOUT.actionSingleRowMinWidth) {
    return Math.min(3, safeOptionCount);
  }

  return safeOptionCount;
}

function getBrowserRulesActionRowCount(optionCount: number, optionsPerRow: number) {
  return Math.ceil(Math.max(1, optionCount) / Math.max(1, optionsPerRow));
}

function getBrowserRulesActionHeight(optionCount: number, optionsPerRow: number) {
  const rowCount = getBrowserRulesActionRowCount(optionCount, optionsPerRow);
  return (
    BROWSER_RULES_CONTROL_LAYOUT.actionTrackY +
    rowCount * BROWSER_RULES_CONTROL_LAYOUT.actionTrackHeight +
    Math.max(0, rowCount - 1) * BROWSER_RULES_CONTROL_LAYOUT.actionOptionGapY +
    BROWSER_RULES_CONTROL_LAYOUT.actionBottomPadding
  );
}

function getBrowserRulesActionTitleY(optionCount: number, optionsPerRow: number) {
  const rowCount = getBrowserRulesActionRowCount(optionCount, optionsPerRow);
  const trackHeight =
    rowCount * BROWSER_RULES_CONTROL_LAYOUT.actionTrackHeight +
    Math.max(0, rowCount - 1) * BROWSER_RULES_CONTROL_LAYOUT.actionOptionGapY;

  return (
    BROWSER_RULES_CONTROL_LAYOUT.actionTrackY + (trackHeight - BROWSER_RULES_CONTROL_LAYOUT.actionTitleHeight) * 0.5
  );
}

function resolveBrowserRulesActionOptionsPerRow(width: number, options: ReturnType<typeof scopeChoiceOptions>) {
  const safeWidth = Math.max(BROWSER_RULES_CONTROL_LAYOUT.actionControlMinWidth, width);
  const initialOptionsPerRow = getBrowserRulesActionInitialOptionsPerRow(options.length, safeWidth);

  for (let optionsPerRow = initialOptionsPerRow; optionsPerRow >= 1; optionsPerRow -= 1) {
    const config = mergeScopeToggleConfig(
      defaultScopeToggleConfig,
      browserRulesActionToggleConfigForRows(safeWidth, options.length, optionsPerRow)
    );
    const metrics = calculateScopeToggleMetrics(config, BROWSER_RULES_COPY.actionTitle, options);
    if (metrics.svgWidth <= safeWidth + BROWSER_RULES_CONTROL_LAYOUT.actionViewportInset) {
      return optionsPerRow;
    }
  }

  return 1;
}

function getBrowserRulesMultiChoiceHeight(question: BrowserRulesQuestion, width: number) {
  const options = scopeChoiceOptions(question.options);
  const config = mergeScopeMultiChoiceConfig(
    defaultScopeMultiChoiceConfig,
    browserRulesMultiChoiceConfig(width, options)
  );
  const metrics = calculateScopeMultiChoiceMetrics(
    config,
    BROWSER_RULES_COPY.actionTitle,
    options,
    width,
    true,
    question.multiSelect ?? true
  );

  return metrics.svgHeight;
}

function getBrowserRulesEnforcementChoiceHeight(width: number) {
  const config = mergeScopeMultiChoiceConfig(defaultScopeMultiChoiceConfig, browserRulesEnforcementChoiceConfig(width));
  const metrics = calculateScopeMultiChoiceMetrics(
    config,
    BROWSER_RULES_COPY.enforcementTitle,
    BROWSER_RULES_ENFORCEMENT_OPTIONS,
    width,
    true,
    false
  );

  return metrics.svgHeight;
}

function getBrowserRulesChoiceHeight(question: BrowserRulesQuestion, width: number) {
  if (question.kind === 'multi') {
    return getBrowserRulesMultiChoiceHeight(question, width);
  }

  const options = scopeChoiceOptions(question.options);
  const optionsPerRow = resolveBrowserRulesActionOptionsPerRow(width, options);

  return getBrowserRulesActionHeight(options.length, optionsPerRow);
}

function getBrowserRulesBubbleMetrics(question: BrowserRulesQuestion, width: number): BrowserRulesBubbleMetrics {
  const frameWidth = Math.max(BROWSER_RULES_QUESTIONNAIRE_LAYOUT.bubbleMinWidth, Math.floor(width));
  const bodyContentWidth = Math.max(
    1,
    frameWidth -
      BROWSER_RULES_QUESTIONNAIRE_LAYOUT.bodyClampWidth -
      BROWSER_RULES_QUESTIONNAIRE_LAYOUT.bodyContentInsetX * 2
  );
  const choiceMinWidth = getBrowserRulesControlMinWidth(question);
  const enforcementHidden = question.enforcementHidden === true;
  const inlineMinWidth =
    choiceMinWidth +
    (enforcementHidden
      ? 0
      : BROWSER_RULES_CONTROL_LAYOUT.enforcementControlMinWidth + BROWSER_RULES_QUESTIONNAIRE_LAYOUT.controlColumnGap);
  const controlsInline = bodyContentWidth >= inlineMinWidth;
  const targetEnforcementWidth = Math.floor(
    bodyContentWidth * BROWSER_RULES_CONTROL_LAYOUT.enforcementControlFluidRatio
  );
  const enforcementControlWidth = controlsInline
    ? clampNumber(
        targetEnforcementWidth,
        BROWSER_RULES_CONTROL_LAYOUT.enforcementControlMinWidth,
        BROWSER_RULES_CONTROL_LAYOUT.enforcementControlMaxWidth
      )
    : Math.min(bodyContentWidth, BROWSER_RULES_CONTROL_LAYOUT.enforcementDefaultWidth);
  const choiceControlWidth = enforcementHidden
    ? bodyContentWidth
    : controlsInline
      ? bodyContentWidth - enforcementControlWidth - BROWSER_RULES_QUESTIONNAIRE_LAYOUT.controlColumnGap
      : bodyContentWidth;
  const choiceControlHeight = getBrowserRulesChoiceHeight(question, choiceControlWidth);
  const enforcementControlHeight = getBrowserRulesEnforcementChoiceHeight(enforcementControlWidth);
  const contentHeight = enforcementHidden
    ? choiceControlHeight
    : controlsInline
      ? Math.max(choiceControlHeight, enforcementControlHeight)
      : choiceControlHeight + BROWSER_RULES_QUESTIONNAIRE_LAYOUT.rowGap + enforcementControlHeight;
  const enforcementAlignSelf = 'flex-start';
  const expandedBodyHeight = contentHeight + BROWSER_RULES_QUESTIONNAIRE_LAYOUT.bodyContentInsetY * 2;
  const bodyHeight = question.collapsed
    ? BROWSER_RULES_QUESTIONNAIRE_LAYOUT.frameCollapsedBodyHeight
    : expandedBodyHeight;
  const frameHeight =
    BROWSER_RULES_QUESTIONNAIRE_LAYOUT.frameHeaderHeight +
    bodyHeight +
    BROWSER_RULES_QUESTIONNAIRE_LAYOUT.frameVerticalOverflowInset;

  return {
    frameWidth,
    frameHeight,
    bodyHeight,
    controlsInline,
    choiceControlWidth,
    enforcementControlWidth,
    enforcementAlignSelf,
  };
}

function distributeQuestions(questions: readonly BrowserRulesQuestion[], columnCount: number) {
  const columns = Array.from({ length: Math.max(1, columnCount) }, () => [] as BrowserRulesQuestion[]);

  questions.forEach((question, questionIndex) => {
    const columnIndex = questionIndex % columns.length;
    const column = columns[columnIndex];
    if (!column) return;

    column.push(question);
  });

  return columns;
}

function useQuestionColumnLayout(availableWidth?: number) {
  const ref = useRef<HTMLDivElement | null>(null);
  const [layout, setLayout] = useState<BrowserRulesQuestionnaireLayout>(() =>
    calculateQuestionnaireLayout(stableQuestionnaireContentWidth(availableWidth))
  );

  useLayoutEffect(() => {
    setQuestionnaireLayoutFromWidth(setLayout, availableWidth);
    if (availableWidth !== undefined) return;

    const node = ref.current;
    if (!node || typeof ResizeObserver === 'undefined') return;

    const updateFromNode = () => {
      const rect = node.getBoundingClientRect();
      setQuestionnaireLayoutFromWidth(setLayout, rect.width || node.clientWidth || availableWidth);
    };

    updateFromNode();
    const observer = new ResizeObserver((entries) => {
      const entry = entries[0];
      if (!entry) return;

      setQuestionnaireLayoutFromWidth(setLayout, entry.contentRect.width || node.clientWidth || availableWidth);
    });

    observer.observe(node);
    return () => observer.disconnect();
  }, [availableWidth]);

  return { ref, layout };
}

function useStableWidth(widthHint: number) {
  const [width, setWidth] = useState(() => stableLayoutWidth(widthHint));

  useLayoutEffect(() => {
    const layoutWidth = stableLayoutWidth(widthHint);
    setWidth((current) => (Math.abs(current - layoutWidth) <= 1 ? current : layoutWidth));
  }, [widthHint]);

  return width;
}

function browserRulesActionToggleConfigForRows(width: number, optionCount: number, maxOptionsPerRow: number) {
  const safeWidth = Math.max(BROWSER_RULES_CONTROL_LAYOUT.actionControlMinWidth, width);
  const actionHeight = getBrowserRulesActionHeight(optionCount, maxOptionsPerRow);

  return {
    svg: {
      width: safeWidth,
      height: actionHeight,
      viewportInset: BROWSER_RULES_CONTROL_LAYOUT.actionViewportInset,
    },
    layout: {
      titleAnchorX: 0,
      titleBoxY: getBrowserRulesActionTitleY(optionCount, maxOptionsPerRow),
      titleBoxMinWidth: BROWSER_RULES_CONTROL_LAYOUT.actionTitleWidth,
      titleBoxPaddingX: 0,
      titleBoxHeight: BROWSER_RULES_CONTROL_LAYOUT.actionTitleHeight,
      trackY: BROWSER_RULES_CONTROL_LAYOUT.actionTrackY,
      trackMinWidth: Math.max(
        BROWSER_RULES_CONTROL_LAYOUT.actionTrackMinWidth,
        safeWidth - BROWSER_RULES_CONTROL_LAYOUT.actionTitleWidth - BROWSER_RULES_CONTROL_LAYOUT.actionTrackInset
      ),
      trackHeight: BROWSER_RULES_CONTROL_LAYOUT.actionTrackHeight,
      maxOptionsPerRow,
      optionGapY: BROWSER_RULES_CONTROL_LAYOUT.actionOptionGapY,
      optionPaddingX: BROWSER_RULES_CONTROL_LAYOUT.actionOptionPaddingX,
      outerPaddingRight: 0,
    },
    text: {
      titleFontSize: BROWSER_RULES_CONTROL_LAYOUT.hiddenTitleFontSize,
      optionFontSize: BROWSER_RULES_CONTROL_LAYOUT.actionOptionFontSize,
      optionFontWeight: BROWSER_RULES_CONTROL_LAYOUT.optionFontWeight,
    },
    outerEdge: {
      glowStrokeWidth: BROWSER_RULES_CONTROL_FRAME_GLOW.outerEdgeGlowStrokeWidth,
    },
    titleBox: {
      glowStrokeWidth: BROWSER_RULES_CONTROL_FRAME_GLOW.titleBoxGlowStrokeWidth,
    },
    track: {
      glowStrokeWidth: BROWSER_RULES_CONTROL_FRAME_GLOW.trackGlowStrokeWidth,
    },
    colors: {
      titleBoxGlow: BROWSER_RULES_CONTROL_FRAME_GLOW.titleBoxGlow,
      outerEdgeGlow: BROWSER_RULES_CONTROL_FRAME_GLOW.outerEdgeGlow,
      trackGlow: BROWSER_RULES_CONTROL_FRAME_GLOW.trackGlow,
    },
    opacity: {
      titleGlowIdle: BROWSER_RULES_CONTROL_FRAME_GLOW.titleGlowIdle,
      titleGlowHover: BROWSER_RULES_CONTROL_FRAME_GLOW.titleGlowHover,
      outerGlowIdle: BROWSER_RULES_CONTROL_FRAME_GLOW.outerGlowIdle,
      outerGlowHover: BROWSER_RULES_CONTROL_FRAME_GLOW.outerGlowHover,
      trackGlowIdle: BROWSER_RULES_CONTROL_FRAME_GLOW.trackGlowIdle,
      trackGlowHover: BROWSER_RULES_CONTROL_FRAME_GLOW.trackGlowHover,
    },
  };
}

function browserRulesActionToggleConfig(width: number, options: ReturnType<typeof scopeChoiceOptions>) {
  const safeWidth = Math.max(BROWSER_RULES_CONTROL_LAYOUT.actionControlMinWidth, width);
  const maxOptionsPerRow = resolveBrowserRulesActionOptionsPerRow(safeWidth, options);

  return browserRulesActionToggleConfigForRows(safeWidth, options.length, maxOptionsPerRow);
}

function browserRulesEnforcementChoiceConfig(width: number) {
  const safeWidth = Math.max(BROWSER_RULES_CONTROL_LAYOUT.enforcementControlMinWidth, width);
  const baseConfig = {
    svg: {
      width: safeWidth,
      minHeight: BROWSER_RULES_CONTROL_LAYOUT.multiMinHeight,
      viewportInset: BROWSER_RULES_CONTROL_LAYOUT.multiViewportInset,
    },
    layout: {
      titleBoxX: 0,
      titleBoxY: BROWSER_RULES_CONTROL_LAYOUT.multiTitleY,
      titleBoxMinWidth: BROWSER_RULES_CONTROL_LAYOUT.enforcementTitleWidth,
      titleBoxPaddingX: 0,
      titleBoxHeight: BROWSER_RULES_CONTROL_LAYOUT.multiTitleHeight,
      titleBoxRightRadius: 0,
      titleBoxBottomRadius: defaultScopeMultiChoiceConfig.layout.titleBoxRadius,
      centerTitleBoxOnTrack: true,
      trackX: BROWSER_RULES_CONTROL_LAYOUT.enforcementTitleWidth - BROWSER_RULES_CONTROL_LAYOUT.titleTrackOverlap,
      trackY: BROWSER_RULES_CONTROL_LAYOUT.multiTitleY,
      trackWidth: Math.max(
        BROWSER_RULES_CONTROL_LAYOUT.enforcementTrackMinWidth,
        safeWidth -
          BROWSER_RULES_CONTROL_LAYOUT.enforcementTitleWidth +
          BROWSER_RULES_CONTROL_LAYOUT.titleTrackOverlap -
          BROWSER_RULES_CONTROL_LAYOUT.enforcementTrackInset
      ),
      optionMinWidth: BROWSER_RULES_CONTROL_LAYOUT.enforcementOptionMinWidth,
      optionMaxWidth: BROWSER_RULES_CONTROL_LAYOUT.enforcementOptionMaxWidth,
      optionHeight: BROWSER_RULES_CONTROL_LAYOUT.multiOptionHeight,
      optionGapX: BROWSER_RULES_CONTROL_LAYOUT.multiOptionGap,
      optionGapY: BROWSER_RULES_CONTROL_LAYOUT.multiOptionGap,
      maxExtraWidthPerOption: BROWSER_RULES_CONTROL_LAYOUT.optionFillExtraWidth,
      optionPaddingX: BROWSER_RULES_CONTROL_LAYOUT.enforcementOptionPaddingX,
      outerPaddingRight: 0,
      outerPaddingBottom: BROWSER_RULES_CONTROL_LAYOUT.multiOuterPaddingBottom,
    },
    text: {
      titleFontSize: BROWSER_RULES_CONTROL_LAYOUT.hiddenTitleFontSize,
      optionFontSize: BROWSER_RULES_CONTROL_LAYOUT.multiOptionFontSize,
      optionFontWeight: BROWSER_RULES_CONTROL_LAYOUT.optionFontWeight,
    },
    outerEdge: {
      glowStrokeWidth: BROWSER_RULES_CONTROL_FRAME_GLOW.outerEdgeGlowStrokeWidth,
    },
    titleBox: {
      glowStrokeWidth: BROWSER_RULES_CONTROL_FRAME_GLOW.titleBoxGlowStrokeWidth,
    },
    track: {
      glowStrokeWidth: BROWSER_RULES_CONTROL_FRAME_GLOW.trackGlowStrokeWidth,
    },
    optionButton: {
      inset: BROWSER_RULES_CONTROL_LAYOUT.multiOptionInsetY,
      insetX: BROWSER_RULES_CONTROL_LAYOUT.multiOptionInsetX,
      insetY: BROWSER_RULES_CONTROL_LAYOUT.multiOptionInsetY,
    },
    indicator: {
      circleRadius: BROWSER_RULES_CONTROL_LAYOUT.multiIndicatorRadius,
      circleStrokeWidth: BROWSER_RULES_CONTROL_LAYOUT.multiIndicatorStrokeWidth,
      outerRingRadiusOffset: BROWSER_RULES_CONTROL_LAYOUT.multiIndicatorOuterRingOffset,
      outerRingStrokeWidth: BROWSER_RULES_CONTROL_LAYOUT.multiIndicatorOuterRingStrokeWidth,
    },
    colors: {
      titleBoxGlow: BROWSER_RULES_CONTROL_FRAME_GLOW.titleBoxGlow,
      outerEdgeGlow: BROWSER_RULES_CONTROL_FRAME_GLOW.outerEdgeGlow,
      trackGlow: BROWSER_RULES_CONTROL_FRAME_GLOW.trackGlow,
    },
    opacity: {
      titleGlowIdle: BROWSER_RULES_CONTROL_FRAME_GLOW.titleGlowIdle,
      titleGlowHover: BROWSER_RULES_CONTROL_FRAME_GLOW.titleGlowHover,
      outerGlowIdle: BROWSER_RULES_CONTROL_FRAME_GLOW.outerGlowIdle,
      outerGlowHover: BROWSER_RULES_CONTROL_FRAME_GLOW.outerGlowHover,
      trackGlowIdle: BROWSER_RULES_CONTROL_FRAME_GLOW.trackGlowIdle,
      trackGlowHover: BROWSER_RULES_CONTROL_FRAME_GLOW.trackGlowHover,
    },
  };
  return baseConfig;
}

function browserRulesMultiChoiceConfig(width: number, options: ReturnType<typeof scopeChoiceOptions>) {
  const safeWidth = Math.max(BROWSER_RULES_CONTROL_LAYOUT.multiControlMinWidth, width);
  const baseConfig = {
    svg: {
      width: safeWidth,
      minHeight: BROWSER_RULES_CONTROL_LAYOUT.multiMinHeight,
      viewportInset: BROWSER_RULES_CONTROL_LAYOUT.multiViewportInset,
    },
    layout: {
      titleBoxX: 0,
      titleBoxY: BROWSER_RULES_CONTROL_LAYOUT.multiTitleY,
      titleBoxMinWidth: BROWSER_RULES_CONTROL_LAYOUT.actionTitleWidth,
      titleBoxPaddingX: 0,
      titleBoxHeight: BROWSER_RULES_CONTROL_LAYOUT.multiTitleHeight,
      titleBoxRightRadius: 0,
      titleBoxBottomRadius: defaultScopeMultiChoiceConfig.layout.titleBoxRadius,
      centerTitleBoxOnTrack: true,
      trackX: BROWSER_RULES_CONTROL_LAYOUT.actionTitleWidth - BROWSER_RULES_CONTROL_LAYOUT.titleTrackOverlap,
      trackY: BROWSER_RULES_CONTROL_LAYOUT.multiTitleY,
      trackWidth: Math.max(
        BROWSER_RULES_CONTROL_LAYOUT.multiTrackMinWidth,
        safeWidth -
          BROWSER_RULES_CONTROL_LAYOUT.actionTitleWidth +
          BROWSER_RULES_CONTROL_LAYOUT.titleTrackOverlap -
          BROWSER_RULES_CONTROL_LAYOUT.multiTrackInset
      ),
      optionMinWidth: BROWSER_RULES_CONTROL_LAYOUT.multiOptionMinWidth,
      optionMaxWidth: BROWSER_RULES_CONTROL_LAYOUT.multiOptionMaxWidth,
      optionHeight: BROWSER_RULES_CONTROL_LAYOUT.multiOptionHeight,
      optionGapX: BROWSER_RULES_CONTROL_LAYOUT.multiOptionGap,
      optionGapY: BROWSER_RULES_CONTROL_LAYOUT.multiOptionGap,
      maxExtraWidthPerOption: BROWSER_RULES_CONTROL_LAYOUT.optionFillExtraWidth,
      optionPaddingX: BROWSER_RULES_CONTROL_LAYOUT.multiOptionPaddingX,
      outerPaddingRight: BROWSER_RULES_CONTROL_LAYOUT.multiOuterPaddingRight,
      outerPaddingBottom: BROWSER_RULES_CONTROL_LAYOUT.multiOuterPaddingBottom,
    },
    text: {
      titleFontSize: BROWSER_RULES_CONTROL_LAYOUT.hiddenTitleFontSize,
      optionFontSize: BROWSER_RULES_CONTROL_LAYOUT.multiOptionFontSize,
      optionFontWeight: BROWSER_RULES_CONTROL_LAYOUT.optionFontWeight,
    },
    outerEdge: {
      glowStrokeWidth: BROWSER_RULES_CONTROL_FRAME_GLOW.outerEdgeGlowStrokeWidth,
    },
    titleBox: {
      glowStrokeWidth: BROWSER_RULES_CONTROL_FRAME_GLOW.titleBoxGlowStrokeWidth,
    },
    track: {
      glowStrokeWidth: BROWSER_RULES_CONTROL_FRAME_GLOW.trackGlowStrokeWidth,
    },
    optionButton: {
      inset: BROWSER_RULES_CONTROL_LAYOUT.multiOptionInsetY,
      insetX: BROWSER_RULES_CONTROL_LAYOUT.multiOptionInsetX,
      insetY: BROWSER_RULES_CONTROL_LAYOUT.multiOptionInsetY,
    },
    indicator: {
      circleRadius: BROWSER_RULES_CONTROL_LAYOUT.multiIndicatorRadius,
      circleStrokeWidth: BROWSER_RULES_CONTROL_LAYOUT.multiIndicatorStrokeWidth,
      outerRingRadiusOffset: BROWSER_RULES_CONTROL_LAYOUT.multiIndicatorOuterRingOffset,
      outerRingStrokeWidth: BROWSER_RULES_CONTROL_LAYOUT.multiIndicatorOuterRingStrokeWidth,
    },
    colors: {
      titleBoxGlow: BROWSER_RULES_CONTROL_FRAME_GLOW.titleBoxGlow,
      outerEdgeGlow: BROWSER_RULES_CONTROL_FRAME_GLOW.outerEdgeGlow,
      trackGlow: BROWSER_RULES_CONTROL_FRAME_GLOW.trackGlow,
    },
    opacity: {
      titleGlowIdle: BROWSER_RULES_CONTROL_FRAME_GLOW.titleGlowIdle,
      titleGlowHover: BROWSER_RULES_CONTROL_FRAME_GLOW.titleGlowHover,
      outerGlowIdle: BROWSER_RULES_CONTROL_FRAME_GLOW.outerGlowIdle,
      outerGlowHover: BROWSER_RULES_CONTROL_FRAME_GLOW.outerGlowHover,
      trackGlowIdle: BROWSER_RULES_CONTROL_FRAME_GLOW.trackGlowIdle,
      trackGlowHover: BROWSER_RULES_CONTROL_FRAME_GLOW.trackGlowHover,
    },
  };
  return baseConfig;
}

function scopeChoiceOptions(options: readonly BrowserRulesChoiceOption[]) {
  return options.map((option) =>
    option.disabled === undefined
      ? { value: option.value, label: option.label }
      : { value: option.value, label: option.label, disabled: option.disabled }
  );
}

function renderActionTitle(slot: {
  readonly x: number;
  readonly y: number;
  readonly width: number;
  readonly height: number;
}) {
  const iconWidth = slot.width * BROWSER_RULES_CONTROL_LAYOUT.actionIconWidthRatio;
  const iconHeight = slot.height * BROWSER_RULES_CONTROL_LAYOUT.actionIconHeightRatio;

  return (
    <Action
      x={slot.x + slot.width * 0.5 - iconWidth * 0.5}
      y={slot.y + slot.height * 0.5 - iconHeight * 0.5}
      width={iconWidth}
      height={iconHeight}
      title={BROWSER_RULES_COPY.actionIconTitle}
      preserveAspectRatio="xMidYMid meet"
    />
  );
}

function renderEnforcementTitle(slot: {
  readonly x: number;
  readonly y: number;
  readonly width: number;
  readonly height: number;
}) {
  const iconWidth = slot.width * BROWSER_RULES_CONTROL_LAYOUT.enforcementIconWidthRatio;
  const iconHeight = slot.height * BROWSER_RULES_CONTROL_LAYOUT.enforcementIconHeightRatio;

  return (
    <EnforcementOfficerIcon
      x={slot.x + slot.width * 0.5 - iconWidth * 0.5}
      y={slot.y + slot.height - iconHeight - BROWSER_RULES_CONTROL_LAYOUT.enforcementIconBottomInset}
      width={iconWidth}
      height={iconHeight}
    />
  );
}

export function BrowserRulesQuestionnaire({
  availableWidth,
  questions,
  disabled = false,
  onInfoClick,
}: BrowserRulesQuestionnaireProps): ReactElement {
  const { ref, layout } = useQuestionColumnLayout(availableWidth);
  const columns = useMemo(() => distributeQuestions(questions, layout.columnCount), [layout.columnCount, questions]);

  return (
    <div
      ref={ref}
      className={BROWSER_RULES_CLASS_NAMES.root}
      onClick={(event) => event.stopPropagation()}
      onPointerDown={(event) => event.stopPropagation()}
    >
      <div
        className={BROWSER_RULES_CLASS_NAMES.columns}
        style={{
          gap: BROWSER_RULES_QUESTIONNAIRE_LAYOUT.columnGap,
          gridTemplateColumns: `repeat(${columns.length}, minmax(0, 1fr))`,
        }}
      >
        {columns.map((columnQuestions, columnIndex) => (
          <div
            className={BROWSER_RULES_CLASS_NAMES.column}
            key={`browser-rule-column:${columnIndex}`}
            style={{ gap: BROWSER_RULES_QUESTIONNAIRE_LAYOUT.rowGap }}
          >
            {columnQuestions.map((question) => (
              <BrowserRulesQuestionBubble
                disabled={disabled || question.disabled === true}
                key={question.id}
                question={question}
                widthHint={layout.columnWidth}
                {...(onInfoClick ? { onInfoClick } : {})}
              />
            ))}
          </div>
        ))}
      </div>
    </div>
  );
}

function BrowserRulesQuestionBubble({
  question,
  disabled,
  onInfoClick,
  widthHint,
}: {
  readonly question: BrowserRulesQuestion;
  readonly disabled: boolean;
  readonly onInfoClick?: () => void;
  readonly widthHint: number;
}) {
  const width = useStableWidth(widthHint);
  const [heightCollapsed, setHeightCollapsed] = useState(question.collapsed);
  const [visualCollapsed, setVisualCollapsed] = useState(question.collapsed);
  const [transitionPhase, setTransitionPhase] = useState<BrowserRulesBubbleTransitionPhase>('idle');
  const transitionTimerRef = useRef<number | undefined>(undefined);
  const transitionFrameRef = useRef<number | undefined>(undefined);
  const transitionLockRef = useRef(false);
  const previousCollapsedRef = useRef(question.collapsed);
  const expandedMetrics = getBrowserRulesBubbleMetrics({ ...question, collapsed: false }, width);
  const collapsedMetrics = getBrowserRulesBubbleMetrics({ ...question, collapsed: true }, width);
  const renderedVisualCollapsed = visualCollapsed;
  const targetMetrics = heightCollapsed ? collapsedMetrics : expandedMetrics;
  const visualMetrics = renderedVisualCollapsed ? collapsedMetrics : expandedMetrics;
  const bubbleIsClosing = transitionPhase === 'collapsing';
  const bubbleIsOpening = transitionPhase === 'expanding';
  const bubbleIsAnimating = transitionPhase !== 'idle';
  const bubbleClassName = [
    BROWSER_RULES_CLASS_NAMES.bubble,
    question.collapsed ? BROWSER_RULES_CLASS_NAMES.bubbleCollapsed : '',
    bubbleIsAnimating ? BROWSER_RULES_CLASS_NAMES.bubbleAnimating : '',
    bubbleIsClosing ? BROWSER_RULES_CLASS_NAMES.bubbleClosing : '',
    bubbleIsOpening ? BROWSER_RULES_CLASS_NAMES.bubbleOpening : '',
  ]
    .filter(Boolean)
    .join(' ');
  const bodyClassName = [
    BROWSER_RULES_CLASS_NAMES.bubbleBody,
    bubbleIsClosing ? BROWSER_RULES_CLASS_NAMES.bubbleBodyClosing : '',
    bubbleIsOpening ? BROWSER_RULES_CLASS_NAMES.bubbleBodyOpening : '',
  ]
    .filter(Boolean)
    .join(' ');
  const bubbleStyle: CSSProperties & Record<typeof BROWSER_RULES_COLLAPSED_HEIGHT_CSS_VAR, string> = {
    [BROWSER_RULES_COLLAPSED_HEIGHT_CSS_VAR]: `${collapsedMetrics.frameHeight}px`,
    animationDuration: `${BROWSER_RULES_QUESTIONNAIRE_LAYOUT.collapseAnimationMs}ms`,
    height: targetMetrics.frameHeight,
    transitionDuration: `${BROWSER_RULES_QUESTIONNAIRE_LAYOUT.collapseAnimationMs}ms`,
  };

  useLayoutEffect(() => {
    const previousCollapsed = previousCollapsedRef.current;
    previousCollapsedRef.current = question.collapsed;

    if (transitionTimerRef.current !== undefined) {
      window.clearTimeout(transitionTimerRef.current);
      transitionTimerRef.current = undefined;
    }
    if (transitionFrameRef.current !== undefined) {
      window.cancelAnimationFrame(transitionFrameRef.current);
      transitionFrameRef.current = undefined;
    }

    if (previousCollapsed === question.collapsed) {
      setHeightCollapsed(question.collapsed);
      setVisualCollapsed(question.collapsed);
      setTransitionPhase('idle');
      transitionLockRef.current = false;
      return undefined;
    }

    const nextPhase: BrowserRulesBubbleTransitionPhase = question.collapsed ? 'collapsing' : 'expanding';
    setTransitionPhase(nextPhase);
    setHeightCollapsed(true);

    if (question.collapsed) {
      setVisualCollapsed(false);
    } else {
      setVisualCollapsed(true);
      transitionFrameRef.current = window.requestAnimationFrame(() => {
        transitionFrameRef.current = window.requestAnimationFrame(() => {
          transitionFrameRef.current = undefined;
          setVisualCollapsed(false);
          setHeightCollapsed(false);
        });
      });
    }

    transitionTimerRef.current = window.setTimeout(() => {
      transitionTimerRef.current = undefined;
      setHeightCollapsed(question.collapsed);
      setVisualCollapsed(question.collapsed);
      setTransitionPhase('idle');
      transitionLockRef.current = false;
    }, BROWSER_RULES_QUESTIONNAIRE_LAYOUT.collapseAnimationMs + BROWSER_RULES_QUESTIONNAIRE_LAYOUT.collapseAnimationSettleMs);

    return () => {
      if (transitionTimerRef.current !== undefined) {
        window.clearTimeout(transitionTimerRef.current);
        transitionTimerRef.current = undefined;
      }
      if (transitionFrameRef.current !== undefined) {
        window.cancelAnimationFrame(transitionFrameRef.current);
        transitionFrameRef.current = undefined;
      }
    };
  }, [question.collapsed]);

  useEffect(() => {
    return () => {
      if (transitionTimerRef.current !== undefined) {
        window.clearTimeout(transitionTimerRef.current);
      }
      if (transitionFrameRef.current !== undefined) {
        window.cancelAnimationFrame(transitionFrameRef.current);
      }
    };
  }, []);

  function handleBubbleTransitionEnd(event: TransitionEvent<HTMLDivElement>) {
    if (event.currentTarget !== event.target || event.propertyName !== 'height') return;
    if (transitionPhase === 'expanding') return;
    if (transitionTimerRef.current !== undefined) {
      window.clearTimeout(transitionTimerRef.current);
      transitionTimerRef.current = undefined;
    }
    if (transitionFrameRef.current !== undefined) {
      window.cancelAnimationFrame(transitionFrameRef.current);
      transitionFrameRef.current = undefined;
    }
    setHeightCollapsed(question.collapsed);
    setVisualCollapsed(question.collapsed);
    setTransitionPhase('idle');
    transitionLockRef.current = false;
  }

  function handleCollapsedChange(nextCollapsed: boolean) {
    if (transitionLockRef.current || transitionPhase !== 'idle' || nextCollapsed === question.collapsed) {
      return;
    }

    transitionLockRef.current = true;
    question.onCollapsedChange(nextCollapsed);
  }

  return (
    <div
      className={bubbleClassName}
      style={bubbleStyle}
      role="group"
      aria-label={question.header}
      onTransitionEnd={handleBubbleTransitionEnd}
    >
      <svg
        className={BROWSER_RULES_CLASS_NAMES.bubbleSvg}
        viewBox={`0 0 ${visualMetrics.frameWidth} ${visualMetrics.frameHeight}`}
        width="100%"
        height={visualMetrics.frameHeight}
        role="presentation"
      >
        <RulesBubbleSvgFrame
          x={0}
          y={0}
          width={visualMetrics.frameWidth}
          bodyHeight={visualMetrics.bodyHeight}
          variant="incoming"
          collapsed={question.collapsed}
          visualCollapsed={renderedVisualCollapsed}
          headerLabel={question.header}
          showInfo
          infoLabel={BROWSER_RULES_COPY.openGuide}
          disabled={disabled}
          collapseDisabled={bubbleIsAnimating}
          collapseLabel={BROWSER_RULES_COPY.collapseQuestion}
          expandLabel={BROWSER_RULES_COPY.expandQuestion}
          {...(onInfoClick ? { onInfoClick } : {})}
          onCollapsedChange={handleCollapsedChange}
        >
          {(slot) =>
            renderedVisualCollapsed ? null : (
              <foreignObject
                x={slot.bodyContentX + BROWSER_RULES_QUESTIONNAIRE_LAYOUT.bodyContentInsetX}
                y={slot.bodyContentY + BROWSER_RULES_QUESTIONNAIRE_LAYOUT.bodyContentInsetY}
                width={Math.max(1, slot.bodyContentW - BROWSER_RULES_QUESTIONNAIRE_LAYOUT.bodyContentInsetX * 2)}
                height={Math.max(1, slot.bodyContentH - BROWSER_RULES_QUESTIONNAIRE_LAYOUT.bodyContentInsetY * 2)}
              >
                <div
                  className={bodyClassName}
                  style={{
                    gap: visualMetrics.controlsInline
                      ? BROWSER_RULES_QUESTIONNAIRE_LAYOUT.controlColumnGap
                      : BROWSER_RULES_QUESTIONNAIRE_LAYOUT.rowGap,
                    flexDirection: visualMetrics.controlsInline ? 'row' : 'column',
                    animationDuration: `${BROWSER_RULES_QUESTIONNAIRE_LAYOUT.collapseAnimationMs}ms`,
                    transitionDuration: `${BROWSER_RULES_QUESTIONNAIRE_LAYOUT.collapseAnimationMs}ms`,
                  }}
                >
                  <ChoicePanel
                    controlWidth={visualMetrics.choiceControlWidth}
                    disabled={disabled}
                    question={question}
                  />
                  {question.enforcementHidden === true ? null : (
                    <EnforcementPanel
                      alignSelf={visualMetrics.enforcementAlignSelf}
                      controlWidth={visualMetrics.enforcementControlWidth}
                      disabled={disabled || question.enforcementDisabled === true}
                      question={question}
                    />
                  )}
                </div>
              </foreignObject>
            )
          }
        </RulesBubbleSvgFrame>
      </svg>
    </div>
  );
}

function ChoicePanel({
  question,
  disabled,
  controlWidth,
}: {
  readonly question: BrowserRulesQuestion;
  readonly disabled: boolean;
  readonly controlWidth: number;
}) {
  const options = useMemo(() => scopeChoiceOptions(question.options), [question.options]);
  if (!BROWSER_RULES_RENDER_INNER_SVG_CONTROLS) {
    return (
      <div
        className={`${BROWSER_RULES_CLASS_NAMES.choicePanel} ${
          question.kind === 'multi'
            ? BROWSER_RULES_CLASS_NAMES.choicePanelMulti
            : BROWSER_RULES_CLASS_NAMES.choicePanelSingle
        }`}
        style={{ width: controlWidth }}
      />
    );
  }

  return (
    <div
      className={`${BROWSER_RULES_CLASS_NAMES.choicePanel} ${
        question.kind === 'multi'
          ? BROWSER_RULES_CLASS_NAMES.choicePanelMulti
          : BROWSER_RULES_CLASS_NAMES.choicePanelSingle
      }`}
      style={{ width: controlWidth }}
    >
      {question.kind === 'multi' ? (
        <ScopeMultiChoice
          className={BROWSER_RULES_CLASS_NAMES.svgControl}
          disabled={disabled}
          multiSelect={question.multiSelect ?? true}
          options={options}
          selected={question.selected ?? []}
          title={BROWSER_RULES_COPY.actionTitle}
          titleRenderer={renderActionTitle}
          width={Math.max(BROWSER_RULES_CONTROL_LAYOUT.multiControlMinWidth, controlWidth)}
          config={browserRulesMultiChoiceConfig(controlWidth, options)}
          onChange={(nextSelected) => question.onMultiChange?.(nextSelected)}
        />
      ) : (
        <ScopeToggle
          className={BROWSER_RULES_CLASS_NAMES.svgControl}
          disabled={disabled}
          options={options}
          title={BROWSER_RULES_COPY.actionTitle}
          titleRenderer={renderActionTitle}
          {...(question.value === undefined ? {} : { value: question.value })}
          config={browserRulesActionToggleConfig(controlWidth, options)}
          onChange={(nextValue) => question.onSingleChange?.(nextValue)}
        />
      )}
    </div>
  );
}

function EnforcementPanel({
  question,
  disabled,
  controlWidth,
  alignSelf,
}: {
  readonly question: BrowserRulesQuestion;
  readonly disabled: boolean;
  readonly controlWidth: number;
  readonly alignSelf: 'center' | 'flex-start';
}) {
  const selectedEnforcement = useMemo(() => [question.enforcementValue], [question.enforcementValue]);
  if (!BROWSER_RULES_RENDER_INNER_SVG_CONTROLS) {
    return (
      <div
        className={`${BROWSER_RULES_CLASS_NAMES.choicePanel} ${BROWSER_RULES_CLASS_NAMES.choicePanelEnforcement}`}
        style={{ alignSelf, width: controlWidth }}
      />
    );
  }

  return (
    <div
      className={`${BROWSER_RULES_CLASS_NAMES.choicePanel} ${BROWSER_RULES_CLASS_NAMES.choicePanelEnforcement}`}
      style={{ alignSelf, width: controlWidth }}
    >
      <ScopeMultiChoice
        className={BROWSER_RULES_CLASS_NAMES.svgControl}
        disabled={disabled}
        multiSelect={false}
        options={BROWSER_RULES_ENFORCEMENT_OPTIONS}
        selected={selectedEnforcement}
        title={BROWSER_RULES_COPY.enforcementTitle}
        titleRenderer={renderEnforcementTitle}
        width={Math.max(BROWSER_RULES_CONTROL_LAYOUT.enforcementControlMinWidth, controlWidth)}
        config={browserRulesEnforcementChoiceConfig(controlWidth)}
        onChange={(nextSelected) => {
          const nextValue = nextSelected[0];
          if (nextValue) {
            question.onEnforcementChange(nextValue);
          }
        }}
      />
    </div>
  );
}
