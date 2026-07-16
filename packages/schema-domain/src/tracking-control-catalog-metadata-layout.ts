/* thin adapter helpers for Rust-seeded tracking control catalog metadata */

import { slugToken, splitOptionLabels, titleFromToken } from './catalog-metadata-text';
import type {
  TrackingControlCardKind,
  TrackingControlEffectStatus,
  TrackingControlKind,
  TrackingControlSelectionMode,
} from './tracking-control-catalog-schema';

const hasOwn = <T extends object>(value: T, key: PropertyKey): key is keyof T =>
  Object.prototype.hasOwnProperty.call(value, key);

const TrackingControlCardKindByControlKind = {
  toggle: 'toggle',
  number: 'number-card',
  duration: 'number-card',
  schedule: 'schedule-card',
  'rule-list': 'rule-list-card',
  'target-list': 'target-list-card',
  'place-list': 'place-list-card',
  'geofence-list': 'geofence-list-card',
  retention: 'retention-card',
  'read-only-status': 'status-card',
} as const satisfies Record<
  Exclude<TrackingControlKind, 'single-choice' | 'multi-choice' | 'action-list'>,
  TrackingControlCardKind
>;

const TrackingControlKindByProposalKind = {
  boolean: 'toggle',
  'single-choice': 'single-choice',
  'multi-choice': 'multi-choice',
  number: 'number',
  duration: 'duration',
  schedule: 'schedule',
  'rule-list': 'rule-list',
  'target-list': 'target-list',
  'place-list': 'place-list',
  'geofence-list': 'geofence-list',
  retention: 'retention',
  'action-list': 'action-list',
  'read-only-status': 'read-only-status',
} as const satisfies Readonly<Record<string, TrackingControlKind>>;

const MultiSelectionKinds = new Set<TrackingControlKind>(['multi-choice', 'action-list', 'rule-list']);
const ManyOptionKinds = new Set<TrackingControlKind>(['multi-choice', 'action-list']);

export function trackingProposalKinds(): Readonly<Record<string, TrackingControlKind>> {
  return TrackingControlKindByProposalKind;
}

export function trackingSelectionModeFor(
  controlKind: TrackingControlKind,
  options: readonly unknown[]
): TrackingControlSelectionMode {
  return MultiSelectionKinds.has(controlKind) || (options.length > 8 && controlKind !== 'single-choice')
    ? 'multi'
    : 'single';
}

export function trackingCardKindFor(controlKind: TrackingControlKind, optionCount: number): TrackingControlCardKind {
  if (controlKind === 'single-choice') {
    return optionCount > 6 ? 'single-choice-many' : 'single-choice-compact';
  }
  if (ManyOptionKinds.has(controlKind)) {
    return optionCount > 6 ? 'multi-choice-many' : 'multi-choice-normal';
  }
  return hasOwn(TrackingControlCardKindByControlKind, controlKind)
    ? TrackingControlCardKindByControlKind[controlKind]
    : 'single-choice-compact';
}

export function trackingLayoutHintsFor(controlKind: TrackingControlKind, optionCount: number) {
  return {
    preferredColumnSpan: optionCount > 6 || controlKind === 'geofence-list' || controlKind === 'place-list' ? 2 : 1,
    collapsible: optionCount > 4 || controlKind === 'retention',
    searchableOptions: optionCount > 6,
    optionGroupCount: optionCount > 8 ? Math.ceil(optionCount / 6) : 1,
    showAsMatrixWhenLarge: optionCount > 10 || controlKind === 'read-only-status',
    showSelectedCount: controlKind === 'multi-choice' || optionCount > 6,
  };
}

export function trackingHelperTextFor(
  effectStatus: TrackingControlEffectStatus,
  capabilityRequirement: string
): string {
  return `${effectStatus} via ${capabilityRequirement}`;
}

export function trackingQuestionFromSourceText(sourceText: string, explicitQuestion: string | null): string {
  if (explicitQuestion !== null) {
    return explicitQuestion;
  }
  const trimmed = sourceText.replace(/[.;:]$/u, '');
  return trimmed.endsWith('?') ? trimmed : `Represent: ${trimmed}`;
}

export function trackingExplicitOptionLabels(sourceText: string): readonly string[] {
  if (/Capability matrix row/iu.test(sourceText)) {
    return sourceText
      .split('|')
      .slice(1)
      .map((part) => part.trim())
      .filter(Boolean);
  }
  const colonParts = sourceText.split(':');
  const candidate = colonParts.length > 1 ? colonParts.slice(1).join(':') : sourceText;
  return /,| or | and /iu.test(candidate) ? splitOptionLabels(candidate, /,| or | and /iu, 16) : [];
}

export const trackingSlugToken = slugToken;
export const trackingTitleFromToken = titleFromToken;
