#[tokio::test]
async fn setup_outbox_repeat_append_is_idempotent() {
    let path = temp_path(TestText::from_display("idempotent"));
    cleanup_path(&path);
    let result = accepted_result().await;
    let store_path = AppGameTimerSetupStorePath(path.clone());
    require_ok(
        append_setup_outbox_record(&result, &store_path),
        "first append",
    );
    require_ok(
        append_setup_outbox_record(&result, &store_path),
        "repeat append",
    );
    let lines = require_ok(
        read_to_string(path.with_extension(
            constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_DURABLE_OUTBOX_FILE_EXTENSION,
        )),
        "read idempotent outbox",
    );
    assert_eq!(lines.lines().count(), 1);
    cleanup_path(&path);
}

#[tokio::test]
async fn setup_outbox_rejects_conflicting_same_record_id() {
    let path = temp_path(TestText::from_display("conflict"));
    cleanup_path(&path);
    let result = accepted_result().await;
    let mut conflict = result.clone();
    conflict.request_id.push_str("-conflict");
    let store_path = AppGameTimerSetupStorePath(path.clone());
    require_ok(
        append_setup_outbox_record(&result, &store_path),
        "first append",
    );
    assert_eq!(append_setup_outbox_record(&conflict, &store_path), Err(()));
    cleanup_path(&path);
}

#[tokio::test]
async fn setup_outbox_rejects_malformed_existing_line() {
    let path = temp_path(TestText::from_display("malformed"));
    cleanup_path(&path);
    let outbox = path.with_extension(
        constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_DURABLE_OUTBOX_FILE_EXTENSION,
    );
    require_ok(
        std::fs::write(&outbox, "not-json\n"),
        "write malformed outbox",
    );
    let result = accepted_result().await;
    assert_eq!(
        append_setup_outbox_record(&result, &AppGameTimerSetupStorePath(path.clone())),
        Err(())
    );
    cleanup_path(&path);
}

#[tokio::test]
async fn setup_outbox_rejects_valid_json_with_wrong_shape() {
    let path = temp_path(TestText::from_display("wrong-shape"));
    cleanup_path(&path);
    let outbox = path.with_extension(
        constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_DURABLE_OUTBOX_FILE_EXTENSION,
    );
    require_ok(std::fs::write(&outbox, "[]\n"), "write wrong-shape outbox");
    let result = accepted_result().await;
    assert_eq!(
        append_setup_outbox_record(&result, &AppGameTimerSetupStorePath(path.clone())),
        Err(())
    );
    cleanup_path(&path);
}

#[tokio::test]
async fn setup_outbox_validates_later_lines_after_matching_record() {
    let path = temp_path(TestText::from_display("later-corruption"));
    cleanup_path(&path);
    let result = accepted_result().await;
    let outbox = path.with_extension(
        constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_DURABLE_OUTBOX_FILE_EXTENSION,
    );
    require_ok(
        append_setup_outbox_record(&result, &AppGameTimerSetupStorePath(path.clone())),
        "write matching outbox record",
    );
    let mut file = require_ok(
        std::fs::OpenOptions::new().append(true).open(&outbox),
        "open later corruption outbox",
    );
    require_ok(
        std::io::Write::write_all(&mut file, b"[]\n"),
        "write later corruption line",
    );
    assert_eq!(
        append_setup_outbox_record(&result, &AppGameTimerSetupStorePath(path.clone())),
        Err(())
    );
    cleanup_path(&path);
}

#[tokio::test]
async fn setup_outbox_rejects_missing_terminal_newline() {
    let path = temp_path(TestText::from_display("partial-line"));
    cleanup_path(&path);
    let outbox = path.with_extension(
        constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_DURABLE_OUTBOX_FILE_EXTENSION,
    );
    require_ok(
        std::fs::write(&outbox, b"{\"recordId\":\"partial"),
        "write partial line",
    );
    let result = accepted_result().await;
    assert_eq!(
        append_setup_outbox_record(&result, &AppGameTimerSetupStorePath(path.clone())),
        Err(())
    );
    cleanup_path(&path);
}

#[tokio::test]
async fn setup_outbox_rejects_empty_new_record_id() {
    let path = temp_path(TestText::from_display("empty-id"));
    cleanup_path(&path);
    let mut result = accepted_result().await;
    result.durable_outbox_record_id = TestString::new();
    assert_eq!(
        append_setup_outbox_record(&result, &AppGameTimerSetupStorePath(path.clone())),
        Err(())
    );
    cleanup_path(&path);
}

#[tokio::test]
async fn setup_outbox_rejects_whitespace_new_record_id() {
    let path = temp_path(TestText::from_display("whitespace-id"));
    cleanup_path(&path);
    let mut result = accepted_result().await;
    result.durable_outbox_record_id = format!(" {} ", result.durable_outbox_record_id);
    assert_eq!(
        append_setup_outbox_record(&result, &AppGameTimerSetupStorePath(path.clone())),
        Err(())
    );
    cleanup_path(&path);
}

#[tokio::test]
async fn setup_outbox_concurrent_distinct_records_are_complete() {
    let path = temp_path(TestText::from_display("concurrent"));
    cleanup_path(&path);
    let result = Arc::new(accepted_result().await);
    let path = Arc::new(path);
    thread::scope(|scope| {
        for index in 0..8 {
            let result = Arc::clone(&result);
            let path = Arc::clone(&path);
            scope.spawn(move || {
                let mut record = (*result).clone();
                record
                    .durable_outbox_record_id
                    .push_str(&format!("-{index}"));
                assert_eq!(
                    append_setup_outbox_record(
                        &record,
                        &AppGameTimerSetupStorePath((*path).clone())
                    ),
                    Ok(())
                );
            });
        }
    });
    let lines = require_ok(
        read_to_string(path.with_extension(
            constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_DURABLE_OUTBOX_FILE_EXTENSION,
        )),
        "read concurrent outbox",
    );
    assert_eq!(lines.lines().count(), 8);
    let ids: BTreeSet<String> = lines
        .lines()
        .map(|line| {
            let value: serde_json::Value =
                require_json_decode(line, "decode concurrent outbox line");
            require_some(
                value
                    .get(constants::field::APP_GAME_PARENT_PREFERENCE_SETUP_OUTBOX_RECORD_ID)
                    .and_then(serde_json::Value::as_str)
                    .map(TestString::from),
                "concurrent outbox record id",
            )
        })
        .collect();
    let expected: BTreeSet<String> = (0..8)
        .map(|index| format!("{}-{index}", result.durable_outbox_record_id))
        .collect();
    assert_eq!(ids, expected);
    cleanup_path(&path);
}

async fn accepted_result() -> AppGameTimerParentPreferenceSetupRequestResult {
    let event = build_timer_preference_report(command_envelope()).await;
    request_payload(&crate::test_invariants::log_field(
        &event.payload,
        constants::field::APP_GAME_TIMER_PARENT_PREFERENCE_SETUP_REQUEST,
        constants::error::AGENT_EVENT_SERIALIZES,
    ))
}

use std::collections::BTreeSet;
use std::fs::read_to_string;
use std::string::String as TestString;
use std::sync::Arc;
use std::thread;

use ocentra_parent_agent_protocol::app_game_timer_parent_preference_setup_request::AppGameTimerParentPreferenceSetupRequestResult;
use ocentra_parent_agent_protocol::constants;

use crate::test_invariants::{require_json_decode, require_ok, require_some};
use crate::test_text::TestText;

use super::super::app_game_timer_parent_preference_setup_request::AppGameTimerSetupStorePath;
use super::super::app_game_timer_parent_preference_setup_request_outbox::append_setup_outbox_record;
use super::{
    build_timer_preference_report, cleanup_path, command_envelope, request_payload, temp_path,
};
