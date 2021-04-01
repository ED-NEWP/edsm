use std::collections::HashMap;
use elite_journal::prelude::{State as JournalState, Happiness, Allegiance, Government};
use serde::Deserialize;

/// A group which inhabits one or more systems
///
/// Both player and non-player minor factions are represented as a `Faction`. Larger factions would
/// be called powers, and aren't supported by the EDSM API.
#[derive(Deserialize, Debug)]
pub struct Faction {
    pub id: u64,
    pub name: String,
    pub allegiance: Option<Allegiance>,
    pub government: Option<Government>,

    pub influence: f64,
    #[serde(rename = "influenceHistory")]
    pub influence_history: Option<HashMap<u64, f64>>,

    pub happieness: Option<Happiness>,
    #[serde(rename = "happienessHistory")]
    pub happieness_history: Option<HashMap<u64, Happiness>>,

    #[serde(rename = "state")]
    pub primary_state: Option<JournalState>,
    #[serde(rename = "stateHistory")]
    pub primary_state_history: Option<StateHistory>,

    #[serde(rename = "activeStates")]
    pub active_states: Vec<State>,
    #[serde(rename = "activeStatesHistory")]
    pub active_states_history: Option<ActiveStatesHistory>,

    #[serde(rename = "recoveringStates")]
    pub recovering_states: Vec<TrendingState>,
    #[serde(rename = "recoveringStatesHistory")]
    pub recovering_states_history: Option<TrendingStatesHistory>,

    #[serde(rename = "pendingStates")]
    pub pending_states: Vec<TrendingState>,
    #[serde(rename = "pendingStatesHistory")]
    pub pending_states_history: Option<TrendingStatesHistory>,


    #[serde(rename = "isPlayer")]
    pub is_player: bool,
    #[serde(rename = "lastUpdated")]
    pub last_updated: Option<u64>,
}

/// The faction which controls the primary starport controls the system
#[derive(Deserialize, Debug)]
pub struct ControllingFaction {
    pub id: u64,
    pub name: Option<String>,
    pub allegiance: Option<Allegiance>,
    pub government: Option<Government>,
}

/// A condition which a faction can be experiencing
///
/// Some states effect every system a faction is present in.
#[derive(Deserialize, Debug)]
pub struct State {
    pub state: JournalState,
}

/// A state with it's trend
#[derive(Deserialize, Debug)]
pub struct TrendingState {
    #[serde(flatten)]
    pub state: State,
    pub trend: u64,
}

/// The history of a faction's states
#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum StateHistory {
    Empty([(); 0]),
    // NOTE: The keys here are really Unix Timestamps.
    // TODO: Make the values `State` structures.
    Map(HashMap<String, String>),
}

/// The history of a faction's active states
// TODO: Unify with `StateHistory`.
#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum ActiveStatesHistory {
    Empty([(); 0]),
    // NOTE: The keys here are really Unix Timestamps.
    // TODO: Make the values `State` structures.
    Map(HashMap<String, Vec<State>>),
}

/// The history of a faction's states w/ their trends
#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum TrendingStatesHistory {
    Empty([(); 0]),
    // NOTE: The keys here are really Unix Timestamps.
    Map(HashMap<String, Vec<TrendingState>>),
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_history() {
        let empty_array = r#"
            []
        "#;
        let state_history: StateHistory = serde_json::from_str(empty_array).unwrap();
        assert!(matches!(state_history, StateHistory::Empty(_)));
        let object = r#"
            { "123": "foo" }
        "#;
        let state_history: StateHistory = serde_json::from_str(object).unwrap();
        assert!(matches!(state_history, StateHistory::Map(_)));
    }
}
