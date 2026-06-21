import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';

export const BrowserGameUrlShapeSchemaVersionSchema = withParser(
  Schema.Literal('browser-game-url-shape-parser-contract')
);

export const BrowserGameUrlShapeResultIdSchema = withParser(
  brandedNonEmptyStringSchema('BrowserGameUrlShapeResultId')
);

export const BrowserGameUrlShapeFingerprintSchema = withParser(
  brandedNonEmptyStringSchema('BrowserGameUrlShapeFingerprint')
);

export const BrowserGameUrlShapeParseStateSchema = withParser(
  Schema.Literal('parsed', 'manual-required', 'unavailable')
);

export const BrowserGameUrlShapeInputCustodySchema = withParser(
  Schema.Literal('transient-parse-only', 'manual-required', 'unavailable')
);

export const BrowserGameUrlProtocolShapeSchema = withParser(
  Schema.Literal('http-family', 'non-http', 'missing', 'unknown')
);

export const BrowserGameUrlHostShapeSchema = withParser(
  Schema.Literal('domain-like', 'localhost-like', 'ip-like', 'unknown')
);

export const BrowserGameUrlPathDepthSchema = withParser(
  Schema.Literal('root', 'one-segment', 'two-segments', 'three-or-more-segments', 'unknown')
);

export const BrowserGameUrlShapeConfidenceSchema = withParser(Schema.Literal('high', 'medium', 'low', 'unknown'));

export const BrowserGameUrlShapeReasonCodeSchema = withParser(
  Schema.Literal(
    'game-route-hint',
    'embed-route-hint',
    'account-route-hint',
    'purchase-route-hint',
    'cloud-session-hint',
    'catalog-route-hint',
    'invalid-url',
    'not-text-input',
    'unsupported-protocol',
    'manual-required',
    'unavailable'
  )
);

export const BrowserGameUrlShapeReasonCodesSchema = Schema.Array(BrowserGameUrlShapeReasonCodeSchema).pipe(
  Schema.filter((value) => value.length > 0 || 'Expected browser-game URL shape reason codes')
);

export type BrowserGameUrlHostShape = Infer<typeof BrowserGameUrlHostShapeSchema>;
export type BrowserGameUrlPathDepth = Infer<typeof BrowserGameUrlPathDepthSchema>;
export type BrowserGameUrlProtocolShape = Infer<typeof BrowserGameUrlProtocolShapeSchema>;
export type BrowserGameUrlShapeConfidence = Infer<typeof BrowserGameUrlShapeConfidenceSchema>;
export type BrowserGameUrlShapeInputCustody = Infer<typeof BrowserGameUrlShapeInputCustodySchema>;
export type BrowserGameUrlShapeParseState = Infer<typeof BrowserGameUrlShapeParseStateSchema>;
export type BrowserGameUrlShapeReasonCode = Infer<typeof BrowserGameUrlShapeReasonCodeSchema>;

