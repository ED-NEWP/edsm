use std::collections::HashMap;
// use chrono::naive::NaiveDateTime;
use serde::Deserialize;
use elite_journal::prelude::{Security, Government, Allegiance, Economy};
use crate::{Body, ControllingFaction, Faction, State};
// use crate::serde_utils;

/// A position in 3 dimensional euclidean space
#[derive(Deserialize, Debug, PartialEq)]
pub struct Coordinate {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// EDSM's representation of a solar system
#[derive(Deserialize, Debug)]
pub struct System {
    pub name: String,
    pub id: Option<u64>,
    pub id64: Option<u64>,

    // TODO: Support Optional date.
    // #[serde(deserialize_with = "serde_utils::space_seperated_datetime")]
    // pub date: Option<NaiveDateTime>,

    // from showCoords API parameter.
    pub coords: Option<Coordinate>,
    #[serde(rename = "coordsLocked")]
    pub coords_locked: Option<bool>,

    // from showInformation API parameter.
    #[serde(flatten, deserialize_with = "deserialize_information")]
    pub information: Information,

    // from showPermit API parameter.
    #[serde(rename = "requirePermit")]
    pub require_permit: Option<bool>,
    #[serde(rename = "permitName")]
    pub permit_name: Option<String>,

    // NOTE: Skipping this for now, use bodies to access if needed. Use the
    // showPrimaryStar API parameter when needed.
    // pub primary_star: Option<PrimaryStar>,

    // from system bodies API.
    #[serde(rename = "bodyCount")]
    pub body_count: Option<u64>,
    pub bodies: Option<Vec<Body>>,

    // TODO
    // pub stations: Option<Vec<Station>>,

    pub factions: Option<Vec<Faction>>,
    #[serde(rename = "controllingFaction")]
    pub controlling_faction: Option<ControllingFaction>,

    pub deaths: Option<Statistic>,

    pub traffic: Option<Statistic>,
    #[serde(rename = "breakdown")]
    pub traffic_breakdown: Option<HashMap<String, u64>>,
}

fn deserialize_information<'de, T, D>(deserializer: D) -> std::result::Result<T, D::Error>
where
    T: serde::Deserialize<'de>,
    D: serde::Deserializer<'de>,
{
    #[derive(serde::Deserialize)]
    struct Both<T> {
        #[serde(flatten)]
        flat: Option<T>,
        #[serde(rename = "information")]
        not_flat: Option<T>,
    }

    let both: Both<T> = serde::Deserialize::deserialize(deserializer)?;
    match (both.flat, both.not_flat) {
        (Some(t), None) | (None, Some(t)) => Ok(t),
        (None, None) => Err(serde::de::Error::missing_field("information")),
        // TODO: Actually merge the fields on each... though I'm not sure we'll ever see a case
        // where this is needed.
        (Some(_), Some(t)) => Ok(t),
    }
}

/// General [System] information
#[derive(Deserialize, Debug)]
pub struct Information {
    pub allegiance: Option<Allegiance>,
    pub government: Option<Government>,

    /// The controlling faction's primary state.
    // TODO: Revisit if these can be a Faction object easily.
    pub faction: Option<String>,
    // TODO: Revisit alias / rename with API.
    // #[serde(rename = "factionState")]
    #[serde(alias = "factionState")]
    #[serde(flatten)]
    pub state: Option<State>,

    pub population: Option<u64>,
    pub security: Option<Security>,
    pub economy: Option<Economy>,
    #[serde(rename = "secondEconomy")]
    pub second_economy: Option<Economy>,
    // TODO: Add type in elite_journal
    pub reserve: Option<String>,
}

/// Statistics for how much ship traffic a [System] gets.
#[derive(Deserialize, Debug)]
pub struct Statistic {
    pub total: u64,
    pub week: u64,
    pub day: u64,
}
