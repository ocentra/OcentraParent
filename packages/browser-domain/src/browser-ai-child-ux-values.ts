import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';

const NonEmptyBrowserChildUxText = Schema.String.pipe(Schema.minLength(1));

export const BrowserAiChildUxSnapshotIdSchema = withParser(
  NonEmptyBrowserChildUxText.pipe(Schema.brand('BrowserAiChildUxSnapshotId'))
);

export const BrowserAiChildUxStateSchema = withParser(
  Schema.Literal(
    'opening',
    'checking',
    'allowed',
    'warning',
    'approval_required',
    'limited',
    'blocked',
    'unclassified',
    'manual_required',
    'unavailable'
  )
);
export const BrowserAiChildUxToneSchema = withParser(Schema.Literal('calm', 'neutral', 'informational'));
export const BrowserAiChildUxSurfaceSchema = withParser(
  Schema.Literal(
    'managed-browser-hold-page',
    'managed-browser-warning-page',
    'managed-browser-block-page',
    'parent-approval-hold-page',
    'portal-row-only',
    'modeled-only'
  )
);
export const BrowserAiChildUxTextTokenSchema = withParser(
  Schema.Literal(
    'browser.child.opening.title',
    'browser.child.checking.title',
    'browser.child.allowed.title',
    'browser.child.warning.title',
    'browser.child.approval.title',
    'browser.child.limited.title',
    'browser.child.blocked.title',
    'browser.child.unclassified.title',
    'browser.child.manual.title',
    'browser.child.unavailable.title'
  )
);

export type BrowserAiChildUxState = Infer<typeof BrowserAiChildUxStateSchema>;
export type BrowserAiChildUxTextToken = Infer<typeof BrowserAiChildUxTextTokenSchema>;

export const BrowserAiChildUxTextToken = {
  Opening: BrowserAiChildUxTextTokenSchema.parse('browser.child.opening.title'),
  Checking: BrowserAiChildUxTextTokenSchema.parse('browser.child.checking.title'),
  Allowed: BrowserAiChildUxTextTokenSchema.parse('browser.child.allowed.title'),
  Warning: BrowserAiChildUxTextTokenSchema.parse('browser.child.warning.title'),
  Approval: BrowserAiChildUxTextTokenSchema.parse('browser.child.approval.title'),
  Limited: BrowserAiChildUxTextTokenSchema.parse('browser.child.limited.title'),
  Blocked: BrowserAiChildUxTextTokenSchema.parse('browser.child.blocked.title'),
  Unclassified: BrowserAiChildUxTextTokenSchema.parse('browser.child.unclassified.title'),
  Manual: BrowserAiChildUxTextTokenSchema.parse('browser.child.manual.title'),
  Unavailable: BrowserAiChildUxTextTokenSchema.parse('browser.child.unavailable.title'),
} as const;
