use std::{fs, path::Path};

use base64::prelude::{Engine as _, BASE64_URL_SAFE_NO_PAD};
use ocentra_parent_agent_core::{
    journal_crypto::{JournalKey, JOURNAL_KEY_BYTES},
    screen_evidence_queue::ScreenEvidenceQueue,
};
use ocentra_parent_screen_capture_adapter::CapturedScreenImage;
use sha2::{Digest, Sha256};

use crate::{
    activity_capture::{record_activity_events_to_paths, ActivityCaptureError},
    screen_ai_service_capture_event_builder::{
        screen_analysis_event, screen_queue_job, ScreenAiServiceCaptureIds, ScreenIdPrefix,
        ScreenText,
    },
    time::{timestamp_after_epoch_seconds, timestamp_now},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ScreenAiServiceCaptureClock {
    pub(crate) epoch_seconds: u64,
    pub(crate) timestamp: String,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct ScreenAiServiceCapturePaths<'a> {
    pub(crate) queue_dir: &'a Path,
    pub(crate) journal_path: &'a Path,
    pub(crate) journal_key_path: &'a Path,
    pub(crate) store_path: &'a Path,
}

#[derive(Clone, Debug)]
pub(crate) struct ScreenAiServiceCaptureRecord<'a> {
    pub(crate) paths: ScreenAiServiceCapturePaths<'a>,
    pub(crate) image: &'a CapturedScreenImage,
    pub(crate) clock: ScreenAiServiceCaptureClock,
    pub(crate) sequence_index: u64,
    pub(crate) capture_reason: &'static str,
    pub(crate) source_id: &'static str,
    pub(crate) queue_job_id_prefix: &'static str,
    pub(crate) result_id_prefix: &'static str,
    pub(crate) event_id_prefix: &'static str,
    pub(crate) evidence_id_prefix: &'static str,
    pub(crate) summary: &'static str,
    pub(crate) model_id: &'static str,
    pub(crate) template_version: &'static str,
    pub(crate) temporary_image_ttl_seconds: u64,
}

pub(crate) fn record_captured_screen_image_to_paths(
    record: ScreenAiServiceCaptureRecord<'_>,
) -> Result<ScreenText, ActivityCaptureError> {
    let ScreenAiServiceCaptureRecord {
        paths,
        image,
        clock,
        sequence_index,
        capture_reason,
        source_id,
        queue_job_id_prefix,
        result_id_prefix,
        event_id_prefix,
        evidence_id_prefix,
        summary,
        model_id,
        template_version,
        temporary_image_ttl_seconds,
    } = record;
    let record = ScreenAiServiceCaptureRecord {
        paths,
        image,
        clock,
        sequence_index,
        capture_reason,
        source_id,
        queue_job_id_prefix,
        result_id_prefix,
        event_id_prefix,
        evidence_id_prefix,
        summary,
        model_id,
        template_version,
        temporary_image_ttl_seconds,
    };
    let key = load_or_create_screen_key(record.paths.journal_key_path)?;
    let image = record.image;
    let image_digest = digest_image(&image.png_bytes);
    let ids = ScreenAiServiceCaptureIds::new(
        ScreenIdPrefix(record.queue_job_id_prefix),
        ScreenIdPrefix(record.result_id_prefix),
        ScreenIdPrefix(record.event_id_prefix),
        ScreenIdPrefix(record.evidence_id_prefix),
        record.clock.epoch_seconds,
        record.sequence_index,
    );
    let job = screen_queue_job(&record, &ids, &image_digest);
    ScreenEvidenceQueue::open(record.paths.queue_dir, key)?
        .append_encrypted_image(&job, &image.png_bytes)?;
    let event = screen_analysis_event(&record, &ids, &job, &image_digest);
    record_activity_events_to_paths(
        record.paths.journal_path,
        record.paths.journal_key_path,
        record.paths.store_path,
        &[event],
    )?;
    Ok(ScreenText::from_display(ids.queue_job_id))
}

impl ScreenAiServiceCaptureClock {
    pub(crate) fn from_system_time() -> Self {
        Self {
            epoch_seconds: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|duration| duration.as_secs())
                .unwrap_or_default(),
            timestamp: timestamp_now(),
        }
    }

    pub(crate) fn expires_after_seconds(&self, seconds: u64) -> ScreenText {
        ScreenText::from_display(timestamp_after_epoch_seconds::<String>(
            self.epoch_seconds,
            seconds,
        ))
    }
}

fn load_or_create_screen_key(path: &Path) -> Result<JournalKey, ActivityCaptureError> {
    match fs::read(path) {
        Ok(bytes) => journal_key_from_bytes(&bytes),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            let key = JournalKey::generate();
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(path, key.as_bytes())?;
            Ok(key)
        }
        Err(_) => Err(ActivityCaptureError::Io),
    }
}

fn journal_key_from_bytes(bytes: &[u8]) -> Result<JournalKey, ActivityCaptureError> {
    if bytes.len() != JOURNAL_KEY_BYTES {
        return Err(ActivityCaptureError::InvalidKeyLength);
    }
    let mut key = [0; JOURNAL_KEY_BYTES];
    key.copy_from_slice(bytes);
    Ok(JournalKey::from_bytes(key))
}

fn digest_image(image_bytes: &[u8]) -> ScreenText {
    ScreenText::from_display(BASE64_URL_SAFE_NO_PAD.encode(Sha256::digest(image_bytes)))
}
