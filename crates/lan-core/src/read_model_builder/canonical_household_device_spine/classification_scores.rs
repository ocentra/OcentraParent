use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceClassification;

use super::WeightedHintText;

#[derive(Clone)]
pub(super) struct ClassificationScore {
    pub(super) classification: LanCanonicalHouseholdDeviceClassification,
    pub(super) total: u16,
    pub(super) strongest_signal: u16,
}

pub(super) fn classification_scores(texts: &[WeightedHintText]) -> [ClassificationScore; 10] {
    let mut scores = initial_classification_scores();
    for text in texts {
        accumulate_all_classification_scores(&mut scores, text);
    }
    scores
}

fn initial_classification_scores() -> [ClassificationScore; 10] {
    [
        score_for(LanCanonicalHouseholdDeviceClassification::Printer),
        score_for(LanCanonicalHouseholdDeviceClassification::Television),
        score_for(LanCanonicalHouseholdDeviceClassification::GameConsole),
        score_for(LanCanonicalHouseholdDeviceClassification::NetworkAttachedStorage),
        score_for(LanCanonicalHouseholdDeviceClassification::Camera),
        score_for(LanCanonicalHouseholdDeviceClassification::Phone),
        score_for(LanCanonicalHouseholdDeviceClassification::Tablet),
        score_for(LanCanonicalHouseholdDeviceClassification::Laptop),
        score_for(LanCanonicalHouseholdDeviceClassification::Desktop),
        score_for(LanCanonicalHouseholdDeviceClassification::InternetOfThings),
    ]
}

fn score_for(classification: LanCanonicalHouseholdDeviceClassification) -> ClassificationScore {
    ClassificationScore {
        classification,
        total: 0,
        strongest_signal: 0,
    }
}

fn accumulate_all_classification_scores(
    scores: &mut [ClassificationScore],
    text: &WeightedHintText,
) {
    for profile in CLASSIFICATION_HINT_PROFILES {
        accumulate_classification_score(scores, &profile.classification, profile.hints, text);
    }
}

fn accumulate_classification_score(
    scores: &mut [ClassificationScore],
    classification: &LanCanonicalHouseholdDeviceClassification,
    hints: &[&str],
    text: &WeightedHintText,
) {
    if !hints.iter().any(|hint| text.text.contains(hint)) {
        return;
    }
    let Some(score) = scores
        .iter_mut()
        .find(|score| &score.classification == classification)
    else {
        return;
    };
    score.total += text.weight;
    score.strongest_signal = score.strongest_signal.max(text.weight);
}

struct ClassificationHintProfile {
    classification: LanCanonicalHouseholdDeviceClassification,
    hints: &'static [&'static str],
}

const CLASSIFICATION_HINT_PROFILES: &[ClassificationHintProfile] = &[
    ClassificationHintProfile {
        classification: LanCanonicalHouseholdDeviceClassification::Printer,
        hints: PRINTER_HINTS,
    },
    ClassificationHintProfile {
        classification: LanCanonicalHouseholdDeviceClassification::Television,
        hints: TELEVISION_HINTS,
    },
    ClassificationHintProfile {
        classification: LanCanonicalHouseholdDeviceClassification::GameConsole,
        hints: GAME_CONSOLE_HINTS,
    },
    ClassificationHintProfile {
        classification: LanCanonicalHouseholdDeviceClassification::NetworkAttachedStorage,
        hints: NAS_HINTS,
    },
    ClassificationHintProfile {
        classification: LanCanonicalHouseholdDeviceClassification::Camera,
        hints: CAMERA_HINTS,
    },
    ClassificationHintProfile {
        classification: LanCanonicalHouseholdDeviceClassification::Phone,
        hints: PHONE_HINTS,
    },
    ClassificationHintProfile {
        classification: LanCanonicalHouseholdDeviceClassification::Tablet,
        hints: TABLET_HINTS,
    },
    ClassificationHintProfile {
        classification: LanCanonicalHouseholdDeviceClassification::Laptop,
        hints: LAPTOP_HINTS,
    },
    ClassificationHintProfile {
        classification: LanCanonicalHouseholdDeviceClassification::Desktop,
        hints: DESKTOP_HINTS,
    },
    ClassificationHintProfile {
        classification: LanCanonicalHouseholdDeviceClassification::InternetOfThings,
        hints: IOT_HINTS,
    },
];

const PHONE_HINTS: &[&str] = &["iphone", "android", "pixel", "galaxy", "phone"];
const TABLET_HINTS: &[&str] = &["ipad", "tablet", "kindle"];
const LAPTOP_HINTS: &[&str] = &["laptop", "macbook", "thinkpad", "notebook"];
const DESKTOP_HINTS: &[&str] = &["desktop", "workstation", "imac", "mac mini", " pc", "tower"];
const PRINTER_HINTS: &[&str] = &[
    "_ipp._tcp.local",
    "printer",
    "jetdirect",
    "laserjet",
    "officejet",
    "epson",
    "brother",
    "canon",
    "cups",
];
const TELEVISION_HINTS: &[&str] = &[
    "_googlecast._tcp.local",
    "_airplay._tcp.local",
    "mediarenderer",
    "media-renderer",
    "chromecast",
    "bravia",
    "roku",
    "appletv",
    "fire tv",
    " tv",
];
const GAME_CONSOLE_HINTS: &[&str] = &["xbox", "playstation", "ps4", "ps5", "nintendo", "switch"];
const CAMERA_HINTS: &[&str] = &["camera", " cam", "arlo", "wyze", "ring", "nest cam"];
const NAS_HINTS: &[&str] = &["nas", "synology", "qnap", "diskstation", "my cloud"];
const IOT_HINTS: &[&str] = &[
    "thermostat",
    "bulb",
    "plug",
    "sensor",
    "homepod",
    "echo",
    "speaker",
    "outlet",
];
