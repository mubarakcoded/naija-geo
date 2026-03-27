use naija_geo::{NaijaGeoError, State, Zone};

#[test]
fn all_returns_37_states() {
    assert_eq!(State::all().len(), 37);
}

#[test]
fn find_by_code_case_insensitive() {
    assert_eq!(State::find("LA").unwrap().name, "Lagos");
    assert_eq!(State::find("la").unwrap().name, "Lagos");
}

#[test]
fn find_by_name_case_insensitive() {
    assert_eq!(State::find_by_name("lagos").unwrap().code, "LA");
    assert_eq!(State::find_by_name("LAGOS").unwrap().code, "LA");
    assert_eq!(State::find_by_name("Rivers").unwrap().code, "RI");
    assert_eq!(State::find_by_name("FCT").unwrap().code, "FC");
}

#[test]
fn find_unknown_returns_none() {
    assert!(State::find("ZZ").is_none());
    assert!(State::find_by_name("Wakanda").is_none());
}

#[test]
fn get_returns_error_for_bad_code() {
    assert_eq!(
        State::get("ZZ").unwrap_err(),
        NaijaGeoError::StateNotFound("ZZ".to_string())
    );
}

#[test]
fn state_capitals() {
    assert_eq!(State::find("LA").unwrap().capital, "Ikeja");
    assert_eq!(State::find("RI").unwrap().capital, "Port Harcourt");
    assert_eq!(State::find("FC").unwrap().capital, "Abuja");
    assert_eq!(State::find("KN").unwrap().capital, "Kano");
}

#[test]
fn state_zone_codes() {
    assert_eq!(State::find("LA").unwrap().zone_code, "SW");
    assert_eq!(State::find("KN").unwrap().zone_code, "NW");
    assert_eq!(State::find("EN").unwrap().zone_code, "SE");
    assert_eq!(State::find("FC").unwrap().zone_code, "NC");
}

#[test]
fn state_lga_counts() {
    assert_eq!(State::find("LA").unwrap().lga_count(), 20);
    assert_eq!(State::find("KN").unwrap().lga_count(), 44);
    assert_eq!(State::find("BY").unwrap().lga_count(), 8);
    assert_eq!(State::find("FC").unwrap().lga_count(), 6);
    assert_eq!(State::find("AK").unwrap().lga_count(), 31);
}

#[test]
fn by_zone_returns_correct_states() {
    assert_eq!(State::by_zone("SW").len(), 6);
    assert_eq!(State::by_zone("NC").len(), 7);
    assert_eq!(State::by_zone("SE").len(), 5);
    assert_eq!(State::by_zone("sw").len(), 6); // case-insensitive
}

#[test]
fn state_zone_traversal() {
    let lagos = State::find("LA").unwrap();
    let zone = lagos.zone().unwrap();
    assert_eq!(zone.code, "SW");
    assert_eq!(zone.name, "South West");
}

#[test]
fn state_lgas_traversal() {
    let lgas = State::find("LA").unwrap().lgas();
    assert_eq!(lgas.len(), 20);
    let names: Vec<&str> = lgas.iter().map(|l| l.name).collect();
    assert!(names.contains(&"Agege"));
    assert!(names.contains(&"Surulere"));
    assert!(names.contains(&"Ikeja"));
}

#[test]
fn total_states_per_zone_sum_to_37() {
    let total: usize = Zone::all().iter().map(|z| z.state_count()).sum();
    assert_eq!(total, 37);
}

#[test]
fn state_display() {
    let s = State::find("LA").unwrap();
    assert_eq!(format!("{s}"), "Lagos (LA)");
}
