import type { TrackingLocationEvidence } from './tracking-evidence';
import type {
  TrackingExpectedPlaceDecision,
  TrackingExpectedPlaceSchedule,
  TrackingGeofenceRule,
  TrackingGeofenceTransition,
} from './tracking-geofence';

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
