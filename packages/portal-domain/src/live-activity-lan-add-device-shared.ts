export function recordHasStringFields<const TField extends string>(
  raw: Record<string, unknown>,
  fields: readonly TField[]
): raw is Record<TField, string> & Record<string, unknown> {
  return fields.every((field) => isString(raw[field]));
}

export function recordHasNullableStringFields<const TField extends string>(
  raw: Record<string, unknown>,
  fields: readonly TField[]
): raw is Record<TField, string | null | undefined> & Record<string, unknown> {
  return fields.every((field) => isNullableString(raw[field]));
}

export function recordHasBooleanFields<const TField extends string>(
  raw: Record<string, unknown>,
  fields: readonly TField[]
): raw is Record<TField, boolean> & Record<string, unknown> {
  return fields.every((field) => typeof raw[field] === 'boolean');
}

export function normalizeRecordArray<T>(value: unknown, normalize: (entry: unknown) => T | null): readonly T[] | null {
  if (!Array.isArray(value)) {
    return null;
  }
  const normalized = value.map((entry) => normalize(entry)).filter(notNull);
  return normalized.length === value.length ? normalized : null;
}

export function normalizeGeneratedArray<T>(value: unknown): readonly T[] | null {
  return Array.isArray(value) ? (value as readonly T[]) : null;
}

export function normalizeStringArray(value: unknown): readonly string[] | null {
  if (!Array.isArray(value) || value.some((entry) => !isString(entry))) {
    return null;
  }

  return value as readonly string[];
}

export function isNullableString(value: unknown): value is string | null {
  return value === null || isString(value);
}

export function isNumber(value: unknown): value is number {
  return typeof value === 'number';
}

export function isString(value: unknown): value is string {
  return typeof value === 'string';
}

export function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === 'object' && value !== null;
}

export function notNull<T>(value: T | null): value is T {
  return value !== null;
}

export function stringOrUnknown(value: unknown): string {
  return isString(value) ? value : 'unknown';
}
