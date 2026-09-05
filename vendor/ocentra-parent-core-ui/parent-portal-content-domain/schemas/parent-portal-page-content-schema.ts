import { schema, type Infer } from '@ocentra-parent/vendor-schema/effect-builder';

export const ParentPortalToneSchema = schema.enum(['cyan', 'gold', 'purple', 'red', 'muted']);
export type ParentPortalTone = Infer<typeof ParentPortalToneSchema>;

export const ParentPortalTabIdSchema = schema.enum(['overall', 'controls', 'aiStatus', 'routines', 'support']);
export type ParentPortalTabId = Infer<typeof ParentPortalTabIdSchema>;

export const ParentPortalModeSchema = schema.enum(['parentOverview', 'parentManage', 'parentGuide']);
export type ParentPortalMode = Infer<typeof ParentPortalModeSchema>;

export const ParentPortalIconNameSchema = schema.enum([
  'quick-glance',
  'overview',
  'start',
  'guide',
  'manage',
  'policy',
  'browser',
  'web',
  'schedule',
  'alerts',
  'chat',
  'report',
  'rules',
  'updates',
  'activity',
  'app',
  'games',
  'portal',
  'privacy',
  'lan',
  'devices',
  'screen',
  'remote',
  'ai-setup',
  'ai-guide',
  'ai-memory-set',
  'api',
  'export',
  'drives',
  'audit',
  'ai-memory',
  'account',
  'enforcement',
]);
export type ParentPortalIconName = Infer<typeof ParentPortalIconNameSchema>;

const ParentPortalTabSchema = schema
  .object({
    id: ParentPortalTabIdSchema,
    label: schema.string().min(1),
    title: schema.string().min(1),
  })
  .strict();
export type ParentPortalTab = Infer<typeof ParentPortalTabSchema>;

const ParentPortalNavItemSchema = schema
  .object({
    label: schema.string().min(1),
    detail: schema.string(),
    icon: ParentPortalIconNameSchema,
    tabId: ParentPortalTabIdSchema,
    groupId: schema.string().min(1).optional(),
    sectionLabel: schema.string().min(1).optional(),
    tone: ParentPortalToneSchema.optional(),
    routePath: schema.string().optional(),
  })
  .strict();
export type ParentPortalNavItem = Infer<typeof ParentPortalNavItemSchema>;

const ParentPortalNavGroupSchema = schema
  .object({
    id: schema.string().min(1),
    label: schema.string().min(1),
    detail: schema.string(),
  })
  .strict();
export type ParentPortalNavGroup = Infer<typeof ParentPortalNavGroupSchema>;

const ParentPortalTabDetailSchema = schema
  .object({
    eyebrow: schema.string(),
    title: schema.string().min(1),
    summary: schema.string(),
    primary: schema.string(),
    secondary: schema.string(),
    action: schema.string(),
    tone: ParentPortalToneSchema,
  })
  .strict();
export type ParentPortalTabDetail = Infer<typeof ParentPortalTabDetailSchema>;

const ParentPortalControlAreaSchema = schema
  .object({
    id: schema.string().min(1),
    order: schema.number(),
    name: schema.string().min(1),
    matches: schema.string(),
    growth: schema.string(),
    tone: ParentPortalToneSchema,
    category: schema.string().optional(),
    subcategory: schema.string().nullable().optional(),
    controlCode: schema.number().optional(),
    routePath: schema.string().optional(),
  })
  .strict();
export type ParentPortalControlArea = Infer<typeof ParentPortalControlAreaSchema>;

const ParentPortalQuickControlSchema = schema
  .object({
    id: schema.string().min(1),
    name: schema.string().min(1),
    detail: schema.string(),
    icon: ParentPortalIconNameSchema,
    tone: ParentPortalToneSchema,
    category: schema.string().optional(),
    subcategory: schema.string().nullable().optional(),
    controlCode: schema.number().optional(),
    routePath: schema.string().optional(),
  })
  .strict();
export type ParentPortalQuickControl = Infer<typeof ParentPortalQuickControlSchema>;

const ParentPortalGuidePageSchema = schema
  .object({
    eyebrow: schema.string(),
    title: schema.string().min(1),
    body: schema.string(),
    steps: schema.array(schema.string()),
  })
  .strict();
export type ParentPortalGuidePage = Infer<typeof ParentPortalGuidePageSchema>;

const ParentPortalGuideNoteSchema = schema
  .object({
    label: schema.string().min(1),
    body: schema.string(),
    tone: ParentPortalToneSchema,
    targetPage: schema.number().optional(),
    targetTopicId: schema.string().optional(),
    targetNavLabel: schema.string().optional(),
    targetRoutePath: schema.string().optional(),
  })
  .strict();
export type ParentPortalGuideNote = Infer<typeof ParentPortalGuideNoteSchema>;

const ParentPortalGuideTopicSchema = schema
  .object({
    id: schema.string().min(1),
    navLabel: schema.string().min(1),
    rank: schema.number(),
    title: schema.string().min(1),
    subtitle: schema.string(),
    detail: schema.string(),
    tone: ParentPortalToneSchema,
    category: schema.string(),
    subcategory: schema.string(),
    pages: schema.array(ParentPortalGuidePageSchema),
    tips: schema.array(ParentPortalGuideNoteSchema),
    actions: schema.array(ParentPortalGuideNoteSchema),
  })
  .strict();
export type ParentPortalGuideTopic = Infer<typeof ParentPortalGuideTopicSchema>;

const ParentPortalRowSchema = schema
  .object({
    label: schema.string().min(1),
    order: schema.number(),
    signalScore: schema.number(),
    readyCount: schema.number().optional(),
    gapCount: schema.number().optional(),
    primaryArea: schema.string().optional(),
    trend: schema.string().optional(),
    tone: ParentPortalToneSchema.optional(),
  })
  .strict();
export type ParentPortalContentRow = Infer<typeof ParentPortalRowSchema>;

const ParentPortalSeasonStatSchema = schema
  .object({
    label: schema.string(),
    value: schema.string(),
  })
  .strict();

const ParentPortalSeasonSchema = schema
  .object({
    label: schema.string(),
    title: schema.string(),
    dateRange: schema.string(),
    actionLabel: schema.string(),
    detailTitle: schema.string(),
    detailSubtitle: schema.string(),
    stats: schema.array(ParentPortalSeasonStatSchema),
  })
  .strict();
export type ParentPortalSeason = Infer<typeof ParentPortalSeasonSchema>;

const ParentPortalModeContentSchema = schema
  .object({
    defaultTab: ParentPortalTabIdSchema,
    selectedControlId: schema.string().optional(),
    title: schema.string(),
    routeLabel: schema.string(),
    rowSource: schema.enum(['api', 'fallbackRows', 'aiBenchmarkRows']),
  })
  .strict();
export type ParentPortalModeContent = Infer<typeof ParentPortalModeContentSchema>;

const ParentPortalMetricLabelsSchema = schema
  .object({
    controlAreas: schema.string(),
    devices: schema.string(),
    readyPaths: schema.string(),
    events: schema.string(),
    season: schema.string(),
    updated: schema.string(),
  })
  .strict();

const ParentPortalUiCopySchema = schema
  .object({
    hubTitle: schema.string(),
    controlAreasTitle: schema.string(),
    distributionTitle: schema.string(),
    distributionCenterLabel: schema.string(),
    feedTitle: schema.string(),
    liveLabel: schema.string(),
    viewAllLabel: schema.string(),
    refreshLabel: schema.string(),
    queueLabel: schema.string(),
    showLabel: schema.string(),
    pageLabel: schema.string(),
    selectedRowLabel: schema.string(),
    detailSnapshotTitle: schema.string(),
    detailSnapshotLines: schema.array(schema.string()),
    loadingTitle: schema.string(),
    loadingBody: schema.string(),
    errorTitle: schema.string(),
  })
  .strict();

export const ParentPortalContentDataSchema = schema
  .object({
    tabs: schema.array(ParentPortalTabSchema),
    navGroups: schema.array(ParentPortalNavGroupSchema),
    navItems: schema.array(ParentPortalNavItemSchema),
    tabDetails: schema
      .object({
        overall: ParentPortalTabDetailSchema,
        controls: ParentPortalTabDetailSchema,
        aiStatus: ParentPortalTabDetailSchema,
        routines: ParentPortalTabDetailSchema,
        support: ParentPortalTabDetailSchema,
      })
      .strict(),
    controlAreas: schema.array(ParentPortalControlAreaSchema),
    quickControls: schema.array(ParentPortalQuickControlSchema),
    guideTopics: schema.array(ParentPortalGuideTopicSchema),
    fallbackRows: schema.array(ParentPortalRowSchema),
    aiBenchmarkRows: schema.array(ParentPortalRowSchema),
    distributionLabels: schema.array(schema.string()),
    season: ParentPortalSeasonSchema,
    metricLabels: ParentPortalMetricLabelsSchema,
    uiCopy: ParentPortalUiCopySchema,
    modes: schema
      .object({
        parentOverview: ParentPortalModeContentSchema,
        parentManage: ParentPortalModeContentSchema,
        parentGuide: ParentPortalModeContentSchema,
      })
      .strict(),
  })
  .strict();
export type ParentPortalContentData = Infer<typeof ParentPortalContentDataSchema>;

export const PartialParentPortalContentDataSchema = ParentPortalContentDataSchema.partial();
export type PartialParentPortalContentData = Infer<typeof PartialParentPortalContentDataSchema>;
