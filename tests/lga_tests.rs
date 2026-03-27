use naija_geo::{Lga, NaijaGeoError};

#[test]
fn all_returns_774_lgas() {
    assert_eq!(Lga::all().len(), 774);
}

#[test]
fn find_by_code_case_insensitive() {
    assert_eq!(Lga::find("LA001").unwrap().name, "Agege");
    assert_eq!(Lga::find("la001").unwrap().name, "Agege");
    assert_eq!(Lga::find("La001").unwrap().name, "Agege");
}

#[test]
fn find_unknown_code_returns_none() {
    assert!(Lga::find("XX999").is_none());
}

#[test]
fn get_returns_error_for_bad_code() {
    assert_eq!(
        Lga::get("XX001").unwrap_err(),
        NaijaGeoError::LgaNotFound("XX001".to_string())
    );
}

#[test]
fn find_by_name_case_insensitive() {
    assert_eq!(Lga::find_by_name("Surulere").unwrap().state_code, "LA");
    assert_eq!(Lga::find_by_name("surulere").unwrap().state_code, "LA");
    assert_eq!(Lga::find_by_name("Port Harcourt").unwrap().state_code, "RI");
    assert_eq!(Lga::find_by_name("Maiduguri").unwrap().state_code, "BO");
}

#[test]
fn find_all_by_name_handles_duplicates() {
    // "Surulere" exists in Lagos (LA) and Oyo (OY)
    let matches = Lga::find_all_by_name("Surulere");
    assert_eq!(matches.len(), 2);
    let state_codes: Vec<&str> = matches.iter().map(|l| l.state_code).collect();
    assert!(state_codes.contains(&"LA"));
    assert!(state_codes.contains(&"OY"));
}

#[test]
fn by_state_returns_correct_lgas() {
    assert_eq!(Lga::by_state("LA").len(), 20);
    assert_eq!(Lga::by_state("KN").len(), 44);
    assert_eq!(Lga::by_state("BY").len(), 8);
    assert_eq!(Lga::by_state("FC").len(), 6);
    assert_eq!(Lga::by_state("la").len(), 20); // case-insensitive
}

#[test]
fn by_zone_aggregates_correctly() {
    // SW: Ekiti(16)+Lagos(20)+Ogun(20)+Ondo(18)+Osun(30)+Oyo(33) = 137
    assert_eq!(Lga::by_zone("SW").len(), 137);
    assert_eq!(Lga::by_zone("sw").len(), 137); // case-insensitive
}

#[test]
fn lga_state_traversal() {
    let lga = Lga::find("LA020").unwrap(); // Surulere
    let state = lga.state().unwrap();
    assert_eq!(state.code, "LA");
    assert_eq!(state.name, "Lagos");
}

#[test]
fn lga_zone_traversal() {
    let lga = Lga::find("LA020").unwrap(); // Surulere
    let zone = lga.zone().unwrap();
    assert_eq!(zone.code, "SW");
    assert_eq!(zone.name, "South West");
}

#[test]
fn specific_lga_names() {
    assert_eq!(Lga::find("AB001").unwrap().name, "Aba North");
    assert_eq!(Lga::find("KN021").unwrap().name, "Kano Municipal");
    assert_eq!(Lga::find("FC006").unwrap().name, "Municipal Area Council");
    assert_eq!(Lga::find("BO006").unwrap().name, "Chibok");
    assert_eq!(Lga::find("ZA014").unwrap().name, "Zurmi");
}

#[test]
fn all_lga_codes_unique() {
    let mut codes: Vec<&str> = Lga::all().iter().map(|l| l.code).collect();
    let original_len = codes.len();
    codes.sort_unstable();
    codes.dedup();
    assert_eq!(codes.len(), original_len, "duplicate LGA codes found");
}

#[test]
fn all_lga_state_codes_valid() {
    use naija_geo::State;
    for lga in Lga::all() {
        assert!(
            State::find(lga.state_code).is_some(),
            "LGA {} has invalid state_code '{}'",
            lga.code,
            lga.state_code
        );
    }
}

#[test]
fn lga_display() {
    let l = Lga::find("LA001").unwrap();
    assert_eq!(format!("{l}"), "Agege (LA001)");
}
