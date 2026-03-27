# naija-geo

[![Crates.io](https://img.shields.io/crates/v/naija-geo.svg)](https://crates.io/crates/naija-geo)
[![Docs.rs](https://docs.rs/naija-geo/badge.svg)](https://docs.rs/naija-geo)
[![CI](https://github.com/mubarakcoded/naija-geo/actions/workflows/ci.yml/badge.svg)](https://github.com/mubarakcoded/naija-geo/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A Rust crate providing accurate, complete data on Nigeria's **6 geopolitical zones**, **37 states** (36 states + FCT), and all **774 local government areas (LGAs)** — with full bi-directional navigation helpers, case-insensitive lookups, and optional `serde` and fuzzy-search support.

All data lives in static slices compiled directly into your binary — no files to read, no network calls, no heap allocations on lookup.

---

## Contents

- [Features](#features)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [API Reference](#api-reference)
  - [Zone](#zone)
  - [State](#state)
  - [Lga](#lga)
  - [NaijaGeoError](#naijageoerror)
- [Data Reference](#data-reference)
  - [Geopolitical Zones](#geopolitical-zones)
  - [States](#states)
- [Feature Flags](#feature-flags)
- [Error Handling](#error-handling)
- [Examples](#examples)
- [Contributing](#contributing)
- [License](#license)

---

## Features

- ✅ All **6 geopolitical zones** with member-state lists
- ✅ All **37 states** (36 + FCT) with codes, names and capitals
- ✅ All **774 LGAs** with codes and parent-state references
- ✅ Bi-directional traversal — go from a zone down to LGAs, or from an LGA back up to its zone
- ✅ Case-insensitive lookups for codes and names
- ✅ `find_all_by_name` to handle real duplicate LGA names (e.g. *Surulere* exists in both Lagos and Oyo)
- ✅ `Result`-returning `get()` helpers for clean `?`-propagation
- ✅ `Display` on all types
- ✅ Optional **serde** feature — `Serialize` / `Deserialize` on all structs
- ✅ Optional **fuzzy** feature — fuzzy name matching via Jaro-Winkler distance
- ✅ Zero runtime I/O — all data is `&'static`
- ✅ No `unsafe` code

---

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
naija-geo = "0.1"
```

With optional features:

```toml
[dependencies]
naija-geo = { version = "0.1", features = ["serde", "fuzzy"] }
```

---

## Quick Start

```rust
use naija_geo::{Zone, State, Lga};

fn main() {
    // ── Zones ──────────────────────────────────────────────────────────────
    println!("There are {} geopolitical zones", Zone::all().len()); // 6

    let sw = Zone::find("SW").unwrap();
    println!("{} has {} states and {} LGAs",
        sw.name, sw.state_count(), sw.lga_count());
    // "South West has 6 states and 137 LGAs"

    // ── States ─────────────────────────────────────────────────────────────
    let lagos = State::find("LA").unwrap();
    println!("{} | Capital: {} | Zone: {}",
        lagos.name, lagos.capital, lagos.zone_code);
    // "Lagos | Capital: Ikeja | Zone: SW"

    // ── LGAs ───────────────────────────────────────────────────────────────
    let lgas = lagos.lgas();
    println!("{} has {} LGAs", lagos.name, lgas.len()); // 20

    // ── Navigate upward from any LGA ───────────────────────────────────────
    let lga   = Lga::find("LA020").unwrap();       // Surulere
    let state = lga.state().unwrap();              // Lagos
    let zone  = lga.zone().unwrap();               // South West
    println!("{} → {} → {}", lga.name, state.name, zone.name);
    // "Surulere → Lagos → South West"

    // ── Duplicate LGA names ────────────────────────────────────────────────
    let matches = Lga::find_all_by_name("Surulere");
    for m in &matches {
        println!("{} ({}) is in {}", m.name, m.code, m.state().unwrap().name);
    }
    // "Surulere (LA020) is in Lagos"
    // "Surulere (OY033) is in Oyo"
}
```

---

## API Reference

### Zone

Represents one of Nigeria's six geopolitical zones.

```rust
pub struct Zone {
    pub code:        &'static str,            // "SW"
    pub name:        &'static str,            // "South West"
    pub state_codes: &'static [&'static str], // ["EK","LA","OG","ON","OS","OY"]
}
```

#### Associated functions

| Function | Return | Description |
|---|---|---|
| `Zone::all()` | `&'static [Zone]` | All six zones |
| `Zone::find(code)` | `Option<&'static Zone>` | Lookup by code (case-insensitive) |
| `Zone::find_by_name(name)` | `Option<&'static Zone>` | Lookup by name (case-insensitive, trimmed) |
| `Zone::get(code)` | `Result<&'static Zone, NaijaGeoError>` | Like `find`, but returns `Err` instead of `None` |
| `Zone::find_fuzzy(name)` *(feature: fuzzy)* | `Option<&'static Zone>` | Best fuzzy match by name |

#### Methods

| Method | Return | Description |
|---|---|---|
| `zone.states()` | `Vec<&'static State>` | All states in the zone |
| `zone.lgas()` | `Vec<&'static Lga>` | All LGAs across every member state |
| `zone.state_count()` | `usize` | Number of states |
| `zone.lga_count()` | `usize` | Total LGA count across all member states |

```rust
use naija_geo::Zone;

let nc = Zone::find("NC").unwrap();
assert_eq!(nc.name, "North Central");
assert_eq!(nc.state_count(), 7);

for state in nc.states() {
    println!("  {} — {} LGAs", state.name, state.lga_count());
}
```

---

### State

Represents one of Nigeria's 36 states or the FCT (37 entries total).

```rust
pub struct State {
    pub code:      &'static str,            // "LA"
    pub name:      &'static str,            // "Lagos"
    pub capital:   &'static str,            // "Ikeja"
    pub zone_code: &'static str,            // "SW"
    pub lga_codes: &'static [&'static str], // ["LA001","LA002",...]
}
```

#### Associated functions

| Function | Return | Description |
|---|---|---|
| `State::all()` | `&'static [State]` | All 37 states |
| `State::find(code)` | `Option<&'static State>` | Lookup by 2-letter code (case-insensitive) |
| `State::find_by_name(name)` | `Option<&'static State>` | Lookup by name (case-insensitive, trimmed) |
| `State::by_zone(zone_code)` | `Vec<&'static State>` | All states in a given zone |
| `State::get(code)` | `Result<&'static State, NaijaGeoError>` | Like `find`, but returns `Err` instead of `None` |
| `State::find_fuzzy(name)` *(feature: fuzzy)* | `Option<&'static State>` | Best fuzzy match by name |

#### Methods

| Method | Return | Description |
|---|---|---|
| `state.zone()` | `Option<&'static Zone>` | The zone this state belongs to |
| `state.lgas()` | `Vec<&'static Lga>` | All LGAs in this state |
| `state.lga_count()` | `usize` | Number of LGAs |

```rust
use naija_geo::State;

// Lookup by code
let rivers = State::find("RI").unwrap();
assert_eq!(rivers.capital, "Port Harcourt");
assert_eq!(rivers.lga_count(), 23);

// Lookup by name
let kano = State::find_by_name("kano").unwrap(); // case-insensitive
assert_eq!(kano.code, "KN");

// All states in the North West
for state in State::by_zone("NW") {
    println!("{} ({})", state.name, state.capital);
}

// Navigate to the parent zone
let zone = rivers.zone().unwrap();
assert_eq!(zone.name, "South South");
```

---

### Lga

Represents one of Nigeria's 774 Local Government Areas.

```rust
pub struct Lga {
    pub code:       &'static str, // "LA001"
    pub name:       &'static str, // "Agege"
    pub state_code: &'static str, // "LA"
}
```

LGA codes follow the format `{STATE_CODE}{3-digit-seq}`, e.g. `LA001`, `KN044`.

#### Associated functions

| Function | Return | Description |
|---|---|---|
| `Lga::all()` | `&'static [Lga]` | All 774 LGAs |
| `Lga::find(code)` | `Option<&'static Lga>` | Lookup by code (case-insensitive) |
| `Lga::find_by_name(name)` | `Option<&'static Lga>` | First match by name (case-insensitive) |
| `Lga::find_all_by_name(name)` | `Vec<&'static Lga>` | **All** LGAs with a given name (handles duplicates) |
| `Lga::by_state(state_code)` | `Vec<&'static Lga>` | All LGAs in a given state |
| `Lga::by_zone(zone_code)` | `Vec<&'static Lga>` | All LGAs in a given zone |
| `Lga::get(code)` | `Result<&'static Lga, NaijaGeoError>` | Like `find`, but returns `Err` instead of `None` |
| `Lga::find_fuzzy(name)` *(feature: fuzzy)* | `Option<&'static Lga>` | Best fuzzy match by name |

#### Methods

| Method | Return | Description |
|---|---|---|
| `lga.state()` | `Option<&'static State>` | The state this LGA belongs to |
| `lga.zone()` | `Option<&'static Zone>` | The zone this LGA belongs to |

```rust
use naija_geo::Lga;

// By code
let lga = Lga::find("KN021").unwrap();
assert_eq!(lga.name, "Kano Municipal");

// By name (case-insensitive)
let chibok = Lga::find_by_name("chibok").unwrap();
assert_eq!(chibok.state_code, "BO");

// Duplicate names — get all matches
let suruleres = Lga::find_all_by_name("Surulere");
assert_eq!(suruleres.len(), 2); // Lagos and Oyo

// All LGAs in Kano state
let kano_lgas = Lga::by_state("KN");
assert_eq!(kano_lgas.len(), 44);

// All LGAs in the South East zone
let se_lgas = Lga::by_zone("SE");
assert_eq!(se_lgas.len(), 95);

// Navigate upward
let zone = Lga::find("BO006").unwrap().zone().unwrap();
assert_eq!(zone.name, "North East");
```

---

### NaijaGeoError

All `get()` methods return a `Result<_, NaijaGeoError>`.

```rust
pub enum NaijaGeoError {
    ZoneNotFound(String),
    StateNotFound(String),
    LgaNotFound(String),
}
```

`NaijaGeoError` implements `std::fmt::Display` and `std::error::Error`, so it composes naturally with `?` and `anyhow`/`thiserror`.

```rust
use naija_geo::{Zone, State, Lga};

fn describe_lga(lga_code: &str) -> Result<String, Box<dyn std::error::Error>> {
    let lga   = Lga::get(lga_code)?;
    let state = lga.state().unwrap();
    let zone  = lga.zone().unwrap();
    Ok(format!("{} — {} State — {} Zone", lga.name, state.name, zone.name))
}

assert_eq!(
    describe_lga("LA020").unwrap(),
    "Surulere — Lagos State — South West Zone"
);
```

---

## Data Reference

### Geopolitical Zones

| Code | Name | States | LGAs |
|------|------|-------:|-----:|
| `NC` | North Central | 7 | 121 |
| `NE` | North East | 6 | 112 |
| `NW` | North West | 7 | 186 |
| `SE` | South East | 5 | 95 |
| `SS` | South South | 6 | 123 |
| `SW` | South West | 6 | 137 |
| | **Total** | **37** | **774** |

### States

| Code | State | Capital | Zone | LGAs |
|------|-------|---------|------|-----:|
| `AB` | Abia | Umuahia | SE | 17 |
| `AD` | Adamawa | Yola | NE | 21 |
| `AK` | Akwa Ibom | Uyo | SS | 31 |
| `AN` | Anambra | Awka | SE | 21 |
| `BA` | Bauchi | Bauchi | NE | 20 |
| `BY` | Bayelsa | Yenagoa | SS | 8 |
| `BE` | Benue | Makurdi | NC | 23 |
| `BO` | Borno | Maiduguri | NE | 27 |
| `CR` | Cross River | Calabar | SS | 18 |
| `DE` | Delta | Asaba | SS | 25 |
| `EB` | Ebonyi | Abakaliki | SE | 13 |
| `ED` | Edo | Benin City | SS | 18 |
| `EK` | Ekiti | Ado-Ekiti | SW | 16 |
| `EN` | Enugu | Enugu | SE | 17 |
| `FC` | FCT | Abuja | NC | 6 |
| `GO` | Gombe | Gombe | NE | 11 |
| `IM` | Imo | Owerri | SE | 27 |
| `JI` | Jigawa | Dutse | NW | 27 |
| `KD` | Kaduna | Kaduna | NW | 23 |
| `KN` | Kano | Kano | NW | 44 |
| `KT` | Katsina | Katsina | NW | 34 |
| `KB` | Kebbi | Birnin Kebbi | NW | 21 |
| `KO` | Kogi | Lokoja | NC | 21 |
| `KW` | Kwara | Ilorin | NC | 16 |
| `LA` | Lagos | Ikeja | SW | 20 |
| `NA` | Nasarawa | Lafia | NC | 13 |
| `NI` | Niger | Minna | NC | 25 |
| `OG` | Ogun | Abeokuta | SW | 20 |
| `ON` | Ondo | Akure | SW | 18 |
| `OS` | Osun | Osogbo | SW | 30 |
| `OY` | Oyo | Ibadan | SW | 33 |
| `PL` | Plateau | Jos | NC | 17 |
| `RI` | Rivers | Port Harcourt | SS | 23 |
| `SO` | Sokoto | Sokoto | NW | 23 |
| `TA` | Taraba | Jalingo | NE | 16 |
| `YO` | Yobe | Damaturu | NE | 17 |
| `ZA` | Zamfara | Gusau | NW | 14 |

---

## Feature Flags

| Feature | Default | Dependency | What it adds |
|---------|---------|------------|---|
| `serde` | No | `serde ^1` | `Serialize` on `Zone`, `State`, `Lga` (serialize out to JSON, TOML, etc.) |
| `fuzzy` | No | `strsim ^0.11` | `find_fuzzy(name)` on all three types |

### Serde

```toml
naija-geo = { version = "0.1", features = ["serde"] }
```

```rust
use naija_geo::State;
use serde_json;

let lagos = State::find("LA").unwrap();
let json  = serde_json::to_string_pretty(lagos).unwrap();
println!("{json}");
// {
//   "code": "LA",
//   "name": "Lagos",
//   "capital": "Ikeja",
//   "zone_code": "SW",
//   "lga_codes": ["LA001", "LA002", ...]
// }
```

### Fuzzy search

```toml
naija-geo = { version = "0.1", features = ["fuzzy"] }
```

```rust
use naija_geo::{State, Lga};

// Handles typos and partial matches
let s = State::find_fuzzy("Lagoss").unwrap();
assert_eq!(s.code, "LA");

let l = Lga::find_fuzzy("Port-Harcort").unwrap();
assert_eq!(l.state_code, "RI");
```

Fuzzy matching uses the [Jaro-Winkler](https://en.wikipedia.org/wiki/Jaro%E2%80%93Winkler_distance) algorithm with a minimum similarity threshold of **0.4** for zones and states, and **0.5** for LGAs (tighter, as LGA names are shorter and more collision-prone).

---

## Error Handling

Every type exposes both an `Option`-returning `find()` and a `Result`-returning `get()`.

```rust
// Option style — when absence is expected
if let Some(lga) = Lga::find("LA001") {
    println!("Found: {}", lga.name);
}

// Result style — when absence is a bug
use naija_geo::NaijaGeoError;

fn process(code: &str) -> Result<(), NaijaGeoError> {
    let state = State::get(code)?;       // returns Err(StateNotFound) on miss
    println!("Processing {}", state.name);
    Ok(())
}
```

---

## Examples

Run the bundled example with:

```bash
cargo run --example basic
```

Output:

```
=== Geopolitical Zones ===
  [NC] North Central   — 7 states, 121 LGAs
  [NE] North East      — 6 states, 112 LGAs
  [NW] North West      — 7 states, 186 LGAs
  [SE] South East      — 5 states, 95 LGAs
  [SS] South South     — 6 states, 123 LGAs
  [SW] South West      — 6 states, 137 LGAs

=== South West States ===
  [EK] Ekiti           | Capital: Ado-Ekiti       | 16 LGAs
  [LA] Lagos           | Capital: Ikeja           | 20 LGAs
  [OG] Ogun            | Capital: Abeokuta        | 20 LGAs
  [ON] Ondo            | Capital: Akure           | 18 LGAs
  [OS] Osun            | Capital: Osogbo          | 30 LGAs
  [OY] Oyo             | Capital: Ibadan          | 33 LGAs

=== Lagos LGAs ===
  [LA001] Agege
  [LA002] Ajeromi-Ifelodun
  ... (20 total)

=== Upward traversal from LGA LA020 ===
  LGA   : Surulere (LA020)
  State : Lagos (LA)
  Zone  : South West (SW)

=== All LGAs named 'Surulere' ===
  Surulere (LA020) — Lagos State
  Surulere (OY033) — Oyo State

=== Grand Totals ===
  Zones  : 6
  States : 37
  LGAs   : 774
```

---

## Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository and create a feature branch:
   ```bash
   git checkout -b fix/lga-name-typo
   ```
2. Make your changes. If correcting data, please cite your source (INEC, NBS, or NPC).
3. Run the full test suite — it must pass with zero warnings:
   ```bash
   cargo test
   cargo clippy -- -D warnings
   ```
4. Open a pull request with a clear description of the change.

### Data sources

All data in this crate is sourced from official Nigerian government publications:

- **INEC** (Independent National Electoral Commission) — authoritative 774 LGA list
- **NBS** (National Bureau of Statistics) — state and LGA codes used in official surveys
- **NPC** (National Population Commission) — census boundary definitions

If you find an error in a name, code, or zone assignment, please open an issue quoting the authoritative source for the correction.

---

## License

This project is licensed under the [MIT License](LICENSE).

---

*Built with ❤️ for the Nigerian developer community.*
