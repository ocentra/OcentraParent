import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';

const NonEmptyBrowserGameChildUxText = Schema.String.pipe(Schema.minLength(1));

export const BrowserGameChildCheckingBlockUxSchemaVersionSchema = withParser(
  Schema.Literal('browser-game-child-checking-block-ux-contract')
);

export const BrowserGameChildCheckingBlockSurfaceIdSchema = withParser(
  NonEmptyBrowserGameChildUxText.pipe(Schema.brand('BrowserGameChildCheckingBlockSurfaceId'))
);

export const BrowserGameChildCheckingBlockSurfaceKindSchema = withParser(
  Schema.Literal(
    'checking-unknown-game',
    'approval-required-game',
    'blocked-game-candidate',
    'educational-game-allowed',
    'game-time-limit-candidate',
    'cloud-gaming-manual-required',
    'native-game-control-unavailable'
  )
);

export const BrowserGameChildCheckingBlockStateSchema = withParser(
  Schema.Literal(
    'checking-contract-only',
    'waiting-parent',
    'blocked-contract-only',
    'child-readable',
    'manual-required',
    'unavailable'
  )
);

export const BrowserGameChildCheckingBlockActionSchema = withParser(
  Schema.Literal(
    'wait-for-classification',
    'wait-for-parent',
    'open-safe-back',
    'acknowledge',
    'manual-review',
    'no-action'
  )
);

export const BrowserGameChildCheckingBlockReasonSchema = withParser(
  Schema.Literal(
    'unknown-game-needs-classification',
    'parent-approval-needed',
    'game-block-candidate',
    'educational-game-allowed-contract',
    'time-limit-not-applied',
    'cloud-gaming-proof-manual-required',
    'native-game-proof-unavailable'
  )
);

export const BrowserGameChildCheckingBlockTextTokenSchema = withParser(
  Schema.Literal(
    'browser-game.child.checking.title',
    'browser-game.child.approval.title',
    'browser-game.child.blocked.title',
    'browser-game.child.educational-allowed.title',
    'browser-game.child.time-limited.title',
    'browser-game.child.manual.title',
    'browser-game.child.unavailable.title'
  )
);

export type BrowserGameChildCheckingBlockSurfaceKind = Infer<typeof BrowserGameChildCheckingBlockSurfaceKindSchema>;
export type BrowserGameChildCheckingBlockState = Infer<typeof BrowserGameChildCheckingBlockStateSchema>;
export type BrowserGameChildCheckingBlockTextToken = Infer<typeof BrowserGameChildCheckingBlockTextTokenSchema>;

export const BrowserGameChildCheckingBlockTextToken = {
  Checking: BrowserGameChildCheckingBlockTextTokenSchema.parse('browser-game.child.checking.title'),
  Approval: BrowserGameChildCheckingBlockTextTokenSchema.parse('browser-game.child.approval.title'),
  Blocked: BrowserGameChildCheckingBlockTextTokenSchema.parse('browser-game.child.blocked.title'),
  EducationalAllowed: BrowserGameChildCheckingBlockTextTokenSchema.parse(
    'browser-game.child.educational-allowed.title'
  ),
  TimeLimited: BrowserGameChildCheckingBlockTextTokenSchema.parse('browser-game.child.time-limited.title'),
  Manual: BrowserGameChildCheckingBlockTextTokenSchema.parse('browser-game.child.manual.title'),
  Unavailable: BrowserGameChildCheckingBlockTextTokenSchema.parse('browser-game.child.unavailable.title'),
} as const;
