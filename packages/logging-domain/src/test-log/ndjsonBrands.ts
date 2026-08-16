import { Schema, brandedNonEmptyStringSchema, withParser } from '../effect';

export const OutputDirSchema = withParser(brandedNonEmptyStringSchema('OutputDir'));
export const FileKeySchema = withParser(brandedNonEmptyStringSchema('FileKey'));
export const TestNameSchema = withParser(brandedNonEmptyStringSchema('TestName'));
export const NdjsonSummaryContentSchema = withParser(Schema.String.pipe(Schema.brand('NdjsonSummaryContent')));

export type OutputDir = typeof OutputDirSchema.Type;
export type FileKey = typeof FileKeySchema.Type;
export type TestName = typeof TestNameSchema.Type;
export type NdjsonSummaryContent = typeof NdjsonSummaryContentSchema.Type;

export const decodeOutputDir = (input: unknown): OutputDir => OutputDirSchema.parse(input);
export const decodeFileKey = (input: unknown): FileKey => FileKeySchema.parse(input);
export const decodeTestName = (input: unknown): TestName => TestNameSchema.parse(input);
export const decodeNdjsonSummaryContent = (input: unknown): NdjsonSummaryContent =>
  NdjsonSummaryContentSchema.parse(input);

export function asOutputDir(value: string): OutputDir {
  return decodeOutputDir(value);
}

export function asFileKey(value: string): FileKey {
  return decodeFileKey(value);
}

export function asTestName(value: string): TestName {
  return decodeTestName(value);
}

export function asNdjsonSummaryContent(value: string): NdjsonSummaryContent {
  return decodeNdjsonSummaryContent(value);
}
