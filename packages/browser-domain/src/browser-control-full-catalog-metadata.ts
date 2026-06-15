import { BrowserControlFieldIdSchema, BrowserControlOptionIdSchema } from './browser-control-identifiers';
import type {
  BrowserControlFullCatalogCapabilityState,
  BrowserControlFullCatalogCardKind,
  BrowserControlFullCatalogControlKind,
  BrowserControlFullCatalogEffectStatus,
  BrowserControlFullCatalogLayoutHints,
  BrowserControlFullCatalogOption,
  BrowserControlFullCatalogRule,
  BrowserControlFullCatalogRuntimeOwner,
  BrowserControlFullCatalogSelectionMode,
} from './browser-control-full-catalog-schema';

interface ControlKindRule {
  readonly pattern: RegExp;
  readonly resolve: (sourceText: string) => BrowserControlFullCatalogControlKind;
}

export const BrowserControlFullCatalogSourceDocument = 'docs/browser-policy-settings-catalog.md';
export const BrowserControlFullCatalogSourceDocuments = [
  BrowserControlFullCatalogSourceDocument,
  'docs/browser-control-schema-proposal.md',
  'docs/managed-unmanaged-browser.md',
  'docs/browser-control-coverage-matrix.md',
] as const;
export const BrowserControlFullCatalogSidePanelCategory = 'browser';

export const BrowserControlFullCatalogTargetScopeOptions = options([
  'Family',
  'Per Child',
  'Per Device',
  'Per Platform',
  'Per Browser',
  'Per Network',
]);

export const BrowserControlFullCatalogEffectModeOptions = options([
  'Off',
  'Observe',
  'Dry Run',
  'Warn',
  'Notify',
  'Ask',
  'Limit',
  'Block',
  'Enforce',
  'Audit Only',
]);

const BrowserControlFullCatalogControlKindRules: readonly ControlKindRule[] = [
  { pattern: /schedule|time window|bedtime|school hours/u, resolve: () => 'schedule' },
  { pattern: /budget|limit|minutes|seconds|retention days/u, resolve: numericOrDurationControl },
  { pattern: /retention|custody|delete|redact/u, resolve: () => 'retention' },
  { pattern: /rule items|target list|allowlist|blocklist/u, resolve: () => 'rule-list' },
  { pattern: /actions|approval actions|notification actions/u, resolve: () => 'action-list' },
  { pattern: /status|state|capability|proof/u, resolve: () => 'read-only-status' },
];

export function optionsFromSourceText(sourceText: string): BrowserControlFullCatalogOption[] {
  const explicit = explicitOptionLabels(sourceText);
  if (explicit.length > 0) {
    return options(explicit);
  }
  return options(['Enabled', 'Disabled']);
}

export function selectionModeFor(
  sourceText: string,
  settingOptions: readonly BrowserControlFullCatalogOption[]
): BrowserControlFullCatalogSelectionMode {
  if (settingOptions.length <= 2 && settingOptions[0]?.value === 'enabled') {
    return 'single';
  }
  const prefix = sourceText.split(':')[0]?.toLowerCase() ?? sourceText.toLowerCase();
  return /targets|actions|approvals|reports|proof|custody|audit|budgets|choose covered browsers/u.test(prefix)
    ? 'multi'
    : 'single';
}

export function cardKindFor(
  selectionMode: BrowserControlFullCatalogSelectionMode,
  settingOptions: readonly BrowserControlFullCatalogOption[]
): BrowserControlFullCatalogCardKind {
  if (settingOptions.length <= 2 && settingOptions[0]?.value === 'enabled') {
    return 'toggle';
  }
  if (selectionMode === 'multi') {
    return settingOptions.length > 4 ? 'multi-choice-many' : 'multi-choice-normal';
  }
  return settingOptions.length > 4 ? 'single-choice-many' : 'single-choice-compact';
}

export function controlKindFor(
  sourceText: string,
  selectionMode: BrowserControlFullCatalogSelectionMode,
  settingOptions: readonly BrowserControlFullCatalogOption[]
): BrowserControlFullCatalogControlKind {
  const prefix = sourceText.split(':')[0]?.toLowerCase() ?? sourceText.toLowerCase();
  const matchedRule = BrowserControlFullCatalogControlKindRules.find((ruleCandidate) =>
    ruleCandidate.pattern.test(prefix)
  );
  if (
    matchedRule !== undefined &&
    (matchedRule.resolve(sourceText) !== 'read-only-status' || settingOptions.length <= 2)
  ) {
    return matchedRule.resolve(sourceText);
  }
  if (settingOptions.length <= 2 && settingOptions[0]?.value === 'enabled') {
    return 'toggle';
  }
  return selectionMode === 'multi' ? 'multi-choice' : 'single-choice';
}

export function layoutHintsFor(
  selectionMode: BrowserControlFullCatalogSelectionMode,
  settingOptions: readonly BrowserControlFullCatalogOption[]
): BrowserControlFullCatalogLayoutHints {
  const manyOptions = settingOptions.length > 4;
  return {
    preferredColumnSpan: manyOptions ? 2 : 1,
    collapsible: manyOptions || selectionMode === 'multi',
    searchableOptions: manyOptions,
    optionGroupCount: manyOptions ? Math.ceil(settingOptions.length / 4) : 1,
    showAsMatrixWhenLarge: manyOptions && selectionMode === 'multi',
    showSelectedCount: selectionMode === 'multi',
  };
}

export function questionFromSourceText(sourceText: string): string {
  const trimmed = sourceText.replace(/\.$/u, '');
  const colonIndex = trimmed.indexOf(':');
  if (colonIndex !== -1) {
    return `Choose ${lowerFirst(trimmed.slice(0, colonIndex))}.`;
  }
  if (
    /^(enable|disable|allow|require|scan|detect|notify|ask|auto-classify|re-scan|show|hide|keep|redact|collect)/iu.test(
      trimmed
    )
  ) {
    return `${trimmed}?`;
  }
  return `Use ${lowerFirst(trimmed)}?`;
}

export function helperTextFor(sectionTitle: string, sourceText: string): string {
  if (proofRequirementFor(sectionTitle, sourceText) !== null) {
    return 'Exact browser evidence must stay proof-gated; Portal renders intent while runtime proves capability.';
  }
  if (effectStatusForSection(sectionTitle, sourceText) !== 'needs-effect-wiring') {
    return 'Render this with its capability state and fallback rather than claiming unsupported enforcement.';
  }
  return 'Portal renders authored intent; child runtime owns persistence, compile, evaluation, and audit.';
}

export function effectStatusForSection(
  sectionTitle: string,
  sourceText: string
): BrowserControlFullCatalogEffectStatus {
  const searchable = `${sectionTitle} ${sourceText}`;
  if (/Gaps To Decide Before UI Contracts/u.test(sectionTitle)) {
    return 'future-gap';
  }
  if (/Never-Collect|Portal Display|Child-Facing|Report|Audit|Custody|Retention|Data Minimization/u.test(searchable)) {
    return 'already-represented';
  }
  if (
    /Exact URL|active tab|page title|download source|browser evidence|required proof|proof requirement/iu.test(
      searchable
    )
  ) {
    return 'proof-required';
  }
  if (/Private|Tor|permission|consent|protected browser/iu.test(searchable)) {
    return 'permission-required';
  }
  if (/Network|Capability Failure|Fallback|Degradation|Unmanaged Browser Recovery|degraded/iu.test(searchable)) {
    return 'degraded';
  }
  if (/Platform|Setup|Provisioning|Managed Browser Operation|Notifications|manual/iu.test(searchable)) {
    return 'manual-required';
  }
  return 'needs-effect-wiring';
}

export function runtimeOwnerForSection(
  sectionTitle: string,
  sourceText: string
): BrowserControlFullCatalogRuntimeOwner {
  const searchable = `${sectionTitle} ${sourceText}`;
  if (/Portal Display|Child-Facing|Report/iu.test(searchable)) {
    return 'portal-only';
  }
  if (/Audit|Custody|Retention|Never-Collect|Data Minimization/iu.test(searchable)) {
    return 'parent-owned-storage';
  }
  if (/AI/iu.test(searchable)) {
    return 'local-ai-runtime';
  }
  if (
    /Platform|Setup|Provisioning|Managed Browser Operation|Private|Tor|Network|Capability Failure/iu.test(searchable)
  ) {
    return 'os-adapter';
  }
  if (/manual|permission/iu.test(searchable)) {
    return 'manual-proof';
  }
  if (/policy value|protocol|patch|replace|rollback/iu.test(searchable)) {
    return 'agent-protocol';
  }
  return 'child-agent';
}

export function capabilityStateForSection(
  sectionTitle: string,
  sourceText: string
): BrowserControlFullCatalogCapabilityState {
  const status = effectStatusForSection(sectionTitle, sourceText);
  if (status === 'future-gap') {
    return 'future-gap';
  }
  if (status === 'permission-required') {
    return 'permission-required';
  }
  if (status === 'degraded') {
    return 'degraded';
  }
  if (status === 'manual-required') {
    return 'manual-required';
  }
  if (status === 'proof-required') {
    return 'protected';
  }
  return 'available';
}

export function capabilityRequirementFor(sectionTitle: string, sourceText: string): string {
  const searchable = `${sectionTitle} ${sourceText}`;
  if (/Exact URL|active tab|page title|download source/iu.test(searchable)) {
    return 'managed-browser-or-explicit-browser-integration';
  }
  if (/Network/iu.test(searchable)) {
    return 'network-metadata-observation-only';
  }
  if (/Private|Tor|permission/iu.test(searchable)) {
    return 'explicit-permission-and-platform-proof';
  }
  if (/AI/iu.test(searchable)) {
    return 'local-ai-runtime-with-parent-enabled-analysis';
  }
  if (/Audit|Retention|Custody|Never-Collect/iu.test(searchable)) {
    return 'parent-owned-local-storage-and-redaction';
  }
  return 'browser-control-capability-registry';
}

export function proofRequirementFor(sectionTitle: string, sourceText: string): string | null {
  const searchable = `${sectionTitle} ${sourceText}`;
  if (/Exact URL|active tab|page title|download source/iu.test(searchable)) {
    return 'managed-browser-or-explicit-browser-integration';
  }
  if (/browser evidence|required proof|proof requirement/iu.test(searchable)) {
    return 'schema-valid-evidence-ref-with-runtime-custody';
  }
  if (/Network/iu.test(searchable)) {
    return 'network-evidence-must-not-be-treated-as-exact-url-or-tab-content';
  }
  if (/process|window|foreground/iu.test(searchable)) {
    return 'process-or-window-evidence-only-with-no-url-claim';
  }
  if (/AI/iu.test(searchable)) {
    return 'local-analysis-summary-and-evidence-refs-without-raw-browser-data-upload';
  }
  return null;
}

export function fallbackFor(sectionTitle: string, sourceText: string): string {
  const status = effectStatusForSection(sectionTitle, sourceText);
  if (status === 'future-gap') {
    return 'Expose as future gap or planning-only control; do not compile into enforcement.';
  }
  if (status === 'manual-required') {
    return 'Disable or degrade until manual setup/proof confirms the required browser capability.';
  }
  if (status === 'permission-required') {
    return 'Disable strict behavior until permission exists; keep observe/audit-only alternatives available.';
  }
  if (status === 'degraded') {
    return 'Show degraded capability and compile only the observable subset without exact URL/tab claims.';
  }
  if (status === 'proof-required') {
    return 'Require explicit proof before enforcement; otherwise compile observe or manual-required behavior.';
  }
  return 'Keep as authored intent until runtime wiring proves the exact effect key.';
}

export function visibilityConditionsFor(): BrowserControlFullCatalogRule[] {
  return [rule('Visible when the Browser side-panel category is selected.')];
}

export function enabledConditionsFor(sectionTitle: string, sourceText: string): BrowserControlFullCatalogRule[] {
  return [
    rule('A family, child, or device target must be selected before writing policy intent.'),
    rule(`Capability state must allow ${effectStatusForSection(sectionTitle, sourceText)} presentation.`),
  ];
}

export function validationRulesFor(sectionTitle: string, sourceText: string): BrowserControlFullCatalogRule[] {
  const proof = proofRequirementFor(sectionTitle, sourceText);
  const rules = [
    rule('Selected option ids must belong to this setting acceptedOptions list.'),
    rule('Portal writes only authored intent; child runtime owns compile, persistence, evaluation, and audit.'),
  ];
  if (proof !== null) {
    rules.push(rule(`Enforcement requires proof: ${proof}.`));
  }
  return rules;
}

function explicitOptionLabels(sourceText: string): string[] {
  const colonIndex = sourceText.indexOf(':');
  if (colonIndex === -1) {
    return [];
  }
  const suffix = sourceText
    .slice(colonIndex + 1)
    .replace(/warn\/ask/giu, 'warn, ask')
    .replace(/\.$/u, '');
  return unique(
    suffix
      .split(/,|;|\bor\b/u)
      .map((part) => cleanOptionLabel(part))
      .filter((part) => part.length > 0)
  );
}

function numericOrDurationControl(sourceText: string): BrowserControlFullCatalogControlKind {
  return /minutes|seconds|days/u.test(sourceText.toLowerCase()) ? 'number' : 'duration';
}

function options(labels: readonly string[]): BrowserControlFullCatalogOption[] {
  return labels.map((label) => {
    const value = slug(label);
    return {
      optionId: BrowserControlOptionIdSchema.parse(`browser-catalog-option-${value}`),
      label,
      value,
      originalSourceText: label,
      meaning: null,
      defaultSelected: false,
    };
  });
}

function rule(description: string): BrowserControlFullCatalogRule {
  return {
    ruleId: BrowserControlFieldIdSchema.parse(`browser-catalog-rule-${slug(description)}`),
    description,
  };
}

function cleanOptionLabel(value: string): string {
  return titleize(value.trim().replace(/\.$/u, '').replace(/\s+/gu, ' '));
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
