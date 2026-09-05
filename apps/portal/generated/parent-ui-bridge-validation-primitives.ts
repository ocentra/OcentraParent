/* generated from crates/schema/src/parent_ui_bridge.rs */

export type ParentUiBridgeRuntimeValidator = (value: unknown) => boolean;

export const ParentUiBridgeDecodeLimit = {
  MaxDepth: 24,
  MaxCollectionEntries: 10_000,
} as const;

export function parentUiBridgeIsRecord(value: unknown): value is Readonly<Record<string, unknown>> {
  return typeof value === 'object' && value !== null && !Array.isArray(value);
}

export const parentUiBridgeString: ParentUiBridgeRuntimeValidator = (value) => typeof value === 'string';
export const parentUiBridgeNumber: ParentUiBridgeRuntimeValidator = (value) =>
  typeof value === 'number' && Number.isFinite(value);
export const parentUiBridgeInteger: ParentUiBridgeRuntimeValidator = (value) =>
  typeof value === 'number' && Number.isSafeInteger(value);
export const parentUiBridgeNonNegativeInteger: ParentUiBridgeRuntimeValidator = (value) =>
  typeof value === 'number' && Number.isSafeInteger(value) && value >= 0;
export const parentUiBridgeBoolean: ParentUiBridgeRuntimeValidator = (value) => typeof value === 'boolean';

export function parentUiBridgeLiteral(values: readonly unknown[]): ParentUiBridgeRuntimeValidator {
  return (value) => values.includes(value);
}

export function parentUiBridgeArray(item: ParentUiBridgeRuntimeValidator): ParentUiBridgeRuntimeValidator {
  return (value) =>
    Array.isArray(value) &&
    value.length <= ParentUiBridgeDecodeLimit.MaxCollectionEntries &&
    value.every((entry) => item(entry));
}

export function parentUiBridgeOptionalNullable(item: ParentUiBridgeRuntimeValidator): ParentUiBridgeRuntimeValidator {
  return (value) => value === undefined || value === null || item(value);
}

export function parentUiBridgeNullable(item: ParentUiBridgeRuntimeValidator): ParentUiBridgeRuntimeValidator {
  return (value) => value === null || item(value);
}

export function parentUiBridgeObject(
  fields: Readonly<Record<string, ParentUiBridgeRuntimeValidator>>
): ParentUiBridgeRuntimeValidator {
  const fieldNames = new Set(Object.keys(fields));
  return (value) =>
    parentUiBridgeIsRecord(value) &&
    Object.keys(value).every((field) => fieldNames.has(field)) &&
    Object.entries(fields).every(([field, validate]) => validate(value[field]));
}

export function parentUiBridgeJsonValue(value: unknown, depth = 0): boolean {
  if (value === null || typeof value === 'string' || typeof value === 'boolean') return true;
  if (typeof value === 'number') return Number.isFinite(value);
  if (depth >= ParentUiBridgeDecodeLimit.MaxDepth) return false;
  if (Array.isArray(value)) {
    return (
      value.length <= ParentUiBridgeDecodeLimit.MaxCollectionEntries &&
      value.every((entry) => parentUiBridgeJsonValue(entry, depth + 1))
    );
  }
  if (!parentUiBridgeIsRecord(value)) return false;
  const entries = Object.values(value);
  return (
    entries.length <= ParentUiBridgeDecodeLimit.MaxCollectionEntries &&
    entries.every((entry) => parentUiBridgeJsonValue(entry, depth + 1))
  );
}

export const parentUiBridgeUnknownRecord: ParentUiBridgeRuntimeValidator = (value) =>
  parentUiBridgeIsRecord(value) && parentUiBridgeJsonValue(value);
export const parentUiBridgeStringArray = parentUiBridgeArray(parentUiBridgeString);
export const parentUiBridgeOptionalString = parentUiBridgeOptionalNullable(parentUiBridgeString);
export const parentUiBridgeOptionalNumber = parentUiBridgeOptionalNullable(parentUiBridgeNumber);
export const parentUiBridgeOptionalInteger = parentUiBridgeOptionalNullable(parentUiBridgeInteger);
export const parentUiBridgeOptionalUnknownRecord = parentUiBridgeOptionalNullable(parentUiBridgeUnknownRecord);

export function parentUiBridgeDecodedBy(
  decode: (value: unknown) => unknown
): ParentUiBridgeRuntimeValidator {
  return (value) => {
    try {
      decode(value);
      return true;
    } catch {
      return false;
    }
  };
}
