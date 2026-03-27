use crate::data::STATES;
use crate::lga::Lga;
use crate::zone::Zone;

#[cfg(feature = "serde")]
use serde::Serialize;

/// A Nigerian state (or the FCT).
///
/// Nigeria has 36 states plus the Federal Capital Territory (FCT),
/// giving 37 entries total.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct State {
    /// Two-letter state code, e.g. `"LA"` for Lagos.
    pub code: &'static str,
    /// State name, e.g. `"Lagos"`.
    pub name: &'static str,
    /// State capital, e.g. `"Ikeja"`.
    pub capital: &'static str,
    /// Code of the geopolitical zone this state belongs to.
    pub zone_code: &'static str,
    /// LGA codes belonging to this state, e.g. `["LA001", "LA002", …]`.
    pub lga_codes: &'static [&'static str],
}

impl State {
    // ── Collection helpers ───────────────────────────────────────────────────

    /// Return a slice of all 37 states (36 states + FCT).
    pub fn all() -> &'static [State] {
        STATES
    }

    // ── Lookups ──────────────────────────────────────────────────────────────

    /// Find a state by its two-letter code (case-insensitive).
    ///
    /// ```
    /// use naija_geo::State;
    /// let s = State::find("LA").unwrap();
    /// assert_eq!(s.name, "Lagos");
    /// ```
    pub fn find(code: &str) -> Option<&'static State> {
        let upper = code.to_uppercase();
        STATES.iter().find(|s| s.code == upper)
    }

    /// Find a state by its name (case-insensitive, whitespace trimmed).
    ///
    /// ```
    /// use naija_geo::State;
    /// let s = State::find_by_name("lagos").unwrap();
    /// assert_eq!(s.code, "LA");
    /// ```
    pub fn find_by_name(name: &str) -> Option<&'static State> {
        let lower = name.trim().to_lowercase();
        STATES.iter().find(|s| s.name.to_lowercase() == lower)
    }

    /// Find a state by code and return a `Result`.
    pub fn get(code: &str) -> Result<&'static State, crate::error::NaijaGeoError> {
        State::find(code)
            .ok_or_else(|| crate::error::NaijaGeoError::StateNotFound(code.to_string()))
    }

    /// Return all states that belong to a given zone (by zone code,
    /// case-insensitive).
    ///
    /// ```
    /// use naija_geo::State;
    /// let sw = State::by_zone("SW");
    /// assert_eq!(sw.len(), 6);
    /// ```
    pub fn by_zone(zone_code: &str) -> Vec<&'static State> {
        let upper = zone_code.to_uppercase();
        STATES.iter().filter(|s| s.zone_code == upper).collect()
    }

    // ── Fuzzy (feature = "fuzzy") ────────────────────────────────────────────

    /// Find the best-matching state by name using fuzzy string similarity.
    /// Returns `None` if the best score is below 0.4.
    ///
    /// Requires the `fuzzy` feature.
    #[cfg(feature = "fuzzy")]
    pub fn find_fuzzy(name: &str) -> Option<&'static State> {
        use strsim::jaro_winkler;
        let lower = name.trim().to_lowercase();
        STATES
            .iter()
            .map(|s| (s, jaro_winkler(&s.name.to_lowercase(), &lower)))
            .filter(|(_, score)| *score >= 0.4)
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .map(|(s, _)| s)
    }

    // ── Traversal ────────────────────────────────────────────────────────────

    /// Return the zone this state belongs to.
    pub fn zone(&self) -> Option<&'static Zone> {
        Zone::find(self.zone_code)
    }

    /// Return all LGAs in this state.
    pub fn lgas(&self) -> Vec<&'static Lga> {
        self.lga_codes
            .iter()
            .filter_map(|c| Lga::find(c))
            .collect()
    }

    /// Number of LGAs in this state.
    pub fn lga_count(&self) -> usize {
        self.lga_codes.len()
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name, self.code)
    }
}
