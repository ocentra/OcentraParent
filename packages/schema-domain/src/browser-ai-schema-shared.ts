import type * as EffectSchema from 'effect/Schema';
import { Schema } from '@ocentra-parent/schema-domain/effect';

export function nonEmptyArraySchema<S extends EffectSchema.Schema.AnyNoContext>(itemSchema: S, message: string) {
  return Schema.Array(itemSchema).pipe(Schema.filter((value) => value.length > 0 || message));
}

export function optionalSchema<S extends EffectSchema.Schema.AnyNoContext>(schema: S) {
  return Schema.Union(schema, Schema.Null);
}

export function includesAll<T>(source: readonly T[], expected: readonly T[]) {
  return expected.every((item) => source.includes(item));
}

export function includesAny<T>(source: readonly T[], expected: readonly T[]) {
  return expected.some((item) => source.includes(item));
}
