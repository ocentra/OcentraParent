import {
  GeneratedPortalAgentActivityAppUseReadModelSchema,
  GeneratedPortalAgentActivityBrowserReadModelSchema,
  GeneratedPortalAgentActivityGamesReadModelSchema,
  GeneratedPortalAgentActivityHistoricalReportListSchema,
  GeneratedPortalAgentActivityReadModelStateSchema,
  GeneratedPortalAgentActivityReportDocumentSchema,
  GeneratedPortalAgentActivityScreenReadModelSchema,
  GeneratedPortalAgentActivitySurfaceAdapterOperationManifest,
  GeneratedPortalAgentProtocolField,
  type GeneratedPortalAgentActivityAppUseReadModel,
  type GeneratedPortalAgentActivityBrowserReadModel,
  type GeneratedPortalAgentActivityGamesReadModel,
  type GeneratedPortalAgentActivityHistoricalReportList,
  type GeneratedPortalAgentActivityReadModelState,
  type GeneratedPortalAgentActivityReportDocument,
  type GeneratedPortalAgentActivityScreenReadModel,
} from './generated-portal-contracts';
import {
  hasOnlyRouteLiveActivityFields,
  isRouteLiveActivityRecord,
  parseRouteLiveActivityJson,
} from './route-live-activity-record';

type ActivityReadModel = Readonly<{ state: GeneratedPortalAgentActivityReadModelState }>;
type ActivityValueSchema<TValue> = Readonly<{
  safeParse: (value: unknown) => { readonly success: true; readonly data: TValue } | { readonly success: false };
}>;
type ActivityReadModelSchema<TReadModel extends ActivityReadModel> = ActivityValueSchema<TReadModel>;

export type DecodedActivityAdapterResult<TValue> =
  | {
      readonly ok: true;
      readonly state: GeneratedPortalAgentActivityReadModelState;
      readonly value: TValue;
    }
  | {
      readonly ok: false;
      readonly state: GeneratedPortalAgentActivityReadModelState;
      readonly reason: string;
    };

export function decodeActivityReportDocumentPayload(
  payload: unknown
): DecodedActivityAdapterResult<GeneratedPortalAgentActivityReportDocument> | null {
  return decodeActivityJsonPayload(
    payload,
    GeneratedPortalAgentProtocolField.ActivityReportDocument,
    GeneratedPortalAgentActivityReportDocumentSchema
  );
}

export function decodeActivityReportHistoryPayload(
  payload: unknown
): DecodedActivityAdapterResult<GeneratedPortalAgentActivityHistoricalReportList> | null {
  const decoded = decodeActivityJsonPayload(
    payload,
    GeneratedPortalAgentProtocolField.ActivityReports,
    GeneratedPortalAgentActivityHistoricalReportListSchema
  );
  return decoded?.ok === true && decoded.value.state !== decoded.state ? null : decoded;
}

export function decodeActivityScreenReadModel(
  value: unknown
): DecodedActivityAdapterResult<GeneratedPortalAgentActivityScreenReadModel> | null {
  return decodeActivityAdapterResult(value, GeneratedPortalAgentActivityScreenReadModelSchema);
}

export function decodeActivityAppUseReadModel(
  value: unknown
): DecodedActivityAdapterResult<GeneratedPortalAgentActivityAppUseReadModel> | null {
  return decodeActivityAdapterResult(value, GeneratedPortalAgentActivityAppUseReadModelSchema);
}

export function decodeActivityBrowserReadModel(
  value: unknown
): DecodedActivityAdapterResult<GeneratedPortalAgentActivityBrowserReadModel> | null {
  return decodeActivityAdapterResult(value, GeneratedPortalAgentActivityBrowserReadModelSchema);
}

export function decodeActivityGamesReadModel(
  value: unknown
): DecodedActivityAdapterResult<GeneratedPortalAgentActivityGamesReadModel> | null {
  return decodeActivityAdapterResult(value, GeneratedPortalAgentActivityGamesReadModelSchema);
}

function decodeActivityAdapterResult<TReadModel extends ActivityReadModel>(
  value: unknown,
  schema: ActivityReadModelSchema<TReadModel>
): DecodedActivityAdapterResult<TReadModel> | null {
  if (!isRouteLiveActivityRecord(value)) return null;
  const parsedState = GeneratedPortalAgentActivityReadModelStateSchema.safeParse(value['state']);
  if (!parsedState.success) return null;
  if (value['ok'] === true && hasOnlyRouteLiveActivityFields(value, ['ok', 'state', 'value'])) {
    const parsedValue = schema.safeParse(value['value']);
    if (!parsedValue.success || parsedValue.data.state !== parsedState.data) return null;
    return { ok: true, state: parsedState.data, value: parsedValue.data };
  }
  if (value['ok'] !== false || !hasOnlyRouteLiveActivityFields(value, ['ok', 'state', 'reason'])) return null;
  if (parsedState.data !== GeneratedPortalAgentActivitySurfaceAdapterOperationManifest[0].failureState) return null;
  const reason = value['reason'];
  if (
    typeof reason !== 'string' ||
    !GeneratedPortalAgentActivitySurfaceAdapterOperationManifest[0].failureReasons.some(
      (allowedReason) => allowedReason === reason
    )
  ) {
    return null;
  }
  return { ok: false, state: parsedState.data, reason };
}

function decodeActivityJsonPayload<TValue>(
  payload: unknown,
  field: string,
  schema: ActivityValueSchema<TValue>
): DecodedActivityAdapterResult<TValue> | null {
  if (!isRouteLiveActivityRecord(payload)) return null;
  const parsedState = GeneratedPortalAgentActivityReadModelStateSchema.safeParse(
    payload[GeneratedPortalAgentProtocolField.ActivitySurfaceState]
  );
  const serializedValue = payload[field];
  if (!parsedState.success || typeof serializedValue !== 'string') return null;
  const parsedJson = parseRouteLiveActivityJson(serializedValue);
  if (parsedJson === null) return null;
  const parsedValue = schema.safeParse(parsedJson);
  if (!parsedValue.success) return null;
  return { ok: true, state: parsedState.data, value: parsedValue.data };
}
