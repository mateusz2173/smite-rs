use crate::client::Client;
use crate::error::Result;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AbilityDescription {
    pub cooldown: Option<String>,
    pub cost: Option<String>,
    pub description: Option<String>,
    #[serde(default = "Vec::new")]
    pub menuitems: Vec<Menuitem>,
    #[serde(default = "Vec::new")]
    pub rankitems: Vec<Rankitem>,
}

#[derive(Deserialize, Debug)]
pub struct Menuitem {
    pub description: String,
    pub value: String,
}

#[derive(Deserialize, Debug)]
pub struct Rankitem {
    pub description: String,
    pub value: String,
}

#[derive(Deserialize, Debug)]
pub struct Ability {
    #[serde(rename = "Description")]
    pub description: BasicAttack,
    #[serde(rename = "Id")]
    pub id: u32,
    #[serde(rename = "Summary")]
    pub summary: String,
    #[serde(rename = "URL")]
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct BasicAttack {
    #[serde(rename = "itemDescription")]
    pub item_description: ItemDescription,
}

#[derive(Deserialize, Debug)]
pub struct ItemDescription {
    pub cooldown: Option<String>,
    pub cost: Option<String>,
    pub description: Option<String>,
    #[serde(default = "Vec::new")]
    pub menuitems: Vec<Menuitem>,
    #[serde(default = "Vec::new")]
    pub rankitems: Vec<Rankitem>,
}

#[derive(Deserialize, Debug)]
pub struct God {
    #[serde(rename = "Ability1")]
    pub ability1: String,
    #[serde(rename = "Ability2")]
    pub ability2: String,
    #[serde(rename = "Ability3")]
    pub ability3: String,
    #[serde(rename = "Ability4")]
    pub ability4: String,
    #[serde(rename = "Ability5")]
    pub ability5: String,
    #[serde(rename = "AbilityId1")]
    pub ability_id1: u32,
    #[serde(rename = "AbilityId2")]
    pub ability_id2: u32,
    #[serde(rename = "AbilityId3")]
    pub ability_id3: u32,
    #[serde(rename = "AbilityId4")]
    pub ability_id4: u32,
    #[serde(rename = "AbilityId5")]
    pub ability_id5: u32,
    #[serde(rename = "Ability_1")]
    pub ability_1: Ability,
    #[serde(rename = "Ability_2")]
    pub ability_2: Ability,
    #[serde(rename = "Ability_3")]
    pub ability_3: Ability,
    #[serde(rename = "Ability_4")]
    pub ability_4: Ability,
    #[serde(rename = "Ability_5")]
    pub ability_5: Ability,
    #[serde(rename = "AttackSpeed")]
    pub attack_speed: f32,
    #[serde(rename = "AttackSpeedPerLevel")]
    pub attack_speed_per_level: f32,
    #[serde(rename = "AutoBanned")]
    pub auto_banned: String,
    #[serde(rename = "Cons")]
    pub cons: String,
    #[serde(rename = "HP5PerLevel")]
    pub hp5_per_level: f32,
    #[serde(rename = "Health")]
    pub health: f32,
    #[serde(rename = "HealthPerFive")]
    pub health_per_five: f32,
    #[serde(rename = "HealthPerLevel")]
    pub health_per_level: f32,
    #[serde(rename = "Lore")]
    pub lore: String,
    #[serde(rename = "MP5PerLevel")]
    pub mp5_per_level: f32,
    #[serde(rename = "MagicProtection")]
    pub magic_protection: f32,
    #[serde(rename = "MagicProtectionPerLevel")]
    pub magic_protection_per_level: f32,
    #[serde(rename = "MagicalPower")]
    pub magical_power: f32,
    #[serde(rename = "MagicalPowerPerLevel")]
    pub magical_power_per_level: f32,
    #[serde(rename = "Mana")]
    pub mana: f32,
    #[serde(rename = "ManaPerFive")]
    pub mana_per_five: f32,
    #[serde(rename = "ManaPerLevel")]
    pub mana_per_level: f32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "OnFreeRotation")]
    pub on_free_rotation: String,
    #[serde(rename = "Pantheon")]
    pub pantheon: String,
    #[serde(rename = "PhysicalPower")]
    pub physical_power: f32,
    #[serde(rename = "PhysicalPowerPerLevel")]
    pub physical_power_per_level: f32,
    #[serde(rename = "PhysicalProtection")]
    pub physical_protection: f32,
    #[serde(rename = "PhysicalProtectionPerLevel")]
    pub physical_protection_per_level: f32,
    #[serde(rename = "Pros")]
    pub pros: String,
    #[serde(rename = "Roles")]
    pub roles: String,
    #[serde(rename = "Speed")]
    pub speed: f32,
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "Type")]
    pub r#type: String,
    #[serde(rename = "abilityDescription1")]
    pub ability_description1: AbilityDescription,
    #[serde(rename = "abilityDescription2")]
    pub ability_description2: AbilityDescription,
    #[serde(rename = "abilityDescription3")]
    pub ability_description3: AbilityDescription,
    #[serde(rename = "abilityDescription4")]
    pub ability_description4: AbilityDescription,
    #[serde(rename = "abilityDescription5")]
    pub ability_description5: AbilityDescription,
    #[serde(rename = "basicAttack")]
    pub basic_attack: BasicAttack,
    #[serde(rename = "godAbility1_URL")]
    pub god_ability1_url: String,
    #[serde(rename = "godAbility2_URL")]
    pub god_ability2_url: String,
    #[serde(rename = "godAbility3_URL")]
    pub god_ability3_url: String,
    #[serde(rename = "godAbility4_URL")]
    pub god_ability4_url: String,
    #[serde(rename = "godAbility5_URL")]
    pub god_ability5_url: String,
    #[serde(rename = "godCard_URL")]
    pub god_card_url: String,
    #[serde(rename = "godIcon_URL")]
    pub god_icon_url: String,
    #[serde(rename = "id")]
    pub id: u32,
    #[serde(rename = "latestGod")]
    pub latest_god: String,
    #[serde(rename = "ret_msg")]
    pub ret_msg: Option<String>,
}

impl Client {
    /// Retrieves all Gods and their various attributes.
    ///
    /// - `language_code` - The language code for the language to retrieve god names in.
    ///   Possible codes:
    ///   1 - English
    ///   2 - German
    ///   3 - French
    ///   5 - Chinese
    ///   7 - Spanish
    ///   9 - Spanish (Latin America)
    ///   10 - Portuguese
    ///   11 - Russian
    ///   12 - Polish
    ///   13 - Turkish
    ///   By default, the English names will be returned.
    ///
    /// # Errors
    ///
    /// - If the API request fails.
    /// - If the `language_code` is not listed above.
    pub async fn get_gods(&self, language_code: Option<u32>) -> Result<Vec<God>> {
        let language_code = language_code.unwrap_or(1).to_string();
        self.make_request("getgods", true, &[&language_code]).await
    }
}
