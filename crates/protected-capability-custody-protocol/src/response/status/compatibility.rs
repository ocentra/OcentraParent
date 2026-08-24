use crate::request::RequestKind;

use super::ResponseStatus;

impl ResponseStatus {
    pub(crate) fn is_compatible_with(self, request_kind: RequestKind) -> bool {
        match self {
            Self::Rejected | Self::Unavailable | Self::UnsupportedPlatform => true,
            Self::Prepared => matches!(
                request_kind,
                RequestKind::Prepare | RequestKind::Recover | RequestKind::ResolveAmbiguity
            ),
            Self::PrepareAmbiguous => matches!(
                request_kind,
                RequestKind::Prepare | RequestKind::Recover | RequestKind::ResolveAmbiguity
            ),
            Self::Committed => matches!(
                request_kind,
                RequestKind::Commit | RequestKind::Recover | RequestKind::ResolveAmbiguity
            ),
            Self::Aborted => matches!(
                request_kind,
                RequestKind::Abort | RequestKind::Recover | RequestKind::ResolveAmbiguity
            ),
            Self::CommitAmbiguous => matches!(
                request_kind,
                RequestKind::Commit | RequestKind::Recover | RequestKind::ResolveAmbiguity
            ),
            Self::AbortAmbiguous => matches!(
                request_kind,
                RequestKind::Abort | RequestKind::Recover | RequestKind::ResolveAmbiguity
            ),
        }
    }
}
