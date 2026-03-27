//! # naija-geo
//!
//! Nigerian geopolitical zones, states and local government areas (LGAs).
//!
//! Covers all **6 zones**, **37 states** (36 states + FCT) and all **774 LGAs**
//! with full bi-directional navigation helpers.
//!
//! ## Quick start
//!
//! ```rust
//! use naija_geo::{Zone, State, Lga};
//!
//! // All zones
//! for z in Zone::all() {
//!     println!("{} — {} states", z.name, z.state_count());
//! }
//!
//! // Lookup by code (case-insensitive)
//! let lagos = State::find("LA").unwrap();
//! assert_eq!(lagos.capital, "Ikeja");
//! assert_eq!(lagos.lga_count(), 20);
//!
//! // Navigate upward from an LGA
//! let lga   = Lga::find("LA020").unwrap();      // Surulere
//! let state = lga.state().unwrap();             // Lagos
//! let zone  = lga.zone().unwrap();              // South West
//! println!("{} → {} → {}", lga.name, state.name, zone.name);
//!
//! // Navigate downward from a zone
//! let sw_lgas = Zone::find("SW").unwrap().lgas();
//! println!("South West has {} LGAs", sw_lgas.len());
//! ```
//!
//! ## Feature flags
//!
//! | Feature | What it adds |
//! |---------|-------------|
//! | `serde` | `Serialize` / `Deserialize` on all public structs |
//! | `fuzzy` | `find_fuzzy(name)` on `Zone`, `State`, `Lga` |

pub mod error;
pub mod lga;
pub mod state;
pub mod zone;

mod data;

// Re-export the three primary types at the crate root for ergonomics.
pub use error::NaijaGeoError;
pub use lga::Lga;
pub use state::State;
pub use zone::Zone;
