import { Schema } from '@ocentra-parent/schema-domain/effect';

export function nonEmptyArraySchema(itemSchema: any, message: string) {
  return Schema.Array(itemSchema).pipe(Schema.filter((value) => value.length > 0 || message));
}

export function optionalSchema(schema: any) {
  return Schema.Union(schema, Schema.Null);
}

export function includesAll<T>(source: readonly T[], expected: readonly T[]) {
  return expected.every((item) => source.includes(item));
}

export function includesAny<T>(source: readonly T[], expected: readonly T[]) {
  return expected.some((item) => source.includes(item));
}
