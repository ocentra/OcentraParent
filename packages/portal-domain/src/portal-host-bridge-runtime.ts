const PORTAL_HOST_BRIDGE_RUNTIME = {
  AbortErrorName: 'AbortError',
  UnavailableState: 'unavailable',
  TransportUnavailableReason: 'transport-unavailable',
  DirectEnforcementCommandBoundaryErrorText:
    'Portal cannot dispatch enforcement mutation commands directly; use the enforcement authority boundary.',
  RouteSchemaMismatchTitle: 'The parent host response failed Rust-owned schema decoding.',
  RouteIdentityMismatchTitle: 'The parent host response did not match the requested Rust-owned route.',
} as const;

export { PORTAL_HOST_BRIDGE_RUNTIME };
