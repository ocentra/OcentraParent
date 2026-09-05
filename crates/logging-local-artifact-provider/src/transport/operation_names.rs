#[path = "operation_name_directory.rs"]
mod directory;
#[path = "operation_name_lifecycle.rs"]
mod lifecycle;
#[path = "operation_name_mutation.rs"]
mod mutation;

use super::{Operation, OperationName};

impl OperationName {
    pub(crate) fn text(self) -> String {
        super::text::OPERATION_TEXT[self as usize].text()
    }
}

impl From<OperationName> for String {
    fn from(value: OperationName) -> Self {
        value.text()
    }
}

impl Operation {
    pub(crate) fn name(&self) -> OperationName {
        if lifecycle::contains(self) {
            return lifecycle::name(self);
        }
        if directory::contains(self) {
            return directory::name(self);
        }
        mutation::name(self)
    }
}
