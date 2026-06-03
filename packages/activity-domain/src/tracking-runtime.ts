import { ActivityEvidenceKind } from './kinds';
import type { TrackingLocationEvidence } from './tracking-evidence';
import {
  TrackingExpectedPlaceDecisionSchema,
  TrackingGeofenceTransitionSchema,
  type TrackingExpectedPlaceDecision,
  type TrackingExpectedPlaceSchedule,
  type TrackingGeofenceRule,
  type TrackingGeofenceTransition,
} from './tracking-geofence';
import { TrackingReasonCodeSchema } from './tracking-primitives';

const EarthRadiusMeters = 6_371_008.8;
const HalfCircleDegrees = 180;
const RadiansPerDegree = Math.PI / HalfCircleDegrees;
type TrackingReasonCode = ReturnType<typeof TrackingReasonCodeSchema.parse>;

export interface TrackingGeofenceEvaluationInput {
  readonly transitionId: TrackingGeofenceTransition['transitionId'];
  readonly observedAt: TrackingGeofenceTransition['observedAt'];
  readonly rule: TrackingGeofenceRule;
  readonly location: TrackingLocationEvidence;
  readonly wasInside: boolean;
}

export interface TrackingExpectedPlaceEvaluationInput {
  readonly decisionId: TrackingExpectedPlaceDecision['decisionId'];
  readonly observedAt: TrackingExpectedPlaceDecision['observedAt'];
  readonly schedule: TrackingExpectedPlaceSchedule;
  readonly location: TrackingLocationEvidence;
  readonly transition: TrackingGeofenceTransition;
}

export function evaluateTrackingGeofenceTransition(input: TrackingGeofenceEvaluationInput): TrackingGeofenceTransition {
  const distanceMeters = distanceFromRule(input.rule, input.location);
  const reasonCodes: TrackingReasonCode[] = [];
  let transition: TrackingGeofenceTransition['transition'] = 'ambiguous';

  if (!input.rule.enabled) {
    reasonCodes.push(reasonCode('geofence-rule-disabled'));
  } else if (input.location.capabilityStatus !== 'live' && input.location.capabilityStatus !== 'recent') {
    reasonCodes.push(reasonCode('fresh-location-required'));
  } else if (input.location.coordinate === null || distanceMeters === null) {
    reasonCodes.push(reasonCode('precise-location-required'));
  } else if (input.location.accuracyMeters === null || input.location.accuracyMeters > input.rule.minAccuracyMeters) {
    reasonCodes.push(reasonCode('location-accuracy-below-rule-threshold'));
  } else {
    const inside = locationInsideRule(input.rule, input.location, distanceMeters);
    transition = inside ? (input.wasInside ? 'dwell' : 'enter') : input.wasInside ? 'exit' : 'ambiguous';
    reasonCodes.push(reasonCode(inside ? 'inside-geofence-with-accuracy' : 'outside-geofence-with-accuracy'));
  }

  return TrackingGeofenceTransitionSchema.parse({
    schemaVersion: input.rule.schemaVersion,
    transitionId: input.transitionId,
    observedAt: input.observedAt,
    ruleId: input.rule.ruleId,
    geofenceId: input.rule.geofenceId,
    locationEvidenceId: input.location.evidenceId,
    transition,
    capabilityStatus: input.location.capabilityStatus,
    distanceMeters,
    reasonCodes,
    evidence: [
      ...input.location.evidence,
      {
        evidenceId: input.location.evidenceId,
        kind: ActivityEvidenceKind.LocalDbRow,
        digest: null,
        uri: null,
      },
    ],
  });
}

export function evaluateTrackingExpectedPlaceDecision(
  input: TrackingExpectedPlaceEvaluationInput
): TrackingExpectedPlaceDecision {
  const activeWindow = input.schedule.windows.some((window) => {
    const observedAt = Date.parse(input.observedAt);
    return observedAt >= Date.parse(window.startsAt) && observedAt <= Date.parse(window.endsAt);
  });

  const reasonCodes: TrackingReasonCode[] = [];
  let outcome: TrackingExpectedPlaceDecision['outcome'] = 'unknown';

  if (!input.schedule.enabled) {
    outcome = 'manual-required';
    reasonCodes.push(reasonCode('expected-place-schedule-disabled'));
  } else if (
    input.location.capabilityStatus === 'stale' ||
    input.location.capabilityStatus === 'offline-last-known-only'
  ) {
    reasonCodes.push(reasonCode('fresh-location-required'));
  } else if (!activeWindow) {
    reasonCodes.push(reasonCode('outside-expected-place-window'));
  } else if (input.transition.transition === 'enter' || input.transition.transition === 'dwell') {
    outcome = 'where-expected';
    reasonCodes.push(reasonCode('inside-expected-place-window'));
  } else if (input.transition.transition === 'exit') {
    outcome = 'left-expected-place';
    reasonCodes.push(reasonCode('exited-expected-place-window'));
  } else {
    reasonCodes.push(reasonCode('expected-place-ambiguous'));
  }

  return TrackingExpectedPlaceDecisionSchema.parse({
    schemaVersion: input.schedule.schemaVersion,
    decisionId: input.decisionId,
    observedAt: input.observedAt,
    scheduleId: input.schedule.scheduleId,
    locationEvidenceId: input.location.evidenceId,
    outcome,
    reasonCodes,
    evidence: input.transition.evidence,
  });
}

function distanceFromRule(
  rule: TrackingGeofenceRule,
  location: TrackingLocationEvidence
): TrackingGeofenceTransition['distanceMeters'] {
  if (location.coordinate === null) {
    return null;
  }

  if (rule.shape.kind === 'circle') {
    if (rule.shape.center === null) {
      return null;
    }
    return distanceMeters(
      location.coordinate.latitude,
      location.coordinate.longitude,
      rule.shape.center.latitude,
      rule.shape.center.longitude
    );
  }

  return pointInRulePolygon(rule, location) ? 0 : null;
}

function locationInsideRule(
  rule: TrackingGeofenceRule,
  location: TrackingLocationEvidence,
  distanceMeters: TrackingGeofenceTransition['distanceMeters']
) {
  if (rule.shape.kind === 'circle') {
    return rule.shape.radiusMeters !== null && distanceMeters !== null && distanceMeters <= rule.shape.radiusMeters;
  }

  return pointInRulePolygon(rule, location);
}

function pointInRulePolygon(rule: TrackingGeofenceRule, location: TrackingLocationEvidence) {
  if (rule.shape.kind !== 'polygon' || location.coordinate === null) {
    return false;
  }

  let inside = false;
  const { latitude, longitude } = location.coordinate;
  for (let index = 0, previous = rule.shape.polygon.length - 1; index < rule.shape.polygon.length; previous = index++) {
    const currentPoint = rule.shape.polygon[index];
    const previousPoint = rule.shape.polygon[previous];
    if (currentPoint === undefined || previousPoint === undefined) {
      continue;
    }
    const currentLongitudeCrosses = currentPoint.longitude > longitude;
    const previousLongitudeCrosses = previousPoint.longitude > longitude;
    const intersects =
      currentLongitudeCrosses !== previousLongitudeCrosses &&
      latitude <
        ((previousPoint.latitude - currentPoint.latitude) * (longitude - currentPoint.longitude)) /
          (previousPoint.longitude - currentPoint.longitude) +
          currentPoint.latitude;
    if (intersects) {
      inside = !inside;
    }
  }
  return inside;
}

function distanceMeters(startLatitude: number, startLongitude: number, endLatitude: number, endLongitude: number) {
  const startLatitudeRadians = startLatitude * RadiansPerDegree;
  const endLatitudeRadians = endLatitude * RadiansPerDegree;
  const deltaLatitude = (endLatitude - startLatitude) * RadiansPerDegree;
  const deltaLongitude = (endLongitude - startLongitude) * RadiansPerDegree;
  const arc =
    Math.sin(deltaLatitude / 2) * Math.sin(deltaLatitude / 2) +
    Math.cos(startLatitudeRadians) *
      Math.cos(endLatitudeRadians) *
      Math.sin(deltaLongitude / 2) *
      Math.sin(deltaLongitude / 2);
  return Math.round(EarthRadiusMeters * 2 * Math.atan2(Math.sqrt(arc), Math.sqrt(1 - arc)));
}

function reasonCode(value: unknown) {
  return TrackingReasonCodeSchema.parse(value);
}
