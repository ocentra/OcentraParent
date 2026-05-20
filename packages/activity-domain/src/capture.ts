import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';

export const ActivityObservationModeSchema = withParser(Schema.Literal('snapshot', 'active-window'));

export const ActivityCaptureCapabilityStatusSchema = withParser(
  Schema.Literal('available', 'unavailable', 'access-denied', 'no-active-window', 'adapter-error')
);

export type ActivityObservationMode = Infer<typeof ActivityObservationModeSchema>;
export type ActivityCaptureCapabilityStatus = Infer<typeof ActivityCaptureCapabilityStatusSchema>;

export const ActivityObservationMode = {
  Snapshot: ActivityObservationModeSchema.parse('snapshot'),
  ActiveWindow: ActivityObservationModeSchema.parse('active-window'),
} as const;

export const ActivityCaptureCapabilityStatus = {
  Available: ActivityCaptureCapabilityStatusSchema.parse('available'),
  Unavailable: ActivityCaptureCapabilityStatusSchema.parse('unavailable'),
  AccessDenied: ActivityCaptureCapabilityStatusSchema.parse('access-denied'),
  NoActiveWindow: ActivityCaptureCapabilityStatusSchema.parse('no-active-window'),
  AdapterError: ActivityCaptureCapabilityStatusSchema.parse('adapter-error'),
} as const;
