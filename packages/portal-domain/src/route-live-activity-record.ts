export type RouteLiveActivityRecord = Readonly<Record<string, unknown>>;

export function isRouteLiveActivityRecord(value: unknown): value is RouteLiveActivityRecord {
  return typeof value === 'object' && value !== null && !Array.isArray(value);
}

export function hasOnlyRouteLiveActivityFields(record: RouteLiveActivityRecord, fields: readonly string[]): boolean {
  return Object.keys(record).every((field) => fields.includes(field));
}

export function parseRouteLiveActivityJson(value: string): unknown | null {
  try {
    return JSON.parse(value) as unknown;
  } catch {
    return null;
  }
}
