import {
  ActivityNetworkAdapterIdSchema,
  ActivityNetworkCustodyStateSchema,
  ActivityNetworkDomainNameSchema,
  ActivityNetworkEndpointAddressSchema,
  ActivityNetworkEndpointSchema,
  ActivityNetworkFlowCountersSchema,
  ActivityNetworkFlowDigestSchema,
  ActivityNetworkFlowIndicatorKindSchema,
  ActivityNetworkFlowIndicatorSchema,
  ActivityNetworkFlowObservationSchema,
  ActivityNetworkFlowReadModelSchema,
  ActivityNetworkFlowRollupSchema,
  ActivityNetworkFlowRowVisibilitySchema,
  ActivityNetworkProcessNameSchema,
  ActivityQuerySchemaVersion,
} from '@ocentra-parent/network-domain/network-flow';
import type {
  ActivityNetworkAdapterId as UpstreamActivityNetworkAdapterId,
  ActivityNetworkCustodyState as UpstreamActivityNetworkCustodyState,
  ActivityNetworkDomainName as UpstreamActivityNetworkDomainName,
  ActivityNetworkEndpoint as UpstreamActivityNetworkEndpoint,
  ActivityNetworkEndpointAddress as UpstreamActivityNetworkEndpointAddress,
  ActivityNetworkFlowCounters as UpstreamActivityNetworkFlowCounters,
  ActivityNetworkFlowDigest as UpstreamActivityNetworkFlowDigest,
  ActivityNetworkFlowIndicator as UpstreamActivityNetworkFlowIndicator,
  ActivityNetworkFlowIndicatorKind as UpstreamActivityNetworkFlowIndicatorKind,
  ActivityNetworkFlowObservation as UpstreamActivityNetworkFlowObservation,
  ActivityNetworkFlowReadModel as UpstreamActivityNetworkFlowReadModel,
  ActivityNetworkFlowRollup as UpstreamActivityNetworkFlowRollup,
  ActivityNetworkFlowRowVisibility as UpstreamActivityNetworkFlowRowVisibility,
  ActivityNetworkProcessName as UpstreamActivityNetworkProcessName,
} from '@ocentra-parent/network-domain/network-flow';

export {
  ActivityQuerySchemaVersion,
  ActivityNetworkEndpointAddressSchema,
  ActivityNetworkDomainNameSchema,
  ActivityNetworkProcessNameSchema,
  ActivityNetworkAdapterIdSchema,
  ActivityNetworkCustodyStateSchema,
  ActivityNetworkFlowIndicatorKindSchema,
  ActivityNetworkFlowRowVisibilitySchema,
  ActivityNetworkEndpointSchema,
  ActivityNetworkFlowCountersSchema,
  ActivityNetworkFlowObservationSchema,
  ActivityNetworkFlowReadModelSchema,
  ActivityNetworkFlowRollupSchema,
  ActivityNetworkFlowIndicatorSchema,
  ActivityNetworkFlowDigestSchema,
};

export type ActivityNetworkEndpointAddress = UpstreamActivityNetworkEndpointAddress;
export type ActivityNetworkDomainName = UpstreamActivityNetworkDomainName;
export type ActivityNetworkProcessName = UpstreamActivityNetworkProcessName;
export type ActivityNetworkAdapterId = UpstreamActivityNetworkAdapterId;
export type ActivityNetworkCustodyState = UpstreamActivityNetworkCustodyState;
export type ActivityNetworkFlowIndicatorKind = UpstreamActivityNetworkFlowIndicatorKind;
export type ActivityNetworkFlowRowVisibility = UpstreamActivityNetworkFlowRowVisibility;
export type ActivityNetworkEndpoint = UpstreamActivityNetworkEndpoint;
export type ActivityNetworkFlowCounters = UpstreamActivityNetworkFlowCounters;
export type ActivityNetworkFlowObservation = UpstreamActivityNetworkFlowObservation;
export type ActivityNetworkFlowReadModel = UpstreamActivityNetworkFlowReadModel;
export type ActivityNetworkFlowRollup = UpstreamActivityNetworkFlowRollup;
export type ActivityNetworkFlowIndicator = UpstreamActivityNetworkFlowIndicator;
export type ActivityNetworkFlowDigest = UpstreamActivityNetworkFlowDigest;
