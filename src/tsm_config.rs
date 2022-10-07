use plotters::prelude::RGBColor;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;

#[derive(Deserialize, Debug)]
pub struct TSMConfig {
    pub key: String,
    pub kingdoms: KingdomConfig,
}

#[derive(Deserialize, Debug)]
pub struct KingdomConfig {
    pub us: String,
    pub allies: Vec<String>,
    pub friends: Vec<String>,
    pub hostiles: Vec<String>,
    pub ennemies: Vec<String>,
    pub neutrals: Vec<String>,
}

const OUR_COLOR: RGBColor = RGBColor(40, 46, 8);
const ALLY_COLOR: RGBColor = RGBColor(16, 82, 106);
const FRIEND_COLOR: RGBColor = RGBColor(28, 153, 197);
const NEUTRAL_COLOR: RGBColor = RGBColor(125, 127, 116);
const DARK_GREY: RGBColor = RGBColor(38, 32, 32);
const ENEMY_COLOR: RGBColor = RGBColor(160, 18, 16);
const HOSTILE_COLOR: RGBColor = RGBColor(242, 147, 21);

pub fn read_config(p: &str) -> Result<TSMConfig, Box<dyn Error>> {
    let f = File::open(p)?;
    let config: TSMConfig = serde_yaml::from_reader(f)?;
    Ok(config)
}

pub fn kingdom_color_vector(kingdoms: &KingdomConfig) -> Vec<(&str, RGBColor)> {
    let mut v = vec![(kingdoms.us.as_str(), OUR_COLOR)];
    for (kv, c) in [
        (&kingdoms.allies, ALLY_COLOR),
        (&kingdoms.friends, FRIEND_COLOR),
        (&kingdoms.hostiles, HOSTILE_COLOR),
        (&kingdoms.ennemies, ENEMY_COLOR),
        (&kingdoms.neutrals, NEUTRAL_COLOR),
    ] {
        kv.iter().for_each(|k| v.push((k.as_str(), c)));
    }
    v
}
