pub(crate) struct TestContext(pub(crate) Box<str>);

use ocentra_eventing::expect_value::{ExpectErrValue, ExpectValue};

pub(crate) fn test_ok<T, E>(result: Result<T, E>, context: TestContext) -> T {
    let TestContext(context) = context;
    result.expect_value(&context)
}

pub(crate) fn test_some<T>(value: Option<T>, context: TestContext) -> T {
    let TestContext(context) = context;
    value.expect_value(&context)
}

pub(crate) fn test_err<T, E>(result: Result<T, E>, context: TestContext) -> E {
    let TestContext(context) = context;
    result.expect_err_value(&context)
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
