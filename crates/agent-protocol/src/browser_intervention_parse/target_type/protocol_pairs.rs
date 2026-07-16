use super::BrowserInterventionTargetType;
use crate::constants;

pub(super) fn find(value: &str) -> Option<BrowserInterventionTargetType> {
    super::super::protocol_lookup(value, site_pairs())
        .or_else(|| super::super::protocol_lookup(value, social_pairs()))
        .or_else(|| super::super::protocol_lookup(value, game_pairs()))
        .or_else(|| super::super::protocol_lookup(value, browser_pairs()))
}

fn site_pairs() -> [(&'static str, BrowserInterventionTargetType); 4] {
    [
        (
            constants::browser::INTERVENTION_TARGET_TYPE_SITE,
            BrowserInterventionTargetType::Site,
        ),
        (
            constants::browser::INTERVENTION_TARGET_TYPE_DOMAIN,
            BrowserInterventionTargetType::Domain,
        ),
        (
            constants::browser::INTERVENTION_TARGET_TYPE_URL,
            BrowserInterventionTargetType::Url,
        ),
        (
            constants::browser::INTERVENTION_TARGET_TYPE_VIDEO,
            BrowserInterventionTargetType::Video,
        ),
    ]
}

fn social_pairs() -> [(&'static str, BrowserInterventionTargetType); 7] {
    [
        (
            constants::browser::INTERVENTION_TARGET_TYPE_SOCIAL_ACCOUNT_CREATION,
            BrowserInterventionTargetType::SocialAccountCreation,
        ),
        (
            constants::browser::INTERVENTION_TARGET_TYPE_SOCIAL_FEED,
            BrowserInterventionTargetType::SocialFeed,
        ),
        (
            constants::browser::INTERVENTION_TARGET_TYPE_SOCIAL_SHORT_VIDEO_FEED,
            BrowserInterventionTargetType::SocialShortVideoFeed,
        ),
        (
            constants::browser::INTERVENTION_TARGET_TYPE_SOCIAL_MESSAGING,
            BrowserInterventionTargetType::SocialMessaging,
        ),
        (
            constants::browser::INTERVENTION_TARGET_TYPE_SOCIAL_UPLOAD_POST,
            BrowserInterventionTargetType::SocialUploadPost,
        ),
        (
            constants::browser::INTERVENTION_TARGET_TYPE_SOCIAL_LIVESTREAM,
            BrowserInterventionTargetType::SocialLivestream,
        ),
        (
            constants::browser::INTERVENTION_TARGET_TYPE_UNKNOWN_SOCIAL_SITE,
            BrowserInterventionTargetType::UnknownSocialSite,
        ),
    ]
}

fn game_pairs() -> [(&'static str, BrowserInterventionTargetType); 6] {
    [
        (
            constants::browser::INTERVENTION_TARGET_TYPE_BROWSER_GAME,
            BrowserInterventionTargetType::BrowserGame,
        ),
        (
            constants::browser::INTERVENTION_TARGET_TYPE_GAME_ACCOUNT,
            BrowserInterventionTargetType::GameAccount,
        ),
        (
            constants::browser::INTERVENTION_TARGET_TYPE_GAME_PURCHASE,
            BrowserInterventionTargetType::GamePurchase,
        ),
        (
            constants::browser::INTERVENTION_TARGET_TYPE_CLOUD_GAMING,
            BrowserInterventionTargetType::CloudGaming,
        ),
        (
            constants::browser::INTERVENTION_TARGET_TYPE_UNKNOWN_GAME,
            BrowserInterventionTargetType::UnknownGame,
        ),
        (
            constants::browser::INTERVENTION_TARGET_TYPE_UNBLOCKED_GAME_SITE,
            BrowserInterventionTargetType::UnblockedGameSite,
        ),
    ]
}

fn browser_pairs() -> [(&'static str, BrowserInterventionTargetType); 3] {
    [
        (
            constants::browser::INTERVENTION_TARGET_TYPE_BROWSER_PROCESS,
            BrowserInterventionTargetType::BrowserProcess,
        ),
        (
            constants::browser::INTERVENTION_TARGET_TYPE_BROWSER_SESSION,
            BrowserInterventionTargetType::BrowserSession,
        ),
        (
            constants::browser::INTERVENTION_TARGET_TYPE_UNKNOWN,
            BrowserInterventionTargetType::Unknown,
        ),
    ]
}
