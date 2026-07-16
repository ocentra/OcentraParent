/* generic helper for Effect Schema parsing and branded string contracts */

import * as Either from 'effect/Either';
import * as ParseResult from 'effect/ParseResult';
import * as EffectSchema from 'effect/Schema';

export const Schema = EffectSchema;

export type SchemaPath = Array<string | number>;

export interface SchemaIssue {
  readonly code?: string;
  readonly path: SchemaPath;
  readonly message: string;
}

type AnySchema = EffectSchema.Schema.Any;
type ParseableSchema = EffectSchema.Schema.AnyNoContext;
type TypeOf<S extends AnySchema> = EffectSchema.Schema.Type<S>;
type EncodedOf<S extends AnySchema> = EffectSchema.Schema.Encoded<S>;
type ContextOf<S extends AnySchema> = EffectSchema.Schema.Context<S>;

export type SafeParseResult<T> =
  | { readonly success: true; readonly data: T }
  | { readonly success: false; readonly error: SchemaDecodeError };

export type Infer<S extends AnySchema> = TypeOf<S>;

export const NonEmptyStringSchema = Schema.String.pipe(Schema.minLength(1));

export function brandedNonEmptyStringSchema<const Brand extends string>(brand: Brand) {
  return NonEmptyStringSchema.pipe(Schema.brand(brand));
}

export type ParsedSchema<S extends AnySchema> = S & {
  parse(input: unknown): TypeOf<S>;
  safeParse(input: unknown): SafeParseResult<TypeOf<S>>;
  partial(): ParsedSchema<EffectSchema.Schema<Partial<TypeOf<S>>, Partial<EncodedOf<S>>, ContextOf<S>>>;
};

export class SchemaDecodeError extends Error {
  readonly issues: SchemaIssue[];

  constructor(issues: readonly SchemaIssue[]) {
    super(issues.map((issue) => `${issue.path.join('.') || '<root>'}: ${issue.message}`).join('; '));
    this.name = 'SchemaDecodeError';
    this.issues = [...issues];
  }

  format(): Record<string, unknown> {
    return { _errors: this.issues.map((issue) => issue.message) };
  }

  flatten(): { fieldErrors: Record<string, string[]>; formErrors: string[] } {
    const fieldErrors: Record<string, string[]> = {};
    const formErrors: string[] = [];
    for (const issue of this.issues) {
      const key = issue.path.join('.');
      if (key.length === 0) {
        formErrors.push(issue.message);
      } else {
        fieldErrors[key] = [...(fieldErrors[key] ?? []), issue.message];
      }
    }
    return { fieldErrors, formErrors };
  }
}

function normalizePath(path: readonly PropertyKey[]): SchemaPath {
  return path.map((part) => (typeof part === 'number' ? part : String(part)));
}

function toDecodeError(error: ParseResult.ParseError): SchemaDecodeError {
  const issues = ParseResult.ArrayFormatter.formatErrorSync(error).map((issue) => ({
    code: issue._tag,
    path: normalizePath(issue.path),
    message: issue.message,
  }));
  return new SchemaDecodeError(issues);
}

export function safeParseUnknown<S extends ParseableSchema>(schema: S, input: unknown): SafeParseResult<TypeOf<S>> {
  const decoded = Schema.decodeUnknownEither(schema)(input, { errors: 'all' });
  if (Either.isRight(decoded)) {
    return { success: true, data: decoded.right };
  }
  return { success: false, error: toDecodeError(decoded.left) };
}

export function parseUnknown<S extends ParseableSchema>(schema: S, input: unknown): TypeOf<S> {
  const parsed = safeParseUnknown(schema, input);
  if (parsed.success) {
    return parsed.data;
  }
  throw parsed.error;
}

export function withParser<S extends AnySchema>(schema: S): ParsedSchema<S> {
  const parsedSchema = schema as unknown as ParsedSchema<S>;
  const parseableSchema = schema as unknown as ParseableSchema;
  Object.defineProperties(parsedSchema, {
    parse: {
      configurable: true,
      value(input: unknown) {
        return parseUnknown(parseableSchema, input) as TypeOf<S>;
      },
    },
    safeParse: {
      configurable: true,
      value(input: unknown) {
        return safeParseUnknown(parseableSchema, input) as SafeParseResult<TypeOf<S>>;
      },
    },
    partial: {
      configurable: true,
      value() {
        return withParser(
          Schema.partial(parseableSchema) as unknown as EffectSchema.Schema<
            Partial<TypeOf<S>>,
            Partial<EncodedOf<S>>,
            ContextOf<S>
          >
        );
      },
    },
  });
  return parsedSchema;
}
