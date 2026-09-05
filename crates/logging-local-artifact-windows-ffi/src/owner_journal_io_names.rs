use super::*;

pub(super) fn receipt_temp<N>(name: &N) -> bool
where
    N: descriptors::generated_names::ReceiptTempNameInput + ?Sized,
{
    name.is_generated_receipt_temp_name()
}
