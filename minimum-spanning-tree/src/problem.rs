use std::f32::consts::PI;

use crate::consts::R;

use serde::{de, Deserialize, Deserializer};

/// Parse angles
fn angle_deserializer<'de, D>(deserializer: D) -> Result<(u32, u32), D::Error>
where
    D: Deserializer<'de>,
{
    let degrees_and_minutes = String::deserialize(deserializer)?
        .split("°")
        .map(|x| {
            x.chars()
                .take_while(|s| s.is_ascii_digit())
                .collect::<String>()
        })
        .map(|x| x.parse::<u32>().map_err(de::Error::custom))
        .collect::<Result<Vec<_>, _>>()?;

    Ok((degrees_and_minutes[0], degrees_and_minutes[1]))
}

#[derive(Deserialize, Debug, Clone)]
pub struct City {
    pub name: String,

    #[serde(deserialize_with = "angle_deserializer")]
    pub long: (u32, u32),
    #[serde(deserialize_with = "angle_deserializer")]
    pub lat: (u32, u32),

    #[serde(skip)]
    pub x: u32,
    #[serde(skip)]
    pub y: u32,
}

impl City {
    pub fn calculate_coordinates(mut self) -> Self {
        self.x = 60 * self.long.0 + self.long.1;
        self.y = 60 * self.lat.0 + self.lat.1;
        self
    }
}

pub trait Distance: 'static + Send {
    fn distance(a: &City, b: &City) -> f32;
}

pub struct Euclidean;
impl Distance for Euclidean {
    fn distance(a: &City, b: &City) -> f32 {
        (((a.x - b.x).pow(2) + (a.y - b.y).pow(2)) as f32).sqrt()
    }
}

pub struct Archaversine;
impl Distance for Archaversine {
    fn distance(a: &City, b: &City) -> f32 {
        let lat_a = (a.lat.0 as f32 + (a.lat.1 as f32 / 60.0)) * PI / 180.0;
        let lat_b = (b.lat.0 as f32 + (b.lat.1 as f32 / 60.0)) * PI / 180.0;
        let long_a = (a.long.0 as f32 + (a.long.1 as f32 / 60.0)) * PI / 180.0;
        let long_b = (b.long.0 as f32 + (b.long.1 as f32 / 60.0)) * PI / 180.0;
        2.0 * R
            * (((1.0 - (lat_a - lat_b).cos())
                + lat_a.cos() * lat_b.cos() * (1.0 - (long_a - long_b).cos()))
                / 2.0)
                .sqrt()
                .asin()
    }
}
