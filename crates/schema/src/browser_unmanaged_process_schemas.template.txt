/* generated from crates/schema/src/browser_generated_values_ts.rs */

import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema,
  NonEmptyStringSchema,
} from '@ocentra-parent/schema-domain/effect';
const BrowserRedactedRefText = NonEmptyStringSchema.pipe(
  Schema.filter((value) => browserRedactedRefIsSafe(value) || 'Expected a redacted browser reference')
);

export const BrowserUnmanagedDetectionConfidenceSchema = withParser(Schema.Literal('high', 'medium', 'low'));
export const BrowserUnmanagedProcessKindSchema = withParser(
  Schema.Literal(
    'supported-browser',
    'unsupported-browser',
    'portable-browser',
    'tor-privacy-browser',
    'packaged-browser',
    'embedded-browser-like',
    'unknown-browser-like',
    'possible-social-bypass',
    'possible-browser-game-bypass',
    'possible-cloud-gaming-bypass'
  )
);
export const BrowserUnmanagedDetectionReasonSchema = withParser(
  Schema.Literal(
    'supported-browser-outside-managed-session',
    'unsupported-browser-process',
    'portable-browser-process',
    'tor-privacy-browser-process',
    'packaged-browser-process',
    'browser-like-process',
    'possible-social-bypass',
    'possible-browser-game-bypass',
    'possible-cloud-gaming-bypass'
  )
);

export const BrowserUnmanagedExecutablePathRefSchema = withParser(
  BrowserRedactedRefText.pipe(Schema.brand('BrowserUnmanagedExecutablePathRef'))
);
export const BrowserUnmanagedProcessHashRefSchema = withParser(
  BrowserRedactedRefText.pipe(Schema.brand('BrowserUnmanagedProcessHashRef'))
);
export const BrowserUnmanagedProcessNameSchema = withParser(brandedNonEmptyStringSchema('BrowserUnmanagedProcessName'));
export const BrowserUnmanagedSignatureRefSchema = withParser(
  BrowserRedactedRefText.pipe(Schema.brand('BrowserUnmanagedSignatureRef'))
);

export type BrowserUnmanagedDetectionConfidence = Infer<typeof BrowserUnmanagedDetectionConfidenceSchema>;
export type BrowserUnmanagedDetectionReason = Infer<typeof BrowserUnmanagedDetectionReasonSchema>;
export type BrowserUnmanagedProcessKind = Infer<typeof BrowserUnmanagedProcessKindSchema>;

function browserRedactedRefIsSafe(value: string): boolean {
  return !value.includes('/') && !value.includes('\\') && !value.includes(':');
}
