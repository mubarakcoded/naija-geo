# naija-geo

[![Crates.io](https://img.shields.io/crates/v/naija-geo.svg)](https://crates.io/crates/naija-geo)
[![Docs.rs](https://docs.rs/naija-geo/badge.svg)](https://docs.rs/naija-geo)
[![CI](https://github.com/mubarakcoded/naija-geo/actions/workflows/ci.yml/badge.svg)](https://github.com/mubarakcoded/naija-geo/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Nigeria's 6 geopolitical zones, 37 states and all 774 LGAs — as a zero-overhead Rust crate.

All data is compiled into your binary as `&'static` slices. No files, no network, no allocations on lookup.

## Usage

```toml
[dependencies]
naija-geo = "0.1"
```

```rust
use naija_geo::{Zone, State, Lga};

// zones → states → LGAs
let zone = Zone::find("SW").unwrap();
println!("{} has {} states and {} LGAs", zone.name, zone.state_count(), zone.lga_count());

let lagos = State::find("LA").unwrap();
for lga in lagos.lgas() {
    println!("{} — {}", lga.code, lga.name);
}

// LGA → state → zone (upward traversal)
let lga = Lga::find("LA020").unwrap();
println!("{} → {} → {}", lga.name, lga.state().unwrap().name, lga.zone().unwrap().name);
// Surulere → Lagos → South West

// some LGA names exist in multiple states
let all = Lga::find_all_by_name("Surulere"); // Lagos + Oyo
```

All lookups are case-insensitive. `find()` returns `Option`, `get()` returns `Result` for use with `?`.

## Features

| Feature | Adds |
|---------|------|
| `serde` | `Serialize` on `Zone`, `State`, `Lga` |
| `fuzzy` | `find_fuzzy(name)` on all types (Jaro-Winkler) |

```toml
naija-geo = { version = "0.1", features = ["serde", "fuzzy"] }
```

## Data

| Zone | Code | States | LGAs |
|------|------|-------:|-----:|
| North Central | `NC` | 7 | 121 |
| North East    | `NE` | 6 | 112 |
| North West    | `NW` | 7 | 186 |
| South East    | `SE` | 5 | 95  |
| South South   | `SS` | 6 | 123 |
| South West    | `SW` | 6 | 137 |

State codes, capitals and full LGA lists are in the [docs](https://docs.rs/naija-geo).

Data sourced from INEC, NBS and NPC official publications. If you spot an error, please open an issue with a source citation.

## License

MIT
