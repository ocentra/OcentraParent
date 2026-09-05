use std::fmt::Display;

use super::super::JournalText;

use super::request::validate_request_id;

pub(crate) trait GeneratedNameInput: Display {
    fn is_generated_intent_temp_name(&self) -> bool;
    fn is_generated_intent_stage_name(&self) -> bool;
}

impl GeneratedNameInput for str {
    fn is_generated_intent_temp_name(&self) -> bool {
        let Some((request_id, suffix)) = self
            .strip_suffix(super::super::TEMP_SUFFIX)
            .and_then(|value| value.rsplit_once('.'))
        else {
            return false;
        };
        validate_request_id(request_id).is_ok()
            && super::super::GENERATED_INTENT_TEMP_SUFFIXES.contains(&suffix)
    }

    fn is_generated_intent_stage_name(&self) -> bool {
        let Some((request_id, suffix)) = self.rsplit_once(super::super::STAGE_SEPARATOR) else {
            return false;
        };
        validate_request_id(request_id).is_ok()
            && !suffix.is_empty()
            && suffix.bytes().all(|byte| byte.is_ascii_digit())
    }
}

impl GeneratedNameInput for String {
    fn is_generated_intent_temp_name(&self) -> bool {
        self.as_str().is_generated_intent_temp_name()
    }

    fn is_generated_intent_stage_name(&self) -> bool {
        self.as_str().is_generated_intent_stage_name()
    }
}

impl<T> GeneratedNameInput for &T
where
    T: GeneratedNameInput + ?Sized,
{
    fn is_generated_intent_temp_name(&self) -> bool {
        (*self).is_generated_intent_temp_name()
    }

    fn is_generated_intent_stage_name(&self) -> bool {
        (*self).is_generated_intent_stage_name()
    }
}

pub(crate) trait ReceiptTempNameInput: Display {
    fn is_generated_receipt_temp_name(&self) -> bool;
}

impl ReceiptTempNameInput for str {
    fn is_generated_receipt_temp_name(&self) -> bool {
        let Some(request_id) = self.strip_suffix(super::super::RECEIPT_TEMP_SUFFIX) else {
            return false;
        };
        validate_request_id(request_id).is_ok()
    }
}

impl ReceiptTempNameInput for String {
    fn is_generated_receipt_temp_name(&self) -> bool {
        self.as_str().is_generated_receipt_temp_name()
    }
}

impl<T> ReceiptTempNameInput for &T
where
    T: ReceiptTempNameInput + ?Sized,
{
    fn is_generated_receipt_temp_name(&self) -> bool {
        (*self).is_generated_receipt_temp_name()
    }
}

impl GeneratedNameInput for JournalText<'_> {
    fn is_generated_intent_temp_name(&self) -> bool {
        self.as_str().is_generated_intent_temp_name()
    }

    fn is_generated_intent_stage_name(&self) -> bool {
        self.as_str().is_generated_intent_stage_name()
    }
}

impl ReceiptTempNameInput for JournalText<'_> {
    fn is_generated_receipt_temp_name(&self) -> bool {
        self.as_str().is_generated_receipt_temp_name()
    }
}
