use crate::zone::Zone;

pub static ZONES: &[Zone] = &[
    Zone {
        code: "NC",
        name: "North Central",
        state_codes: &["BE", "FC", "KO", "KW", "NA", "NI", "PL"],
    },
    Zone {
        code: "NE",
        name: "North East",
        state_codes: &["AD", "BA", "BO", "GO", "TA", "YO"],
    },
    Zone {
        code: "NW",
        name: "North West",
        state_codes: &["JI", "KD", "KN", "KT", "KB", "SO", "ZA"],
    },
    Zone {
        code: "SE",
        name: "South East",
        state_codes: &["AB", "AN", "EB", "EN", "IM"],
    },
    Zone {
        code: "SS",
        name: "South South",
        state_codes: &["AK", "BY", "CR", "DE", "ED", "RI"],
    },
    Zone {
        code: "SW",
        name: "South West",
        state_codes: &["EK", "LA", "OG", "ON", "OS", "OY"],
    },
];
