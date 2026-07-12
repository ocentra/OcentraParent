/* generated from crates/network-core/src/network_control_catalog_metadata_classifiers.ts.txt */

import { matchOptionalPatternValue, matchPatternValue, type PatternValue } from './catalog-metadata-text';
import type {
  NetworkControlCapabilityState,
  NetworkControlEffectStatus,
  NetworkControlRuntimeOwner,
} from './network-control-catalog-schema';

const hasOwn = <T extends object>(value: T, key: PropertyKey): key is keyof T =>
  Object.prototype.hasOwnProperty.call(value, key);

const PolicyLanePatterns = [
  [/retention|custody|audit|journal|deletion|expiry|redact|export|storage|cache/iu, 'audit'],
  [/report|summary|visible|parent sees|top /iu, 'reports'],
  [/budget|schedule|time window|time budget|network-active time|bandwidth|bytes|connection-count/iu, 'schedule'],
  [/approval|ask parent|override|parent approval/iu, 'approvals'],
  [
    /block|enforce|firewall|wfp|packet filter|vpn|proxy|tunnel|adapter|rollback|strict action|terminate|router/iu,
    'enforcement',
  ],
  [
    /evidence|dns|domain|ip|port|protocol|process|flow|metadata|exact URL|encrypted|https|indicator|attribution/iu,
    'evidence',
  ],
  [/setup|managed|mdm|entitlement|permission|profile|service installation|admin/iu, 'setup'],
] as const satisfies readonly PatternValue<string>[];

const EffectStatusPatterns = [
  [/future|later|not yet|planned|missing|gap/iu, 'future-gap'],
  [
    /manual-required|manual required|admin|privilege|service installation|driver|mdm|supervision|entitlement|router api|WFP|Windows Filtering Platform|always-on|lockdown|force all traffic/iu,
    'manual-required',
  ],
  [/permission|profile|TCC|protected|review|signing|notarization|user.*setup/iu, 'permission-required'],
  [/limited|partial|varies|ambiguous|stale|unavailable|unsupported|bypass|miss|cannot|usually cannot/iu, 'degraded'],
  [
    /exact URL|path\/query|HTTPS|decrypted|payload|page body|chat content|search terms|form values|cookies|tokens|credentials|proof|evidence id|confidence|must cite|not proof/iu,
    'proof-required',
  ],
  [/retention|custody|report|redact|local-first|audit|summary|show /iu, 'already-represented'],
] as const satisfies readonly PatternValue<NetworkControlEffectStatus>[];

const RuntimeOwnerPatterns = [
  [/Portal UI|Portal authors|does not run capture|reports|visible|show /iu, 'portal-only'],
  [/retention|custody|journal|storage|export|cache|audit|redact/iu, 'parent-owned-storage'],
  [/AI|deterministic\/AI|local AI/iu, 'local-ai-runtime'],
  [
    /firewall|WFP|packet filter|VPN|proxy|router|DNS|resolver|Network Extension|DevicePolicyManager|VpnService|adapter|process|endpoint|ETW|IP Helper/iu,
    'os-adapter',
  ],
  [/policy|rule|compile|decision|fallback|conflict|proof/iu, 'rust-parent-runtime'],
] as const satisfies readonly PatternValue<NetworkControlRuntimeOwner>[];

const CapabilityRequirementPatterns = [
  [/exact URL|path\/query|active tab|page title|download source/iu, 'managed-browser-or-explicit-url-filter-proof'],
  [/domain|DNS|resolver/iu, 'dns-or-domain-attribution-source-with-confidence'],
  [/IP|port|protocol|flow|endpoint|process/iu, 'local-network-flow-metadata-evidence'],
  [/block|firewall|WFP|packet filter|VPN|router|strict|enforcement/iu, 'real-platform-network-adapter-proof'],
  [/retention|custody|report|audit|storage/iu, 'local-first-custody-and-retention-policy'],
] as const satisfies readonly PatternValue<string>[];

const ProofRequirementPatterns = [
  [
    /exact URL|path\/query|active tab|page title|download source/iu,
    'Exact URL evidence requires managed browser, explicit URL filter, or proxy proof; passive network flow metadata is not enough.',
  ],
  [
    /decrypted|payload|page body|chat content|search terms|form values|cookies|tokens|credentials|packet payload/iu,
    'Network controls must not collect decrypted content or payload fields; use metadata evidence only.',
  ],
  [
    /block|firewall|WFP|packet filter|VPN|router|always-on|lockdown|force all traffic|strict/iu,
    'Strict network enforcement requires parent rule, evidence reference, real adapter result, audit event, and rollback or unavailable state.',
  ],
  [
    /domain|DNS|IP|port|protocol|flow|process|indicator|attribution|confidence|evidence id/iu,
    'Network claims require stored metadata evidence with source, freshness, confidence, and custody labels.',
  ],
] as const satisfies readonly PatternValue<string>[];

const SourceFallbackPatterns = [
  [
    /exact URL|path\/query|active tab|page title/iu,
    'Hide or disable exact URL controls unless managed browser, explicit URL filter, or proxy proof exists.',
  ],
  [
    /decrypted|payload|page body|chat content|search terms|form values|cookies|tokens|credentials/iu,
    'Never collect decrypted content or sensitive payload fields in the network-control catalog.',
  ],
] as const satisfies readonly PatternValue<string>[];

const EffectStatusFallbacks = {
  'manual-required':
    'Show manual-required until setup, privileges, and adapter proof exist; compile observe or unavailable fallback.',
  degraded: 'Render degraded state and keep unsupported behavior out of compiled enforcement plans.',
  'proof-required':
    'Require evidence proof before strict effect; otherwise fall back to observe, warn, ask, or unavailable.',
  'future-gap': 'Expose as future or planning-only and do not claim current runtime behavior.',
} as const satisfies Partial<Record<NetworkControlEffectStatus, string>>;

const CapabilityStateByEffectStatus = {
  'already-represented': 'available',
  'needs-effect-wiring': 'degraded',
  'manual-required': 'manual-required',
  'permission-required': 'permission-required',
  'permission-limited': 'permission-limited',
  'future-gap': 'future-gap',
  degraded: 'degraded',
  unavailable: 'unavailable',
  'proof-required': 'protected',
} as const satisfies Record<NetworkControlEffectStatus, NetworkControlCapabilityState>;

const SourceStateCapabilityMap = {
  ready: 'available',
  'ready-if-browser-capability-ready': 'protected',
  'manual-required': 'manual-required',
  'not-implemented': 'unavailable',
  'authoring-only': 'disabled',
} as const satisfies Partial<Record<string, NetworkControlCapabilityState>>;

export function networkPolicyLaneFor(sectionTitle: string, groupTitle: string, sourceText: string): string {
  const searchable = `${sectionTitle} ${groupTitle} ${sourceText}`;
  return /retention|custody|audit|journal|deletion|expiry|redact|export|storage|cache/iu.test(searchable) &&
    /report|summary|visible|parent sees|top /iu.test(searchable)
    ? 'reports'
    : matchPatternValue(searchable, PolicyLanePatterns, 'rules');
}

export function networkEffectStatusFor(
  sectionTitle: string,
  groupTitle: string,
  sourceText: string
): NetworkControlEffectStatus {
  return matchPatternValue(`${sectionTitle} ${groupTitle} ${sourceText}`, EffectStatusPatterns, 'needs-effect-wiring');
}

export function networkRuntimeOwnerFor(
  sectionTitle: string,
  groupTitle: string,
  sourceText: string
): NetworkControlRuntimeOwner {
  return matchPatternValue(`${sectionTitle} ${groupTitle} ${sourceText}`, RuntimeOwnerPatterns, 'child-agent');
}

export function networkCapabilityStateFor(effectStatus: NetworkControlEffectStatus): NetworkControlCapabilityState {
  return CapabilityStateByEffectStatus[effectStatus];
}

export function networkCapabilityStateFromSourceState(sourceState: string): NetworkControlCapabilityState {
  return hasOwn(SourceStateCapabilityMap, sourceState) ? SourceStateCapabilityMap[sourceState] : 'degraded';
}

export function networkCapabilityRequirementFor(sectionTitle: string, groupTitle: string, sourceText: string): string {
  return matchPatternValue(
    `${sectionTitle} ${groupTitle} ${sourceText}`,
    CapabilityRequirementPatterns,
    'network-control-capability-registry'
  );
}

export function networkProofRequirementFor(
  sectionTitle: string,
  groupTitle: string,
  sourceText: string
): string | null {
  return matchOptionalPatternValue(`${sectionTitle} ${groupTitle} ${sourceText}`, ProofRequirementPatterns);
}

export function networkFallbackFor(effectStatus: NetworkControlEffectStatus, sourceText: string): string {
  return (
    matchOptionalPatternValue(sourceText, SourceFallbackPatterns) ??
    (hasOwn(EffectStatusFallbacks, effectStatus) ? EffectStatusFallbacks[effectStatus] : undefined) ??
    'Portal renders the control; child-agent/runtime ownership remains explicit.'
  );
}
