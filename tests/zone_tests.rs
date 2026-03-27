use naija_geo::{NaijaGeoError, Zone};

#[test]
fn all_returns_six_zones() {
    assert_eq!(Zone::all().len(), 6);
}

#[test]
fn find_by_code_case_insensitive() {
    assert!(Zone::find("SW").is_some());
    assert!(Zone::find("sw").is_some());
    assert!(Zone::find("Sw").is_some());
}

#[test]
fn find_unknown_code_returns_none() {
    assert!(Zone::find("XX").is_none());
}

#[test]
fn find_by_name_case_insensitive() {
    assert_eq!(Zone::find_by_name("South West").unwrap().code, "SW");
    assert_eq!(Zone::find_by_name("south west").unwrap().code, "SW");
    assert_eq!(Zone::find_by_name("NORTH CENTRAL").unwrap().code, "NC");
}

#[test]
fn get_returns_error_for_bad_code() {
    assert_eq!(
        Zone::get("ZZ").unwrap_err(),
        NaijaGeoError::ZoneNotFound("ZZ".to_string())
    );
}

#[test]
fn zone_state_counts() {
    assert_eq!(Zone::find("NC").unwrap().state_count(), 7); // NC has 7 members (inc FCT)
    assert_eq!(Zone::find("SE").unwrap().state_count(), 5);
    assert_eq!(Zone::find("SW").unwrap().state_count(), 6);
    assert_eq!(Zone::find("SS").unwrap().state_count(), 6);
    assert_eq!(Zone::find("NE").unwrap().state_count(), 6);
    assert_eq!(Zone::find("NW").unwrap().state_count(), 7);
}

#[test]
fn zone_states_traversal() {
    let sw = Zone::find("SW").unwrap();
    let names: Vec<&str> = sw.states().iter().map(|s| s.name).collect();
    assert!(names.contains(&"Lagos"));
    assert!(names.contains(&"Oyo"));
    assert!(names.contains(&"Ekiti"));
}

#[test]
fn zone_lga_count_sw() {
    // SW: Ekiti(16) + Lagos(20) + Ogun(20) + Ondo(18) + Osun(30) + Oyo(33) = 137
    assert_eq!(Zone::find("SW").unwrap().lga_count(), 137);
}

#[test]
fn total_lgas_across_all_zones_is_774() {
    let total: usize = Zone::all().iter().map(|z| z.lga_count()).sum();
    assert_eq!(total, 774);
}

#[test]
fn zone_display() {
    let z = Zone::find("NC").unwrap();
    assert_eq!(format!("{z}"), "North Central (NC)");
}
