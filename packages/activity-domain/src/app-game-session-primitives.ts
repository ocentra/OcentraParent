import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';

const NonEmptyAppGameSessionText = Schema.String.pipe(Schema.minLength(1));

export const AppGameSessionRollupDateSchema = NonEmptyAppGameSessionText.pipe(Schema.brand('AppGameSessionRollupDate'));

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
