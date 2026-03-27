use crate::state::State;

pub static STATES: &[State] = &[
    // ── South East ───────────────────────────────────────────────────────────
    State {
        code: "AB", name: "Abia", capital: "Umuahia", zone_code: "SE",
        lga_codes: &[
            "AB001","AB002","AB003","AB004","AB005","AB006","AB007","AB008",
            "AB009","AB010","AB011","AB012","AB013","AB014","AB015","AB016","AB017",
        ],
    },
    State {
        code: "AN", name: "Anambra", capital: "Awka", zone_code: "SE",
        lga_codes: &[
            "AN001","AN002","AN003","AN004","AN005","AN006","AN007","AN008",
            "AN009","AN010","AN011","AN012","AN013","AN014","AN015","AN016",
            "AN017","AN018","AN019","AN020","AN021",
        ],
    },
    State {
        code: "EB", name: "Ebonyi", capital: "Abakaliki", zone_code: "SE",
        lga_codes: &[
            "EB001","EB002","EB003","EB004","EB005","EB006","EB007","EB008",
            "EB009","EB010","EB011","EB012","EB013",
        ],
    },
    State {
        code: "EN", name: "Enugu", capital: "Enugu", zone_code: "SE",
        lga_codes: &[
            "EN001","EN002","EN003","EN004","EN005","EN006","EN007","EN008",
            "EN009","EN010","EN011","EN012","EN013","EN014","EN015","EN016","EN017",
        ],
    },
    State {
        code: "IM", name: "Imo", capital: "Owerri", zone_code: "SE",
        lga_codes: &[
            "IM001","IM002","IM003","IM004","IM005","IM006","IM007","IM008",
            "IM009","IM010","IM011","IM012","IM013","IM014","IM015","IM016",
            "IM017","IM018","IM019","IM020","IM021","IM022","IM023","IM024",
            "IM025","IM026","IM027",
        ],
    },

    // ── South South ──────────────────────────────────────────────────────────
    State {
        code: "AK", name: "Akwa Ibom", capital: "Uyo", zone_code: "SS",
        lga_codes: &[
            "AK001","AK002","AK003","AK004","AK005","AK006","AK007","AK008",
            "AK009","AK010","AK011","AK012","AK013","AK014","AK015","AK016",
            "AK017","AK018","AK019","AK020","AK021","AK022","AK023","AK024",
            "AK025","AK026","AK027","AK028","AK029","AK030","AK031",
        ],
    },
    State {
        code: "BY", name: "Bayelsa", capital: "Yenagoa", zone_code: "SS",
        lga_codes: &[
            "BY001","BY002","BY003","BY004","BY005","BY006","BY007","BY008",
        ],
    },
    State {
        code: "CR", name: "Cross River", capital: "Calabar", zone_code: "SS",
        lga_codes: &[
            "CR001","CR002","CR003","CR004","CR005","CR006","CR007","CR008",
            "CR009","CR010","CR011","CR012","CR013","CR014","CR015","CR016",
            "CR017","CR018",
        ],
    },
    State {
        code: "DE", name: "Delta", capital: "Asaba", zone_code: "SS",
        lga_codes: &[
            "DE001","DE002","DE003","DE004","DE005","DE006","DE007","DE008",
            "DE009","DE010","DE011","DE012","DE013","DE014","DE015","DE016",
            "DE017","DE018","DE019","DE020","DE021","DE022","DE023","DE024","DE025",
        ],
    },
    State {
        code: "ED", name: "Edo", capital: "Benin City", zone_code: "SS",
        lga_codes: &[
            "ED001","ED002","ED003","ED004","ED005","ED006","ED007","ED008",
            "ED009","ED010","ED011","ED012","ED013","ED014","ED015","ED016",
            "ED017","ED018",
        ],
    },
    State {
        code: "RI", name: "Rivers", capital: "Port Harcourt", zone_code: "SS",
        lga_codes: &[
            "RI001","RI002","RI003","RI004","RI005","RI006","RI007","RI008",
            "RI009","RI010","RI011","RI012","RI013","RI014","RI015","RI016",
            "RI017","RI018","RI019","RI020","RI021","RI022","RI023",
        ],
    },

    // ── South West ───────────────────────────────────────────────────────────
    State {
        code: "EK", name: "Ekiti", capital: "Ado-Ekiti", zone_code: "SW",
        lga_codes: &[
            "EK001","EK002","EK003","EK004","EK005","EK006","EK007","EK008",
            "EK009","EK010","EK011","EK012","EK013","EK014","EK015","EK016",
        ],
    },
    State {
        code: "LA", name: "Lagos", capital: "Ikeja", zone_code: "SW",
        lga_codes: &[
            "LA001","LA002","LA003","LA004","LA005","LA006","LA007","LA008",
            "LA009","LA010","LA011","LA012","LA013","LA014","LA015","LA016",
            "LA017","LA018","LA019","LA020",
        ],
    },
    State {
        code: "OG", name: "Ogun", capital: "Abeokuta", zone_code: "SW",
        lga_codes: &[
            "OG001","OG002","OG003","OG004","OG005","OG006","OG007","OG008",
            "OG009","OG010","OG011","OG012","OG013","OG014","OG015","OG016",
            "OG017","OG018","OG019","OG020",
        ],
    },
    State {
        code: "ON", name: "Ondo", capital: "Akure", zone_code: "SW",
        lga_codes: &[
            "ON001","ON002","ON003","ON004","ON005","ON006","ON007","ON008",
            "ON009","ON010","ON011","ON012","ON013","ON014","ON015","ON016",
            "ON017","ON018",
        ],
    },
    State {
        code: "OS", name: "Osun", capital: "Osogbo", zone_code: "SW",
        lga_codes: &[
            "OS001","OS002","OS003","OS004","OS005","OS006","OS007","OS008",
            "OS009","OS010","OS011","OS012","OS013","OS014","OS015","OS016",
            "OS017","OS018","OS019","OS020","OS021","OS022","OS023","OS024",
            "OS025","OS026","OS027","OS028","OS029","OS030",
        ],
    },
    State {
        code: "OY", name: "Oyo", capital: "Ibadan", zone_code: "SW",
        lga_codes: &[
            "OY001","OY002","OY003","OY004","OY005","OY006","OY007","OY008",
            "OY009","OY010","OY011","OY012","OY013","OY014","OY015","OY016",
            "OY017","OY018","OY019","OY020","OY021","OY022","OY023","OY024",
            "OY025","OY026","OY027","OY028","OY029","OY030","OY031","OY032","OY033",
        ],
    },

    // ── North Central ────────────────────────────────────────────────────────
    State {
        code: "BE", name: "Benue", capital: "Makurdi", zone_code: "NC",
        lga_codes: &[
            "BE001","BE002","BE003","BE004","BE005","BE006","BE007","BE008",
            "BE009","BE010","BE011","BE012","BE013","BE014","BE015","BE016",
            "BE017","BE018","BE019","BE020","BE021","BE022","BE023",
        ],
    },
    State {
        code: "FC", name: "FCT", capital: "Abuja", zone_code: "NC",
        lga_codes: &[
            "FC001","FC002","FC003","FC004","FC005","FC006",
        ],
    },
    State {
        code: "KO", name: "Kogi", capital: "Lokoja", zone_code: "NC",
        lga_codes: &[
            "KO001","KO002","KO003","KO004","KO005","KO006","KO007","KO008",
            "KO009","KO010","KO011","KO012","KO013","KO014","KO015","KO016",
            "KO017","KO018","KO019","KO020","KO021",
        ],
    },
    State {
        code: "KW", name: "Kwara", capital: "Ilorin", zone_code: "NC",
        lga_codes: &[
            "KW001","KW002","KW003","KW004","KW005","KW006","KW007","KW008",
            "KW009","KW010","KW011","KW012","KW013","KW014","KW015","KW016",
        ],
    },
    State {
        code: "NA", name: "Nasarawa", capital: "Lafia", zone_code: "NC",
        lga_codes: &[
            "NA001","NA002","NA003","NA004","NA005","NA006","NA007","NA008",
            "NA009","NA010","NA011","NA012","NA013",
        ],
    },
    State {
        code: "NI", name: "Niger", capital: "Minna", zone_code: "NC",
        lga_codes: &[
            "NI001","NI002","NI003","NI004","NI005","NI006","NI007","NI008",
            "NI009","NI010","NI011","NI012","NI013","NI014","NI015","NI016",
            "NI017","NI018","NI019","NI020","NI021","NI022","NI023","NI024","NI025",
        ],
    },
    State {
        code: "PL", name: "Plateau", capital: "Jos", zone_code: "NC",
        lga_codes: &[
            "PL001","PL002","PL003","PL004","PL005","PL006","PL007","PL008",
            "PL009","PL010","PL011","PL012","PL013","PL014","PL015","PL016","PL017",
        ],
    },

    // ── North East ───────────────────────────────────────────────────────────
    State {
        code: "AD", name: "Adamawa", capital: "Yola", zone_code: "NE",
        lga_codes: &[
            "AD001","AD002","AD003","AD004","AD005","AD006","AD007","AD008",
            "AD009","AD010","AD011","AD012","AD013","AD014","AD015","AD016",
            "AD017","AD018","AD019","AD020","AD021",
        ],
    },
    State {
        code: "BA", name: "Bauchi", capital: "Bauchi", zone_code: "NE",
        lga_codes: &[
            "BA001","BA002","BA003","BA004","BA005","BA006","BA007","BA008",
            "BA009","BA010","BA011","BA012","BA013","BA014","BA015","BA016",
            "BA017","BA018","BA019","BA020",
        ],
    },
    State {
        code: "BO", name: "Borno", capital: "Maiduguri", zone_code: "NE",
        lga_codes: &[
            "BO001","BO002","BO003","BO004","BO005","BO006","BO007","BO008",
            "BO009","BO010","BO011","BO012","BO013","BO014","BO015","BO016",
            "BO017","BO018","BO019","BO020","BO021","BO022","BO023","BO024",
            "BO025","BO026","BO027",
        ],
    },
    State {
        code: "GO", name: "Gombe", capital: "Gombe", zone_code: "NE",
        lga_codes: &[
            "GO001","GO002","GO003","GO004","GO005","GO006","GO007","GO008",
            "GO009","GO010","GO011",
        ],
    },
    State {
        code: "TA", name: "Taraba", capital: "Jalingo", zone_code: "NE",
        lga_codes: &[
            "TA001","TA002","TA003","TA004","TA005","TA006","TA007","TA008",
            "TA009","TA010","TA011","TA012","TA013","TA014","TA015","TA016",
        ],
    },
    State {
        code: "YO", name: "Yobe", capital: "Damaturu", zone_code: "NE",
        lga_codes: &[
            "YO001","YO002","YO003","YO004","YO005","YO006","YO007","YO008",
            "YO009","YO010","YO011","YO012","YO013","YO014","YO015","YO016","YO017",
        ],
    },

    // ── North West ───────────────────────────────────────────────────────────
    State {
        code: "JI", name: "Jigawa", capital: "Dutse", zone_code: "NW",
        lga_codes: &[
            "JI001","JI002","JI003","JI004","JI005","JI006","JI007","JI008",
            "JI009","JI010","JI011","JI012","JI013","JI014","JI015","JI016",
            "JI017","JI018","JI019","JI020","JI021","JI022","JI023","JI024",
            "JI025","JI026","JI027",
        ],
    },
    State {
        code: "KD", name: "Kaduna", capital: "Kaduna", zone_code: "NW",
        lga_codes: &[
            "KD001","KD002","KD003","KD004","KD005","KD006","KD007","KD008",
            "KD009","KD010","KD011","KD012","KD013","KD014","KD015","KD016",
            "KD017","KD018","KD019","KD020","KD021","KD022","KD023",
        ],
    },
    State {
        code: "KN", name: "Kano", capital: "Kano", zone_code: "NW",
        lga_codes: &[
            "KN001","KN002","KN003","KN004","KN005","KN006","KN007","KN008",
            "KN009","KN010","KN011","KN012","KN013","KN014","KN015","KN016",
            "KN017","KN018","KN019","KN020","KN021","KN022","KN023","KN024",
            "KN025","KN026","KN027","KN028","KN029","KN030","KN031","KN032",
            "KN033","KN034","KN035","KN036","KN037","KN038","KN039","KN040",
            "KN041","KN042","KN043","KN044",
        ],
    },
    State {
        code: "KT", name: "Katsina", capital: "Katsina", zone_code: "NW",
        lga_codes: &[
            "KT001","KT002","KT003","KT004","KT005","KT006","KT007","KT008",
            "KT009","KT010","KT011","KT012","KT013","KT014","KT015","KT016",
            "KT017","KT018","KT019","KT020","KT021","KT022","KT023","KT024",
            "KT025","KT026","KT027","KT028","KT029","KT030","KT031","KT032",
            "KT033","KT034",
        ],
    },
    State {
        code: "KB", name: "Kebbi", capital: "Birnin Kebbi", zone_code: "NW",
        lga_codes: &[
            "KB001","KB002","KB003","KB004","KB005","KB006","KB007","KB008",
            "KB009","KB010","KB011","KB012","KB013","KB014","KB015","KB016",
            "KB017","KB018","KB019","KB020","KB021",
        ],
    },
    State {
        code: "SO", name: "Sokoto", capital: "Sokoto", zone_code: "NW",
        lga_codes: &[
            "SO001","SO002","SO003","SO004","SO005","SO006","SO007","SO008",
            "SO009","SO010","SO011","SO012","SO013","SO014","SO015","SO016",
            "SO017","SO018","SO019","SO020","SO021","SO022","SO023",
        ],
    },
    State {
        code: "ZA", name: "Zamfara", capital: "Gusau", zone_code: "NW",
        lga_codes: &[
            "ZA001","ZA002","ZA003","ZA004","ZA005","ZA006","ZA007","ZA008",
            "ZA009","ZA010","ZA011","ZA012","ZA013","ZA014",
        ],
    },
];
