import {
  appGamePlatformExtensionStateForRows,
  readAppGamePlatformExtensionRows,
  type AppGamePlatformExtensionRow,
  type AppGamePlatformExtensionState,
} from './route-live-activity-app-game-extension-row';
import {
  isExactRecord,
  readLiteral,
  readNonEmptyText,
  readSchemaVersion,
} from './route-live-activity-app-game-extension-values';

type AppGamePlatformExtensionReadModel = Readonly<{
  schemaVersion: number;
  state: AppGamePlatformExtensionState;
  generatedAt: string;
  summary: string;
  rows: readonly AppGamePlatformExtensionRow[];
}>;

export type ActivityAppGamePlatformExtensionAdapterResult = Readonly<{
  ok: true;
  value: AppGamePlatformExtensionReadModel;
}>;

const ADAPTER_FIELDS = ['ok', 'value'] as const;
const READ_MODEL_FIELDS = ['schemaVersion', 'state', 'generatedAt', 'summary', 'rows'] as const;
const READ_MODEL_STATES = ['ready', 'manual-required', 'unavailable'] as const;

export function decodeActivityAppGamePlatformExtensionReadModel(
  value: unknown
): ActivityAppGamePlatformExtensionAdapterResult | null {
  if (!isExactRecord(value, ADAPTER_FIELDS) || value['ok'] !== true) return null;
  const readModel = decodeReadModel(value['value']);
  return readModel === null ? null : { ok: true, value: readModel };
}

function decodeReadModel(value: unknown): AppGamePlatformExtensionReadModel | null {
  if (!isExactRecord(value, READ_MODEL_FIELDS)) return null;
  try {
    const rows = readAppGamePlatformExtensionRows(value['rows']);
    const state = readLiteral(value['state'], READ_MODEL_STATES);
    if (state !== appGamePlatformExtensionStateForRows(rows)) return null;
    return {
      schemaVersion: readSchemaVersion(value['schemaVersion']),
      state,
      generatedAt: readNonEmptyText(value['generatedAt']),
      summary: readNonEmptyText(value['summary']),
      rows,
    };
  } catch {
    return null;
  }
}
