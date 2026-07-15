use std::collections::BTreeSet;

#[path = "support/support_assertions.rs"]
mod support_assertions;

#[macro_export]
macro_rules! assert_context {
    ($value:expr $(,)?) => {
        $crate::support::AssertionContext($value)
    };
}

#[macro_export]
macro_rules! contract_text {
    ($value:expr $(,)?) => {
        $crate::support::ContractText($value)
    };
}

#[macro_export]
macro_rules! text_boundary {
    ($prefix:expr, $suffix:expr $(,)?) => {
        $crate::support::TextBoundary {
            prefix: $prefix,
            suffix: $suffix,
        }
    };
}

#[macro_export]
macro_rules! ts_block {
    ($value:expr $(,)?) => {
        $crate::support::TypeScriptBlock(($value).to_string())
    };
}

#[macro_export]
macro_rules! module_specifiers {
    ($($value:expr),* $(,)?) => {
        $crate::support::ModuleSpecifiers(vec![$($value),*])
    };
}

#[macro_export]
macro_rules! contract_texts {
    ($($value:expr),* $(,)?) => {
        $crate::support::ContractTexts(vec![$($value.to_string()),*])
    };
}

#[derive(Clone, Copy)]
pub struct AssertionContext<'a>(pub &'a str);

#[derive(Clone, Copy)]
pub struct ContractText<'a>(pub &'a str);

#[derive(Clone, Copy)]
pub struct TextBoundary<'a> {
    pub prefix: &'a str,
    pub suffix: &'a str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ContractLine<'a>(pub &'a str);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContractString(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContractNames(pub BTreeSet<String>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeScriptBlock(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContractTexts(pub Vec<String>);

pub trait ValueOrUnreachable<T> {
    fn value_or_unreachable(self, context: AssertionContext<'_>) -> T;
}

impl<T> ValueOrUnreachable<T> for Option<T> {
    fn value_or_unreachable(self, context: AssertionContext<'_>) -> T {
        option_or_unreachable(self, context)
    }
}

impl<T, E: std::fmt::Debug> ValueOrUnreachable<T> for Result<T, E> {
    fn value_or_unreachable(self, context: AssertionContext<'_>) -> T {
        result_or_unreachable(self, context)
    }
}

pub trait ErrorOrUnreachable<E> {
    fn error_or_unreachable(self, context: AssertionContext<'_>) -> E;
}

impl<T: std::fmt::Debug, E: std::fmt::Debug> ErrorOrUnreachable<E> for Result<T, E> {
    fn error_or_unreachable(self, context: AssertionContext<'_>) -> E {
        error_or_unreachable(self, context)
    }
}

pub fn option_or_unreachable<T>(value: Option<T>, context: AssertionContext<'_>) -> T {
    support_assertions::option_or_unreachable(value, context)
}

pub fn result_or_unreachable<T, E: std::fmt::Debug>(
    value: Result<T, E>,
    context: AssertionContext<'_>,
) -> T {
    support_assertions::result_or_unreachable(value, context)
}

pub fn error_or_unreachable<T: std::fmt::Debug, E: std::fmt::Debug>(
    value: Result<T, E>,
    context: AssertionContext<'_>,
) -> E {
    support_assertions::error_or_unreachable(value, context)
}

fn exported_name_from_line(line: ContractLine<'_>) -> Option<ContractString> {
    let trimmed = line.0.trim_start();
    [
        "export const ",
        "export function ",
        "export interface ",
        "export type ",
    ]
    .iter()
    .find_map(|prefix| {
        let rest = trimmed.strip_prefix(prefix)?;
        let name = rest
            .split(|ch: char| ch.is_whitespace() || ch == '(' || ch == '=' || ch == '{')
            .next()
            .unwrap_or_default();
        (!name.is_empty()).then(|| ContractString(name.to_owned()))
    })
}

pub fn exported_names(source: ContractText<'_>) -> ContractNames {
    ContractNames(
        source
            .0
            .lines()
            .map(ContractLine)
            .filter_map(exported_name_from_line)
            .map(|name| name.0)
            .collect(),
    )
}

pub fn assert_exports_include(source: ContractText<'_>, expected: ContractNames) {
    let actual = exported_names(source);
    let expected = expected.0;
    assert!(expected.is_subset(&actual.0));
}

pub fn string_const_value(
    source: ContractText<'_>,
    name: ContractText<'_>,
) -> Option<ContractString> {
    let prefix = format!("export const {} =", name.0);
    let rest = source.0.split_once(&prefix)?.1;
    let quoted = rest.split_once('\'')?.1;
    Some(ContractString(quoted.split_once('\'')?.0.to_owned()))
}

pub fn request_policy_ids(source: ContractText<'_>) -> ContractNames {
    ContractNames(
        source
            .0
            .split("policyRequestId: '")
            .skip(1)
            .filter_map(|rest| rest.split_once('\'').map(|(value, _)| value.to_owned()))
            .collect(),
    )
}

pub fn generated_line<'a>(
    source: ContractText<'a>,
    line_start: ContractText<'_>,
) -> ContractLine<'a> {
    ContractLine(option_or_unreachable(
        source
            .0
            .lines()
            .find(|line| line.trim_start().starts_with(line_start.0)),
        crate::assert_context!("expected generated line to exist"),
    ))
}

pub fn assert_generated_line_eq(
    source: ContractText<'_>,
    line_start: ContractText<'_>,
    expected: ContractLine<'_>,
) {
    assert_eq!(generated_line(source, line_start), expected);
}

pub fn assert_contract_contains(source: ContractText<'_>, expected: ContractText<'_>) {
    assert!(
        source.0.contains(expected.0),
        "missing generated TypeScript fragment: {}",
        expected.0
    );
}

pub fn assert_contract_contains_all(source: ContractText<'_>, expected: ContractTexts) {
    expected.0.into_iter().for_each(|fragment| {
        assert_contract_contains(source, crate::contract_text!(fragment.as_str()))
    });
}

pub fn assert_contract_has_lines(source: ContractText<'_>, expected: ContractTexts) {
    expected
        .0
        .into_iter()
        .for_each(|line| assert_contract_contains(source, crate::contract_text!(line.as_str())));
}

pub fn extract_typescript_block(
    source: ContractText<'_>,
    boundary: TextBoundary<'_>,
) -> TypeScriptBlock {
    let remainder = option_or_unreachable(
        source.0.split_once(boundary.prefix),
        crate::assert_context!("typescript block prefix to exist"),
    )
    .1;
    let block = option_or_unreachable(
        remainder.split_once(boundary.suffix),
        crate::assert_context!("typescript block suffix to exist"),
    )
    .0;
    TypeScriptBlock(block.trim().replace("\r\n", "\n"))
}
