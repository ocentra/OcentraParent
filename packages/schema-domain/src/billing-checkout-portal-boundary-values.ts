import { type Infer, NonEmptyStringSchema, Schema, brandedNonEmptyStringSchema, withParser } from './effect';
import { BillingPlanIdSchema } from './billing-entitlement-values';
import {
  GeneratedBillingCheckoutAbuseGateStates,
  GeneratedBillingCheckoutPortalBoundarySchemaVersion,
  GeneratedBillingHostedCheckoutPlanIds,
  GeneratedBillingHostedCsrfStates,
  GeneratedBillingHostedOriginGateStates,
  GeneratedBillingHostedReturnPaths,
  GeneratedBillingHostedReturnResolutions,
  GeneratedBillingHostedReturnRouteIds,
  GeneratedBillingHostedSessionKinds,
  GeneratedBillingHostedSessionRejectionReasons,
  GeneratedBillingHostedSessionStatuses,
  GeneratedBillingHostedSurfaceSecretCustodies,
} from './generated-billing-checkout-portal-boundary-values';

export const BillingCheckoutPortalBoundarySchemaVersionSchema = withParser(
  Schema.Literal(GeneratedBillingCheckoutPortalBoundarySchemaVersion)
);

export const BillingHostedSessionKindSchema = withParser(Schema.Literal(...GeneratedBillingHostedSessionKinds));

export const BillingHostedSessionStatusSchema = withParser(Schema.Literal(...GeneratedBillingHostedSessionStatuses));

export const BillingCheckoutAbuseGateStateSchema = withParser(
  Schema.Literal(...GeneratedBillingCheckoutAbuseGateStates)
);

export const BillingHostedCheckoutPlanIdSchema = withParser(
  BillingPlanIdSchema.pipe(
    Schema.filter(
      (value) =>
        GeneratedBillingHostedCheckoutPlanIds.some((planId) => planId === value) ||
        'Expected hosted checkout requests to reject unknown or non-billable plan ids'
    )
  )
);

export const BillingHostedOriginGateStateSchema = withParser(Schema.Literal(...GeneratedBillingHostedOriginGateStates));

export const BillingHostedCsrfStateSchema = withParser(Schema.Literal(...GeneratedBillingHostedCsrfStates));

export const BillingHostedSurfaceSecretCustodySchema = withParser(
  Schema.Literal(...GeneratedBillingHostedSurfaceSecretCustodies)
);

export const BillingHostedSessionRejectionReasonSchema = withParser(
  Schema.Literal(...GeneratedBillingHostedSessionRejectionReasons)
);

export const BillingHostedReturnRouteIdSchema = withParser(Schema.Literal(...GeneratedBillingHostedReturnRouteIds));

export const BillingHostedReturnPathSchema = withParser(
  NonEmptyStringSchema.pipe(
    Schema.filter(
      (value) =>
        GeneratedBillingHostedReturnPaths.some((path) => path === value) ||
        'Expected billing checkout and portal paths to stay inside the allowlisted family billing routes'
    )
  )
);

export const BillingHostedReturnResolutionSchema = withParser(
  Schema.Literal(...GeneratedBillingHostedReturnResolutions)
);

export const BillingHostedSessionRequestIdSchema = brandedNonEmptyStringSchema('BillingHostedSessionRequestId');
export const BillingHostedSessionIdSchema = brandedNonEmptyStringSchema('BillingHostedSessionId');

export const BillingHostedCheckoutUrlSchema = withParser(
  NonEmptyStringSchema.pipe(
    Schema.filter(
      (value) =>
        value.startsWith('https://checkout.stripe.com/') ||
        'Expected checkout session redirects to stay on Stripe-hosted checkout'
    ),
    Schema.filter(
      (value) =>
        !value.includes('client_secret=') ||
        'Expected checkout session redirects not to leak client_secret values into the browser boundary'
    )
  )
);

export const BillingHostedPortalUrlSchema = withParser(
  NonEmptyStringSchema.pipe(
    Schema.filter(
      (value) =>
        value.startsWith('https://billing.stripe.com/') ||
        'Expected billing portal redirects to stay on Stripe-hosted billing portal'
    ),
    Schema.filter(
      (value) =>
        !value.includes('client_secret=') ||
        'Expected billing portal redirects not to leak client_secret values into the browser boundary'
    )
  )
);

export type BillingHostedSessionKind = Infer<typeof BillingHostedSessionKindSchema>;
export type BillingHostedSessionStatus = Infer<typeof BillingHostedSessionStatusSchema>;
export type BillingCheckoutAbuseGateState = Infer<typeof BillingCheckoutAbuseGateStateSchema>;
export type BillingHostedCheckoutPlanId = Infer<typeof BillingHostedCheckoutPlanIdSchema>;
export type BillingHostedOriginGateState = Infer<typeof BillingHostedOriginGateStateSchema>;
export type BillingHostedCsrfState = Infer<typeof BillingHostedCsrfStateSchema>;
export type BillingHostedSurfaceSecretCustody = Infer<typeof BillingHostedSurfaceSecretCustodySchema>;
export type BillingHostedSessionRejectionReason = Infer<typeof BillingHostedSessionRejectionReasonSchema>;
export type BillingHostedReturnRouteId = Infer<typeof BillingHostedReturnRouteIdSchema>;
export type BillingHostedReturnResolution = Infer<typeof BillingHostedReturnResolutionSchema>;
