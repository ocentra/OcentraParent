import { Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';

export const BrowserSocialRiskBenefitSignalSchemaVersion = 1;

export const BrowserSocialRiskBenefitSignalSetIdSchema = withParser(
  brandedNonEmptyStringSchema('BrowserSocialRiskBenefitSignalSetId')
);
export const BrowserSocialRiskSignalIdSchema = withParser(brandedNonEmptyStringSchema('BrowserSocialRiskSignalId'));
export const BrowserSocialBenefitSignalIdSchema = withParser(
  brandedNonEmptyStringSchema('BrowserSocialBenefitSignalId')
);

export const BrowserSocialRiskSignalKindSchema = withParser(
  Schema.Literal(
    'adult-content',
    'grooming-contact',
    'unknown-adult-contact',
    'cyberbullying',
    'self-harm',
    'violence',
    'addictive-feed',
    'misinformation',
    'secondary-account-risk',
    'privacy-exposure',
    'unsafe-messaging',
    'unknown-risk'
  )
);
export const BrowserSocialBenefitSignalKindSchema = withParser(
  Schema.Literal(
    'educational-video',
    'homework-help',
    'creativity',
    'skill-building',
    'healthy-social-connection',
    'parent-approved-account',
    'neutral-benefit',
    'unknown-benefit'
  )
);
export const BrowserSocialSignalSeveritySchema = withParser(Schema.Literal('none', 'low', 'medium', 'high', 'unknown'));
export const BrowserSocialSignalStateSchema = withParser(Schema.Literal('candidate', 'manual-required', 'unavailable'));
export const BrowserSocialSignalSourceKindSchema = withParser(
  Schema.Literal('social-ai-analysis', 'social-route-evidence', 'social-metadata', 'manual-required')
);
