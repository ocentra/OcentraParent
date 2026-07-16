/* generic helper for display text contracts */

import { Schema, brandedNonEmptyStringSchema } from './effect';

export const DisplayTextSchema = brandedNonEmptyStringSchema('DisplayText');
export type DisplayText = typeof DisplayTextSchema.Type;

export const TextTokenIdSchema = brandedNonEmptyStringSchema('TextTokenId');
export type TextTokenId = typeof TextTokenIdSchema.Type;

export const decodeDisplayText = Schema.decodeUnknownSync(DisplayTextSchema);
export const decodeTextTokenId = Schema.decodeUnknownSync(TextTokenIdSchema);
