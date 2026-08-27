const PORTAL_HOST_BRIDGE_RUNTIME = {
  AbortErrorName: 'AbortError',
  UnavailableState: 'unavailable',
  TransportUnavailableReason: 'transport-unavailable',
  DirectEnforcementCommandBoundaryErrorText:
    'Portal cannot dispatch enforcement mutation commands directly; use the enforcement authority boundary.',
} as const;

export { PORTAL_HOST_BRIDGE_RUNTIME };
