import { ScreenControlOptionIdSchema, ScreenControlRuleIdSchema } from './screen-control-catalog-schema';
import type {
  ScreenControlCatalogCapabilityState,
  ScreenControlCatalogCardKind,
  ScreenControlCatalogControlKind,
  ScreenControlCatalogEffectStatus,
  ScreenControlCatalogLayoutHints,
  ScreenControlCatalogOption,
  ScreenControlCatalogRule,
  ScreenControlCatalogRuntimeOwner,
  ScreenControlCatalogSelectionMode,
  ScreenControlCatalogSourceKind,
  ScreenControlCatalogUiTab,
} from './screen-control-catalog-schema';

export const ScreenControlCatalogSourceDocuments = [
  'docs/screen-evidence-analysis-capability-guide.md',
  'docs/screen-evidence-analysis-schema-proposal.md',
] as const;
export const ScreenControlCatalogSidePanelCategory = 'screen';

export const ScreenControlCatalogTargetScopeOptions = options(['Family', 'Per Child', 'Per Device', 'Per Platform']);

export const ScreenControlCatalogEffectModeOptions = options([
  'Off',
  'Observe',
  'Dry Run',
  'Notify',
  'Ask',
  'Warn',
  'Limit',
  'Block',
  'Enforce',
  'Audit Only',
]);

const ScreenControlCatalogFixedCardKinds: Partial<
  Record<ScreenControlCatalogControlKind, ScreenControlCatalogCardKind>
> = {
  duration: 'duration-card',
  number: 'number-card',
  'read-only-status': 'status-card',
  retention: 'retention-card',
  'rule-list': 'rule-list-card',
  schedule: 'schedule-card',
  'target-list': 'target-list-card',
  threshold: 'threshold-card',
  toggle: 'toggle',
};

export function optionsForSetting(
  sourceText: string,
  controlKind: ScreenControlCatalogControlKind,
  acceptedOptions: readonly string[]
): ScreenControlCatalogOption[] {
  if (acceptedOptions.length > 0) {
    return options(acceptedOptions);
  }
  if (controlKind === 'toggle') {
    return options(['Enabled', 'Disabled']);
  }
  if (controlKind === 'number' || controlKind === 'duration' || controlKind === 'threshold') {
    return options(['Configured Value', 'Minimum', 'Maximum']);
  }
  const explicit = explicitOptionLabels(sourceText);
  if (explicit.length > 0) {
    return options(explicit);
  }
  return options(['Configured', 'Unavailable']);
}

export function selectionModeFor(
  controlKind: ScreenControlCatalogControlKind,
  settingOptions: readonly ScreenControlCatalogOption[]
): ScreenControlCatalogSelectionMode {
  if (controlKind === 'number' || controlKind === 'duration' || controlKind === 'threshold') {
    return 'numeric';
  }
  if (controlKind === 'schedule') {
    return 'schedule';
  }
  if (controlKind === 'read-only-status') {
    return 'status';
  }
  if (controlKind === 'multi-choice' || controlKind === 'rule-list' || controlKind === 'target-list') {
    return 'multi';
  }
  return settingOptions.length > 4 ? 'multi' : 'single';
}

export function cardKindFor(
  controlKind: ScreenControlCatalogControlKind,
  selectionMode: ScreenControlCatalogSelectionMode,
  settingOptions: readonly ScreenControlCatalogOption[]
): ScreenControlCatalogCardKind {
  const fixedCardKind = ScreenControlCatalogFixedCardKinds[controlKind];
  if (fixedCardKind !== undefined) {
    return fixedCardKind;
  }
  if (selectionMode === 'multi') {
    return settingOptions.length > 4 ? 'multi-choice-many' : 'multi-choice-normal';
  }
  return settingOptions.length > 4 ? 'single-choice-many' : 'single-choice-compact';
}

export function layoutHintsFor(
  selectionMode: ScreenControlCatalogSelectionMode,
  settingOptions: readonly ScreenControlCatalogOption[]
): ScreenControlCatalogLayoutHints {
  const manyOptions = settingOptions.length > 4;
  return {
    preferredColumnSpan: manyOptions ? 2 : 1,
    collapsible: manyOptions || selectionMode === 'multi' || selectionMode === 'status',
    searchableOptions: manyOptions,
    optionGroupCount: manyOptions ? Math.ceil(settingOptions.length / 4) : 1,
    showAsMatrixWhenLarge: manyOptions && selectionMode === 'multi',
    showSelectedCount: selectionMode === 'multi',
  };
}

export function questionFromSeed(
  sourceKind: ScreenControlCatalogSourceKind,
  sourceText: string,
  controlKind: ScreenControlCatalogControlKind
): string {
  if (sourceKind === 'authoring-field') {
    return sourceText.endsWith('?') ? sourceText : `${sourceText}?`;
  }
  if (sourceKind === 'update-command') {
    return `Support ${sourceText.split(':')[0]}?`;
  }
  if (sourceKind === 'capability-state-meaning') {
    return `Show ${sourceText.split(':')[0]} capability state?`;
  }
  if (controlKind === 'read-only-status') {
    return `Represent ${lowerFirst(sourceText.replace(/\.$/u, ''))}.`;
  }
  const colonIndex = sourceText.indexOf(':');
  if (colonIndex !== -1) {
    return `Choose ${lowerFirst(sourceText.slice(0, colonIndex))}.`;
  }
  return `Use ${lowerFirst(sourceText.replace(/\.$/u, ''))}?`;
}

export function uiTabFor(sectionTitle: string, groupTitle: string, sourceText: string): ScreenControlCatalogUiTab {
  const searchable = `${sectionTitle} ${groupTitle} ${sourceText}`;
  if (/schedule|cadence|trigger|time/i.test(searchable)) {
    return 'schedule';
  }
  if (/approval|ask[- ]parent/i.test(searchable)) {
    return 'approvals';
  }
  if (/audit|proof|decision/i.test(searchable)) {
    return 'audit';
  }
  if (/report|disclosure|child-facing/i.test(searchable)) {
    return 'reports';
  }
  if (/queue|retention|custody|delete|redaction|minimization|store|storage/i.test(searchable)) {
    return 'data';
  }
  if (/OCR|vision|model|classification|confidence|AI|analysis/i.test(searchable)) {
    return 'ai';
  }
  if (/platform|Windows|macOS|Linux|Android|iOS|permission|protected|capture scope/i.test(searchable)) {
    return 'platform';
  }
  if (/setup|manual test|capability registry|management/i.test(searchable)) {
    return 'setup';
  }
  if (/policy|rule|target/i.test(searchable)) {
    return 'rules';
  }
  if (/enforce|block|limit/i.test(searchable)) {
    return 'enforcement';
  }
  return 'evidence';
}

export function effectStatusFor(
  sourceKind: ScreenControlCatalogSourceKind,
  sectionTitle: string,
  sourceText: string
): ScreenControlCatalogEffectStatus {
  const searchable = `${sectionTitle} ${sourceText}`;
  if (/future|authoring-only|ReplayKit|iOS|iPadOS/i.test(searchable)) {
    return 'future-gap';
  }
  if (
    /hosted processing|cloud\/API AI|hidden capture|permanent screenshot|retain raw|raw screenshot|continuous/i.test(
      searchable
    )
  ) {
    return 'unavailable';
  }
  if (/permission|consent|protected|locked|secure|DRM|TCC|entitlement/i.test(searchable)) {
    return /limited/i.test(searchable) ? 'permission-limited' : 'permission-required';
  }
  if (/proof|validated|exact URL|enforcement|policy use|evidence ref|deletion state/i.test(searchable)) {
    return 'proof-required';
  }
  if (/manual-required|manual proof|real host|platform|Windows|macOS|Linux|Android/i.test(searchable)) {
    return 'manual-required';
  }
  if (
    /unavailable|degraded|low confidence|adapter error|model unavailable|queue unavailable|redaction unavailable/i.test(
      searchable
    )
  ) {
    return 'degraded';
  }
  if (/report|audit|custody|retention|delete|redaction|disclosure/i.test(searchable)) {
    return 'already-represented';
  }
  return sourceKind === 'agent-rule' || sourceKind === 'update-command' ? 'already-represented' : 'needs-effect-wiring';
}

export function capabilityStateFor(
  sourceKind: ScreenControlCatalogSourceKind,
  sectionTitle: string,
  sourceText: string
): ScreenControlCatalogCapabilityState {
  const status = effectStatusFor(sourceKind, sectionTitle, sourceText);
  if (status === 'future-gap') {
    return 'future-gap';
  }
  if (status === 'unavailable') {
    return 'unavailable';
  }
  if (status === 'permission-limited') {
    return 'permission-limited';
  }
  if (status === 'permission-required') {
    return /protected|secure|DRM/i.test(sourceText) ? 'protected' : 'permission-required';
  }
  if (status === 'manual-required') {
    return 'manual-required';
  }
  if (status === 'degraded') {
    return 'degraded';
  }
  return /disabled by default|disabled-by-parent/i.test(sourceText) ? 'disabled' : 'available';
}

export function runtimeOwnerFor(sectionTitle: string, sourceText: string): ScreenControlCatalogRuntimeOwner {
  const searchable = `${sectionTitle} ${sourceText}`;
  if (/Portal|report|disclosure|visible/i.test(searchable)) {
    return 'portal-only';
  }
  if (/audit|journal|SQLite|custody|retention|delete|storage|queue/i.test(searchable)) {
    return 'parent-owned-storage';
  }
  if (/OCR|vision|classification|model|AI|redaction|confidence/i.test(searchable)) {
    return 'local-ai-runtime';
  }
  if (/platform|capture|permission|protected|Windows|macOS|Linux|Android|iOS|window|screen/i.test(searchable)) {
    return 'os-adapter';
  }
  if (/command|patch|replace|rollback|protocol|agent rule/i.test(searchable)) {
    return 'agent-protocol';
  }
  if (/proof|manual/i.test(searchable)) {
    return 'manual-proof';
  }
  return 'child-agent';
}

export function capabilityRequirementFor(sectionTitle: string, sourceText: string): string {
  const searchable = `${sectionTitle} ${sourceText}`;
  if (/exact URL|managed browser/i.test(searchable)) {
    return 'managed-browser-evidence-required-for-exact-web-claims';
  }
  if (/capture|screen|window|display|recording/i.test(searchable)) {
    return 'platform-capture-capability-and-permission-proof';
  }
  if (/queue|encrypt|delete|TTL/i.test(searchable)) {
    return 'encrypted-local-temp-queue-with-deletion-proof';
  }
  if (/OCR|vision|classification|model|AI/i.test(searchable)) {
    return 'local-ocr-vision-runtime-with-schema-valid-output';
  }
  if (/redaction/i.test(searchable)) {
    return 'local-redaction-validation-before-summary-storage';
  }
  return 'screen-analysis-capability-registry';
}

export function proofRequirementFor(sectionTitle: string, sourceText: string): string | null {
  const searchable = `${sectionTitle} ${sourceText}`;
  if (/exact URL|web claims/i.test(searchable)) {
    return 'managed-browser-evidence-required';
  }
  if (/enforcement|policy use|policy can use|policy decision/i.test(searchable)) {
    return 'validated-screen-summary-evidence-ref-deleted-image-and-deterministic-policy-decision';
  }
  if (/queue|delete|deletion|raw image|frame/i.test(searchable)) {
    return 'encrypted-temporary-queue-and-raw-capture-deletion-proof';
  }
  if (/capture|screen|window|display|recording|permission|protected/i.test(searchable)) {
    return 'real-platform-capture-permission-and-scope-proof';
  }
  if (/OCR|vision|classification|model|AI|redaction/i.test(searchable)) {
    return 'schema-valid-local-analysis-output-with-confidence-and-redaction-state';
  }
  return null;
}

export function fallbackFor(
  sourceKind: ScreenControlCatalogSourceKind,
  sectionTitle: string,
  sourceText: string
): string {
  const status = effectStatusFor(sourceKind, sectionTitle, sourceText);
  if (status === 'unavailable') {
    return 'Disable or reject this state; do not retain raw capture or use hosted child screen processing by default.';
  }
  if (status === 'future-gap') {
    return 'Expose as future/manual-required planning only; do not compile into capture or enforcement.';
  }
  if (status === 'permission-required' || status === 'permission-limited') {
    return 'Skip capture and audit permission-required or permission-limited state before any queue job is created.';
  }
  if (status === 'manual-required') {
    return 'Show manual-required until real platform or host proof demonstrates the capability.';
  }
  if (status === 'degraded') {
    return 'Show degraded/unavailable state and compile only report-only or retry-within-ttl behavior.';
  }
  if (status === 'proof-required') {
    return 'Require validated summary, evidence refs, deletion proof, and deterministic policy before policy use.';
  }
  return 'Portal renders authored intent; child agent owns capture gating, queue, analysis, compile, and audit.';
}

export function visibilityConditionsFor(): ScreenControlCatalogRule[] {
  return [rule('Visible when the Screen side-panel category is selected.')];
}

export function enabledConditionsFor(
  sourceKind: ScreenControlCatalogSourceKind,
  sectionTitle: string,
  sourceText: string
): ScreenControlCatalogRule[] {
  return [
    rule('A family, child, or device target must be selected before writing Screen policy intent.'),
    rule(`Capability state must allow ${effectStatusFor(sourceKind, sectionTitle, sourceText)} presentation.`),
  ];
}

export function validationRulesFor(
  sourceKind: ScreenControlCatalogSourceKind,
  sectionTitle: string,
  sourceText: string
): ScreenControlCatalogRule[] {
  const proof = proofRequirementFor(sectionTitle, sourceText);
  const rules = [
    rule('Selected option ids must belong to this setting acceptedOptions list.'),
    rule('Raw screen capture stays local-only by default and is deleted after queue processing or TTL expiry.'),
    rule(
      'Portal writes authoring intent only; child runtime owns capture, analysis, compile, policy handoff, and audit.'
    ),
  ];
  if (proof !== null) {
    rules.push(rule(`Strict behavior requires proof: ${proof}.`));
  }
  if (effectStatusFor(sourceKind, sectionTitle, sourceText) === 'unavailable') {
    rules.push(rule('Unavailable states must fail closed and must not be promoted to enforcement support.'));
  }
  return rules;
}

function explicitOptionLabels(sourceText: string): string[] {
  const colonIndex = sourceText.indexOf(':');
  if (colonIndex === -1) {
    return [];
  }
  return unique(
    sourceText
      .slice(colonIndex + 1)
      .replace(/\.$/u, '')
      .split(/,|;|\bor\b/u)
      .map((part) => titleize(part.trim()))
      .filter((part) => part.length > 0 && part.length < 80)
  );
}

function options(labels: readonly string[]): ScreenControlCatalogOption[] {
  return labels.map((label) => {
    const value = slug(label);
    return {
      optionId: ScreenControlOptionIdSchema.parse(`screen-catalog-option-${value}`),
      label,
      value,
      originalSourceText: label,
      meaning: null,
      defaultSelected: false,
    };
  });
}

function rule(description: string): ScreenControlCatalogRule {
  return {
    ruleId: ScreenControlRuleIdSchema.parse(`screen-catalog-rule-${slug(description)}`),
    description,
  };
}

function titleize(value: string): string {
  return value
    .split(/[\s-]+/u)
    .filter((part) => part.length > 0)
    .map((part) => `${part.charAt(0).toUpperCase()}${part.slice(1)}`)
    .join(' ');
}

function slug(value: string): string {
  const normalized = value
    .toLowerCase()
    .replace(/&/gu, ' and ')
    .replace(/[^a-z0-9]+/gu, '-')
    .replace(/^-+|-+$/gu, '')
    .replace(/-{2,}/gu, '-');
  return normalized.length > 0 ? normalized : 'option';
}

function lowerFirst(value: string): string {
  return `${value.charAt(0).toLowerCase()}${value.slice(1)}`;
}

function unique(values: readonly string[]): string[] {
  return [...new Set(values)];
}
