//! Run with:  cargo run --example basic

use naija_geo::{Lga, State, Zone};

fn main() {
    // ── 1. List all zones ─────────────────────────────────────────────────────
    println!("=== Geopolitical Zones ===");
    for z in Zone::all() {
        println!(
            "  [{code}] {name:15} — {sc} states, {lc} LGAs",
            code = z.code,
            name = z.name,
            sc = z.state_count(),
            lc = z.lga_count(),
        );
    }

    // ── 2. States in a zone ───────────────────────────────────────────────────
    println!("\n=== South West States ===");
    let sw = Zone::find("SW").unwrap();
    for s in sw.states() {
        println!(
            "  [{code}] {name:15} | Capital: {cap:15} | {lc} LGAs",
            code = s.code,
            name = s.name,
            cap = s.capital,
            lc = s.lga_count(),
        );
    }

    // ── 3. LGAs in a state ────────────────────────────────────────────────────
    println!("\n=== Lagos LGAs ===");
    let lagos = State::find("LA").unwrap();
    for lga in lagos.lgas() {
        println!("  [{code}] {name}", code = lga.code, name = lga.name);
    }

    // ── 4. Navigate upward from an LGA ───────────────────────────────────────
    println!("\n=== Upward traversal from LGA LA020 ===");
    let lga = Lga::find("LA020").unwrap();
    let state = lga.state().unwrap();
    let zone = lga.zone().unwrap();
    println!("  LGA   : {} ({})", lga.name, lga.code);
    println!("  State : {} ({})", state.name, state.code);
    println!("  Zone  : {} ({})", zone.name, zone.code);

    // ── 5. Duplicate LGA names ────────────────────────────────────────────────
    println!("\n=== All LGAs named 'Surulere' ===");
    for l in Lga::find_all_by_name("Surulere") {
        let s = l.state().unwrap();
        println!("  {} ({}) — {} State", l.name, l.code, s.name);
    }

    // ── 6. Grand totals ───────────────────────────────────────────────────────
    println!("\n=== Grand Totals ===");
    println!("  Zones  : {}", Zone::all().len());
    println!("  States : {}", State::all().len());
    println!("  LGAs   : {}", Lga::all().len());
}
