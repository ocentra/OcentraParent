import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';

export const BrowserGameHoldBlockAdapterSchemaVersionSchema = withParser(
  Schema.Literal('browser-game-hold-block-adapter-contract')
);

export const BrowserGameHoldBlockAdapterPlanIdSchema = withParser(
  brandedNonEmptyStringSchema('BrowserGameHoldBlockAdapterPlanId')
);

export const BrowserGameHoldBlockTargetKindSchema = withParser(
  Schema.Literal(
    'managed-browser-game-page',
    'managed-game-portal',
    'managed-cloud-gaming-session',
    'managed-game-account-flow',
    'managed-game-purchase-flow',
    'unknown-managed-game',
    'manual-required'
  )
);

export const BrowserGameHoldBlockActionSchema = withParser(
  Schema.Literal(
    'hold-until-classified',
    'hold-until-parent-approval',
    'block-game-route',
    'warn-before-play',
    'allow-educational-game',
    'time-limit-candidate',
    'manual-required',
    'unavailable'
  )
);

export const BrowserGameHoldBlockAdapterStateSchema = withParser(
  Schema.Literal('adapter-proof-present', 'candidate-only', 'manual-required', 'unavailable')
);

export const BrowserGameHoldBlockDeliveryModeSchema = withParser(
  Schema.Literal('managed-intervention-proof-ref', 'contract-only', 'manual-required', 'unavailable')
);

export const BrowserGameHoldBlockFallbackActionSchema = withParser(
  Schema.Literal(
    'show-checking-page',
    'show-approval-page',
    'show-block-page',
    'show-warning-page',
    'continue-session',
    'manual-review',
    'no-action'
  )
);

export const BrowserGameHoldBlockReasonSchema = withParser(
  Schema.Literal(
    'policy-candidate-block',
    'policy-candidate-parent-review',
    'policy-candidate-warn',
    'policy-candidate-time-limit',
    'educational-allow-candidate',
    'unknown-game-needs-classification',
    'managed-intervention-proof-present',
    'cloud-gaming-proof-manual-required',
    'missing-managed-route-proof',
    'missing-adapter-proof',
    'native-game-control-unavailable',
    'unmanaged-browser-not-supported'
  )
);

export type BrowserGameHoldBlockAction = Infer<typeof BrowserGameHoldBlockActionSchema>;
export type BrowserGameHoldBlockAdapterState = Infer<typeof BrowserGameHoldBlockAdapterStateSchema>;
export type BrowserGameHoldBlockDeliveryMode = Infer<typeof BrowserGameHoldBlockDeliveryModeSchema>;

