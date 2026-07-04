pub(crate) struct TestContext(pub(crate) Box<str>);

pub(crate) fn test_ok<T, E>(result: Result<T, E>, context: TestContext) -> T
where
    E: std::fmt::Display + std::fmt::Debug,
{
    result.expect(&context.0)
}

pub(crate) fn test_some<T>(value: Option<T>, context: TestContext) -> T {
    value.expect(&context.0)
}

pub(crate) fn test_err<T, E>(result: Result<T, E>, context: TestContext) -> E
where
    E: std::fmt::Display + std::fmt::Debug,
{
    result.err().expect(&context.0)
}

macro_rules! test_ok {
    ($expr:expr, $context:expr) => {{
        $crate::test_support::test_ok(
            $expr,
            $crate::test_support::TestContext($context.to_string().into_boxed_str()),
        )
    }};
}

macro_rules! test_some {
    ($expr:expr, $context:expr) => {{
        $crate::test_support::test_some(
            $expr,
            $crate::test_support::TestContext($context.to_string().into_boxed_str()),
        )
    }};
}

macro_rules! test_err {
    ($expr:expr, $context:expr) => {{
        $crate::test_support::test_err(
            $expr,
            $crate::test_support::TestContext($context.to_string().into_boxed_str()),
        )
    }};
}
