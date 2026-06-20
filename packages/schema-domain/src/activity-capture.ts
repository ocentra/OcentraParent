import { type Infer, Schema, withParser } from './effect';

export const ActivityObservationModeSchema = withParser(
  Schema.Literal('snapshot', 'active-window', 'network-snapshot')
);

export const ActivityCaptureCapabilityStatusSchema = withParser(
  Schema.Literal(
    'available',
    'unavailable',
    'access-denied',
    'no-active-window',
    'no-network-observations',
    'adapter-error'
  )
);

export const ActivityNetworkProtocolSchema = withParser(Schema.Literal('tcp', 'udp'));

export const ActivityNetworkTcpStateSchema = withParser(
  Schema.Literal(
    'closed',
    'listen',
    'syn-sent',
    'syn-received',
    'established',
    'fin-wait-1',
    'fin-wait-2',
    'close-wait',
    'closing',
    'last-ack',
    'time-wait',
    'delete-tcb',
    'unknown'
  )
);

export const ActivityDomainAttributionStatusSchema = withParser(
  Schema.Literal('domain-observed', 'ip-only', 'unavailable')
);

export const ActivityProcessAttributionStatusSchema = withParser(
  Schema.Literal('process-attributed', 'process-unknown')
);

export type ActivityObservationMode = Infer<typeof ActivityObservationModeSchema>;
export type ActivityCaptureCapabilityStatus = Infer<typeof ActivityCaptureCapabilityStatusSchema>;
export type ActivityNetworkProtocol = Infer<typeof ActivityNetworkProtocolSchema>;
export type ActivityNetworkTcpState = Infer<typeof ActivityNetworkTcpStateSchema>;
export type ActivityDomainAttributionStatus = Infer<typeof ActivityDomainAttributionStatusSchema>;
export type ActivityProcessAttributionStatus = Infer<typeof ActivityProcessAttributionStatusSchema>;

export const ActivityObservationMode = {
  Snapshot: ActivityObservationModeSchema.parse('snapshot'),
  ActiveWindow: ActivityObservationModeSchema.parse('active-window'),
  NetworkSnapshot: ActivityObservationModeSchema.parse('network-snapshot'),
} as const;

export const ActivityCaptureCapabilityStatus = {
  Available: ActivityCaptureCapabilityStatusSchema.parse('available'),
  Unavailable: ActivityCaptureCapabilityStatusSchema.parse('unavailable'),
  AccessDenied: ActivityCaptureCapabilityStatusSchema.parse('access-denied'),
  NoActiveWindow: ActivityCaptureCapabilityStatusSchema.parse('no-active-window'),
  NoNetworkObservations: ActivityCaptureCapabilityStatusSchema.parse('no-network-observations'),
  AdapterError: ActivityCaptureCapabilityStatusSchema.parse('adapter-error'),
} as const;

export const ActivityNetworkProtocol = {
  Tcp: ActivityNetworkProtocolSchema.parse('tcp'),
  Udp: ActivityNetworkProtocolSchema.parse('udp'),
} as const;

export const ActivityNetworkTcpState = {
  Closed: ActivityNetworkTcpStateSchema.parse('closed'),
  Listen: ActivityNetworkTcpStateSchema.parse('listen'),
  SynSent: ActivityNetworkTcpStateSchema.parse('syn-sent'),
  SynReceived: ActivityNetworkTcpStateSchema.parse('syn-received'),
  Established: ActivityNetworkTcpStateSchema.parse('established'),
  FinWait1: ActivityNetworkTcpStateSchema.parse('fin-wait-1'),
  FinWait2: ActivityNetworkTcpStateSchema.parse('fin-wait-2'),
  CloseWait: ActivityNetworkTcpStateSchema.parse('close-wait'),
  Closing: ActivityNetworkTcpStateSchema.parse('closing'),
  LastAck: ActivityNetworkTcpStateSchema.parse('last-ack'),
  TimeWait: ActivityNetworkTcpStateSchema.parse('time-wait'),
  DeleteTcb: ActivityNetworkTcpStateSchema.parse('delete-tcb'),
  Unknown: ActivityNetworkTcpStateSchema.parse('unknown'),
} as const;

export const ActivityDomainAttributionStatus = {
  DomainObserved: ActivityDomainAttributionStatusSchema.parse('domain-observed'),
  IpOnly: ActivityDomainAttributionStatusSchema.parse('ip-only'),
  Unavailable: ActivityDomainAttributionStatusSchema.parse('unavailable'),
} as const;

export const ActivityProcessAttributionStatus = {
  ProcessAttributed: ActivityProcessAttributionStatusSchema.parse('process-attributed'),
  ProcessUnknown: ActivityProcessAttributionStatusSchema.parse('process-unknown'),
} as const;
