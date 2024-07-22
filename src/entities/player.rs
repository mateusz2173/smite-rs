use serde::Deserialize;

use crate::client::Client;
use crate::error::Result;

#[derive(Debug, Deserialize)]
pub struct Player {
    #[serde(rename = "ActivePlayerId")]
    pub active_player_id: u32,
    #[serde(rename = "Avatar_URL")]
    pub avatar_url: String,
    #[serde(rename = "Created_Datetime")]
    pub created_datetime: String,
    #[serde(rename = "HoursPlayed")]
    pub hours_played: f64, // Numbers might be represented as f64
    #[serde(rename = "Id")]
    pub id: u32,
    #[serde(rename = "Last_Login_Datetime")]
    pub last_login_datetime: String,
    #[serde(rename = "Leaves")]
    pub leaves: u32,
    #[serde(rename = "Level")]
    pub level: u32,
    #[serde(rename = "Losses")]
    pub losses: u32,
    #[serde(rename = "MasteryLevel")]
    pub mastery_level: u32,
    #[serde(rename = "MergedPlayers", default)]
    pub merged_players: Option<Vec<Merged>>,
    #[serde(rename = "MinutesPlayed")]
    pub minutes_played: u32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Personal_Status_Message")]
    pub personal_status_message: String,
    #[serde(rename = "Platform")]
    pub platform: String,
    #[serde(rename = "Rank_Stat_Conquest")]
    pub rank_stat_conquest: Option<u32>, // Might be null
    #[serde(rename = "Rank_Stat_Conquest_Controller")]
    pub rank_stat_conquest_controller: Option<u32>,
    #[serde(rename = "Rank_Stat_Duel")]
    pub rank_stat_duel: Option<u32>,
    #[serde(rename = "Rank_Stat_Duel_Controller")]
    pub rank_stat_duel_controller: Option<u32>,
    #[serde(rename = "Rank_Stat_Joust")]
    pub rank_stat_joust: Option<u32>,
    #[serde(rename = "Rank_Stat_Joust_Controller")]
    pub rank_stat_joust_controller: Option<u32>,
    #[serde(rename = "RankedConquest")]
    pub ranked_conquest: RankedStats,
    #[serde(rename = "RankedConquestController")]
    pub ranked_conquest_controller: RankedStats,
    #[serde(rename = "RankedDuel")]
    pub ranked_duel: RankedStats,
    #[serde(rename = "RankedDuelController")]
    pub ranked_duel_controller: RankedStats,
    #[serde(rename = "RankedJoust")]
    pub ranked_joust: RankedStats,
    #[serde(rename = "RankedJoustController")]
    pub ranked_joust_controller: RankedStats,
    #[serde(rename = "Region")]
    pub region: String,
    #[serde(rename = "TeamId")]
    pub team_id: u32,
    #[serde(rename = "Team_Name")]
    pub team_name: String,
    #[serde(rename = "Tier_Conquest")]
    pub tier_conquest: u32,
    #[serde(rename = "Tier_Duel")]
    pub tier_duel: u32,
    #[serde(rename = "Tier_Joust")]
    pub tier_joust: u32,
    #[serde(rename = "Total_Achievements")]
    pub total_achievements: u32,
    #[serde(rename = "Total_Worshippers")]
    pub total_worshippers: u32,
    #[serde(rename = "Wins")]
    pub wins: u32,
    #[serde(rename = "hz_gamer_tag")]
    pub hz_gamer_tag: Option<String>,
    #[serde(rename = "hz_player_name")]
    pub hz_player_name: String,
    #[serde(rename = "ret_msg")]
    pub ret_msg: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RankedStats {
    #[serde(rename = "Leaves")]
    pub leaves: u32,
    #[serde(rename = "Losses")]
    pub losses: u32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Points")]
    pub points: u32,
    #[serde(rename = "PrevRank")]
    pub prev_rank: u32,
    #[serde(rename = "Rank")]
    pub rank: u32,
    #[serde(rename = "Rank_Stat")]
    pub rank_stat: u32,
    #[serde(rename = "Rank_Stat_Conquest")]
    pub rank_stat_conquest: Option<u32>, // Might be null
    #[serde(rename = "Rank_Stat_Joust")]
    pub rank_stat_joust: Option<u32>,
    #[serde(rename = "Rank_Variance")]
    pub rank_variance: u32,
    #[serde(rename = "Round")]
    pub round: u32,
    #[serde(rename = "Season")]
    pub season: u32,
    #[serde(rename = "Tier")]
    pub tier: u32,
    #[serde(rename = "Trend")]
    pub trend: u32,
    #[serde(rename = "Wins")]
    pub wins: u32,
    #[serde(rename = "player_id")]
    pub player_id: Option<String>,
    #[serde(rename = "ret_msg")]
    pub ret_msg: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Merged {
    pub player_id: String,
    pub portal_id: String,
    #[serde(rename = "merge_datetime")]
    pub merge_datetime: String,
}

impl Client {
    /// Returns information about a player.
    /// This method requires a valid session.
    ///
    /// # Errors
    /// Returns an error if the request fails or if the response cannot be parsed.
    /// This function will return an error if .
    ///
    /// # Examples
    /// ```
    /// use hirez::Client;
    /// use hirez::error::Result;
    ///
    /// fn example() -> Result<()> {
    ///    let mut client = Client::new("dev_id".to_string(), "auth_key".to_string());
    ///    let player = client.get_player("my_player")?[0]; // API may return multiple players.
    ///    Ok(())
    /// }
    pub fn get_player(&mut self, player_name: &str) -> Result<Vec<Player>> {
        self.make_request("getplayer", true, &[player_name.to_string()])
    }
}
