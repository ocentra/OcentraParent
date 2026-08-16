/* generated from crates/schema/src/billing_support_admin_common_values_ts.rs */

import { Schema, withParser } from './effect';
import { ParentTimestampSchema } from './family-reference-primitives';
import {
  BillingFailureKindSchema,
  BillingLocalSafetyBehaviorSchema,
  BillingParentResolutionSchema,
  BillingParentVisibleStateSchema,
} from './billing-entitlement-values';

export function buildBillingFailureStateSchema(failureLabel: string) {
  return withParser(
    Schema.Struct({
      failureKind: BillingFailureKindSchema,
      parentVisibleState: BillingParentVisibleStateSchema,
      localSafetyBehavior: BillingLocalSafetyBehaviorSchema,
      retainEvidenceExportAccess: Schema.Boolean,
      existingLocalSafetyContinues: Schema.Boolean,
      parentResolution: BillingParentResolutionSchema,
      retryAllowed: Schema.Boolean,
      retryAfter: Schema.Union(ParentTimestampSchema, Schema.Null),
    }).pipe(
      Schema.filter(
        (failure) =>
          failure.retainEvidenceExportAccess ||
          `Expected ${failureLabel} failures to retain evidence export and safety-critical audit access`
      ),
      Schema.filter(
        (failure) =>
          failure.existingLocalSafetyContinues ||
          `Expected ${failureLabel} failures to keep existing local safety behavior explicit`
      )
    )
  );
}
