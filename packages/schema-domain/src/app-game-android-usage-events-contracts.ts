import { EventingEventTypeSchema } from '@ocentra-parent/schema-domain/eventing';
import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';

export const AppGameAndroidUsageEventsCommandName = {
  CapabilityGet: 'app-game.android.usage-events.capability.get',
  ReplayBoundaryGet: 'app-game.android.usage-events.replay-boundary.get',
  RuntimePreflightGet: 'app-game.android.usage-events.runtime-preflight.get',
} as const;

export const AppGameAndroidUsageEventsEventNameLiteral = {
  CapabilityReported: 'app-game.android.usage-events.capability.reported',
  ReplayBoundaryReported: 'app-game.android.usage-events.replay-boundary.reported',
  RuntimePreflightReported: 'app-game.android.usage-events.runtime-preflight.reported',
} as const;

export const AppGameAndroidUsageEventsEventName = {
  CapabilityReported: EventingEventTypeSchema.parse(AppGameAndroidUsageEventsEventNameLiteral.CapabilityReported),
  ReplayBoundaryReported: EventingEventTypeSchema.parse(
    AppGameAndroidUsageEventsEventNameLiteral.ReplayBoundaryReported
  ),
  RuntimePreflightReported: EventingEventTypeSchema.parse(
    AppGameAndroidUsageEventsEventNameLiteral.RuntimePreflightReported
  ),
} as const;

export const AppGameAndroidUsageEventsCommandNameSchema = withParser(
  Schema.Literal(...Object.values(AppGameAndroidUsageEventsCommandName))
);

export const AppGameAndroidUsageEventsEventNameSchema = withParser(
  Schema.Literal(...Object.values(AppGameAndroidUsageEventsEventName))
);

export type AppGameAndroidUsageEventsCommandName = Infer<typeof AppGameAndroidUsageEventsCommandNameSchema>;
export type AppGameAndroidUsageEventsEventName = Infer<typeof AppGameAndroidUsageEventsEventNameSchema>;
