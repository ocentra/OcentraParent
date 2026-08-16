/* generated from crates/schema/src/billing_checkout_portal_boundary_ts.rs */

import { type Infer, Schema, withParser } from './effect';
import { FamilyReferenceSchema, ParentAccountReferenceSchema, ParentActorReferenceSchema } from './family-references';
import { ParentTimestampSchema } from './family-reference-primitives';
import {
  BillingCheckoutAbuseGateStateSchema,
  BillingHostedCheckoutPlanIdSchema,
  BillingHostedCsrfStateSchema,
  BillingCheckoutPortalBoundarySchemaVersionSchema,
  BillingHostedCheckoutUrlSchema,
  BillingHostedOriginGateStateSchema,
  BillingHostedPortalUrlSchema,
  BillingHostedReturnPathSchema,
  BillingHostedReturnResolutionSchema,
  BillingHostedReturnRouteIdSchema,
  BillingHostedSessionIdSchema,
  BillingHostedSessionRejectionReasonSchema,
  BillingHostedSessionRequestIdSchema,
  BillingHostedSessionStatusSchema,
  BillingHostedSurfaceSecretCustodySchema,
} from './billing-checkout-portal-boundary-values';
import {
  GeneratedBillingHostedRouteContractById,
  GeneratedBillingHostedReturnRoute,
} from './generated-billing-checkout-portal-boundary-values';

type BillingHostedReturnRouteId = Infer<typeof BillingHostedReturnRouteIdSchema>;
type BillingHostedReturnPath = Infer<typeof BillingHostedReturnPathSchema>;
type BillingHostedReturnResolution = Infer<typeof BillingHostedReturnResolutionSchema>;

const BillingHostedReturnRouteStruct = Schema.Struct({
  routeId: BillingHostedReturnRouteIdSchema,
  relativePath: BillingHostedReturnPathSchema,
  resolution: BillingHostedReturnResolutionSchema,
});
type BillingHostedReturnRouteShape = Infer<typeof BillingHostedReturnRouteStruct>;

export const BillingHostedReturnRouteSchema = withParser(
  BillingHostedReturnRouteStruct.pipe(
    Schema.filter(
      (route: BillingHostedReturnRouteShape) =>
        billingHostedReturnRouteContract(route.routeId).relativePath === route.relativePath ||
        'Expected billing hosted session routes to use the exact allowlisted relative path for each route id'
    ),
    Schema.filter(
      (route: BillingHostedReturnRouteShape) =>
        billingHostedReturnRouteContract(route.routeId).resolution === route.resolution ||
        'Expected billing hosted session routes to keep explicit return resolution state per allowlisted route id'
    )
  )
);

const BillingCheckoutSessionRequestStruct = Schema.Struct({
  schemaVersion: BillingCheckoutPortalBoundarySchemaVersionSchema,
  requestId: BillingHostedSessionRequestIdSchema,
  kind: Schema.Literal('checkout-session-create'),
  actor: ParentActorReferenceSchema,
  parentAccount: ParentAccountReferenceSchema,
  family: FamilyReferenceSchema,
  planId: BillingHostedCheckoutPlanIdSchema,
  originGateState: BillingHostedOriginGateStateSchema,
  csrfState: BillingHostedCsrfStateSchema,
  surfaceSecretCustody: BillingHostedSurfaceSecretCustodySchema,
  successRoute: BillingHostedReturnRouteSchema,
  cancelRoute: BillingHostedReturnRouteSchema,
  abuseGateState: BillingCheckoutAbuseGateStateSchema,
});
type BillingCheckoutSessionRequestShape = Infer<typeof BillingCheckoutSessionRequestStruct>;

export const BillingCheckoutSessionRequestSchema = withParser(
  BillingCheckoutSessionRequestStruct.pipe(
    Schema.filter(
      (request: BillingCheckoutSessionRequestShape) =>
        billingActorMayCreateHostedSession(request.actor) ||
        'Expected interactive checkout session creation to require a parent or guardian actor'
    ),
    Schema.filter(
      (request: BillingCheckoutSessionRequestShape) =>
        request.successRoute.routeId === 'family-billing-checkout-success' ||
        'Expected checkout success redirects to use the allowlisted checkout success route'
    ),
    Schema.filter(
      (request: BillingCheckoutSessionRequestShape) =>
        request.cancelRoute.routeId === 'family-billing-checkout-cancel' ||
        'Expected checkout cancel redirects to use the allowlisted checkout cancel route'
    ),
    Schema.filter(
      (request: BillingCheckoutSessionRequestShape) =>
        request.successRoute.relativePath !== request.cancelRoute.relativePath ||
        'Expected checkout success and cancel redirects to remain distinct'
    )
  )
);

const BillingPortalSessionRequestStruct = Schema.Struct({
  schemaVersion: BillingCheckoutPortalBoundarySchemaVersionSchema,
  requestId: BillingHostedSessionRequestIdSchema,
  kind: Schema.Literal('billing-portal-session-create'),
  actor: ParentActorReferenceSchema,
  parentAccount: ParentAccountReferenceSchema,
  family: FamilyReferenceSchema,
  originGateState: BillingHostedOriginGateStateSchema,
  csrfState: BillingHostedCsrfStateSchema,
  surfaceSecretCustody: BillingHostedSurfaceSecretCustodySchema,
  returnRoute: BillingHostedReturnRouteSchema,
  abuseGateState: BillingCheckoutAbuseGateStateSchema,
});
type BillingPortalSessionRequestShape = Infer<typeof BillingPortalSessionRequestStruct>;

export const BillingPortalSessionRequestSchema = withParser(
  BillingPortalSessionRequestStruct.pipe(
    Schema.filter(
      (request: BillingPortalSessionRequestShape) =>
        billingActorMayCreateHostedSession(request.actor) ||
        'Expected interactive billing portal session creation to require a parent or guardian actor'
    ),
    Schema.filter(
      (request: BillingPortalSessionRequestShape) =>
        request.returnRoute.routeId === 'family-billing-portal-return' ||
        'Expected billing portal sessions to return through the allowlisted billing management route'
    )
  )
);

const BillingCheckoutSessionResponseStruct = Schema.Struct({
  schemaVersion: BillingCheckoutPortalBoundarySchemaVersionSchema,
  requestId: BillingHostedSessionRequestIdSchema,
  kind: Schema.Literal('checkout-session-create'),
  status: BillingHostedSessionStatusSchema,
  hostedSessionId: Schema.Union(BillingHostedSessionIdSchema, Schema.Null),
  hostedUrl: Schema.Union(BillingHostedCheckoutUrlSchema, Schema.Null),
  expiresAt: Schema.Union(ParentTimestampSchema, Schema.Null),
  rejectionReason: Schema.Union(BillingHostedSessionRejectionReasonSchema, Schema.Null),
});
type BillingCheckoutSessionResponseShape = Infer<typeof BillingCheckoutSessionResponseStruct>;

export const BillingCheckoutSessionResponseSchema = withParser(
  BillingCheckoutSessionResponseStruct.pipe(
    Schema.filter(
      (response: BillingCheckoutSessionResponseShape) =>
        billingHostedSessionResponseIsConsistent(
          response.status,
          response.hostedSessionId,
          response.hostedUrl,
          response.expiresAt,
          response.rejectionReason
        ) ||
        'Expected checkout session responses to be either accepted with a Stripe checkout URL or rejected with an explicit reason'
    )
  )
);

const BillingPortalSessionResponseStruct = Schema.Struct({
  schemaVersion: BillingCheckoutPortalBoundarySchemaVersionSchema,
  requestId: BillingHostedSessionRequestIdSchema,
  kind: Schema.Literal('billing-portal-session-create'),
  status: BillingHostedSessionStatusSchema,
  hostedSessionId: Schema.Union(BillingHostedSessionIdSchema, Schema.Null),
  hostedUrl: Schema.Union(BillingHostedPortalUrlSchema, Schema.Null),
  expiresAt: Schema.Union(ParentTimestampSchema, Schema.Null),
  rejectionReason: Schema.Union(BillingHostedSessionRejectionReasonSchema, Schema.Null),
});
type BillingPortalSessionResponseShape = Infer<typeof BillingPortalSessionResponseStruct>;

export const BillingPortalSessionResponseSchema = withParser(
  BillingPortalSessionResponseStruct.pipe(
    Schema.filter(
      (response: BillingPortalSessionResponseShape) =>
        billingHostedSessionResponseIsConsistent(
          response.status,
          response.hostedSessionId,
          response.hostedUrl,
          response.expiresAt,
          response.rejectionReason
        ) ||
        'Expected billing portal responses to be either accepted with a Stripe billing portal URL or rejected with an explicit reason'
    )
  )
);

export type BillingHostedReturnRoute = Infer<typeof BillingHostedReturnRouteSchema>;
export type BillingCheckoutSessionRequest = Infer<typeof BillingCheckoutSessionRequestSchema>;
export type BillingPortalSessionRequest = Infer<typeof BillingPortalSessionRequestSchema>;
export type BillingCheckoutSessionResponse = Infer<typeof BillingCheckoutSessionResponseSchema>;
export type BillingPortalSessionResponse = Infer<typeof BillingPortalSessionResponseSchema>;

export const BillingHostedReturnRoute = {
  CheckoutSuccess: BillingHostedReturnRouteSchema.parse(GeneratedBillingHostedReturnRoute.CheckoutSuccess),
  CheckoutCancel: BillingHostedReturnRouteSchema.parse(GeneratedBillingHostedReturnRoute.CheckoutCancel),
  PortalReturn: BillingHostedReturnRouteSchema.parse(GeneratedBillingHostedReturnRoute.PortalReturn),
} as const;

function billingHostedReturnRouteContract(routeId: BillingHostedReturnRouteId): {
  readonly relativePath: BillingHostedReturnPath;
  readonly resolution: BillingHostedReturnResolution;
} {
  switch (routeId) {
    case 'family-billing-checkout-success':
      return GeneratedBillingHostedRouteContractById['family-billing-checkout-success'];
    case 'family-billing-checkout-cancel':
      return GeneratedBillingHostedRouteContractById['family-billing-checkout-cancel'];
    case 'family-billing-portal-return':
      return GeneratedBillingHostedRouteContractById['family-billing-portal-return'];
    default:
      throw new Error(`Unexpected billing hosted return route id: ${routeId}`);
  }
}

function billingActorMayCreateHostedSession(actor: Infer<typeof ParentActorReferenceSchema>): boolean {
  return actor.role === 'parent' || actor.role === 'guardian';
}

function billingHostedSessionResponseIsConsistent(
  status: Infer<typeof BillingHostedSessionStatusSchema>,
  hostedSessionId: Infer<typeof BillingHostedSessionIdSchema> | null,
  hostedUrl: string | null,
  expiresAt: Infer<typeof ParentTimestampSchema> | null,
  rejectionReason: Infer<typeof BillingHostedSessionRejectionReasonSchema> | null
): boolean {
  if (status === 'accepted') {
    return hostedSessionId !== null && hostedUrl !== null && expiresAt !== null && rejectionReason === null;
  }
  return hostedSessionId === null && hostedUrl === null && expiresAt === null && rejectionReason !== null;
}
