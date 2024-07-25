use crate::client::Client;
use crate::error::Result;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SessionInfo {
    #[serde(rename = "Active_Sessions")]
    pub active_sessions: i64,
    #[serde(rename = "Concurrent_Sessions")]
    pub concurrent_sessions: i64,
    #[serde(rename = "Request_Limit_Daily")]
    pub request_limit_daily: i64,
    #[serde(rename = "Session_Cap")]
    pub session_cap: i64,
    #[serde(rename = "Session_Time_Limit")]
    pub session_time_limit: i64,
    #[serde(rename = "Total_Requests_Today")]
    pub total_requests_today: i64,
    #[serde(rename = "Total_Sessions_Today")]
    pub total_sessions_today: i64,
    pub ret_msg: Option<String>,
}

impl SessionInfo {
    #[must_use]
    pub fn reached_session_limit(&self) -> bool {
        self.active_sessions >= self.session_cap
    }

    #[must_use]
    pub fn reached_request_limit(&self) -> bool {
        self.total_requests_today >= self.request_limit_daily
    }
}

impl Client {
    /// Retrieves information about the current session.
    /// This method requires a valid session.
    ///
    /// # Errors
    /// Returns an error if the request fails or if the response cannot be parsed.
    pub async fn get_data_used(&self) -> Result<Vec<SessionInfo>> {
        self.make_request("getdataused", true, &[]).await
    }
}
