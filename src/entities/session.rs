use chrono::{DateTime, Utc};
use serde::Deserialize;

const SESSION_DURATION: i64 = 15 * 60;

#[derive(Deserialize, Debug)]
pub(crate) struct Session {
    #[serde(rename = "ret_msg")]
    pub(crate) _ret_msg: String,
    #[serde(rename = "session_id")]
    pub(crate) id: String,
    #[serde(deserialize_with = "crate::utils::timestamp_from_string")]
    pub(crate) timestamp: DateTime<Utc>,
}

impl Session {
    pub fn is_alive(&self) -> bool {
        let now = Utc::now();
        let duration = now.signed_duration_since(self.timestamp);

        duration.num_seconds() < SESSION_DURATION
    }
}
