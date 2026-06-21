import { type Infer, Schema, withParser } from './effect';

export const ParentAssistantRunStateSchema = withParser(
  Schema.Literal('queued', 'active', 'completed', 'failed', 'cancelled', 'degraded', 'unavailable')
);

export type ParentAssistantRunState = Infer<typeof ParentAssistantRunStateSchema>;
