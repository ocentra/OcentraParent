import { NonEmptyStringSchema, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  BillingFeatureCodeSchema,
  BillingPlanActiveStateSchema,
  BillingPlanIdSchema,
  NonNegativeBillingCountSchema,
  PositiveBillingLimitSchema,
} from '@ocentra-parent/schema-domain/billing-entitlement-values';
import { ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import type { PricingPlanSummary } from './fixtures.js';

export type HealthStatusResponse = {
  readonly status: 'ok';
  readonly service: 'cloudflare-control-plane';
  readonly environment: string;
  readonly authAdapterMode: string;
  readonly routeCount: number;
  readonly implementedHandlerCount: number;
  readonly bindingStatus: 'ready' | 'degraded';
  readonly missingBindingCount: number;
  readonly seedSummary: {
    readonly pricingPlanCount: number;
    readonly adminAccountCount: number;
    readonly referralFixtureCount: number;
  } | null;
};

export type BillingPricingPublicResponse = {
  readonly status: 'ok';
  readonly plans: ReadonlyArray<PricingPlanSummary>;
  readonly updatedAt: string;
};

const NonNegativeRouteCountSchema = Schema.Number.pipe(Schema.int(), Schema.greaterThanOrEqualTo(0));

const PricingFeatureSchema = Schema.Struct({
  code: BillingFeatureCodeSchema,
  label: NonEmptyStringSchema,
  included: Schema.Boolean,
  safetyCritical: Schema.Boolean,
});

const PricingPlanSchema = Schema.Struct({
  planId: BillingPlanIdSchema,
  displayName: NonEmptyStringSchema,
  interval: Schema.Literal('monthly', 'yearly'),
  priceCents: NonNegativeBillingCountSchema,
  currency: Schema.Literal('USD'),
  deviceLimit: PositiveBillingLimitSchema,
  activeState: BillingPlanActiveStateSchema,
  featureSummary: Schema.Array(PricingFeatureSchema),
});

export const HealthStatusResponseSchema = withParser(
  Schema.Struct({
    status: Schema.Literal('ok'),
    service: Schema.Literal('cloudflare-control-plane'),
    environment: NonEmptyStringSchema,
    authAdapterMode: NonEmptyStringSchema,
    routeCount: NonNegativeRouteCountSchema,
    implementedHandlerCount: NonNegativeRouteCountSchema,
    bindingStatus: Schema.Literal('ready', 'degraded'),
    missingBindingCount: NonNegativeRouteCountSchema,
    seedSummary: Schema.NullOr(
      Schema.Struct({
        pricingPlanCount: NonNegativeRouteCountSchema,
        adminAccountCount: NonNegativeRouteCountSchema,
        referralFixtureCount: NonNegativeRouteCountSchema,
      })
    ),
  })
);

export const BillingPricingPublicResponseSchema = withParser(
  Schema.Struct({
    status: Schema.Literal('ok'),
    plans: Schema.Array(PricingPlanSchema),
    updatedAt: ParentTimestampSchema,
  })
);
