import { GeneratedPortalAgentProtocolRuntime } from './generated-portal-contracts';

export function isExactRecord<const TField extends string>(
  value: unknown,
  fields: readonly TField[]
): value is Readonly<Record<TField, unknown>> {
  if (!isRecord(value)) return false;
  const keys = Object.keys(value);
  return keys.length === fields.length && fields.every((field) => Object.prototype.hasOwnProperty.call(value, field));
}

export function isUnknownArray(value: unknown): value is readonly unknown[] {
  return Array.isArray(value);
}

export function readSchemaVersion(value: unknown): number {
  if (value !== GeneratedPortalAgentProtocolRuntime.SchemaVersion) {
    throw new TypeError('invalid app/game platform extension schema version');
  }
  return GeneratedPortalAgentProtocolRuntime.SchemaVersion;
}

export function readLiteral<const TValue extends string>(value: unknown, allowed: readonly TValue[]): TValue {
  for (const candidate of allowed) {
    if (value === candidate) return candidate;
  }
  throw new TypeError('invalid app/game platform extension literal');
}

export function readNonEmptyText(value: unknown): string {
  if (typeof value !== 'string' || value.length === 0) {
    throw new TypeError('invalid app/game platform extension text');
  }
  return value;
}

export function readNonEmptyTextArray(value: unknown): readonly string[] {
  if (!isUnknownArray(value)) throw new TypeError('app/game platform extension refs must be an array');
  return value.map(readNonEmptyText);
}

export function readBoolean(value: unknown): boolean {
  if (typeof value !== 'boolean') throw new TypeError('invalid app/game platform extension boolean');
  return value;
}

function isRecord(value: unknown): value is Readonly<Record<string, unknown>> {
  return typeof value === 'object' && value !== null && !Array.isArray(value);
}
