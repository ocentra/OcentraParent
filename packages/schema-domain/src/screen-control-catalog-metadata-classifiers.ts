/* thin adapter helpers for Rust-seeded screen control catalog metadata */

import { matchOptionalPatternValue, matchPatternValue, type PatternValue } from './catalog-metadata-text';
import type {
  ScreenControlCatalogCapabilityState,
  ScreenControlCatalogEffectStatus,
  ScreenControlCatalogRuntimeOwner,
  ScreenControlCatalogSourceKind,
  ScreenControlCatalogUiTab,
} from './screen-control-catalog-schema';

const UiTabPatterns = [
  [/schedule|cadence|trigger|time/i, 'schedule'],
  [/approval|ask[- ]parent/i, 'approvals'],
  [/audit|proof|decision/i, 'audit'],
  [/report|disclosure|child-facing/i, 'reports'],
  [/queue|retention|custody|delete|redaction|minimization|store|storage/i, 'data'],
  [/OCR|vision|model|classification|confidence|AI|analysis/i, 'ai'],
  [/platform|Windows|macOS|Linux|Android|iOS|permission|protected|capture scope/i, 'platform'],
  [/setup|manual test|capability registry|management/i, 'setup'],
  [/policy|rule|target/i, 'rules'],
  [/enforce|block|limit/i, 'enforcement'],
] as const satisfies readonly PatternValue<ScreenControlCatalogUiTab>[];

const EffectStatusPatterns = [
  [/future|authoring-only|ReplayKit|iOS|iPadOS/i, 'future-gap'],
  [/hosted processing|cloud\/API AI|hidden capture|permanent screenshot|retain raw|raw screenshot|continuous/i, 'unavailable'],
  [/proof|validated|exact URL|enforcement|policy use|evidence ref|deletion state/i, 'proof-required'],
  [/manual-required|manual proof|real host|platform|Windows|macOS|Linux|Android/i, 'manual-required'],
  [/unavailable|degraded|low confidence|adapter error|model unavailable|queue unavailable|redaction unavailable/i, 'degraded'],
  [/report|audit|custody|retention|delete|redaction|disclosure/i, 'already-represented'],
] as const satisfies readonly PatternValue<ScreenControlCatalogEffectStatus>[];

const RuntimeOwnerPatterns = [
  [/Portal|report|disclosure|visible/i, 'portal-only'],
  [/audit|journal|SQLite|custody|retention|delete|storage|queue/i, 'parent-owned-storage'],
  [/OCR|vision|classification|model|AI|redaction|confidence/i, 'local-ai-runtime'],
  [/platform|capture|permission|protected|Windows|macOS|Linux|Android|iOS|window|screen/i, 'os-adapter'],
  [/command|patch|replace|rollback|protocol|agent rule/i, 'agent-protocol'],
  [/proof|manual/i, 'manual-proof'],
] as const satisfies readonly PatternValue<ScreenControlCatalogRuntimeOwner>[];

const CapabilityRequirementPatterns = [
  [/exact URL|managed browser/i, 'managed-browser-evidence-required-for-exact-web-claims'],
  [/capture|screen|window|display|recording/i, 'platform-capture-capability-and-permission-proof'],
  [/queue|encrypt|delete|TTL/i, 'encrypted-local-temp-queue-with-deletion-proof'],
  [/OCR|vision|classification|model|AI/i, 'local-ocr-vision-runtime-with-schema-valid-output'],
  [/redaction/i, 'local-redaction-validation-before-summary-storage'],
] as const satisfies readonly PatternValue<string>[];

const ProofRequirementPatterns = [
  [/exact URL|web claims/i, 'managed-browser-evidence-required'],
  [/enforcement|policy use|policy can use|policy decision/i, 'validated-screen-summary-evidence-ref-deleted-image-and-deterministic-policy-decision'],
  [/queue|delete|deletion|raw image|frame/i, 'encrypted-temporary-queue-and-raw-capture-deletion-proof'],
  [/capture|screen|window|display|recording|permission|protected/i, 'real-platform-capture-permission-and-scope-proof'],
  [/OCR|vision|classification|model|AI|redaction/i, 'schema-valid-local-analysis-output-with-confidence-and-redaction-state'],
] as const satisfies readonly PatternValue<string>[];

const CapabilityStateByEffectStatus = {
  'future-gap': 'future-gap',
  unavailable: 'unavailable',
  'permission-limited': 'permission-limited',
  'manual-required': 'manual-required',
  degraded: 'degraded',
} as const satisfies Partial<Record<ScreenControlCatalogEffectStatus, ScreenControlCatalogCapabilityState>>;

const FallbackByEffectStatus = {
  unavailable: 'Disable or reject this state; do not retain raw capture or use hosted child screen processing by default.',
  'future-gap': 'Expose as future/manual-required planning only; do not compile into capture or enforcement.',
  'manual-required': 'Show manual-required until real platform or host proof demonstrates the capability.',
  degraded: 'Show degraded/unavailable state and compile only report-only or retry-within-ttl behavior.',
  'proof-required': 'Require validated summary, evidence refs, deletion proof, and deterministic policy before policy use.',
} as const satisfies Partial<Record<ScreenControlCatalogEffectStatus, string>>;

export function screenUiTabFor(sectionTitle: string, groupTitle: string, sourceText: string): ScreenControlCatalogUiTab {
  return matchPatternValue(`${sectionTitle} ${groupTitle} ${sourceText}`, UiTabPatterns, 'evidence');
}

export function screenEffectStatusFor(
  sourceKind: ScreenControlCatalogSourceKind,
  sectionTitle: string,
  sourceText: string
): ScreenControlCatalogEffectStatus {
  const searchable = `${sectionTitle} ${sourceText}`;
  if (/permission|consent|protected|locked|secure|DRM|TCC|entitlement/i.test(searchable)) {
    return /limited/i.test(searchable) ? 'permission-limited' : 'permission-required';
  }
  return matchPatternValue(
    searchable,
    EffectStatusPatterns,
    sourceKind === 'agent-rule' || sourceKind === 'update-command' ? 'already-represented' : 'needs-effect-wiring'
  );
}

export function screenCapabilityStateFor(
  status: ScreenControlCatalogEffectStatus,
  sourceText: string
): ScreenControlCatalogCapabilityState {
  if (status === 'permission-required') {
    return /protected|secure|DRM/i.test(sourceText) ? 'protected' : 'permission-required';
  }
  return CapabilityStateByEffectStatus[status] ?? (/disabled by default|disabled-by-parent/i.test(sourceText) ? 'disabled' : 'available');
}

export function screenRuntimeOwnerFor(
  sectionTitle: string,
  sourceText: string
): ScreenControlCatalogRuntimeOwner {
  return matchPatternValue(`${sectionTitle} ${sourceText}`, RuntimeOwnerPatterns, 'child-agent');
}

export function screenCapabilityRequirementFor(sectionTitle: string, sourceText: string): string {
  return matchPatternValue(
    `${sectionTitle} ${sourceText}`,
    CapabilityRequirementPatterns,
    'screen-analysis-capability-registry'
  );
}

export function screenProofRequirementFor(sectionTitle: string, sourceText: string): string | null {
  return matchOptionalPatternValue(`${sectionTitle} ${sourceText}`, ProofRequirementPatterns);
}

export function screenFallbackFor(status: ScreenControlCatalogEffectStatus): string {
  return (
    FallbackByEffectStatus[status] ??
    'Portal renders authored intent; child agent owns capture gating, queue, analysis, compile, and audit.'
  );
}
