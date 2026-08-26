#![forbid(unsafe_code)]

pub(crate) const SESSION_SCHEMA_SQL: &str =
    "CREATE TABLE IF NOT EXISTS account_identity_session (
         credential_class TEXT NOT NULL CHECK (credential_class = 'browser-user-session'),
         digest_algorithm TEXT NOT NULL CHECK (digest_algorithm = 'sha256'),
         access_digest_domain TEXT NOT NULL CHECK (access_digest_domain = 'ocentra-account-session-access-v1'),
         refresh_digest_domain TEXT NOT NULL CHECK (refresh_digest_domain = 'ocentra-account-session-refresh-v1'),
         access_digest TEXT NOT NULL PRIMARY KEY
           CHECK (length(access_digest) = 64 AND access_digest NOT GLOB '*[^0-9a-f]*'),
         refresh_digest TEXT NOT NULL UNIQUE
           CHECK (length(refresh_digest) = 64 AND refresh_digest NOT GLOB '*[^0-9a-f]*'),
         session_id TEXT NOT NULL UNIQUE CHECK (length(trim(session_id)) > 0),
         account_id TEXT NOT NULL CHECK (length(trim(account_id)) > 0),
         provider TEXT NOT NULL CHECK (provider IN ('authjs','firebase')),
         provider_subject TEXT NOT NULL CHECK (length(trim(provider_subject)) > 0),
         household_id TEXT NOT NULL CHECK (length(trim(household_id)) > 0),
         member_id TEXT NOT NULL CHECK (length(trim(member_id)) > 0),
         device_id TEXT NOT NULL CHECK (length(trim(device_id)) > 0),
         authority_session_id TEXT NOT NULL CHECK (length(trim(authority_session_id)) > 0),
         authority_session_generation INTEGER NOT NULL CHECK (authority_session_generation > 0),
         authority_generation INTEGER NOT NULL CHECK (authority_generation > 0),
         authority_expires_at_epoch_millis INTEGER NOT NULL CHECK (authority_expires_at_epoch_millis > 0),
         refresh_family_id TEXT NOT NULL
           CHECK (length(refresh_family_id) = 79
             AND refresh_family_id GLOB 'session-family-*'
             AND substr(refresh_family_id, 16) NOT GLOB '*[^0-9a-f]*'),
         refresh_generation INTEGER NOT NULL CHECK (refresh_generation > 0),
         issued_at_epoch_millis INTEGER NOT NULL CHECK (issued_at_epoch_millis > 0),
         access_expires_at_epoch_millis INTEGER NOT NULL,
         refresh_expires_at_epoch_millis INTEGER NOT NULL,
         fresh_until_epoch_millis INTEGER NOT NULL,
         activity_state TEXT NOT NULL CHECK (activity_state IN ('active','logged-out','revoked','globally-revoked')),
         global_revoke_epoch INTEGER NOT NULL CHECK (global_revoke_epoch > 0),
         last_transition_at_epoch_millis INTEGER NOT NULL,
         CHECK (issued_at_epoch_millis < access_expires_at_epoch_millis),
         CHECK (access_expires_at_epoch_millis <= refresh_expires_at_epoch_millis),
         CHECK (refresh_expires_at_epoch_millis <= authority_expires_at_epoch_millis),
          CHECK (fresh_until_epoch_millis > 0),
         CHECK (fresh_until_epoch_millis <= access_expires_at_epoch_millis),
          CHECK (last_transition_at_epoch_millis >= issued_at_epoch_millis),
          CHECK (access_digest != refresh_digest),
          CHECK (session_id = authority_session_id),
          CHECK ((activity_state = 'active' AND last_transition_at_epoch_millis = issued_at_epoch_millis)
             OR (activity_state != 'active' AND last_transition_at_epoch_millis > issued_at_epoch_millis))
     ) STRICT;
     CREATE INDEX IF NOT EXISTS account_identity_session_account
       ON account_identity_session(account_id);
     CREATE TABLE IF NOT EXISTS account_identity_session_revoke_epoch (
         account_id TEXT NOT NULL PRIMARY KEY CHECK (length(trim(account_id)) > 0),
         epoch INTEGER NOT NULL CHECK (epoch > 0)
     ) STRICT;
     CREATE TABLE IF NOT EXISTS account_identity_session_refresh_replay (
         digest_algorithm TEXT NOT NULL CHECK (digest_algorithm = 'sha256'),
         refresh_digest_domain TEXT NOT NULL CHECK (refresh_digest_domain = 'ocentra-account-session-refresh-v1'),
         consumed_refresh_digest TEXT NOT NULL PRIMARY KEY
           CHECK (length(consumed_refresh_digest) = 64
             AND consumed_refresh_digest NOT GLOB '*[^0-9a-f]*'),
         session_id TEXT NOT NULL CHECK (length(trim(session_id)) > 0),
         refresh_family_id TEXT NOT NULL
           CHECK (length(refresh_family_id) = 79
             AND refresh_family_id GLOB 'session-family-*'
             AND substr(refresh_family_id, 16) NOT GLOB '*[^0-9a-f]*'),
         consumed_generation INTEGER NOT NULL CHECK (consumed_generation > 0),
         consumed_at_epoch_millis INTEGER NOT NULL CHECK (consumed_at_epoch_millis > 0)
     ) STRICT;
     CREATE TABLE IF NOT EXISTS account_identity_session_audit_outbox (
         sequence INTEGER PRIMARY KEY AUTOINCREMENT,
         event_id TEXT NOT NULL UNIQUE
           CHECK (length(event_id) = 78
             AND event_id GLOB 'session-audit-*'
             AND substr(event_id, 15) NOT GLOB '*[^0-9a-f]*'),
         session_id TEXT NOT NULL CHECK (length(trim(session_id)) > 0),
         account_id TEXT NOT NULL CHECK (length(trim(account_id)) > 0),
         provider_subject TEXT NOT NULL CHECK (length(trim(provider_subject)) > 0),
         member_id TEXT NOT NULL CHECK (length(trim(member_id)) > 0),
         device_id TEXT NOT NULL CHECK (length(trim(device_id)) > 0),
         action TEXT NOT NULL CHECK (action IN ('created','rotated','logged-out','revoked','globally-revoked')),
         occurred_at_epoch_millis INTEGER NOT NULL CHECK (occurred_at_epoch_millis > 0),
          delivery_state TEXT NOT NULL CHECK (delivery_state IN ('pending','in-flight','delivered')),
          delivery_attempt_id TEXT,
          delivery_claimed_at_epoch_millis INTEGER,
          delivered_at_epoch_millis INTEGER,
          CHECK ((delivery_state = 'pending' AND delivery_attempt_id IS NULL
                  AND delivery_claimed_at_epoch_millis IS NULL
                  AND delivered_at_epoch_millis IS NULL)
              OR (delivery_state = 'in-flight'
                 AND length(delivery_attempt_id) = 81
                 AND delivery_attempt_id GLOB 'delivery-attempt-*'
                  AND substr(delivery_attempt_id, 18) NOT GLOB '*[^0-9a-f]*'
                  AND delivery_claimed_at_epoch_millis >= occurred_at_epoch_millis
                  AND delivered_at_epoch_millis IS NULL)
              OR (delivery_state = 'delivered' AND delivery_attempt_id IS NULL
                  AND delivery_claimed_at_epoch_millis IS NULL
                  AND delivered_at_epoch_millis >= occurred_at_epoch_millis))
     ) STRICT;
     CREATE TABLE IF NOT EXISTS account_identity_parent_local_bridge_session (
         capability_digest TEXT NOT NULL PRIMARY KEY
           CHECK (length(capability_digest) = 64
             AND capability_digest NOT GLOB '*[^0-9a-f]*'),
         digest_algorithm TEXT NOT NULL CHECK (digest_algorithm = 'sha256'),
         capability_digest_domain TEXT NOT NULL CHECK (
             capability_digest_domain = 'ocentra-account-parent-local-bridge-capability-v1'
         ),
         audience TEXT NOT NULL CHECK (audience = 'parent-desktop-agent-service'),
         connection_nonce_digest TEXT NOT NULL
           CHECK (length(connection_nonce_digest) = 64
             AND connection_nonce_digest NOT GLOB '*[^0-9a-f]*'),
         account_id TEXT NOT NULL CHECK (length(trim(account_id)) > 0),
         provider TEXT NOT NULL CHECK (provider IN ('authjs','firebase')),
         provider_subject TEXT NOT NULL CHECK (length(trim(provider_subject)) > 0),
         household_id TEXT NOT NULL CHECK (length(trim(household_id)) > 0),
         member_id TEXT NOT NULL CHECK (length(trim(member_id)) > 0),
         device_id TEXT NOT NULL CHECK (length(trim(device_id)) > 0),
         authority_session_id TEXT NOT NULL CHECK (length(trim(authority_session_id)) > 0),
         authority_session_generation INTEGER NOT NULL CHECK (authority_session_generation > 0),
         authority_generation INTEGER NOT NULL CHECK (authority_generation > 0),
         authority_expires_at_epoch_millis INTEGER NOT NULL
           CHECK (authority_expires_at_epoch_millis > 0),
         issued_at_epoch_millis INTEGER NOT NULL CHECK (issued_at_epoch_millis > 0),
         expires_at_epoch_millis INTEGER NOT NULL
           CHECK (expires_at_epoch_millis > issued_at_epoch_millis
             AND expires_at_epoch_millis <= authority_expires_at_epoch_millis),
         global_revoke_epoch INTEGER NOT NULL CHECK (global_revoke_epoch > 0),
         state TEXT NOT NULL CHECK (state IN ('active','consumed','revoked')),
         last_transition_at_epoch_millis INTEGER NOT NULL,
         CHECK (last_transition_at_epoch_millis >= issued_at_epoch_millis),
         CHECK ((state = 'active' AND last_transition_at_epoch_millis = issued_at_epoch_millis)
             OR (state != 'active' AND last_transition_at_epoch_millis > issued_at_epoch_millis))
     ) STRICT;
     CREATE INDEX IF NOT EXISTS account_identity_parent_local_bridge_account
       ON account_identity_parent_local_bridge_session(account_id);";
