import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';

export const ParentAssistantRunStateSchema = withParser(
  Schema.Literal('queued', 'active', 'completed', 'failed', 'cancelled', 'degraded', 'unavailable')
);

export type ParentAssistantRunState = Infer<typeof ParentAssistantRunStateSchema>;
