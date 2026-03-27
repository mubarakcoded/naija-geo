use crate::data::LGAS;
use crate::state::State;
use crate::zone::Zone;

#[cfg(feature = "serde")]
use serde::Serialize;

/// A Nigerian Local Government Area (LGA).
///
/// Nigeria has exactly 774 LGAs spread across 36 states and the FCT.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Lga {
    /// LGA code in the format `{STATE_CODE}{3-digit-seq}`, e.g. `"LA001"`.
    pub code: &'static str,
    /// LGA name, e.g. `"Agege"`.
    pub name: &'static str,
    /// Code of the state this LGA belongs to, e.g. `"LA"`.
    pub state_code: &'static str,
}

impl Lga {
    // ── Collection helpers ───────────────────────────────────────────────────

    /// Return a slice of all 774 LGAs.
    pub fn all() -> &'static [Lga] {
        LGAS
    }

    // ── Lookups ──────────────────────────────────────────────────────────────

    /// Find an LGA by its code (case-insensitive).
    ///
    /// ```
    /// use naija_geo::Lga;
    /// let lga = Lga::find("LA001").unwrap();
    /// assert_eq!(lga.name, "Agege");
    /// ```
    pub fn find(code: &str) -> Option<&'static Lga> {
        let upper = code.to_uppercase();
        LGAS.iter().find(|l| l.code == upper)
    }

    /// Find an LGA by name (case-insensitive, whitespace trimmed).
    ///
    /// If multiple LGAs share the same name, the first one in the dataset
    /// is returned. Use [`Lga::find_all_by_name`] to get every match.
    ///
    /// ```
    /// use naija_geo::Lga;
    /// let lga = Lga::find_by_name("Surulere").unwrap();
    /// assert_eq!(lga.state_code, "LA");
    /// ```
    pub fn find_by_name(name: &str) -> Option<&'static Lga> {
        let lower = name.trim().to_lowercase();
        LGAS.iter().find(|l| l.name.to_lowercase() == lower)
    }

    /// Return **all** LGAs whose name matches (useful for duplicate names such
    /// as "Surulere" which exists in both Lagos and Oyo).
    pub fn find_all_by_name(name: &str) -> Vec<&'static Lga> {
        let lower = name.trim().to_lowercase();
        LGAS.iter()
            .filter(|l| l.name.to_lowercase() == lower)
            .collect()
    }

    /// Find an LGA by code and return a `Result`.
    pub fn get(code: &str) -> Result<&'static Lga, crate::error::NaijaGeoError> {
        Lga::find(code).ok_or_else(|| crate::error::NaijaGeoError::LgaNotFound(code.to_string()))
    }

    /// Return all LGAs in a given state (by state code, case-insensitive).
    ///
    /// ```
    /// use naija_geo::Lga;
    /// assert_eq!(Lga::by_state("LA").len(), 20);
    /// ```
    pub fn by_state(state_code: &str) -> Vec<&'static Lga> {
        let upper = state_code.to_uppercase();
        LGAS.iter().filter(|l| l.state_code == upper).collect()
    }

    /// Return all LGAs in a given zone (by zone code, case-insensitive).
    pub fn by_zone(zone_code: &str) -> Vec<&'static Lga> {
        let upper = zone_code.to_uppercase();
        State::by_zone(&upper)
            .iter()
            .flat_map(|s| Lga::by_state(s.code))
            .collect()
    }

    // ── Fuzzy (feature = "fuzzy") ────────────────────────────────────────────

    /// Find the best-matching LGA by name using fuzzy string similarity.
    /// Returns `None` if the best score is below 0.5.
    ///
    /// Requires the `fuzzy` feature.
    #[cfg(feature = "fuzzy")]
    pub fn find_fuzzy(name: &str) -> Option<&'static Lga> {
        use strsim::jaro_winkler;
        let lower = name.trim().to_lowercase();
        LGAS.iter()
            .map(|l| (l, jaro_winkler(&l.name.to_lowercase(), &lower)))
            .filter(|(_, score)| *score >= 0.5)
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .map(|(l, _)| l)
    }

    // ── Traversal ────────────────────────────────────────────────────────────

    /// Return the state this LGA belongs to.
    pub fn state(&self) -> Option<&'static State> {
        State::find(self.state_code)
    }

    /// Return the geopolitical zone this LGA belongs to.
    pub fn zone(&self) -> Option<&'static Zone> {
        self.state().and_then(|s| s.zone())
    }
}

impl std::fmt::Display for Lga {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name, self.code)
    }
}
