import { type Infer, Schema, brandedNonEmptyStringSchema, withParser } from './effect';

export const AppGameSessionRollupDateSchema = brandedNonEmptyStringSchema('AppGameSessionRollupDate');

export const AppGameSessionEndReasonSchema = withParser(
  Schema.Literal('processExit', 'timeoutInferred', 'deviceShutdown', 'agentRestart', 'unknown')
);

export const AppGameSessionEndReason = {
  ProcessExit: AppGameSessionEndReasonSchema.parse('processExit'),
  TimeoutInferred: AppGameSessionEndReasonSchema.parse('timeoutInferred'),
  DeviceShutdown: AppGameSessionEndReasonSchema.parse('deviceShutdown'),
  AgentRestart: AppGameSessionEndReasonSchema.parse('agentRestart'),
  Unknown: AppGameSessionEndReasonSchema.parse('unknown'),
} as const;

export type AppGameSessionEndReason = Infer<typeof AppGameSessionEndReasonSchema>;
export type AppGameSessionRollupDate = Infer<typeof AppGameSessionRollupDateSchema>;
