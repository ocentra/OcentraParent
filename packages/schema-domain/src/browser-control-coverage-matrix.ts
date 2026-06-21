import {
  type Infer,
  Schema,
  withParser,
  NonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import {
  PolicyCompilerCapabilityState,
  PolicyCompilerCapabilityStateSchema,
} from './policy-compiler';

import { BrowserControlFieldIdSchema } from './browser-control-identifiers';
import {
  BrowserControlCapabilityStateSchema,
  BrowserControlSchemaKnownWritesToPathSchema,
  BrowserControlWritesToPath,
  type BrowserControlSchemaKnownWritesToPath,
} from '@ocentra-parent/schema-domain/browser-control-values';

export const BrowserControlCoverageKindSchema = withParser(Schema.Literal('candidate-mvp', 'catalog-section'));

export const BrowserControlCoverageStatusSchema = withParser(
  Schema.Literal(
    'implemented-manifest-control',
    'represented-through-nested-rule',
    'represented-through-capability',
    'manual-required',
    'unavailable',
    'future-gap',
    'documentation-only'
  )
);

export const BrowserControlCoverageEntrySchema = withParser(
  Schema.Struct({
    coverageKind: BrowserControlCoverageKindSchema,
    catalogSection: NonEmptyStringSchema,
    catalogItem: NonEmptyStringSchema,
    coverageStatus: BrowserControlCoverageStatusSchema,
    manifestFieldIds: Schema.Array(BrowserControlFieldIdSchema),
    writesTo: Schema.Array(BrowserControlSchemaKnownWritesToPathSchema),
    policyShape: Schema.Union(NonEmptyStringSchema, Schema.Null),
    capabilityState: Schema.Union(BrowserControlCapabilityStateSchema, Schema.Null),
    compilerCapabilityState: PolicyCompilerCapabilityStateSchema,
    notes: NonEmptyStringSchema,
  })
);

export const BrowserControlCoverageMatrixSchema = withParser(Schema.Array(BrowserControlCoverageEntrySchema));

export const BrowserControlCatalogMajorSections = [
  'How To Read This Catalog',
  'Global Rule Dimensions',
  'Master Browser Control Settings',
  'Browser Discovery Settings',
  'Browser Coverage Settings',
  'Managed Browser Setup Settings',
  'Managed Browser Operation Settings',
  'Unmanaged Browser Handling Settings',
  'URL And Tab Evidence Settings',
  'Rule Target Settings',
  'Rule Action Settings',
  'Observe Versus Enforce Settings',
  'Schedule Settings',
  'Time Budget Settings',
  'Parent Approval Settings',
  'Override Settings',
  'Downloads Settings',
  'Search Settings',
  'Video And Channel Settings',
  'Private, Incognito, Tor, And Anti-Bypass Settings',
  'Network And Domain Fallback Settings',
  'Browser App And Process Settings',
  'Child-Facing Experience Settings',
  'Parent Report Settings',
  'Portal Display Settings',
  'Portal Action Settings',
  'Portal AI Settings',
  'Data Source And Custody Settings',
  'Retention Settings',
  'Audit Settings',
  'Capability Failure Settings',
  'Conflict Resolution Settings',
  'Local AI Browser Settings',
  'Never-Collect Settings',
  'Platform Settings',
  'Setup And Provisioning Settings',
  'Notifications And Escalation Settings',
  'Gaps To Decide Before UI Contracts',
] as const;

export const BrowserControlCandidateMvpItems = [
  'Enable browser controls.',
  'Mode: observe, dry-run, warn/ask, enforce.',
  'Require managed browser for exact web rules.',
  'Scan installed browsers.',
  'Scan running browsers.',
  'Detect unmanaged browsers.',
  'Allow managed browser.',
  'Launch or repair managed browser setup.',
  'Allow URL/domain/title evidence from managed browser.',
  'Redact query strings.',
  'Keep exact URL evidence for selected retention.',
  'Allow unmanaged browser: monitor, warn, ask, relaunch, block.',
  'Choose covered browsers: Edge, Chrome, Chrome for Testing, unsupported as unmanaged.',
  'Rule targets: exact URL, domain/origin, category, browser process, browser session, capability state.',
  'Rule actions: allow, warn, ask, limit, block.',
  'Time budgets: daily, session, site/domain, grace, blackout.',
  'Parent approvals: new domain, blocked site, unmanaged browser, downloads, time extension.',
  'Reports: managed status, recent URL/domain/title, unmanaged use, decisions, block results, time budget, source/capability.',
  'Proof requirement: process, foreground, managed tab list, proven active tab, fresh only, stale/degraded.',
  'Data custody: child local, LAN live, parent cache, parent export/report, unavailable.',
  'Audit: policy decision, evidence ref, adapter result, timer state, parent override, rollback, policy version.',
] as const;

const implemented = 'implemented-manifest-control';
const nested = 'represented-through-nested-rule';
const capability = 'represented-through-capability';
const future = 'future-gap';
const docsOnly = 'documentation-only';

const candidateEntries = [
  entry(
    'candidate-mvp',
    'Candidate MVP Setting Set',
    BrowserControlCandidateMvpItems[0],
    implemented,
    [field('browser.enabled')],
    [BrowserControlWritesToPath.Enabled],
    'BrowserControlPolicyValue.enabled',
    null,
    'Covered as the top-level manifest switch and patchable policy value.'
  ),
  entry(
    'candidate-mvp',
    'Candidate MVP Setting Set',
    BrowserControlCandidateMvpItems[1],
    implemented,
    [field('browser.executionMode'), field('browser.defaultPosture')],
    [BrowserControlWritesToPath.ExecutionMode, BrowserControlWritesToPath.DefaultPosture],
    'BrowserControlPolicyValue.executionMode and defaultPosture',
    null,
    'Execution mode now covers observe, dry-run, warn/ask, and enforce while default posture carries allow/warn/ask/limit/block decisions.'
  ),
  entry(
    'candidate-mvp',
    'Candidate MVP Setting Set',
    BrowserControlCandidateMvpItems[2],
    implemented,
    [field('managedBrowser.mode'), field('evidence.requiredProof')],
    [BrowserControlWritesToPath.ManagedBrowserMode, BrowserControlWritesToPath.RequiredProof],
    'managedBrowser.mode with evidence.requiredProof',
    null,
    'Exact URL honesty remains enforced by managed proof or an explicit fallback.'
  ),
  entry(
    'candidate-mvp',
    'Candidate MVP Setting Set',
    BrowserControlCandidateMvpItems[3],
    implemented,
    [field('discovery.scanInstalledBrowsers')],
    [BrowserControlWritesToPath.DiscoveryScanInstalledBrowsers],
    'discovery.scanInstalledBrowsers',
    null,
    'Source of truth now carries intent separately from adapter proof.'
  ),
  entry(
    'candidate-mvp',
    'Candidate MVP Setting Set',
    BrowserControlCandidateMvpItems[4],
    implemented,
    [field('discovery.scanRunningBrowsers')],
    [BrowserControlWritesToPath.DiscoveryScanRunningBrowsers],
    'discovery.scanRunningBrowsers',
    null,
    'Source of truth now carries running-browser discovery intent.'
  ),
  entry(
    'candidate-mvp',
    'Candidate MVP Setting Set',
    BrowserControlCandidateMvpItems[5],
    implemented,
    [field('discovery.detectUnmanagedBrowsers'), field('unmanagedBrowser.classificationTargets')],
    [
      BrowserControlWritesToPath.DiscoveryDetectUnmanagedBrowsers,
      BrowserControlWritesToPath.UnmanagedBrowserClassificationTargets,
    ],
    'discovery.detectUnmanagedBrowsers plus unmanagedBrowser.classificationTargets',
    null,
    'Detection intent and classification targets are separate from OS adapter proof.'
  ),
  entry(
    'candidate-mvp',
    'Candidate MVP Setting Set',
    BrowserControlCandidateMvpItems[6],
    implemented,
    [field('managedBrowser.mode'), field('managedBrowser.allowedFamilies')],
    [BrowserControlWritesToPath.ManagedBrowserMode, BrowserControlWritesToPath.ManagedBrowserAllowedFamilies],
    'managedBrowser mode and allowed families',
    null,
    'Managed browser authoring is explicit without claiming setup success.'
  ),
  entry(
    'candidate-mvp',
    'Candidate MVP Setting Set',
    BrowserControlCandidateMvpItems[7],
    capability,
    [field('managedBrowser.launchMode')],
    [BrowserControlWritesToPath.ManagedBrowserLaunchMode],
    'managedBrowser.launchMode plus capability registry',
    'manual-required',
    'Launch preference is authored; repair/provisioning remains capability/manual-required until adapter proof exists.'
  ),
  entry(
    'candidate-mvp',
    'Candidate MVP Setting Set',
    BrowserControlCandidateMvpItems[8],
    implemented,
    [field('evidence.urlScope'), field('evidence.requiredProof')],
    [BrowserControlWritesToPath.EvidenceUrlScope, BrowserControlWritesToPath.RequiredProof],
    'evidence url scope and proof level',
    null,
    'Managed URL/domain/title evidence is represented through evidence scope and proof requirements.'
  ),
  entry(
    'candidate-mvp',
    'Candidate MVP Setting Set',
    BrowserControlCandidateMvpItems[9],
    implemented,
    [field('evidence.urlScope')],
    [BrowserControlWritesToPath.EvidenceUrlScope],
    'evidence.urlScope',
    null,
    'The full-url-without-query scope is the contract path for query redaction.'
  ),
  entry(
    'candidate-mvp',
    'Candidate MVP Setting Set',
    BrowserControlCandidateMvpItems[10],
    implemented,
    [field('retention.exactUrl')],
    [BrowserControlWritesToPath.RetentionExactUrl],
    'retention.exactUrl',
    null,
    'Exact URL retention has a typed manifest field and policy value.'
  ),
  entry(
    'candidate-mvp',
    'Candidate MVP Setting Set',
    BrowserControlCandidateMvpItems[11],
    implemented,
    [field('unmanagedBrowser.mode')],
    [BrowserControlWritesToPath.UnmanagedBrowserMode],
    'unmanagedBrowser.mode',
    null,
    'Monitor, warn, ask, relaunch, and block are enum-backed modes.'
  ),
  entry(
    'candidate-mvp',
    'Candidate MVP Setting Set',
    BrowserControlCandidateMvpItems[12],
    implemented,
    [field('managedBrowser.allowedFamilies'), field('unmanagedBrowser.classificationTargets')],
    [
      BrowserControlWritesToPath.ManagedBrowserAllowedFamilies,
      BrowserControlWritesToPath.UnmanagedBrowserClassificationTargets,
    ],
    'managedBrowser.allowedFamilies with unmanaged classification targets',
    null,
    'Unsupported browser handling is represented by unmanaged classification rather than pretending all browsers are controllable.'
  ),
  entry(
    'candidate-mvp',
    'Candidate MVP Setting Set',
    BrowserControlCandidateMvpItems[13],
    nested,
    [field('rules.allowedTargetTypes'), field('rules.items')],
    [BrowserControlWritesToPath.AllowedTargetTypes, BrowserControlWritesToPath.RuleItems],
    'rules.allowedTargetTypes and rule items',
    null,
    'Targets are enabled by schema-known target kinds and structured rule items.'
  ),
  entry(
    'candidate-mvp',
    'Candidate MVP Setting Set',
    BrowserControlCandidateMvpItems[14],
    implemented,
    [field('rules.allowedActions')],
    [BrowserControlWritesToPath.AllowedActions],
    'rules.allowedActions',
    null,
    'The core allow, warn, ask, limit, and block actions are contract-backed.'
  ),
  entry(
    'candidate-mvp',
    'Candidate MVP Setting Set',
    BrowserControlCandidateMvpItems[15],
    nested,
    [
      field('budgets.enabled'),
      field('budgets.defaultDailyMinutes'),
      field('budgets.countingMode'),
      field('unmanagedBrowser.graceSeconds'),
    ],
    [
      BrowserControlWritesToPath.BudgetsEnabled,
      BrowserControlWritesToPath.DailyBudgetMinutes,
      BrowserControlWritesToPath.BudgetCountingMode,
      BrowserControlWritesToPath.UnmanagedBrowserGraceSeconds,
    ],
    'budgets plus rule schedule/budget ids',
    null,
    'Daily and grace are direct controls; session, site/domain, and blackout are represented through schedule/budget-linked rules until a richer quota UI slice.'
  ),
  entry(
    'candidate-mvp',
    'Candidate MVP Setting Set',
    BrowserControlCandidateMvpItems[16],
    implemented,
    [field('approvals.requiredFor'), field('approvals.unansweredDefault')],
    [BrowserControlWritesToPath.ApprovalRequiredFor, BrowserControlWritesToPath.ApprovalUnansweredDefault],
    'approvals.requiredFor and unansweredDefault',
    null,
    'Approval triggers and unanswered behavior are manifest-backed.'
  ),
  entry(
    'candidate-mvp',
    'Candidate MVP Setting Set',
    BrowserControlCandidateMvpItems[17],
    implemented,
    [field('reports.visibleFields')],
    [BrowserControlWritesToPath.ReportVisibleFields],
    'reports.visibleFields',
    null,
    'Parent-visible report fields include managed status, recent URL/domain/title, decisions, budget, and source capability.'
  ),
  entry(
    'candidate-mvp',
    'Candidate MVP Setting Set',
    BrowserControlCandidateMvpItems[18],
    implemented,
    [field('evidence.requiredProof'), field('evidence.whenProofUnavailable')],
    [BrowserControlWritesToPath.RequiredProof, BrowserControlWritesToPath.WhenProofUnavailable],
    'evidence proof and proof-unavailable fallback',
    null,
    'Proof levels and stale/degraded handling are enum-backed.'
  ),
  entry(
    'candidate-mvp',
    'Candidate MVP Setting Set',
    BrowserControlCandidateMvpItems[19],
    implemented,
    [field('custody.allowedUses')],
    [BrowserControlWritesToPath.CustodyAllowedUses],
    'custody.allowedUses',
    null,
    'Custody destinations include child local, LAN live, parent cache/export/report, and unavailable.'
  ),
  entry(
    'candidate-mvp',
    'Candidate MVP Setting Set',
    BrowserControlCandidateMvpItems[20],
    implemented,
    [field('audit.requiredFields')],
    [BrowserControlWritesToPath.AuditRequiredFields],
    'audit.requiredFields',
    null,
    'Audit field requirements cover decisions, evidence, adapter result, timers, overrides, rollback, and policy version.'
  ),
];

const sectionEntries = [
  section(
    'How To Read This Catalog',
    docsOnly,
    [],
    [],
    null,
    null,
    'Reading guidance is reflected by matrix statuses and does not become a runtime control.'
  ),
  section(
    'Global Rule Dimensions',
    nested,
    [field('rules.items')],
    [BrowserControlWritesToPath.RuleItems],
    'rule target/action/schedule/budget ids',
    null,
    'Family, child, device, schedule, and rule specificity are represented in structured rules and future policy scopes.'
  ),
  section(
    'Master Browser Control Settings',
    implemented,
    [field('browser.enabled'), field('browser.executionMode'), field('browser.defaultPosture')],
    [
      BrowserControlWritesToPath.Enabled,
      BrowserControlWritesToPath.ExecutionMode,
      BrowserControlWritesToPath.DefaultPosture,
    ],
    'enabled, executionMode, defaultPosture',
    null,
    'Top-level enablement, observe/dry-run/enforce mode, and posture are direct controls.'
  ),
  section(
    'Browser Discovery Settings',
    implemented,
    [
      field('discovery.scanInstalledBrowsers'),
      field('discovery.scanRunningBrowsers'),
      field('discovery.detectUnmanagedBrowsers'),
    ],
    [
      BrowserControlWritesToPath.DiscoveryScanInstalledBrowsers,
      BrowserControlWritesToPath.DiscoveryScanRunningBrowsers,
      BrowserControlWritesToPath.DiscoveryDetectUnmanagedBrowsers,
    ],
    'discovery booleans',
    null,
    'Discovery intent is typed; OS scan adapters remain capability-gated.'
  ),
  section(
    'Browser Coverage Settings',
    implemented,
    [field('managedBrowser.allowedFamilies'), field('unmanagedBrowser.classificationTargets')],
    [
      BrowserControlWritesToPath.ManagedBrowserAllowedFamilies,
      BrowserControlWritesToPath.UnmanagedBrowserClassificationTargets,
    ],
    'managed allowed families and unmanaged classifications',
    null,
    'Covered browser families are explicit; unsupported browsers route to unmanaged handling.'
  ),
  section(
    'Managed Browser Setup Settings',
    capability,
    [
      field('managedBrowser.mode'),
      field('managedBrowser.launchMode'),
      field('managedBrowser.profileMode'),
      field('managedBrowser.bridgeRequirements'),
    ],
    [
      BrowserControlWritesToPath.ManagedBrowserMode,
      BrowserControlWritesToPath.ManagedBrowserLaunchMode,
      BrowserControlWritesToPath.ManagedBrowserProfileMode,
      BrowserControlWritesToPath.ManagedBrowserBridgeRequirements,
    ],
    'managedBrowser setup contract plus capability registry',
    'manual-required',
    'Setup preferences are contract-backed; install, repair, and provision remain manual/capability states until adapter proof exists.'
  ),
  section(
    'Managed Browser Operation Settings',
    capability,
    [field('managedBrowser.launchMode'), field('managedBrowser.bridgeRequirements')],
    [BrowserControlWritesToPath.ManagedBrowserLaunchMode, BrowserControlWritesToPath.ManagedBrowserBridgeRequirements],
    'managedBrowser launch and bridge requirements',
    'manual-required',
    'Operational controls are represented where policy can state intent; process control is not claimed without adapter proof.'
  ),
  section(
    'Unmanaged Browser Handling Settings',
    implemented,
    [
      field('unmanagedBrowser.mode'),
      field('unmanagedBrowser.graceSeconds'),
      field('unmanagedBrowser.allowRecoverLaunchUrl'),
      field('unmanagedBrowser.classificationTargets'),
    ],
    [
      BrowserControlWritesToPath.UnmanagedBrowserMode,
      BrowserControlWritesToPath.UnmanagedBrowserGraceSeconds,
      BrowserControlWritesToPath.UnmanagedBrowserAllowRecoverLaunchUrl,
      BrowserControlWritesToPath.UnmanagedBrowserClassificationTargets,
    ],
    'unmanagedBrowser policy group',
    null,
    'Unmanaged handling, grace, relaunch recovery, and detection classes are direct controls.'
  ),
  section(
    'URL And Tab Evidence Settings',
    implemented,
    [field('evidence.urlScope'), field('evidence.requiredProof'), field('evidence.whenProofUnavailable')],
    [
      BrowserControlWritesToPath.EvidenceUrlScope,
      BrowserControlWritesToPath.RequiredProof,
      BrowserControlWritesToPath.WhenProofUnavailable,
    ],
    'evidence requirements',
    null,
    'URL scope, tab proof, and fallback are typed; body/screenshot collection remains forbidden by never-collect.'
  ),
  section(
    'Rule Target Settings',
    nested,
    [field('rules.allowedTargetTypes'), field('rules.items')],
    [BrowserControlWritesToPath.AllowedTargetTypes, BrowserControlWritesToPath.RuleItems],
    'allowedTargetTypes and structured rule targets',
    null,
    'Targets are represented by schema-known target enums and rule list items.'
  ),
  section(
    'Rule Action Settings',
    implemented,
    [field('rules.allowedActions'), field('rules.items')],
    [BrowserControlWritesToPath.AllowedActions, BrowserControlWritesToPath.RuleItems],
    'allowedActions and structured rule action plans',
    null,
    'Core rule actions are enum-backed; redirect/close actions are typed but adapter-gated.'
  ),
  section(
    'Observe Versus Enforce Settings',
    implemented,
    [field('browser.executionMode'), field('browser.defaultPosture')],
    [BrowserControlWritesToPath.ExecutionMode, BrowserControlWritesToPath.DefaultPosture],
    'executionMode and defaultPosture',
    null,
    'Execution mode separates observe/dry-run/enforce from parent-facing posture.'
  ),
  section(
    'Schedule Settings',
    nested,
    [field('rules.items')],
    [BrowserControlWritesToPath.RuleItems],
    'schedules array and rule scheduleId references',
    null,
    'Schedule support exists as nested policy shape; individual calendar presets are not separate manifest controls.'
  ),
  section(
    'Time Budget Settings',
    nested,
    [field('budgets.enabled'), field('budgets.defaultDailyMinutes'), field('budgets.countingMode')],
    [
      BrowserControlWritesToPath.BudgetsEnabled,
      BrowserControlWritesToPath.DailyBudgetMinutes,
      BrowserControlWritesToPath.BudgetCountingMode,
    ],
    'budgets plus rule budgetId references',
    null,
    'Daily/default budget fields are direct; richer per-site quota authoring remains a future UI slice.'
  ),
  section(
    'Parent Approval Settings',
    implemented,
    [field('approvals.requiredFor'), field('approvals.unansweredDefault')],
    [BrowserControlWritesToPath.ApprovalRequiredFor, BrowserControlWritesToPath.ApprovalUnansweredDefault],
    'approvals policy group',
    null,
    'Approval triggers and unanswered defaults are direct controls.'
  ),
  section(
    'Override Settings',
    nested,
    [field('approvals.requiredFor'), field('rules.items')],
    [BrowserControlWritesToPath.ApprovalRequiredFor, BrowserControlWritesToPath.RuleItems],
    'approval triggers and rule precedence',
    null,
    'Temporary override storage/action protocol is not a separate browser-control manifest field in this slice.'
  ),
  section(
    'Downloads Settings',
    implemented,
    [field('downloads.mode'), field('downloads.blockedTypes')],
    [BrowserControlWritesToPath.DownloadMode, BrowserControlWritesToPath.DownloadBlockedTypes],
    'downloads policy group',
    null,
    'Download mode and risky types are typed; native download interception remains capability-gated.'
  ),
  section(
    'Search Settings',
    nested,
    [field('rules.allowedTargetTypes'), field('evidence.neverCollect')],
    [BrowserControlWritesToPath.AllowedTargetTypes, BrowserControlWritesToPath.EvidenceNeverCollect],
    'search-terms target plus never-collect restrictions',
    null,
    'Search rules are represented by target type; term collection defaults to redaction/never-collect boundaries.'
  ),
  section(
    'Video And Channel Settings',
    nested,
    [field('rules.allowedTargetTypes')],
    [BrowserControlWritesToPath.AllowedTargetTypes],
    'video-channel target type',
    null,
    'Video/channel controls are rule targets rather than separate one-off fields.'
  ),
  section(
    'Private, Incognito, Tor, And Anti-Bypass Settings',
    capability,
    [field('unmanagedBrowser.classificationTargets'), field('managedBrowser.bridgeRequirements')],
    [
      BrowserControlWritesToPath.UnmanagedBrowserClassificationTargets,
      BrowserControlWritesToPath.ManagedBrowserBridgeRequirements,
    ],
    'classification targets and bridge requirements',
    'manual-required',
    'Private/incognito/Tor detection is classified honestly; hard blocking requires platform/browser adapter proof.'
  ),
  section(
    'Network And Domain Fallback Settings',
    capability,
    [field('evidence.requiredProof'), field('evidence.whenProofUnavailable')],
    [BrowserControlWritesToPath.RequiredProof, BrowserControlWritesToPath.WhenProofUnavailable],
    'evidence proof fallback and capability registry',
    'degraded',
    'Network/domain fallback can be represented as degraded proof; network enforcement is not claimed without adapter support.'
  ),
  section(
    'Browser App And Process Settings',
    nested,
    [field('rules.allowedTargetTypes'), field('budgets.countingMode')],
    [BrowserControlWritesToPath.AllowedTargetTypes, BrowserControlWritesToPath.BudgetCountingMode],
    'browser-process targets and all-browser-process-time counting',
    null,
    'Process/app controls are represented as target/counting modes; process blocking stays adapter-gated.'
  ),
  section(
    'Child-Facing Experience Settings',
    nested,
    [],
    [],
    'childFacing policy group',
    null,
    'Child-facing flags are typed in BrowserControlPolicyValue and intentionally not separate C visual controls in this D slice.'
  ),
  section(
    'Parent Report Settings',
    implemented,
    [field('reports.visibleFields')],
    [BrowserControlWritesToPath.ReportVisibleFields],
    'reports.visibleFields',
    null,
    'Report visibility is a typed manifest control.'
  ),
  section(
    'Portal Display Settings',
    nested,
    [field('reports.visibleFields')],
    [BrowserControlWritesToPath.ReportVisibleFields],
    'reports.visibleFields with capability registry',
    null,
    'Portal display uses report visibility plus capability state; C owns visual rendering.'
  ),
  section(
    'Portal Action Settings',
    nested,
    [],
    [],
    'typed update protocol get, preview, patch, replace, rollback',
    null,
    'Portal actions are represented by update protocol contracts rather than settings fields.'
  ),
  section(
    'Portal AI Settings',
    nested,
    [],
    [],
    'portalAi policy group',
    null,
    'Portal AI permissions are typed in policy value and remain non-visual here.'
  ),
  section(
    'Data Source And Custody Settings',
    implemented,
    [field('custody.allowedUses')],
    [BrowserControlWritesToPath.CustodyAllowedUses],
    'custody.allowedUses',
    null,
    'Custody destinations are direct controls.'
  ),
  section(
    'Retention Settings',
    implemented,
    [field('retention.exactUrl')],
    [BrowserControlWritesToPath.RetentionExactUrl],
    'retention.exactUrl and retention.state',
    null,
    'Exact URL retention is manifest-backed; generic retention state exists for runtime policy.'
  ),
  section(
    'Audit Settings',
    implemented,
    [field('audit.requiredFields')],
    [BrowserControlWritesToPath.AuditRequiredFields],
    'audit.requiredFields and audit.state',
    null,
    'Required audit fields are direct controls; audit state exists in runtime policy.'
  ),
  section(
    'Capability Failure Settings',
    capability,
    [],
    [],
    'fallbacks policy group and capability registry',
    'degraded',
    'Failure behavior is represented by fallbacks/capability state and does not pretend unsupported adapters are installed.'
  ),
  section(
    'Conflict Resolution Settings',
    nested,
    [field('rules.items')],
    [BrowserControlWritesToPath.RuleItems],
    'rule priority and update revision checks',
    null,
    'Rule priority and stale-revision checks cover conflict resolution in current source truth.'
  ),
  section(
    'Local AI Browser Settings',
    nested,
    [],
    [],
    'portalAi and evidence refs',
    null,
    'AI can reference evidence refs by policy shape; raw browser content remains disallowed unless explicitly reviewed.'
  ),
  section(
    'Never-Collect Settings',
    implemented,
    [field('evidence.neverCollect')],
    [BrowserControlWritesToPath.EvidenceNeverCollect],
    'evidence.neverCollect',
    null,
    'Never-collect restrictions are direct manifest controls.'
  ),
  section(
    'Platform Settings',
    capability,
    [],
    [],
    'platforms policy group and capability registry',
    'manual-required',
    'Platform support is represented by capability state; OS-specific installers, device-owner policy, and native hosts remain manual/future.'
  ),
  section(
    'Setup And Provisioning Settings',
    capability,
    [field('managedBrowser.launchMode'), field('managedBrowser.integrationMechanisms')],
    [
      BrowserControlWritesToPath.ManagedBrowserLaunchMode,
      BrowserControlWritesToPath.ManagedBrowserIntegrationMechanisms,
    ],
    'managed setup intent plus capability registry',
    'manual-required',
    'Provisioning is documented as manual/capability state until real setup adapters exist.'
  ),
  section(
    'Notifications And Escalation Settings',
    future,
    [field('approvals.requiredFor')],
    [BrowserControlWritesToPath.ApprovalRequiredFor],
    'approval triggers only',
    null,
    'Approval-triggering is covered; notification delivery channels are a future non-browser-control slice.'
  ),
  section(
    'Gaps To Decide Before UI Contracts',
    docsOnly,
    [],
    [],
    null,
    null,
    'The matrix records source-of-truth placement so future UI contracts do not invent arbitrary questions.'
  ),
];

export const BrowserControlCoverageMatrix = BrowserControlCoverageMatrixSchema.parse([
  ...candidateEntries,
  ...sectionEntries,
]);

export type BrowserControlCoverageKind = Infer<typeof BrowserControlCoverageKindSchema>;
export type BrowserControlCoverageStatus = Infer<typeof BrowserControlCoverageStatusSchema>;
export type BrowserControlCoverageEntry = Infer<typeof BrowserControlCoverageEntrySchema>;

function entry(
  coverageKind: BrowserControlCoverageKind,
  catalogSection: string,
  catalogItem: string,
  coverageStatus: BrowserControlCoverageStatus,
  manifestFieldIds: ReturnType<typeof field>[],
  writesTo: BrowserControlSchemaKnownWritesToPath[],
  policyShape: string | null,
  capabilityState:
    | 'supported'
    | 'unsupported'
    | 'degraded'
    | 'unavailable'
    | 'unknown'
    | 'ready'
    | 'manual-required'
    | null,
  notes: string
) {
  return {
    coverageKind,
    catalogSection,
    catalogItem,
    coverageStatus,
    manifestFieldIds,
    writesTo,
    policyShape,
    capabilityState,
    compilerCapabilityState: compilerCapabilityStateForEntry(coverageStatus, capabilityState),
    notes,
  };
}

function section(
  catalogSection: string,
  coverageStatus: BrowserControlCoverageStatus,
  manifestFieldIds: ReturnType<typeof field>[],
  writesTo: BrowserControlSchemaKnownWritesToPath[],
  policyShape: string | null,
  capabilityState:
    | 'supported'
    | 'unsupported'
    | 'degraded'
    | 'unavailable'
    | 'unknown'
    | 'ready'
    | 'manual-required'
    | null,
  notes: string
) {
  return entry(
    'catalog-section',
    catalogSection,
    'section coverage summary',
    coverageStatus,
    manifestFieldIds,
    writesTo,
    policyShape,
    capabilityState,
    notes
  );
}

function field(value: string) {
  return BrowserControlFieldIdSchema.parse(value);
}

function compilerCapabilityStateForEntry(
  coverageStatus: BrowserControlCoverageStatus,
  capabilityState:
    | 'supported'
    | 'unsupported'
    | 'degraded'
    | 'unavailable'
    | 'unknown'
    | 'ready'
    | 'manual-required'
    | null
) {
  if (
    coverageStatus === 'documentation-only' ||
    coverageStatus === 'future-gap' ||
    coverageStatus === 'unavailable'
  ) {
    return PolicyCompilerCapabilityState.Unsupported;
  }
  if (coverageStatus === 'manual-required') {
    return PolicyCompilerCapabilityState.ManualRequired;
  }
  if (coverageStatus === 'represented-through-capability') {
    return capabilityState === 'manual-required'
      ? PolicyCompilerCapabilityState.ManualRequired
      : PolicyCompilerCapabilityState.Supported;
  }
  return PolicyCompilerCapabilityState.Supported;
}

