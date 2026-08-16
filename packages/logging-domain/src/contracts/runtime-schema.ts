export type RuntimeParseResult<TValue> =
  | {
      readonly success: true;
      readonly data: TValue;
    }
  | {
      readonly success: false;
      readonly error: Error;
    };

export interface RuntimeSchema<TValue> {
  readonly parse: (input: unknown) => TValue;
  readonly safeParse: (input: unknown) => RuntimeParseResult<TValue>;
}

export interface RuntimeRecord {
  readonly [key: string]: unknown;
  readonly agent?: unknown;
  readonly column?: unknown;
  readonly consumer?: unknown;
  readonly context?: unknown;
  readonly correlation_id?: unknown;
  readonly correlationId?: unknown;
  readonly data?: unknown;
  readonly deviceId?: unknown;
  readonly entries?: unknown;
  readonly environment?: unknown;
  readonly fields?: unknown;
  readonly file?: unknown;
  readonly file_path?: unknown;
  readonly filePath?: unknown;
  readonly hostname?: unknown;
  readonly id?: unknown;
  readonly level?: unknown;
  readonly line?: unknown;
  readonly log?: unknown;
  readonly log_timestamp?: unknown;
  readonly message?: unknown;
  readonly origin?: unknown;
  readonly platform?: unknown;
  readonly runId?: unknown;
  readonly runType?: unknown;
  readonly schemaVersion?: unknown;
  readonly scope?: unknown;
  readonly serviceVersion?: unknown;
  readonly sessionId?: unknown;
  readonly source?: unknown;
  readonly stack?: unknown;
  readonly suite_type?: unknown;
  readonly suiteType?: unknown;
  readonly tags?: unknown;
  readonly testName?: unknown;
  readonly timestamp?: unknown;
  readonly type?: unknown;
}

function safeParseValue<TValue>(parseInput: (input: unknown) => TValue, input: unknown): RuntimeParseResult<TValue> {
  try {
    return {
      success: true,
      data: parseInput(input),
    };
  } catch (error) {
    return {
      success: false,
      error: error instanceof Error ? error : new Error(String(error)),
    };
  }
}

export function createRuntimeSchema<TValue>(parseInput: (input: unknown) => TValue): RuntimeSchema<TValue> {
  return {
    parse: parseInput,
    safeParse: (input) => safeParseValue(parseInput, input),
  };
}

export function parseRecord(input: unknown, label: string): RuntimeRecord {
  if (input === null || typeof input !== 'object' || Array.isArray(input)) {
    throw new Error(`${label} must be an object`);
  }
  return input as RuntimeRecord;
}

export function parseLiteral<TValue extends string | number | boolean>(
  input: unknown,
  values: readonly TValue[],
  label: string
): TValue {
  if (values.includes(input as TValue)) {
    return input as TValue;
  }
  throw new Error(`${label} must be one of: ${values.join(', ')}`);
}

export function parseString(input: unknown, label: string): string {
  if (typeof input !== 'string') {
    throw new Error(`${label} must be a string`);
  }
  return input;
}

export function parseNonEmptyString(input: unknown, label: string): string {
  const value = parseString(input, label);
  if (value.length === 0) {
    throw new Error(`${label} must be non-empty`);
  }
  return value;
}

export function parseNullableString(input: unknown, label: string): string | null {
  if (input === null) {
    return null;
  }
  return parseString(input, label);
}

export function parseNumber(input: unknown, label: string): number {
  if (typeof input !== 'number' || Number.isNaN(input)) {
    throw new Error(`${label} must be a number`);
  }
  return input;
}

export function parseNullableInteger(input: unknown, label: string): number | null {
  if (input === null) {
    return null;
  }
  const value = parseNumber(input, label);
  if (!Number.isInteger(value)) {
    throw new Error(`${label} must be an integer`);
  }
  return value;
}

export function parseNonNegativeInteger(input: unknown, label: string): number {
  const value = parseNullableInteger(input, label);
  if (value === null || value < 0) {
    throw new Error(`${label} must be a non-negative integer`);
  }
  return value;
}

export function parseStringArray(input: unknown, label: string): readonly string[] {
  if (!Array.isArray(input) || input.some((entry) => typeof entry !== 'string')) {
    throw new Error(`${label} must be a string array`);
  }
  return [...input];
}
