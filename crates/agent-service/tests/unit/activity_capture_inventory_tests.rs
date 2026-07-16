use std::{
    error::Error,
    fs::{create_dir_all, read, remove_dir_all, remove_file, write},
    io::Error as IoError,
    path::Path,
};

use crate::test_text::TestText;
use ocentra_parent_agent_core::{
    activity_store::ActivityStore,
    journal::ActivityJournal,
    journal_crypto::{JournalKey, JOURNAL_KEY_BYTES},
};
use ocentra_parent_agent_protocol::activity::{ActivityEvent, ActivityEventKind};
use ocentra_parent_agent_protocol::app_game::{
    AppGameServiceReadModel, APP_GAME_FOREGROUND_NOT_CLAIMED,
    APP_GAME_INVENTORY_SOURCE_OS_INSTALLED_RECORD, APP_GAME_INVENTORY_SOURCE_SHORTCUT,
    APP_GAME_INVENTORY_SOURCE_STORE_PACKAGE, APP_GAME_INVENTORY_STATE_INSTALLED,
    APP_GAME_JOURNAL_INVENTORY_SUBJECT_ID, APP_GAME_RUNTIME_NOT_CLAIMED,
    APP_GAME_TEST_DISPLAY_LABEL, APP_GAME_TEST_LIVE_INVENTORY_SUFFIX,
    APP_GAME_TEST_SHORTCUT_FILE_NAME, APP_GAME_TEST_STORE_APP_DISPLAY_LABEL,
    APP_GAME_TEST_STORE_APP_PACKAGE_ID, APP_GAME_TEST_STORE_PACKAGE_MANIFEST_USER_MODEL_ID,
    APP_GAME_TEST_STORE_PACKAGE_MANIFEST_XML, APP_GAME_WINDOWS_APPX_MANIFEST_FILE_NAME,
    APP_GAME_WINDOWS_REGISTRY_DISPLAY_NAME_VALUE, APP_GAME_WINDOWS_REGISTRY_EXPORT_HEADER,
    APP_GAME_WINDOWS_REGISTRY_FILE_EXTENSION, APP_GAME_WINDOWS_REGISTRY_INSTALL_LOCATION_VALUE,
    APP_GAME_WINDOWS_REGISTRY_LOCAL_MACHINE_HIVE, APP_GAME_WINDOWS_REGISTRY_UNINSTALL_PATH,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_service::test_support::{
    record_activity_capture_to_paths_at_with_inventory_roots_for_test,
    record_activity_capture_to_paths_at_with_registry_inventory_roots_for_test,
    record_activity_capture_to_paths_at_with_store_package_roots_for_test,
};

type TestResult = Result<(), Box<dyn Error>>;

#[test]
fn record_capture_with_inventory_root_writes_inventory_journal_and_sqlite_rows() -> TestResult {
    let build_path = |suffix: &str, extension: &str| {
        let mut name = String::from(constants::activity_store::TEST_FILE_PREFIX);
        name.push_str(&std::process::id().to_string());
        name.push(constants::delimiter::HYPHEN);
        name.push_str(suffix);

        let mut path = std::env::temp_dir();
        path.push(name);
        path.set_extension(extension);
        path
    };
    let journal_path = build_path(
        constants::activity_store::TEST_CAPTURE_APP_GAME_JOURNAL_SUFFIX,
        constants::journal::FILE_EXTENSION,
    );
    let key_path = build_path(
        constants::activity_store::TEST_CAPTURE_APP_GAME_KEY_SUFFIX,
        constants::activity_store::FILE_EXTENSION,
    );
    let store_path = build_path(
        constants::activity_store::TEST_CAPTURE_APP_GAME_STORE_SUFFIX,
        constants::activity_store::FILE_EXTENSION,
    );
    let inventory_root = {
        let mut name = String::from(constants::activity_store::TEST_FILE_PREFIX);
        name.push_str(&std::process::id().to_string());
        name.push(constants::delimiter::HYPHEN);
        name.push_str(APP_GAME_TEST_LIVE_INVENTORY_SUFFIX);

        let mut path = std::env::temp_dir();
        path.push(name);
        path
    };
    cleanup_paths(&journal_path, &key_path, &store_path);
    cleanup_inventory_root(&inventory_root);
    let result = (|| -> TestResult {
        create_dir_all(&inventory_root)?;
        let mut shortcut_path = inventory_root.clone();
        shortcut_path.push(APP_GAME_TEST_SHORTCUT_FILE_NAME);
        write(&shortcut_path, [])?;

        let status = record_activity_capture_to_paths_at_with_inventory_roots_for_test(
            &journal_path,
            &key_path,
            &store_path,
            1,
            1,
            TestText::from_display(constants::activity_store::TEST_FIRST_OBSERVED_AT),
            std::slice::from_ref(&inventory_root),
        )
        .map_err(|error| {
            IoError::other(format!(
                "{}: {error:?}",
                constants::error::ACTIVITY_CAPTURE_RECORDS
            ))
        })?;
        let events = decrypted_events(&journal_path, &key_path)?;
        let app_game = app_game_read_model(&store_path)?;

        assert_eq!(status.events_ingested, status.events_stored);
        assert!(events.iter().any(|event| event.kind
            == ActivityEventKind::DeviceIdleStateObserved
            && event.subject.subject_id == APP_GAME_JOURNAL_INVENTORY_SUBJECT_ID));
        assert_eq!(app_game.inventory_returned, 1);
        assert_eq!(
            app_game.inventory_rows[0].source_kind,
            APP_GAME_INVENTORY_SOURCE_SHORTCUT
        );
        assert_eq!(
            app_game.inventory_rows[0].inventory_state,
            APP_GAME_INVENTORY_STATE_INSTALLED
        );
        assert_eq!(
            app_game.inventory_rows[0].runtime_state,
            APP_GAME_RUNTIME_NOT_CLAIMED
        );
        assert_eq!(
            app_game.inventory_rows[0].foreground_state,
            APP_GAME_FOREGROUND_NOT_CLAIMED
        );

        Ok(())
    })();

    cleanup_paths(&journal_path, &key_path, &store_path);
    cleanup_inventory_root(&inventory_root);

    result
}

#[test]
fn record_capture_with_store_package_root_writes_inventory_journal_and_sqlite_rows() -> TestResult {
    let build_path = |suffix: &str, extension: &str| {
        let mut name = String::from(constants::activity_store::TEST_FILE_PREFIX);
        name.push_str(&std::process::id().to_string());
        name.push(constants::delimiter::HYPHEN);
        name.push_str(suffix);

        let mut path = std::env::temp_dir();
        path.push(name);
        path.set_extension(extension);
        path
    };
    let journal_path = build_path(
        constants::activity_store::TEST_CAPTURE_STORE_PACKAGE_JOURNAL_SUFFIX,
        constants::journal::FILE_EXTENSION,
    );
    let key_path = build_path(
        constants::activity_store::TEST_CAPTURE_STORE_PACKAGE_KEY_SUFFIX,
        constants::activity_store::FILE_EXTENSION,
    );
    let store_path = build_path(
        constants::activity_store::TEST_CAPTURE_STORE_PACKAGE_STORE_SUFFIX,
        constants::activity_store::FILE_EXTENSION,
    );
    let store_package_root = {
        let mut name = String::from(constants::activity_store::TEST_FILE_PREFIX);
        name.push_str(&std::process::id().to_string());
        name.push(constants::delimiter::HYPHEN);
        name.push_str(constants::activity_store::TEST_APP_GAME_STORE_PACKAGE_MANIFEST_SUFFIX);

        let mut path = std::env::temp_dir();
        path.push(name);
        path
    };
    cleanup_paths(&journal_path, &key_path, &store_path);
    cleanup_inventory_root(&store_package_root);
    let result = (|| -> TestResult {
        write_store_package_manifest(&store_package_root)?;

        let status = record_activity_capture_to_paths_at_with_store_package_roots_for_test(
            &journal_path,
            &key_path,
            &store_path,
            1,
            1,
            TestText::from_display(constants::activity_store::TEST_FIRST_OBSERVED_AT),
            std::slice::from_ref(&store_package_root),
        )
        .map_err(|error| {
            IoError::other(format!(
                "{}: {error:?}",
                constants::error::ACTIVITY_CAPTURE_RECORDS
            ))
        })?;
        let events = decrypted_events(&journal_path, &key_path)?;
        let app_game = app_game_read_model(&store_path)?;

        assert_eq!(status.events_ingested, status.events_stored);
        assert!(events.iter().any(|event| event.kind
            == ActivityEventKind::DeviceIdleStateObserved
            && event.subject.subject_id == APP_GAME_JOURNAL_INVENTORY_SUBJECT_ID));
        assert_eq!(app_game.inventory_returned, 1);
        assert_eq!(
            app_game.inventory_rows[0].display_label,
            APP_GAME_TEST_STORE_APP_DISPLAY_LABEL
        );
        assert_eq!(
            app_game.inventory_rows[0].source_kind,
            APP_GAME_INVENTORY_SOURCE_STORE_PACKAGE
        );
        assert_eq!(
            app_game.inventory_rows[0].package_id.as_deref(),
            Some(APP_GAME_TEST_STORE_APP_PACKAGE_ID)
        );
        assert_eq!(
            app_game.inventory_rows[0].app_user_model_id.as_deref(),
            Some(APP_GAME_TEST_STORE_PACKAGE_MANIFEST_USER_MODEL_ID)
        );
        assert_eq!(
            app_game.inventory_rows[0].runtime_state,
            APP_GAME_RUNTIME_NOT_CLAIMED
        );
        assert_eq!(
            app_game.inventory_rows[0].foreground_state,
            APP_GAME_FOREGROUND_NOT_CLAIMED
        );

        Ok(())
    })();

    cleanup_paths(&journal_path, &key_path, &store_path);
    cleanup_inventory_root(&store_package_root);

    result
}

#[test]
fn record_capture_with_registry_root_writes_inventory_journal_and_sqlite_rows() -> TestResult {
    let build_path = |suffix: &str, extension: &str| {
        let mut name = String::from(constants::activity_store::TEST_FILE_PREFIX);
        name.push_str(&std::process::id().to_string());
        name.push(constants::delimiter::HYPHEN);
        name.push_str(suffix);

        let mut path = std::env::temp_dir();
        path.push(name);
        path.set_extension(extension);
        path
    };
    let journal_path = build_path(
        constants::activity_store::TEST_CAPTURE_REGISTRY_INVENTORY_JOURNAL_SUFFIX,
        constants::journal::FILE_EXTENSION,
    );
    let key_path = build_path(
        constants::activity_store::TEST_CAPTURE_REGISTRY_INVENTORY_KEY_SUFFIX,
        constants::activity_store::FILE_EXTENSION,
    );
    let store_path = build_path(
        constants::activity_store::TEST_CAPTURE_REGISTRY_INVENTORY_STORE_SUFFIX,
        constants::activity_store::FILE_EXTENSION,
    );
    let registry_root = {
        let mut name = String::from(constants::activity_store::TEST_FILE_PREFIX);
        name.push_str(&std::process::id().to_string());
        name.push(constants::delimiter::HYPHEN);
        name.push_str(constants::activity_store::TEST_CAPTURE_REGISTRY_INVENTORY_STORE_SUFFIX);

        let mut path = std::env::temp_dir();
        path.push(name);
        path
    };
    cleanup_paths(&journal_path, &key_path, &store_path);
    cleanup_inventory_root(&registry_root);
    let result = (|| -> TestResult {
        write_registry_inventory_export(&registry_root)?;

        let status = record_activity_capture_to_paths_at_with_registry_inventory_roots_for_test(
            &journal_path,
            &key_path,
            &store_path,
            1,
            1,
            TestText::from_display(constants::activity_store::TEST_FIRST_OBSERVED_AT),
            std::slice::from_ref(&registry_root),
        )
        .map_err(|error| {
            IoError::other(format!(
                "{}: {error:?}",
                constants::error::ACTIVITY_CAPTURE_RECORDS
            ))
        })?;
        let events = decrypted_events(&journal_path, &key_path)?;
        let app_game = app_game_read_model(&store_path)?;

        assert_eq!(status.events_ingested, status.events_stored);
        assert!(events.iter().any(|event| event.kind
            == ActivityEventKind::DeviceIdleStateObserved
            && event.subject.subject_id == APP_GAME_JOURNAL_INVENTORY_SUBJECT_ID));
        assert_eq!(app_game.inventory_returned, 1);
        assert_eq!(
            app_game.inventory_rows[0].display_label,
            APP_GAME_TEST_DISPLAY_LABEL
        );
        assert_eq!(
            app_game.inventory_rows[0].source_kind,
            APP_GAME_INVENTORY_SOURCE_OS_INSTALLED_RECORD
        );
        assert_eq!(
            app_game.inventory_rows[0].inventory_state,
            APP_GAME_INVENTORY_STATE_INSTALLED
        );
        assert_eq!(
            app_game.inventory_rows[0].runtime_state,
            APP_GAME_RUNTIME_NOT_CLAIMED
        );
        assert_eq!(
            app_game.inventory_rows[0].foreground_state,
            APP_GAME_FOREGROUND_NOT_CLAIMED
        );

        Ok(())
    })();

    cleanup_paths(&journal_path, &key_path, &store_path);
    cleanup_inventory_root(&registry_root);

    result
}

fn decrypted_events(journal_path: &Path, key_path: &Path) -> Result<Vec<ActivityEvent>, IoError> {
    let key_bytes = read(key_path)?;
    let mut key = [0; JOURNAL_KEY_BYTES];
    key.copy_from_slice(&key_bytes);
    let journal = ActivityJournal::open(journal_path.to_path_buf(), JournalKey::from_bytes(key))
        .map_err(|error| {
            IoError::other(format!("{}: {error:?}", constants::error::JOURNAL_OPENS))
        })?;
    journal
        .lines()
        .map_err(|error| IoError::other(format!("{}: {error:?}", constants::error::JOURNAL_READS)))?
        .iter()
        .map(|line| journal.decrypt_line(line))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| {
            IoError::other(format!("{}: {error:?}", constants::error::JOURNAL_DECRYPTS))
        })
}

fn app_game_read_model(store_path: &Path) -> Result<AppGameServiceReadModel, IoError> {
    let store = ActivityStore::open(store_path).map_err(|error| {
        IoError::other(format!(
            "{}: {error:?}",
            constants::error::ACTIVITY_STORE_OPENS
        ))
    })?;
    store
        .app_game_service_read_model(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        )
        .map_err(|error| {
            IoError::other(format!(
                "{}: {error:?}",
                constants::error::ACTIVITY_STORE_QUERIES
            ))
        })
}

fn write_store_package_manifest(root: &Path) -> Result<(), IoError> {
    create_dir_all(root)?;
    let mut path = root.to_path_buf();
    path.push(APP_GAME_WINDOWS_APPX_MANIFEST_FILE_NAME);
    write(path, APP_GAME_TEST_STORE_PACKAGE_MANIFEST_XML)?;
    Ok(())
}

fn write_registry_inventory_export(root: &Path) -> Result<(), IoError> {
    create_dir_all(root)?;
    let mut path = root.to_path_buf();
    path.push(constants::activity_store::TEST_CAPTURE_REGISTRY_INVENTORY_STORE_SUFFIX);
    path.set_extension(APP_GAME_WINDOWS_REGISTRY_FILE_EXTENSION);

    let mut export = String::from(APP_GAME_WINDOWS_REGISTRY_EXPORT_HEADER);
    export.push(constants::delimiter::NEWLINE);
    export.push(constants::delimiter::OPEN_BRACKET);
    export.push_str(APP_GAME_WINDOWS_REGISTRY_LOCAL_MACHINE_HIVE);
    export.push(constants::delimiter::BACKSLASH);
    export.push_str(APP_GAME_WINDOWS_REGISTRY_UNINSTALL_PATH);
    export.push(constants::delimiter::BACKSLASH);
    export.push_str(constants::activity_store::TEST_APP_GAME_SESSION_ID);
    export.push(constants::delimiter::CLOSE_BRACKET);
    export.push(constants::delimiter::NEWLINE);

    let mut push_registry_value = |registry_value_name: &str, value: &str| {
        export.push(constants::delimiter::QUOTE);
        export.push_str(registry_value_name);
        export.push(constants::delimiter::QUOTE);
        export.push(constants::delimiter::EQUALS);
        export.push(constants::delimiter::QUOTE);
        export.push_str(value);
        export.push(constants::delimiter::QUOTE);
        export.push(constants::delimiter::NEWLINE);
    };

    push_registry_value(
        APP_GAME_WINDOWS_REGISTRY_DISPLAY_NAME_VALUE,
        APP_GAME_TEST_DISPLAY_LABEL,
    );
    push_registry_value(
        APP_GAME_WINDOWS_REGISTRY_INSTALL_LOCATION_VALUE,
        constants::activity_store::TEST_APP_GAME_PROCESS_PATH,
    );
    write(path, export)?;
    Ok(())
}

fn cleanup_paths(
    journal_path: impl AsRef<Path>,
    key_path: impl AsRef<Path>,
    store_path: impl AsRef<Path>,
) {
    let journal_path = journal_path.as_ref();
    let key_path = key_path.as_ref();
    let store_path = store_path.as_ref();
    let _ = remove_file(journal_path);
    let _ = remove_file(key_path);
    let _ = remove_file(store_path);
    let mut store_wal_path = store_path.to_path_buf();
    store_wal_path.set_extension(constants::activity_store::WAL_FILE_EXTENSION);
    let _ = remove_file(store_wal_path);
    let mut store_shm_path = store_path.to_path_buf();
    store_shm_path.set_extension(constants::activity_store::SHM_FILE_EXTENSION);
    let _ = remove_file(store_shm_path);
}

fn cleanup_inventory_root(path: &Path) {
    let _ = remove_dir_all(path);
}
