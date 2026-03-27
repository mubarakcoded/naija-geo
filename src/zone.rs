use crate::data::ZONES;
use crate::lga::Lga;
use crate::state::State;

#[cfg(feature = "serde")]
use serde::Serialize;

/// A Nigerian geopolitical zone.
///
/// Nigeria has six geopolitical zones (NC, NE, NW, SE, SS, SW).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Zone {
    /// Two-letter zone code, e.g. `"SW"`.
    pub code: &'static str,
    /// Full zone name, e.g. `"South West"`.
    pub name: &'static str,
    /// Codes of the states that belong to this zone.
    pub state_codes: &'static [&'static str],
}

impl Zone {
    // ── Collection helpers ───────────────────────────────────────────────────

    /// Return a slice of all six geopolitical zones.
    pub fn all() -> &'static [Zone] {
        ZONES
    }

    // ── Lookup ───────────────────────────────────────────────────────────────

    /// Find a zone by its two-letter code (case-insensitive).
    ///
    /// ```
    /// use naija_geo::Zone;
    /// let z = Zone::find("sw").unwrap();
    /// assert_eq!(z.name, "South West");
    /// ```
    pub fn find(code: &str) -> Option<&'static Zone> {
        let upper = code.to_uppercase();
        ZONES.iter().find(|z| z.code == upper)
    }

    /// Find a zone by its full name (case-insensitive, leading/trailing
    /// whitespace is ignored).
    ///
    /// ```
    /// use naija_geo::Zone;
    /// let z = Zone::find_by_name("north central").unwrap();
    /// assert_eq!(z.code, "NC");
    /// ```
    pub fn find_by_name(name: &str) -> Option<&'static Zone> {
        let lower = name.trim().to_lowercase();
        ZONES.iter().find(|z| z.name.to_lowercase() == lower)
    }

    /// Find a zone by code and return a `Result`, useful for `?`-propagation.
    pub fn get(code: &str) -> Result<&'static Zone, crate::error::NaijaGeoError> {
        Zone::find(code).ok_or_else(|| crate::error::NaijaGeoError::ZoneNotFound(code.to_string()))
    }

    // ── Fuzzy (feature = "fuzzy") ────────────────────────────────────────────

    /// Find the best-matching zone by name using fuzzy string similarity.
    /// Returns `None` if the best score is below 0.4.
    ///
    /// Requires the `fuzzy` feature.
    #[cfg(feature = "fuzzy")]
    pub fn find_fuzzy(name: &str) -> Option<&'static Zone> {
        use strsim::jaro_winkler;
        let lower = name.trim().to_lowercase();
        ZONES
            .iter()
            .map(|z| (z, jaro_winkler(&z.name.to_lowercase(), &lower)))
            .filter(|(_, score)| *score >= 0.4)
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .map(|(z, _)| z)
    }

    // ── Traversal ────────────────────────────────────────────────────────────

    /// Return all states that belong to this zone.
    pub fn states(&self) -> Vec<&'static State> {
        self.state_codes
            .iter()
            .filter_map(|c| State::find(c))
            .collect()
    }

    /// Return all LGAs that belong to this zone (across all member states).
    pub fn lgas(&self) -> Vec<&'static Lga> {
        self.states().iter().flat_map(|s| s.lgas()).collect()
    }

    /// Number of states in this zone.
    pub fn state_count(&self) -> usize {
        self.state_codes.len()
    }

    /// Total number of LGAs across all states in this zone.
    pub fn lga_count(&self) -> usize {
        self.states().iter().map(|s| s.lga_count()).sum()
    }
}

impl std::fmt::Display for Zone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name, self.code)
    }
}
