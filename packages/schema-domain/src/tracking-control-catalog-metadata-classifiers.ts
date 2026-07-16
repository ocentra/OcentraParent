/* thin adapter helpers for Rust-seeded tracking control catalog metadata */

import { matchOptionalPatternValue, matchPatternValue, type PatternValue } from './catalog-metadata-text';
import type {
  TrackingControlCapabilityState,
  TrackingControlEffectStatus,
  TrackingControlKind,
  TrackingControlRuntimeOwner,
} from './tracking-control-catalog-schema';

const hasOwn = <T extends object>(value: T, key: PropertyKey): key is keyof T =>
  Object.prototype.hasOwnProperty.call(value, key);

const PolicyLanePatterns = [
  [/live tracking|live map|live session|cadence|temporary live/iu, 'live'],
  [/geofence|place|arrival|departure|dwell|radius|known place|school|home/iu, 'places'],
  [/permission|disclosure|setup|foreground|background|precise|approximate|service disabled/iu, 'setup'],
  [/history|retention|custody|delete|export|storage|journal|audit|parent reveal/iu, 'data'],
  [/report|map|family map|last known|last seen|summary|freshness|accuracy label/iu, 'reports'],
  [/alert|notification|notify|check-in|prompt|response|missed|unanswered/iu, 'approvals'],
  [/platform|windows|macOS|linux|android|ios|ipadOS|desktop|mobile|mdm|device-owner|lost mode/iu, 'platform'],
  [/enforce|policy decision|block|limit|strict|fallback|proof unavailable/iu, 'enforcement'],
  [/evidence|source|provider|adapter|timestamp|accuracy|fresh|current|coordinate|capability matrix/iu, 'evidence'],
  [/schedule|time window|expected|between|after|before/iu, 'schedule'],
] as const satisfies readonly PatternValue<string>[];

const ControlKindPatterns = [
  [/Capability matrix row/iu, 'read-only-status'],
  [/enabled|show on map|allow approximate|child-facing disclosure|toggle|switch/iu, 'toggle'],
  [/minutes|meters|radius|duration|cadence|interval|maximum|stale after|unanswered after|count limit/iu, 'number'],
  [/schedule|time window|school day|overnight|expected arrival/iu, 'schedule'],
  [/retention|delete|history|export|custody|storage/iu, 'retention'],
  [/place|home|school|friend's house|known places/iu, 'place-list'],
  [/geofence|radius|arrival|departure|dwell/iu, 'geofence-list'],
  [/possible|allowed|types|states|fields|provider kind|permission state|reasons|responses|alerts/iu, 'multi-choice'],
  [/mode|posture|fallback|when |what |minimum|required|default|state|level|handling/iu, 'single-choice'],
] as const satisfies readonly PatternValue<TrackingControlKind>[];

const EffectStatusPatterns = [
  [/Portal-side policy evaluation|Guessing current location from stale last-known evidence/iu, 'unavailable'],
  [
    /not a generic third-party API|not-implemented|future|relay|Find My|lost mode|MDM|supervision|iOS.*limits/iu,
    'future-gap',
  ],
  [
    /denied|revok|permission|background|foreground|precise|approximate|reduced-accuracy|service disabled|entitlement/iu,
    'permission-required',
  ],
  [
    /manual|required setup|manual-required|user-entered|parent must|child must respond|consent|disclosure/iu,
    'manual-required',
  ],
  [
    /stale|offline|battery|throttle|low power|airplane|no signal|approximate|weak|limited|degrade|not instant|may fail/iu,
    'degraded',
  ],
  [
    /current location|fresh fix|live tracking|continuous|geofence|arrival|departure|dwell|policy decision|exact|coordinates|proof/iu,
    'proof-required',
  ],
  [
    /history|retention|custody|audit|report|map|delete|export|summary|label|source|timestamp|accuracy/iu,
    'already-represented',
  ],
] as const satisfies readonly PatternValue<TrackingControlEffectStatus>[];

const RuntimeOwnerPatterns = [
  [/Portal|UI must|authoring UI|Future UI|side panel/iu, 'portal-only'],
  [/policy decision|Enforce location-based policy|local policy/iu, 'child-agent'],
  [
    /history|retention|custody|export|delete|parent-owned storage|report|map|journal|query store/iu,
    'parent-owned-storage',
  ],
  [
    /location API|GPS|GNSS|Wi-Fi|cellular|Bluetooth|fused provider|geofence|background|foreground|permission|OS|adapter|lost mode|MDM/iu,
    'os-adapter',
  ],
  [/child agent|check-in|prompt|local policy|offline|last known|device state|battery|radio/iu, 'child-agent'],
  [/manual|user-entered|setup|supervised/iu, 'manual-proof'],
] as const satisfies readonly PatternValue<TrackingControlRuntimeOwner>[];

const CapabilityRequirementPatterns = [
  [
    /current location|fresh fix|live|coordinate|GPS|GNSS|fused provider/iu,
    'location-provider-freshness-accuracy-proof',
  ],
  [/background|always|throttle|battery|foreground service/iu, 'background-location-permission-and-disclosure-proof'],
  [/precise|accuracy|reduced|approximate/iu, 'precision-permission-and-accuracy-proof'],
  [/geofence|arrival|departure|dwell|radius/iu, 'geofence-region-schedule-transition-proof'],
  [/history|retention|custody|export|delete|journal/iu, 'local-history-custody-retention-proof'],
  [/lost mode|Find My|MDM|supervised|device-owner/iu, 'platform-managed-lost-mode-or-supervision-proof'],
  [/relay|cloud|remote live|away from LAN/iu, 'authenticated-relay-proof-without-default-location-history-storage'],
] as const satisfies readonly PatternValue<string>[];

const ProofRequirementPatterns = [
  [
    /policy decision|Enforce location-based policy|Missing proof/iu,
    'Location-based policy decisions require fresh location evidence, typed parent rule, deterministic child-agent evaluation, and explicit ask/warn/report fallback.',
  ],
  [
    /current location|fresh fix|live tracking|continuous|exact|last known/iu,
    'Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.',
  ],
  [
    /geofence|arrival|departure|dwell|radius/iu,
    'Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.',
  ],
  [
    /history|retention|custody|export|delete|storage/iu,
    'Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.',
  ],
  [
    /permission|background|foreground|precise|approximate|disclosure/iu,
    'Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.',
  ],
] as const satisfies readonly PatternValue<string>[];

const SourceFallbackPatterns = [
  [
    /Portal-side policy evaluation/iu,
    'Portal renders authoring metadata and reports only; child-agent/local runtime owns location policy evaluation.',
  ],
  [
    /last known|current location|live map|continuous/iu,
    'Show freshness, accuracy, and source labels; never imply stale last-known evidence is current live location.',
  ],
  [
    /history|storage|Ocentra-hosted|retention|custody/iu,
    'Keep location history local or parent-owned by default; never silently upload location history to Ocentra-hosted storage.',
  ],
  [
    /geofence|arrival|departure|dwell/iu,
    'When geofence proof is missing or throttled, degrade to ask, warn, report, or manual check-in instead of claiming enforcement.',
  ],
] as const satisfies readonly PatternValue<string>[];

const EffectStatusFallbacks = {
  'permission-required':
    'Disable strict tracking until required OS permission, precision, and disclosure proof are present.',
  'manual-required': 'Show manual-required setup and compile an unavailable or ask/report fallback until proof exists.',
  'future-gap': 'Expose as unavailable or future only; do not claim current runtime support.',
  degraded: 'Render degraded state with stale, offline, battery, throttle, or approximate labels.',
  'proof-required':
    'Require evidence proof before strict alerts or policy effects; otherwise fall back to observe, ask, warn, or report.',
} as const satisfies Partial<Record<TrackingControlEffectStatus, string>>;

const CapabilityStateByEffectStatus = {
  'already-represented': 'available',
  'needs-effect-wiring': 'degraded',
  'permission-required': 'permission-required',
  'manual-required': 'manual-required',
  'permission-limited': 'permission-limited',
  'future-gap': 'future-gap',
  unavailable: 'unavailable',
  degraded: 'degraded',
  'proof-required': 'protected',
} as const satisfies Record<TrackingControlEffectStatus, TrackingControlCapabilityState>;

const SourceStatePatterns = [
  [/ready/iu, 'available'],
  [/permission/iu, 'permission-required'],
  [/manual/iu, 'manual-required'],
  [/blocked|protected/iu, 'protected'],
  [/not-implemented|future/iu, 'future-gap'],
] as const satisfies readonly PatternValue<TrackingControlCapabilityState>[];

export function trackingPolicyLaneFor(sectionTitle: string, groupTitle: string, sourceText: string): string {
  return matchPatternValue(`${sectionTitle} ${groupTitle} ${sourceText}`, PolicyLanePatterns, 'rules');
}

export function trackingControlKindFor(
  sourceText: string,
  explicitControlKind: string | null,
  proposalKinds: Readonly<Record<string, TrackingControlKind>>
): TrackingControlKind {
  return explicitControlKind === null
    ? matchPatternValue(sourceText, ControlKindPatterns, 'multi-choice')
    : (proposalKinds[explicitControlKind] ?? 'multi-choice');
}

export function trackingEffectStatusFor(
  sectionTitle: string,
  groupTitle: string,
  sourceText: string
): TrackingControlEffectStatus {
  return matchPatternValue(`${sectionTitle} ${groupTitle} ${sourceText}`, EffectStatusPatterns, 'needs-effect-wiring');
}

export function trackingRuntimeOwnerFor(
  sectionTitle: string,
  groupTitle: string,
  sourceText: string
): TrackingControlRuntimeOwner {
  return matchPatternValue(`${sectionTitle} ${groupTitle} ${sourceText}`, RuntimeOwnerPatterns, 'rust-parent-runtime');
}

export function trackingCapabilityStateFor(effectStatus: TrackingControlEffectStatus): TrackingControlCapabilityState {
  return CapabilityStateByEffectStatus[effectStatus];
}

export function trackingCapabilityStateFromSourceState(sourceState: string): TrackingControlCapabilityState {
  return matchPatternValue(sourceState, SourceStatePatterns, 'unavailable');
}

export function trackingCapabilityRequirementFor(sectionTitle: string, groupTitle: string, sourceText: string): string {
  return matchPatternValue(
    `${sectionTitle} ${groupTitle} ${sourceText}`,
    CapabilityRequirementPatterns,
    'location-capability-registry'
  );
}

export function trackingProofRequirementFor(
  sectionTitle: string,
  groupTitle: string,
  sourceText: string
): string | null {
  return matchOptionalPatternValue(`${sectionTitle} ${groupTitle} ${sourceText}`, ProofRequirementPatterns);
}

export function trackingFallbackFor(effectStatus: TrackingControlEffectStatus, sourceText: string): string {
  return (
    matchOptionalPatternValue(sourceText, SourceFallbackPatterns) ??
    (hasOwn(EffectStatusFallbacks, effectStatus) ? EffectStatusFallbacks[effectStatus] : undefined) ??
    'Portal renders authoring metadata only; child-agent and OS adapters own tracking, sampling, persistence, audit, and fallback behavior.'
  );
}
