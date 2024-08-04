pub mod player_game_info;

use player_game_info::PlayerGameInfo;
use serde::Deserialize;

use serde_json::{Map, Value};

use crate::client::Client;
use crate::error::{Error, Result};

use super::player::Player;

pub enum Queue {
    Assault,
    Arena,
    Joust,
    Conquest,
    Motd,
    Other(u32),
}

#[derive(Debug)]
pub struct MatchId {
    pub id: usize,
    pub ret_msg: Option<String>,
    pub active_flag: bool,
}

#[derive(Debug, Deserialize)]
pub struct MatchInfo {
    #[serde(flatten)]
    pub player_info: Player,
    #[serde(flatten)]
    pub additional_fields: Map<String, Value>,
}

impl Client {
    /// Retrieves IDs of matches by queue and date.
    /// The date must be in the format `MM-DD-YYYY`.
    ///
    /// Set `hour` to `None` to get all matches for the given date.
    ///
    /// # Errors
    ///
    /// - If the `hour` is not between -1 and 23.
    /// - If the API request fails.
    pub async fn get_match_ids_by_queue(
        &self,
        queue: Queue,
        date: &str,
        hour: Option<i8>,
    ) -> Result<Vec<MatchId>> {
        let hour = hour.unwrap_or(-1);

        if !(-1..=23).contains(&hour) {
            return Err(Error::InvalidArgument {
                given: hour.to_string(),
                expected: "hour between -1 and 23".to_string(),
            });
        }

        self.make_request(
            "getmatchidsbyqueue",
            true,
            &[&queue.to_id().to_string(), date, &hour.to_string()],
        )
        .await
    }

    /// Retrieves the details of a match by its ID.
    /// Details are returned as a list of `PlayerGameInfo`.
    /// Each `PlayerGameInfo` contains information about a player in the match.
    ///
    /// # Errors
    /// - If the API request fails.
    pub async fn get_match_details(
        &self,
        match_id: impl Into<usize>,
    ) -> Result<Vec<PlayerGameInfo>> {
        let match_id: usize = match_id.into();

        self.make_request("getmatchdetails", true, &[&match_id.to_string()])
            .await
    }

    /// This is a batch version of `get_match_details`.
    /// It retrieves the details of multiple matches by their IDs.
    ///
    /// # Errors
    /// - If the API request fails.
    pub async fn get_match_details_batch(
        &self,
        match_ids: Vec<usize>,
    ) -> Result<Vec<PlayerGameInfo>> {
        let match_ids: Vec<String> = match_ids.iter().map(ToString::to_string).collect();

        let f: Value = self.make_request("getmatchdetailsbatch", true, &[&match_ids.join(",")])
            .await?;
        dbg!(&f);

        self.make_request("getmatchdetailsbatch", true, &[&match_ids.join(",")])
            .await
    }
}

impl Queue {
    fn to_id(&self) -> u32 {
        match self {
            Queue::Assault => 445,
            Queue::Motd => 434,
            Queue::Arena => 435,
            Queue::Joust => 448,
            Queue::Conquest => 426,
            Queue::Other(id) => *id,
        }
    }
}

impl<'de> Deserialize<'de> for MatchId {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let val: Value = Deserialize::deserialize(deserializer)?;

        let Some(id) = val.get("Match") else {
            return Err(serde::de::Error::missing_field("Match"));
        };

        let Some(ret_msg) = val.get("ret_msg").map(|ret_msg| {
            if ret_msg.is_string() {
                Some(ret_msg.as_str().unwrap().to_string())
            } else {
                None
            }
        }) else {
            return Err(serde::de::Error::missing_field("ret_msg"));
        };

        let Some(active_flag) = val.get("Active_Flag").and_then(|val| val.as_str()) else {
            return Err(serde::de::Error::missing_field("Active_Flag"));
        };

        let Some(id) = id
            .as_str()
            .map(str::parse::<usize>)
            .and_then(std::result::Result::ok)
        else {
            return Err(serde::de::Error::custom(
                "Match ID is not a String parseable to number",
            ));
        };

        Ok(MatchId {
            id,
            ret_msg,
            active_flag: active_flag == "y",
        })
    }
}

impl From<MatchId> for usize {
    fn from(value: MatchId) -> Self {
        value.id
    }
}
