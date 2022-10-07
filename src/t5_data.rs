use serde::{de, Deserialize, Deserializer};
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
struct RawGameWorld {
    name: String,
    start_time: i32,
    speed: u8,
    speed_troops: u8,
    last_update_time: i32,
    date: i32,
    version: String,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
struct RawVillage {
    #[serde(deserialize_with = "de_i32")]
    village_id: i32,
    #[serde(deserialize_with = "de_i32")]
    x: i32,
    #[serde(deserialize_with = "de_i32")]
    y: i32,
    #[serde(deserialize_with = "de_i32")]
    population: i32,
    name: String,
    is_main_village: bool,
    is_city: bool,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
struct RawPlayer {
    #[serde(deserialize_with = "de_i32")]
    player_id: i32,
    name: String,
    tribe_id: String,
    #[serde(deserialize_with = "de_i32")]
    kingdom_id: i32,
    treasures: i32,
    role: i32,
    external_login_token: String,
    villages: Vec<RawVillage>,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
struct RawKingdom {
    #[serde(deserialize_with = "de_i32")]
    kingdom_id: i32,
    kingdom_tag: String,
    #[serde(deserialize_with = "de_i32")]
    creation_time: i32,
    #[serde(deserialize_with = "de_i32")]
    victory_points: i32,
}

#[derive(Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct RawCell {
    #[serde(deserialize_with = "de_i32")]
    id: i32,
    #[serde(deserialize_with = "de_i32")]
    x: i32,
    #[serde(deserialize_with = "de_i32")]
    y: i32,
    res_type: String,
    oasis: String,
    #[serde(deserialize_with = "de_i32")]
    landscape: i32,
    #[serde(deserialize_with = "de_i32")]
    kingdom_id: i32,
}

#[derive(Deserialize, PartialEq)]
struct RawMap {
    #[serde(deserialize_with = "de_i32")]
    radius: i32,
    cells: Vec<RawCell>,
    landscapes: HashMap<i32, String>,
}

#[derive(Deserialize, PartialEq)]
pub struct RawMapData {
    gameworld: RawGameWorld,
    players: Vec<RawPlayer>,
    kingdoms: Vec<RawKingdom>,
    map: RawMap,
}

#[derive(Deserialize, PartialEq)]
pub struct RawResponse {
    response: RawMapData,
}

fn de_i32<'de, D: Deserializer<'de>>(deserializer: D) -> Result<i32, D::Error> {
    Ok(match Value::deserialize(deserializer)? {
        Value::String(s) => s.parse().map_err(de::Error::custom)?,
        Value::Number(num) => num.as_i64().ok_or(de::Error::custom("Invalid number"))? as i32,
        _ => return Err(de::Error::custom("wrong type")),
    })
}

#[test]
fn test_de() {
    let data = r#"
    {
        playerId: "123",
        name: "toto",
        tribeId: "0",
        kingdomId: "123",
        treasures: 123,
        role: 0,
        externalLoginToken: "abc",
        villages: [
            villageId: 123,
            x: "-12",
            y: -12,
            population: "123",
            name: "toto's village",
            is_main_village: true,
            is_city: false
        ]
    }
    "#;

    let expected = RawPlayer {
        player_id: 123,
        name: "toto".to_string(),
        tribe_id: "0".to_string(),
        kingdom_id: 123,
        treasures: 123,
        role: 0,
        external_login_token: "abc".to_string(),
        villages: vec![RawVillage {
            village_id: 123,
            x: -12,
            y: -12,
            population: 123,
            name: "toto's village".to_string(),
            is_main_village: true,
            is_city: false,
        }],
    };

    let data_de: RawPlayer = serde_json::from_str(data).expect("Could not parse data");
    assert_eq!(data_de, expected);
}

fn parse_raw_map_data(s: &str) -> Result<RawMapData, serde_json::Error> {
    let parsed_resp: RawResponse = serde_json::from_str(s)?;
    Ok(parsed_resp.response)
}

#[derive(Clone, Copy, Debug)]
pub enum Tribe {
    Gaul,
    Teuton,
    Roman,
}

impl Tribe {
    fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "1" => Ok(Tribe::Roman),
            "2" => Ok(Tribe::Teuton),
            "3" => Ok(Tribe::Gaul),
            _ => Err(format!("Unknown tribe code {}", s)),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Role {
    King,
    Vice,
    Duke,
    Gov,
}

impl Role {
    fn from_i32(i: i32) -> Result<Self, String> {
        match i {
            0 => Ok(Role::Gov),
            1 => Ok(Role::King),
            2 => Ok(Role::Duke),
            3 => Ok(Role::Vice),
            _ => Err(format!("Unknown role code {}", i)),
        }
    }
}

pub type Id = usize;
pub type IdMap<T> = HashMap<Id, T>;

#[derive(Clone, Copy, Debug)]
pub enum Cell {
    Oasis(Option<Id>),
    Empty(Option<Id>),
    Occupied(Option<Id>, Id, Id),
    Other(Option<Id>),
}

impl Cell {
    fn new(
        res_type: &str,
        oasis: &str,
        influenced_by: Option<Id>,
        belongs_to: Option<(Option<Id>, Id, Id)>,
    ) -> Self {
        match (oasis, res_type, belongs_to) {
            ("0", "0", _) => Cell::Other(influenced_by),
            (_, "0", _) => Cell::Oasis(influenced_by),
            ("0", _, None) => Cell::Empty(influenced_by),
            ("0", _, Some((kid, pid, vid))) => Cell::Occupied(kid, pid, vid),
            _ => Cell::Other(influenced_by),
        }
    }
}

pub trait Named {
    fn name(&self) -> &str;
}

#[derive(Clone, Debug)]
pub struct Kingdom {
    pub name: String,
    pub victory_points: i32,
    pub player_ids: Vec<Id>,
}

impl Named for Kingdom {
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

#[derive(Clone, Debug)]
pub struct Player {
    pub name: String,
    pub tribe: Tribe,
    pub role: Role,
    pub treasures: i32,
    pub village_ids: Vec<Id>,
}

impl Named for Player {
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

#[derive(Clone, Debug)]
pub struct Village {
    pub name: String,
    pub pop: i32,
    pub is_capital: bool,
    pub is_city: bool,
    pub coords: (i32, i32),
    pub crop_fields: Option<i32>,
}

impl Named for Village {
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

pub struct GameWorld {
    pub radius: i32,
    pub kingdoms: HashMap<Id, Kingdom>,
    pub players: HashMap<Id, Player>,
    pub villages: HashMap<Id, Village>,
    pub cells: HashMap<(i32, i32), Cell>,
}

impl GameWorld {
    pub fn from_raw_data(raw_data: &str) -> Result<Self, Box<dyn Error>> {
        let data = parse_raw_map_data(raw_data)?;
        GameWorld::from_data(&data)
    }

    pub fn from_data(data: &RawMapData) -> Result<Self, Box<dyn Error>> {
        let mut kingdoms = HashMap::new();
        for k in &data.kingdoms {
            kingdoms.insert(
                k.kingdom_id as Id,
                Kingdom {
                    name: k.kingdom_tag.to_owned(),
                    victory_points: k.victory_points,
                    player_ids: Vec::new(),
                },
            );
        }
        println!("Filled {} kingdoms", kingdoms.len());
        let mut players = HashMap::new();
        let mut villages = HashMap::new();
        for p in &data.players {
            let mut village_ids = Vec::new();
            // Fill player villages
            for v in &p.villages {
                let vid = v.village_id as Id;
                villages.insert(
                    vid,
                    Village {
                        name: v.name.to_owned(),
                        pop: v.population,
                        is_capital: v.is_main_village,
                        is_city: v.is_city,
                        coords: (v.x, v.y),
                        crop_fields: None,
                    },
                );
                village_ids.push(vid);
            }
            // Fill players
            let pid = p.player_id as Id;
            players.insert(
                pid,
                Player {
                    name: p.name.to_owned(),
                    tribe: Tribe::from_str(&p.tribe_id)?,
                    role: Role::from_i32(p.role)?,
                    treasures: p.treasures,
                    village_ids: village_ids,
                },
            );
            if let Some(k) = kingdoms.get_mut(&(p.kingdom_id as Id)) {
                k.player_ids.push(pid);
            }
        }
        println!("Filled {} players", players.len());

        // build a coord to ids map
        let mut coord_to_ids = HashMap::new();
        for (pid, p) in players.iter() {
            let kid = kingdoms
                .iter()
                .find(|(_kid, k)| k.player_ids.iter().any(|x| x == pid))
                .and_then(|x| Some(x.0.to_owned()));
            for (vid, v) in p
                .village_ids
                .iter()
                .filter_map(|x| villages.get_key_value(x))
            {
                coord_to_ids.insert(v.coords, (kid, pid.to_owned(), vid.to_owned()));
            }
        }
        println!("Filled {} villages", villages.len());

        // Fill the map
        let mut cells = HashMap::new();
        for c in &data.map.cells {
            let influenced_by = kingdoms
                .get_key_value(&(c.kingdom_id as Id))
                .and_then(|(kid, _)| Some(*kid));
            let belongs_to = coord_to_ids.get(&(c.x, c.y)).and_then(|v| Some(*v));
            cells.insert(
                (c.x, c.y),
                Cell::new(&c.res_type, &c.oasis, influenced_by, belongs_to),
            );
        }
        println!("Filled {} cells", cells.len());
        Ok(GameWorld {
            radius: data.map.radius,
            kingdoms: kingdoms,
            players: players,
            villages: villages,
            cells: cells,
        })
    }

    pub fn player_villages(&self, pid: &Id) -> Vec<Village> {
        let mut villages = Vec::new();
        if let Some(p) = self.players.get(pid) {
            p.village_ids
                .iter()
                .filter_map(|vid| self.villages.get(vid))
                .for_each(|v| villages.push(v.to_owned()));
        };
        villages
    }

    pub fn kingdom_villages(&self, kid: &Id) -> Vec<Village> {
        let mut villages = Vec::new();
        if let Some(k) = self.kingdoms.get(kid) {
            for pid in &k.player_ids {
                self.player_villages(pid)
                    .iter()
                    .for_each(|v| villages.push(v.to_owned()));
            }
        }
        villages
    }
}
