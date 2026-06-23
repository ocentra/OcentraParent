import { Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';

export const ProductionIncidentSupportStatusSchemaVersionSchema = withParser(
  Schema.Literal('production-incident-support-status-proof')
);

export const ProductionIncidentSupportStatusSurfaceSchema = withParser(
  Schema.Literal(
    'support-incident-intake',
    'parent-consent-status',
    'privacy-legal-disclosure-status',
    'data-export-request-status',
    'delete-request-status',
    'incident-publication-status',
    'case-resolution-handoff-status'
  )
);

export const ProductionIncidentSupportStatusStateSchema = withParser(
  Schema.Literal(
    'source-contract-ready',
    'parent-consent-required',
    'disclosure-required',
    'custody-review-required',
    'export-delete-ready',
    'manual-required',
    'publication-required',
    'backend-required',
    'not-implemented',
    'implemented',
    'executed'
  )
);

export const ProductionIncidentSupportStatusSourceProofSchema = withParser(
  Schema.Literal(
    'support-incident-workflow-proof',
    'production-support-backend-upload-custody-audit-proof',
    'production-support-case-resolution-status-proof',
    'production-support-publication-workflow-proof',
    'production-release-public-docs-status-proof',
    'public-support-contact-status-proof',
    'data-custody-expectation'
  )
);

export const ProductionIncidentSupportStatusDataClassSchema = withParser(
  Schema.Literal(
    'incident-status-metadata',
    'parent-consent-reference',
    'privacy-legal-disclosure-status',
    'data-export-delete-status',
    'redaction-summary-reference',
    'custody-audit-reference',
    'case-resolution-status',
    'manual-proof-reference',
    'support-runbook-reference',
    'public-status-reference',
    'child-activity-evidence',
    'raw-support-bundle',
    'browser-url-history',
    'screenshots',
    'journals',
    'sqlite-snapshots',
    'private-paths',
    'provider-secrets',
    'account-lookup-result',
    'billing-provider-contact-record',
    'remote-support-session-transcript',
    'parent-rules-source-of-truth'
  )
);

export const ProductionIncidentSupportStatusNonClaimSchema = withParser(
  Schema.Literal(
    'no-family-ocentra-publication',
    'no-legal-execution',
    'no-support-backend-upload-execution',
    'no-account-lookup-execution',
    'no-billing-provider-contact',
    'no-remote-support-session',
    'no-production-sla',
    'no-child-activity-custody',
    'no-provider-secrets'
  )
);

export const ProductionIncidentSupportStatusReferenceSchema = brandedNonEmptyStringSchema(
  'ProductionIncidentSupportStatusReference'
);
export const ProductionIncidentSupportStatusRequirementSchema = brandedNonEmptyStringSchema(
  'ProductionIncidentSupportStatusRequirement'
);

export const ForbiddenProductionIncidentSupportStatusDataClasses = [
  'child-activity-evidence',
  'raw-support-bundle',
  'browser-url-history',
  'screenshots',
  'journals',
  'sqlite-snapshots',
  'private-paths',
  'provider-secrets',
  'account-lookup-result',
  'billing-provider-contact-record',
  'remote-support-session-transcript',
  'parent-rules-source-of-truth',
] as const;

export const RequiredProductionIncidentSupportStatusSurfaces = [
  'support-incident-intake',
  'parent-consent-status',
  'privacy-legal-disclosure-status',
  'data-export-request-status',
  'delete-request-status',
  'incident-publication-status',
  'case-resolution-handoff-status',
] as const;

export const RequiredProductionIncidentSupportStatusNonClaims = [
  'no-family-ocentra-publication',
  'no-legal-execution',
  'no-support-backend-upload-execution',
  'no-account-lookup-execution',
  'no-billing-provider-contact',
  'no-remote-support-session',
  'no-production-sla',
  'no-child-activity-custody',
  'no-provider-secrets',
] as const;
