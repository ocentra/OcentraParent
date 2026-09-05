import {
  GeneratedPortalTrackingContracts,
  type GeneratedPortalAgentActivitySurfaceAdapterFailureReason,
} from './generated-portal-contracts';

type ActivityTrackingReadModel = NonNullable<
  ReturnType<typeof GeneratedPortalTrackingContracts.ActivityTrackingReadModel.decode>
>;
type TrackingRetentionSettingsWriteResult = NonNullable<
  ReturnType<typeof GeneratedPortalTrackingContracts.RetentionSettingsWrite.Result.decode>
>;

export interface PortalActivityTrackingReadModelResultSnapshot {
  readonly ok: boolean;
  readonly value?: ActivityTrackingReadModel | null;
  readonly reason?: GeneratedPortalAgentActivitySurfaceAdapterFailureReason | null;
}

export type DecodedPortalActivityTrackingReadModelResult =
  | {
      readonly ok: true;
      readonly value: ActivityTrackingReadModel;
    }
  | {
      readonly ok: false;
      readonly reason: GeneratedPortalAgentActivitySurfaceAdapterFailureReason;
    };

export type DecodedPortalTrackingRetentionSettingsWriteResult =
  | {
      readonly parseState: 'parsed';
      readonly value: TrackingRetentionSettingsWriteResult;
    }
  | {
      readonly parseState: 'failed';
      readonly reason: 'invalid-payload';
    };

export function decodeActivityTrackingReadModel(
  value: PortalActivityTrackingReadModelResultSnapshot | null | undefined
): DecodedPortalActivityTrackingReadModelResult | null {
  if (value === null || value === undefined) return null;
  if (value.ok) {
    if (value.value === null || value.value === undefined || (value.reason !== null && value.reason !== undefined)) {
      return null;
    }
    return { ok: true, value: value.value };
  }
  if (value.reason === null || value.reason === undefined || (value.value !== null && value.value !== undefined)) {
    return null;
  }
  return { ok: false, reason: value.reason };
}

export function decodeTrackingRetentionSettingsWriteResult(
  value: unknown
): DecodedPortalTrackingRetentionSettingsWriteResult | null {
  if (value === null || value === undefined) return null;
  try {
    const decoded = GeneratedPortalTrackingContracts.RetentionSettingsWrite.Result.decode(value);
    if (decoded === null) return { parseState: 'failed', reason: 'invalid-payload' };
    return { parseState: 'parsed', value: decoded };
  } catch {
    return { parseState: 'failed', reason: 'invalid-payload' };
  }
}
