use async_mutex::Mutex;

use crate::entities::session::Session;
use crate::error::Error;
use crate::error::Result;
use serde::Deserialize;
use serde_json::Value;

const BASE_URL: &str = "https://api.smitegame.com/smiteapi.svc";

pub struct Client {
    dev_id: String,
    auth_key: String,
    session: Mutex<Option<Session>>,
}

impl Client {
    #[must_use]
    pub fn new(dev_id: String, auth_key: String) -> Client {
        Client {
            dev_id,
            auth_key,
            session: Mutex::new(None),
        }
    }

    /// Makes a request to the Hi-Rez API.
    /// The `method` parameter is the name of the API method to call.
    /// The `requires_session` parameter indicates whether the method requires a valid session.
    /// The `T` parameter is the type of the response.
    ///
    /// # Errors
    /// Returns an error if the request fails or if the response cannot be parsed.
    #[allow(clippy::missing_panics_doc)]
    pub async fn make_request<T>(
        &self,
        method: &str,
        requires_session: bool,
        additional_args: &[&str],
    ) -> Result<T>
    where
        for<'a> T: Deserialize<'a>,
    {
        let dev_id = self.dev_id.clone();
        let signature = self.signature(method);
        let timestamp = current_timestamp();

        let mut endpoint = if requires_session {
            let session_id = self.ensure_session().await?;
            format!("{BASE_URL}/{method}Json/{dev_id}/{signature}/{session_id}/{timestamp}")
        } else {
            format!("{BASE_URL}/{method}Json/{dev_id}/{signature}/{timestamp}")
        };

        for arg in additional_args {
            endpoint.push_str(&format!("/{arg}"));
        }

        self._make_request(&endpoint).await
    }

    async fn _make_request<T>(&self, endpoint: &str) -> Result<T>
    where
        for<'a> T: Deserialize<'a>,
    {
        let response = reqwest::get(endpoint).await?.text().await?;

        if response.starts_with('<') {
            let re = regex::Regex::new(r"<p>(.*)</p>").unwrap();
            let msg = re
                .captures(&response)
                .map_or("", |cap| cap.get(1).unwrap().as_str());

            return Err(Error::SmiteApi(msg.to_string()));
        }

        serde_json::from_str::<T>(&response).map_err(Error::Parsing)
    }

    pub(crate) async fn request_session(&self) -> Result<Session> {
        let endpoint = format!(
            "{BASE_URL}/createsessionJson/{dev_id}/{signature}/{timestamp}",
            BASE_URL = BASE_URL,
            dev_id = self.dev_id,
            signature = self.signature("createsession"),
            timestamp = current_timestamp()
        );
        let val: Value = self._make_request(&endpoint).await?;

        let ret_msg = val
            .get("ret_msg")
            .and_then(Value::as_str)
            .unwrap_or_default();

        if ret_msg == "Approved" {
            serde_json::from_value(val).map_err(Error::Parsing)
        } else {
            Err(Error::SmiteApi(ret_msg.to_string()))
        }
    }

    fn signature(&self, method: &str) -> String {
        let input = format!(
            "{}{method}{}{}",
            self.dev_id,
            self.auth_key,
            current_timestamp()
        );
        let hash = md5::compute(input);

        format!("{hash:?}")
    }

    async fn ensure_session(&self) -> Result<String> {
        let mut session = self.session.lock().await;
        if session.is_none() || session.as_ref().is_some_and(|session| !session.is_alive()) {
            *session = Some(self.request_session().await?);
        }

        let id = session.as_ref().map(|session| session.id.clone());

        id.ok_or(Error::Session)
    }
}

fn current_timestamp() -> String {
    chrono::Utc::now().format("%Y%m%d%H%M%S").to_string()
}
