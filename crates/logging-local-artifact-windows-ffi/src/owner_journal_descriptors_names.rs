use std::fmt::Display;
use std::path::Path;

use super::super::{
    JournalPath, BRIDGE_DIRECTORY, INTENTS_DIRECTORY, JSON_SUFFIX, MUTATION_OWNER_DIRECTORY,
    RECEIPTS_DIRECTORY, TEMP_SUFFIX,
};

pub(crate) trait TempNameInput: NameOutputInput<String> {
    fn write_with_suffix<S>(&self, suffix: &S, output: &mut super::super::NameText)
    where
        S: Display + ?Sized;
}

pub(crate) trait NameOutputInput<D> {}

impl TempNameInput for str {
    fn write_with_suffix<S>(&self, suffix: &S, output: &mut super::super::NameText)
    where
        S: Display + ?Sized,
    {
        output.push_text(self);
        output.push('.');
        output.push_text(&suffix.to_string());
        output.push_text(TEMP_SUFFIX);
    }
}

impl NameOutputInput<String> for str {}

impl TempNameInput for super::super::JournalText<'_> {
    fn write_with_suffix<S>(&self, suffix: &S, output: &mut super::super::NameText)
    where
        S: Display + ?Sized,
    {
        self.as_str().write_with_suffix(suffix, output);
    }
}

impl NameOutputInput<String> for super::super::JournalText<'_> {}

impl TempNameInput for String {
    fn write_with_suffix<S>(&self, suffix: &S, output: &mut super::super::NameText)
    where
        S: Display + ?Sized,
    {
        self.as_str().write_with_suffix(suffix, output);
    }
}

impl NameOutputInput<String> for String {}

impl<T> TempNameInput for &T
where
    T: TempNameInput + ?Sized,
{
    fn write_with_suffix<S>(&self, suffix: &S, output: &mut super::super::NameText)
    where
        S: Display + ?Sized,
    {
        (*self).write_with_suffix(suffix, output);
    }
}

impl<T> NameOutputInput<String> for &T where T: NameOutputInput<String> + ?Sized {}

pub(in crate::owner_journal) fn intent_temp_name<R, S>(
    request_id: &R,
    suffix: &S,
) -> super::super::NameText
where
    R: TempNameInput + ?Sized,
    S: Display + ?Sized,
{
    let mut name = super::super::NameText::new();
    request_id.write_with_suffix(suffix, &mut name);
    name
}

pub(in crate::owner_journal) fn receipt_path<R>(root: &Path, request_id: &R) -> JournalPath
where
    R: Display + ?Sized,
{
    JournalPath(
        root.join(BRIDGE_DIRECTORY)
            .join(MUTATION_OWNER_DIRECTORY)
            .join(RECEIPTS_DIRECTORY)
            .join(format!("{request_id}{JSON_SUFFIX}")),
    )
}

pub(in crate::owner_journal) fn intent_path<R>(root: &Path, request_id: &R) -> JournalPath
where
    R: Display + ?Sized,
{
    JournalPath(
        root.join(BRIDGE_DIRECTORY)
            .join(MUTATION_OWNER_DIRECTORY)
            .join(INTENTS_DIRECTORY)
            .join(format!("{request_id}{JSON_SUFFIX}")),
    )
}
