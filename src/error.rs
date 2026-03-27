use std::fmt;

/// All errors that can be returned by naija-geo look-ups.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NaijaGeoError {
    /// No zone was found for the given code or name.
    ZoneNotFound(String),
    /// No state was found for the given code or name.
    StateNotFound(String),
    /// No LGA was found for the given code or name.
    LgaNotFound(String),
}

impl fmt::Display for NaijaGeoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NaijaGeoError::ZoneNotFound(s) => write!(f, "Zone not found: '{s}'"),
            NaijaGeoError::StateNotFound(s) => write!(f, "State not found: '{s}'"),
            NaijaGeoError::LgaNotFound(s) => write!(f, "LGA not found: '{s}'"),
        }
    }
}

impl std::error::Error for NaijaGeoError {}
