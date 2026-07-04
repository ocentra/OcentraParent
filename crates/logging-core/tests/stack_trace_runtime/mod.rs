use ocentra_parent_logging_core::stack_trace_runtime::{
    decode_file_path, file_name_from_path, module_name_from_path, normalize_path,
    parse_stack_trace, resolve_logger_context, resolve_logger_source,
    stack_trace_runtime_typescript, StackFrame,
};

#[test]
fn stack_trace_runtime_parses_normal_minimal_and_malformed_lines() {
    let parsed = parse_stack_trace(
        "Error\n    at LoggerTestFixture.emitHelloWorldLogs (file:///C:/repo/packages/logging-domain/tests/unit/logger.test.ts:21:9)\n    at file:///C:/repo/packages/logging-domain/src/core/logger.ts:88:17\nnot a frame",
    );

    assert_eq!(parsed.len(), 2);
    assert_eq!(
        parsed[0],
        StackFrame {
            function_name: Some("LoggerTestFixture.emitHelloWorldLogs".to_string()),
            file: Some("logger.test.ts".to_string()),
            file_path: Some(
                "C:/repo/packages/logging-domain/tests/unit/logger.test.ts".to_string()
            ),
            line: Some(21),
            column: Some(9),
        }
    );
    assert_eq!(
        parsed[1],
        StackFrame {
            function_name: None,
            file: Some("logger.ts".to_string()),
            file_path: Some("C:/repo/packages/logging-domain/src/core/logger.ts".to_string()),
            line: Some(88),
            column: Some(17),
        }
    );
}

#[test]
fn stack_trace_runtime_handles_path_and_location_rules() {
    assert_eq!(
        normalize_path(r"apps\portal\src\dev-logger.ts"),
        "apps/portal/src/dev-logger.ts".to_string()
    );
    assert_eq!(
        decode_file_path("file:///C:/repo/packages/logging-domain/src/core/logger.ts"),
        "C:/repo/packages/logging-domain/src/core/logger.ts".to_string()
    );
    assert_eq!(
        file_name_from_path(Some("packages/logging-domain/tests/unit/logger.test.ts")),
        Some("logger.test.ts".to_string())
    );
    assert_eq!(
        module_name_from_path("packages/logging-domain/tests/unit/logger.test.ts"),
        "LoggerTest".to_string()
    );
}

#[test]
fn stack_trace_runtime_derives_logger_context_and_source() {
    let frame = StackFrame {
        function_name: Some("LoggerTestFixture.emitHelloWorldLogs".to_string()),
        file: Some("logger.test.ts".to_string()),
        file_path: Some("packages/logging-domain/tests/unit/logger.test.ts".to_string()),
        line: Some(21),
        column: Some(9),
    };

    assert_eq!(
        resolve_logger_context("LoggerTest", Some(&frame), "module"),
        "LoggerTestFixture.emitHelloWorldLogs".to_string()
    );
    assert_eq!(
        resolve_logger_source("LoggerTest", Some(&frame)),
        "LoggerTestFixture".to_string()
    );

    let no_function = StackFrame {
        function_name: None,
        file: None,
        file_path: None,
        line: None,
        column: None,
    };
    assert_eq!(
        resolve_logger_context("LoggerTest", Some(&no_function), "module"),
        "LoggerTest.module".to_string()
    );
    assert_eq!(
        resolve_logger_source("LoggerTest", Some(&no_function)),
        "LoggerTest".to_string()
    );
}

#[test]
fn generated_stack_trace_runtime_helper_stays_checked_in() {
    let checked_in = include_str!("../../../../packages/logging-domain/src/stack-trace-runtime.ts");

    assert_eq!(checked_in, stack_trace_runtime_typescript());
    assert_eq!(
        checked_in.lines().next(),
        Some("/* generated from crates/logging-core/src/stack_trace_runtime.rs */")
    );
}
