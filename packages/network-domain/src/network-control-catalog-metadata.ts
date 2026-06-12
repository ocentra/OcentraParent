import type {
  NetworkControlCapabilityState,
  NetworkControlCardKind,
  NetworkControlEffectStatus,
  NetworkControlKind,
  NetworkControlLayoutHints,
  NetworkControlOption,
  NetworkControlRuntimeOwner,
  NetworkControlSelectionMode,
} from './network-control-catalog-schema';

export function policyLaneFor(sectionTitle: string, groupTitle: string, sourceText: string) {
  const searchable = `${sectionTitle} ${groupTitle} ${sourceText}`;
  if (/retention|custody|audit|journal|deletion|expiry|redact|export|storage|cache/iu.test(searchable)) {
    return /report|summary|visible|parent sees|top /iu.test(searchable) ? 'reports' : 'audit';
  }
  if (
    /budget|schedule|time window|time budget|network-active time|bandwidth|bytes|connection-count/iu.test(searchable)
  ) {
    return 'schedule';
  }
  if (/approval|ask parent|override|parent approval/iu.test(searchable)) {
    return 'approvals';
  }
  if (
    /block|enforce|firewall|wfp|packet filter|vpn|proxy|tunnel|adapter|rollback|strict action|terminate|router/iu.test(
      searchable
    )
  ) {
    return 'enforcement';
  }
  if (
    /evidence|dns|domain|ip|port|protocol|process|flow|metadata|exact URL|encrypted|https|indicator|attribution/iu.test(
      searchable
    )
  ) {
    return 'evidence';
  }
  if (/setup|managed|mdm|entitlement|permission|profile|service installation|admin/iu.test(searchable)) {
    return 'setup';
  }
  return 'rules';
}

export function controlKindFor(sourceText: string, explicitKind: string | null): NetworkControlKind {
  if (explicitKind === 'boolean') {
    return 'toggle';
  }
  if (explicitKind === 'single-choice') {
    return 'single-choice';
  }
  if (explicitKind === 'multi-choice') {
    return 'multi-choice';
  }
  if (/^Capability matrix row \|/u.test(sourceText)) {
    return 'read-only-status';
  }
  const searchable = sourceText.toLowerCase();
  if (/budget|bytes|count|minutes|duration|timer|threshold|retention/u.test(searchable)) {
    return /retention|custody|deletion|expiry/u.test(searchable) ? 'retention' : 'number';
  }
  if (/schedule|time window/u.test(searchable)) {
    return 'schedule';
  }
  if (/allow|block|warn|ask|enforce|terminate|force|route|adapter|actions/u.test(searchable)) {
    return 'action-list';
  }
  if (/target|domain|ip|port|protocol|process|exception|indicator|source|field|scope|category/u.test(searchable)) {
    return 'multi-choice';
  }
  return explicitOptionLabels(sourceText).length > 0 ? 'single-choice' : 'toggle';
}

export function selectionModeFor(
  controlKind: NetworkControlKind,
  optionsForSetting: readonly NetworkControlOption[]
): NetworkControlSelectionMode {
  if (controlKind === 'multi-choice' || controlKind === 'action-list' || controlKind === 'target-list') {
    return 'multi';
  }
  if (optionsForSetting.length > 4 && controlKind !== 'read-only-status') {
    return 'multi';
  }
  return 'single';
}

export function cardKindFor(
  controlKind: NetworkControlKind,
  selectionMode: NetworkControlSelectionMode,
  optionsForSetting: readonly NetworkControlOption[]
): NetworkControlCardKind {
  if (controlKind === 'toggle') {
    return 'toggle';
  }
  if (controlKind === 'schedule') {
    return 'schedule-card';
  }
  if (controlKind === 'rule-list') {
    return 'rule-list-card';
  }
  if (controlKind === 'target-list') {
    return 'target-list-card';
  }
  if (controlKind === 'retention') {
    return 'retention-card';
  }
  if (controlKind === 'read-only-status') {
    return 'status-card';
  }
  if (controlKind === 'number' || controlKind === 'duration') {
    return 'number-card';
  }
  if (selectionMode === 'multi') {
    return optionsForSetting.length > 4 ? 'multi-choice-many' : 'multi-choice-normal';
  }
  return optionsForSetting.length > 4 ? 'single-choice-many' : 'single-choice-compact';
}

export function layoutHintsFor(
  controlKind: NetworkControlKind,
  selectionMode: NetworkControlSelectionMode,
  optionsForSetting: readonly NetworkControlOption[]
): NetworkControlLayoutHints {
  const manyOptions = optionsForSetting.length > 4;
  const listLike = selectionMode === 'multi' || controlKind === 'action-list' || controlKind === 'target-list';
  return {
    preferredColumnSpan: manyOptions || controlKind === 'retention' || controlKind === 'read-only-status' ? 2 : 1,
    collapsible: manyOptions || listLike || controlKind === 'read-only-status',
    searchableOptions: manyOptions,
    optionGroupCount: manyOptions ? Math.ceil(optionsForSetting.length / 4) : 1,
    showAsMatrixWhenLarge: manyOptions && listLike,
    showSelectedCount: listLike,
  };
}

export function effectStatusFor(
  sectionTitle: string,
  groupTitle: string,
  sourceText: string
): NetworkControlEffectStatus {
  const searchable = `${sectionTitle} ${groupTitle} ${sourceText}`;
  if (/future|later|not yet|planned|missing|gap/iu.test(sourceText)) {
    return 'future-gap';
  }
  if (
    /manual-required|manual required|admin|privilege|service installation|driver|mdm|supervision|entitlement|router api|WFP|Windows Filtering Platform|always-on|lockdown|force all traffic/iu.test(
      searchable
    )
  ) {
    return 'manual-required';
  }
  if (/permission|profile|TCC|protected|review|signing|notarization|user.*setup/iu.test(searchable)) {
    return 'permission-required';
  }
  if (
    /limited|partial|varies|ambiguous|stale|unavailable|unsupported|bypass|miss|cannot|usually cannot/iu.test(
      searchable
    )
  ) {
    return 'degraded';
  }
  if (
    /exact URL|path\/query|HTTPS|decrypted|payload|page body|chat content|search terms|form values|cookies|tokens|credentials|proof|evidence id|confidence|must cite|not proof/iu.test(
      searchable
    )
  ) {
    return 'proof-required';
  }
  if (/retention|custody|report|redact|local-first|audit|summary|show /iu.test(searchable)) {
    return 'already-represented';
  }
  return 'needs-effect-wiring';
}

export function runtimeOwnerFor(
  sectionTitle: string,
  groupTitle: string,
  sourceText: string
): NetworkControlRuntimeOwner {
  const searchable = `${sectionTitle} ${groupTitle} ${sourceText}`;
  if (/Portal UI|Portal authors|does not run capture|reports|visible|show /iu.test(searchable)) {
    return 'portal-only';
  }
  if (/retention|custody|journal|storage|export|cache|audit|redact/iu.test(searchable)) {
    return 'parent-owned-storage';
  }
  if (/AI|deterministic\/AI|local AI/iu.test(searchable)) {
    return 'local-ai-runtime';
  }
  if (
    /firewall|WFP|packet filter|VPN|proxy|router|DNS|resolver|Network Extension|DevicePolicyManager|VpnService|adapter|process|endpoint|ETW|IP Helper/iu.test(
      searchable
    )
  ) {
    return 'os-adapter';
  }
  if (/policy|rule|compile|decision|fallback|conflict|proof/iu.test(searchable)) {
    return 'parent-domain';
  }
  return 'child-agent';
}

export function capabilityStateFor(effectStatus: NetworkControlEffectStatus): NetworkControlCapabilityState {
  if (effectStatus === 'manual-required') {
    return 'manual-required';
  }
  if (effectStatus === 'permission-required') {
    return 'permission-required';
  }
  if (effectStatus === 'permission-limited') {
    return 'permission-limited';
  }
  if (effectStatus === 'future-gap') {
    return 'future-gap';
  }
  if (effectStatus === 'degraded') {
    return 'degraded';
  }
  if (effectStatus === 'unavailable') {
    return 'unavailable';
  }
  if (effectStatus === 'proof-required') {
    return 'protected';
  }
  return 'available';
}

export function capabilityStateFromSourceState(sourceState: string): NetworkControlCapabilityState {
  if (sourceState === 'ready') {
    return 'available';
  }
  if (sourceState === 'ready-if-browser-capability-ready') {
    return 'protected';
  }
  if (sourceState === 'manual-required') {
    return 'manual-required';
  }
  if (sourceState === 'not-implemented') {
    return 'unavailable';
  }
  if (sourceState === 'authoring-only') {
    return 'disabled';
  }
  return 'degraded';
}

export function capabilityRequirementFor(sectionTitle: string, groupTitle: string, sourceText: string) {
  const searchable = `${sectionTitle} ${groupTitle} ${sourceText}`;
  if (/exact URL|path\/query|active tab|page title|download source/iu.test(searchable)) {
    return 'managed-browser-or-explicit-url-filter-proof';
  }
  if (/domain|DNS|resolver/iu.test(searchable)) {
    return 'dns-or-domain-attribution-source-with-confidence';
  }
  if (/IP|port|protocol|flow|endpoint|process/iu.test(searchable)) {
    return 'local-network-flow-metadata-evidence';
  }
  if (/block|firewall|WFP|packet filter|VPN|router|strict|enforcement/iu.test(searchable)) {
    return 'real-platform-network-adapter-proof';
  }
  if (/retention|custody|report|audit|storage/iu.test(searchable)) {
    return 'local-first-custody-and-retention-policy';
  }
  return 'network-control-capability-registry';
}

export function proofRequirementFor(sectionTitle: string, groupTitle: string, sourceText: string) {
  const searchable = `${sectionTitle} ${groupTitle} ${sourceText}`;
  if (/exact URL|path\/query|active tab|page title|download source/iu.test(searchable)) {
    return 'Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.';
  }
  if (
    /decrypted|payload|page body|chat content|search terms|form values|cookies|tokens|credentials|packet payload/iu.test(
      searchable
    )
  ) {
    return 'Network controls must not collect decrypted content or payload fields; use metadata evidence only.';
  }
  if (/block|firewall|WFP|packet filter|VPN|router|always-on|lockdown|force all traffic|strict/iu.test(searchable)) {
    return 'Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.';
  }
  if (/domain|DNS|IP|port|protocol|flow|process|indicator|attribution|confidence|evidence id/iu.test(searchable)) {
    return 'Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.';
  }
  return null;
}

export function fallbackFor(effectStatus: NetworkControlEffectStatus, sourceText: string) {
  if (/exact URL|path\/query|active tab|page title/iu.test(sourceText)) {
    return 'Hide or disable exact URL controls unless managed browser, explicit URL filter, or proxy proof exists.';
  }
  if (
    /decrypted|payload|page body|chat content|search terms|form values|cookies|tokens|credentials/iu.test(sourceText)
  ) {
    return 'Never collect decrypted content or sensitive payload fields in the network-control catalog.';
  }
  if (effectStatus === 'manual-required') {
    return 'Show manual-required until setup, privileges, and adapter proof exist; compile observe or unavailable fallback.';
  }
  if (effectStatus === 'degraded') {
    return 'Render degraded state and keep unsupported behavior out of compiled enforcement plans.';
  }
  if (effectStatus === 'proof-required') {
    return 'Require evidence proof before strict effect; otherwise fall back to observe, warn, ask, or unavailable.';
  }
  if (effectStatus === 'future-gap') {
    return 'Expose as future or planning-only and do not claim current runtime behavior.';
  }
  return 'Portal renders the control; child-agent/runtime ownership remains explicit.';
}

export function helperTextFor(sectionTitle: string, groupTitle: string, sourceText: string) {
  return (
    proofRequirementFor(sectionTitle, groupTitle, sourceText) ??
    capabilityRequirementFor(sectionTitle, groupTitle, sourceText)
  );
}

export function questionFromSourceText(sourceText: string, explicitQuestion: string | null) {
  if (explicitQuestion !== null && explicitQuestion.length > 0) {
    return explicitQuestion;
  }
  const trimmed = sourceText.replace(/\.$/u, '');
  if (trimmed.endsWith('?')) {
    return trimmed;
  }
  if (/^Capability matrix row \|/u.test(trimmed)) {
    const capability = /Capability=([^|]+)/u.exec(trimmed)?.[1]?.trim() ?? 'network capability';
    return `Represent ${capability} capability status.`;
  }
  const colonIndex = trimmed.indexOf(':');
  if (colonIndex !== -1) {
    return `Configure ${trimmed.slice(0, colonIndex).toLowerCase()}.`;
  }
  return `Represent ${trimmed.charAt(0).toLowerCase()}${trimmed.slice(1)}?`;
}

export function explicitOptionLabels(sourceText: string) {
  const matrixOptions = matrixOptionLabels(sourceText);
  if (matrixOptions.length > 0) {
    return matrixOptions;
  }
  const colonIndex = sourceText.indexOf(':');
  if (colonIndex === -1) {
    return [];
  }
  return sourceText
    .slice(colonIndex + 1)
    .replace(/\.$/u, '')
    .split(/,|;|\bor\b/iu)
    .map((part) => cleanOptionLabel(part))
    .filter((part) => part.length > 0);
}

function matrixOptionLabels(sourceText: string) {
  if (!/^Capability matrix row \|/u.test(sourceText)) {
    return [];
  }
  return sourceText
    .split(' | ')
    .slice(1)
    .map((part) => {
      const separatorIndex = part.indexOf('=');
      const heading = separatorIndex === -1 ? 'Cell' : part.slice(0, separatorIndex);
      const value = separatorIndex === -1 ? part : part.slice(separatorIndex + 1);
      return `${heading}: ${value}`;
    });
}

export function slugToken(value: string) {
  const slugged = value
    .toLowerCase()
    .replace(/[^a-z0-9]+/gu, '-')
    .replace(/^-+|-+$/gu, '');
  return slugged.length > 0 ? slugged : 'item';
}

export function titleFromToken(value: string) {
  return value
    .split('-')
    .map((part) => part.charAt(0).toUpperCase() + part.slice(1))
    .join(' ');
}

function cleanOptionLabel(value: string) {
  return titleFromToken(value.trim().replace(/\.$/u, '').replace(/\s+/gu, '-'));
}
