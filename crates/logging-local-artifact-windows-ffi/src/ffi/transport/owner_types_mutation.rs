use super::Mutation;
use crate::constants::{
    APPEND_OPERATION, REMOVE_OPERATION, REMOVE_TREE_OPERATION, REPLACE_OPERATION,
};

impl Mutation {
    pub(crate) fn operation_name(&self) -> &'static str {
        match self {
            Self::Append { .. } => APPEND_OPERATION,
            Self::Replace { .. } => REPLACE_OPERATION,
            Self::Remove { .. } => REMOVE_OPERATION,
            Self::RemoveTree { .. } => REMOVE_TREE_OPERATION,
        }
    }

    pub(crate) fn relative_path(&self) -> &str {
        match self {
            Self::Append { relative_path, .. }
            | Self::Replace { relative_path, .. }
            | Self::Remove { relative_path }
            | Self::RemoveTree { relative_path } => relative_path,
        }
    }

    pub(crate) fn payload(&self) -> Option<&[u8]> {
        match self {
            Self::Append { payload, .. } | Self::Replace { payload, .. } => Some(payload),
            Self::Remove { .. } | Self::RemoveTree { .. } => None,
        }
    }
}
