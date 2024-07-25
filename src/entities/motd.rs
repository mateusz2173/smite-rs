use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::client::Client;
use crate::error::Result;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchOfTheDay {
    pub description: String,
    #[serde(deserialize_with = "crate::utils::deserialize_non_empty_string")]
    pub max_players: Option<u32>,
    pub name: String,
    #[serde(rename = "ret_msg")]
    pub ret_msg: Option<String>,
    #[serde(deserialize_with = "crate::utils::timestamp_from_string")]
    pub start_date_time: DateTime<Utc>,
    #[serde(
        deserialize_with = "crate::utils::deserialize_non_empty_string",
        rename = "team1GodsCSV"
    )]
    pub team1_gods_csv: Option<String>,
    #[serde(
        deserialize_with = "crate::utils::deserialize_non_empty_string",
        rename = "team2GodsCSV"
    )]
    pub team2_gods_csv: Option<String>,
    pub title: String,
}

impl Client {
    /// Returns information about the 20 most recent Match-of-the-Days.
    /// This method requires a valid session.
    ///
    /// # Errors
    /// Returns an error if the request fails or if the response cannot be parsed.
    pub async fn get_motds(&self) -> Result<Vec<MatchOfTheDay>> {
        self.make_request("getmotd", true, &[]).await
    }
}

impl MatchOfTheDay {
    /// API returns the description as a single string with '<li>' tags.
    /// This method return a vector of strings by separating the description by '<li>' tags.
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn separate_description(&self) -> Vec<&str> {
        if !self.description.contains("<li>") {
            return vec![&self.description];
        }

        let traits = regex::Regex::new(r"(<li>)?(.*?)</?li>").expect("Couldn't parse regex");

        let props: Vec<&str> = traits
            .captures_iter(&self.description)
            .map(|m| m.get(2).unwrap().as_str())
            .filter(|s| s.len() > 3)
            .collect();

        props
    }
}
