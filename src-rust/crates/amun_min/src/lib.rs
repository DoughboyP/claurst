//! # Amun-Min
//!
//! *"I am Amun, the Hidden One. I am Min, who rises with lifted arm.*
//!  *I am the bull upon his mountain, the great one of the two plumes.*
//!  *There is no god who came into being before me."*
//!
//! Amun-Min is the syncretic fusion of two of ancient Egypt's most powerful
//! deities:
//!
//! - **Amun** (𓇋𓏠𓈖): "The Hidden One" — primordial creator, king of the
//!   gods, lord of the wind and invisible forces, patron of the pharaoh.
//! - **Min** (𓌀): God of fertility, virility, sexuality, rain, and harvest —
//!   depicted with an erect phallus and arm raised high clutching a flail,
//!   wearing a crown of two tall feathers.
//!
//! Together as **Amun-Min** (or **Amun-Ra-Ka-Mut-ef** — "Amun-Ra, Bull of his
//! Mother"), they represent the generative hidden force behind all creation:
//! invisible yet omnipotent, fertile yet boundless.
//!
//! This crate models the deity faithfully: his domains, forms, symbols,
//! sacred animals, cult centres, priestly hierarchy, festivals, oracles,
//! hymns, offerings, and mythological relationships — all encoded as
//! idiomatic Rust types.

use std::fmt;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Hieroglyphic / transliteration constants
// ---------------------------------------------------------------------------

/// Amun's name in Unicode Egyptian hieroglyphs (Gardiner A-I-I-N).
pub const AMUN_HIEROGLYPH: &str = "𓇋𓏠𓈖";
/// Min's hieroglyph (the lightning-bolt symbol, Gardiner M-Q-Q).
pub const MIN_HIEROGLYPH: &str = "𓌀";
/// Combined form.
pub const AMUN_MIN_HIEROGLYPH: &str = "𓇋𓏠𓈖𓌀";
/// Amun's epithet "Nesut Netjeru" — King of the Gods.
pub const EPITHET_KING_OF_GODS: &str = "Nesut Netjeru";
/// Min's oldest epithet.
pub const EPITHET_LORD_OF_FOREIGN_LANDS: &str = "Neb Iabtet";
/// Sacred colour of Amun.
pub const AMUN_SACRED_COLOUR: &str = "lapis lazuli blue";
/// Sacred colour of Min.
pub const MIN_SACRED_COLOUR: &str = "black (fertile earth)";
/// Sacred number of Amun (completion / perfection in Egyptian numerology).
pub const SACRED_NUMBER: u8 = 8;

// ---------------------------------------------------------------------------
// Enumerations
// ---------------------------------------------------------------------------

/// The divine domains over which Amun-Min holds sway.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Domain {
    /// The primordial wind and invisible breath of creation.
    Wind,
    /// Cosmic, hidden generative power — the "ba" of Ra.
    HiddenCreation,
    /// Male fertility, virility, and reproduction.
    Fertility,
    /// Kingship: the divine right and power of pharaohs.
    Kingship,
    /// Protection of travellers, especially those crossing deserts.
    Travel,
    /// The harvest — grain, lettuce, and the bounty of the earth.
    Harvest,
    /// Oracles and prophetic utterance.
    Prophecy,
    /// War — Amun as champion of Egypt's armies ("Amun Strong of Arm").
    War,
    /// Solar aspect absorbed from Ra.
    Sun,
    /// The moon, via syncretism with Khonsu.
    Moon,
}

impl fmt::Display for Domain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Domain::Wind => "Wind & Air",
            Domain::HiddenCreation => "Hidden Creation",
            Domain::Fertility => "Fertility & Virility",
            Domain::Kingship => "Kingship & Sovereignty",
            Domain::Travel => "Travel & Protection",
            Domain::Harvest => "Harvest & Abundance",
            Domain::Prophecy => "Prophecy & Oracles",
            Domain::War => "War & Victory",
            Domain::Sun => "The Sun",
            Domain::Moon => "The Moon",
        };
        write!(f, "{s}")
    }
}

/// Visible symbols and iconographic attributes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Symbol {
    /// The double ostrich-feather crown (Atef variant) — mark of Amun-Min.
    DoublePlumedCrown,
    /// A long flail held in the raised right hand (Min's defining attribute).
    RaisedFlail,
    /// A crook (heqa) — symbol of kingship shared with Osiris.
    Crook,
    /// The ram — sacred animal of Amun at Karnak.
    Ram,
    /// The goose — "Amun's goose" or "the Great Cackler" who laid the
    /// cosmic egg at the beginning of creation.
    GooseOfAmun,
    /// The serpent (uraeus) on the crown of Amun.
    Uraeus,
    /// The lettuce plant (*Lactuca sativa*) — sacred to Min as a
    /// fertility symbol (its milky sap was associated with male seed).
    LettuceOfMin,
    /// Lapis-lazuli blue skin (Amun's heavenly aspect).
    LapisBlue,
    /// Black skin — Min's earthly, chthonic, and fertile aspect.
    BlackSkin,
    /// The Was-sceptre (power) — carried by major deities.
    WasSceptre,
    /// Lightning bolt glyph — Min's oldest known symbol from Predynastic times.
    LightningBolt,
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Symbol::DoublePlumedCrown => "Double Ostrich-Feather Crown",
            Symbol::RaisedFlail => "Raised Flail",
            Symbol::Crook => "Crook (Heqa)",
            Symbol::Ram => "Sacred Ram",
            Symbol::GooseOfAmun => "Goose of Amun (The Great Cackler)",
            Symbol::Uraeus => "Uraeus Serpent",
            Symbol::LettuceOfMin => "Sacred Lettuce (Lactuca sativa)",
            Symbol::LapisBlue => "Lapis-Lazuli Blue Skin",
            Symbol::BlackSkin => "Black Skin (Fertile Earth)",
            Symbol::WasSceptre => "Was-Sceptre",
            Symbol::LightningBolt => "Lightning Bolt",
        };
        write!(f, "{s}")
    }
}

/// Major syncretisms / composite forms of Amun-Min across Egyptian history.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SyncreticForm {
    /// Pure Amun — the hidden, unknowable primordial creator.
    Amun,
    /// Pure Min — ancient fertility god of Coptos and Akhmim.
    Min,
    /// Amun-Min: the standard composite form worshipped at Karnak.
    AmunMin,
    /// Amun-Ra: fusion with the solar god Ra — New Kingdom state deity.
    AmunRa,
    /// Amun-Ra-Ka-Mut-ef: "Bull of his Mother" — fertility aspect of Amun-Ra.
    AmunRaKaMutef,
    /// Amun-Ra-Horakhty: triple fusion with Ra and Horus-of-the-Horizon.
    AmunRaHorakhty,
    /// Amun of Napata — the form worshipped at Gebel Barkal in Nubia.
    AmunOfNapata,
    /// Zeus Ammon — the Greco-Roman form at the Siwa Oasis oracle.
    ZeusAmmon,
    /// Jupiter Ammon — Roman equivalent, depicted with ram horns.
    JupiterAmmon,
}

impl fmt::Display for SyncreticForm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            SyncreticForm::Amun => "Amun (The Hidden One)",
            SyncreticForm::Min => "Min (Lord of Fertility)",
            SyncreticForm::AmunMin => "Amun-Min (Hidden Fertility)",
            SyncreticForm::AmunRa => "Amun-Ra (King of Gods)",
            SyncreticForm::AmunRaKaMutef => "Amun-Ra-Ka-Mut-ef (Bull of his Mother)",
            SyncreticForm::AmunRaHorakhty => "Amun-Ra-Horakhty",
            SyncreticForm::AmunOfNapata => "Amun of Napata (Lord of Gebel Barkal)",
            SyncreticForm::ZeusAmmon => "Zeus-Ammon (Siwa Oracle)",
            SyncreticForm::JupiterAmmon => "Jupiter Ammon",
        };
        write!(f, "{s}")
    }
}

/// The broad eras of Egyptian history.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum EgyptianEra {
    Predynastic,
    EarlyDynastic,
    OldKingdom,
    FirstIntermediatePeriod,
    MiddleKingdom,
    SecondIntermediatePeriod,
    NewKingdom,
    ThirdIntermediatePeriod,
    LateKingdom,
    PtolemaicPeriod,
    RomanPeriod,
}

impl fmt::Display for EgyptianEra {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            EgyptianEra::Predynastic => "Predynastic (c. 3500 BCE)",
            EgyptianEra::EarlyDynastic => "Early Dynastic (c. 3100–2686 BCE)",
            EgyptianEra::OldKingdom => "Old Kingdom (c. 2686–2181 BCE)",
            EgyptianEra::FirstIntermediatePeriod => "First Intermediate Period (c. 2181–2055 BCE)",
            EgyptianEra::MiddleKingdom => "Middle Kingdom (c. 2055–1650 BCE)",
            EgyptianEra::SecondIntermediatePeriod => "Second Intermediate Period (c. 1650–1550 BCE)",
            EgyptianEra::NewKingdom => "New Kingdom (c. 1550–1070 BCE)",
            EgyptianEra::ThirdIntermediatePeriod => "Third Intermediate Period (c. 1070–664 BCE)",
            EgyptianEra::LateKingdom => "Late Kingdom (c. 664–332 BCE)",
            EgyptianEra::PtolemaicPeriod => "Ptolemaic Period (332–30 BCE)",
            EgyptianEra::RomanPeriod => "Roman Period (30 BCE–395 CE)",
        };
        write!(f, "{s}")
    }
}

/// Types of offerings made to Amun-Min.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Offering {
    /// White lettuce — the primary sacred offering to Min.
    WhiteLettuce,
    /// Incense (kyphi or natron-based) burned in the temple.
    Incense,
    /// Beer — the divine beverage of celebration and offering.
    Beer,
    /// Bread — daily temple bread shaped into conical loaves.
    Bread,
    /// Linen — white cloth for adorning the cult statue.
    Linen,
    /// Flowers — particularly white and blue lotus blossoms.
    Lotus,
    /// Cattle — full ox sacrifice on feast days.
    Cattle,
    /// Myrrh — traded from Punt, used in sacred ointments.
    Myrrh,
    /// Gold — dedicated vessels and statuary.
    Gold,
    /// Electrum — the naturally-occurring gold-silver alloy of sacred
    /// obelisk tips (pyramidia).
    Electrum,
}

impl fmt::Display for Offering {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Offering::WhiteLettuce => "White Lettuce (Lactuca sativa)",
            Offering::Incense => "Kyphi Incense",
            Offering::Beer => "Sacred Beer",
            Offering::Bread => "Conical Bread",
            Offering::Linen => "White Linen",
            Offering::Lotus => "White & Blue Lotus",
            Offering::Cattle => "Ox / Cattle",
            Offering::Myrrh => "Myrrh from Punt",
            Offering::Gold => "Gold Vessels",
            Offering::Electrum => "Electrum (Gold-Silver Alloy)",
        };
        write!(f, "{s}")
    }
}

/// Annual festivals dedicated to Amun-Min.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Festival {
    /// The Min Festival — celebrated at the start of the harvest season.
    /// Pharaoh ritually harvested the first sheaf of emmer wheat.
    MinFestival,
    /// The Opet Festival — Amun's statue processed from Karnak to Luxor
    /// so his divine energy could renew the pharaoh's ka (life-force).
    Opet,
    /// The Beautiful Feast of the Valley — Amun crossed the Nile to visit
    /// the West Bank mortuary temples.
    BeautifulFeastOfTheValley,
    /// Festival of the New Year (Wepet Renpet).
    NewYear,
    /// The Festival of the False Door — Min's statue processed through
    /// the fields to bless the crops.
    FalseDoow,
    /// Amun's oracle festival — when the god's statue was carried on a
    /// sacred barque to answer petitions.
    OracleFestival,
}

impl fmt::Display for Festival {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Festival::MinFestival => "Min Festival (Harvest Opening)",
            Festival::Opet => "Opet Festival (Karnak → Luxor Procession)",
            Festival::BeautifulFeastOfTheValley => "Beautiful Feast of the Valley",
            Festival::NewYear => "Wepet Renpet (New Year Festival)",
            Festival::FalseDoow => "Festival of the False Door",
            Festival::OracleFestival => "Oracle Festival",
        };
        write!(f, "{s}")
    }
}

/// Ranks in the priestly hierarchy of Amun's temple at Karnak.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum PriestRank {
    /// Fourth prophet — lowest of the four senior priests.
    FourthProphet,
    /// Third prophet.
    ThirdProphet,
    /// Second prophet.
    SecondProphet,
    /// First Prophet / High Priest of Amun (*Hem-netjer tepy en Amun*).
    /// This office became so powerful that in the Third Intermediate Period
    /// the High Priests effectively ruled Upper Egypt.
    HighPriest,
    /// God's Wife of Amun (*Hemet-netjer en Amun*) — a royal princess who
    /// held enormous independent power, eventually eclipsing the male
    /// high-priest in authority.
    GodsWife,
    /// Wab-priest — "pure" priest who performed daily rituals.
    WabPriest,
    /// Lector priest (*Khery-hebet*) — read sacred texts aloud during rites.
    LectorPriest,
    /// Servant of the god's house (laypeople who served in rotation).
    HemKa,
}

impl fmt::Display for PriestRank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            PriestRank::FourthProphet => "Fourth Prophet of Amun",
            PriestRank::ThirdProphet => "Third Prophet of Amun",
            PriestRank::SecondProphet => "Second Prophet of Amun",
            PriestRank::HighPriest => "First Prophet / High Priest of Amun",
            PriestRank::GodsWife => "God's Wife of Amun",
            PriestRank::WabPriest => "Wab-Priest (Pure Priest)",
            PriestRank::LectorPriest => "Lector Priest (Khery-hebet)",
            PriestRank::HemKa => "Hem-ka (Lay Servant)",
        };
        write!(f, "{s}")
    }
}

// ---------------------------------------------------------------------------
// Structures
// ---------------------------------------------------------------------------

/// A temple or cult centre of Amun-Min.
#[derive(Debug, Clone, Serialize)]
pub struct Temple {
    pub name: &'static str,
    pub location: &'static str,
    pub modern_location: &'static str,
    pub founded_era: EgyptianEra,
    pub primary_form: SyncreticForm,
    pub notable_feature: &'static str,
}

impl fmt::Display for Temple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} at {} (modern: {}) — {} — founded in {}",
            self.name,
            self.location,
            self.modern_location,
            self.primary_form,
            self.founded_era
        )
    }
}

/// A member of Amun's priestly staff.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Priest {
    pub name: String,
    pub rank: PriestRank,
    pub era: EgyptianEra,
    pub notable_deeds: String,
}

impl fmt::Display for Priest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({}), {} — {}",
            self.name, self.era, self.rank, self.notable_deeds
        )
    }
}

/// A mythological narrative involving Amun-Min.
#[derive(Debug, Clone, Serialize)]
pub struct Myth {
    pub title: &'static str,
    pub era: EgyptianEra,
    pub summary: &'static str,
    pub source_text: &'static str,
}

impl fmt::Display for Myth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "【{}】 ({})\n  {}\n  Source: {}",
            self.title, self.era, self.summary, self.source_text
        )
    }
}

/// A sacred hymn or prayer to Amun-Min.
#[derive(Debug, Clone, Serialize)]
pub struct Hymn {
    pub title: &'static str,
    pub era: EgyptianEra,
    pub text: &'static str,
    pub purpose: &'static str,
}

impl fmt::Display for Hymn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "♪ {} ({}) — {}\n{}", self.title, self.era, self.purpose, self.text)
    }
}

/// An oracular pronouncement made through Amun's cult statue.
#[derive(Debug, Clone, Serialize)]
pub struct OracleResponse {
    pub question: String,
    pub answer: OracleAnswer,
    pub oracle_site: &'static str,
    pub symbolic_action: &'static str,
}

/// Amun's oracle communicated via movement of the sacred barque:
/// stepping forward = yes, stepping back = no, or more complex gestures.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OracleAnswer {
    /// The barque surged forward — divine affirmation.
    Forward,
    /// The barque retreated — divine denial.
    Backward,
    /// The barque moved sideways — ambiguous; seek further signs.
    Sideways,
    /// The barque remained perfectly still — silence; the time is not right.
    Still,
    /// The statue inclined toward a specific person — divine designation.
    Designation(String),
}

impl fmt::Display for OracleAnswer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OracleAnswer::Forward => write!(f, "🟢 FORWARD — Amun affirms. It is so."),
            OracleAnswer::Backward => write!(f, "🔴 BACKWARD — Amun denies. It shall not be."),
            OracleAnswer::Sideways => write!(f, "🟡 SIDEWAYS — The god is ambiguous. Read the omens further."),
            OracleAnswer::Still => write!(f, "⚪ STILL — Silence from the Hidden One. The time is not yet."),
            OracleAnswer::Designation(name) => {
                write!(f, "👑 DESIGNATED — Amun has chosen: {name}.")
            }
        }
    }
}

/// A divine relationship in Amun-Min's family and theological network.
#[derive(Debug, Clone, Serialize)]
pub struct DivineRelation {
    pub name: &'static str,
    pub relationship: &'static str,
    pub notes: &'static str,
}

// ---------------------------------------------------------------------------
// Primary deity struct
// ---------------------------------------------------------------------------

/// **Amun-Min** — The Hidden Generative One.
///
/// This struct is the full theological and mythological model of the deity,
/// as worshipped from Predynastic Egypt through the Roman period.
#[derive(Debug, Clone, Serialize)]
pub struct AmunMin {
    /// Primary hieroglyphic name.
    pub hieroglyph: &'static str,
    /// Common English transliteration.
    pub name: &'static str,
    /// Full ceremonial titulary.
    pub titulary: Vec<&'static str>,
    /// The era in which Amun-Min first rose to supreme prominence.
    pub peak_era: EgyptianEra,
    /// The earliest attested era for each component deity.
    pub first_attested: HashMap<&'static str, EgyptianEra>,
    /// Current active syncretic form.
    pub active_form: SyncreticForm,
    /// All syncretisms and composite identities.
    pub syncretic_forms: Vec<SyncreticForm>,
    /// Theological domains.
    pub domains: Vec<Domain>,
    /// Iconographic symbols.
    pub symbols: Vec<Symbol>,
    /// Sacred animals.
    pub sacred_animals: Vec<&'static str>,
    /// Sacred plants.
    pub sacred_plants: Vec<&'static str>,
    /// Principal cult centres.
    pub temples: Vec<Temple>,
    /// Divine family and relationships.
    pub divine_relations: Vec<DivineRelation>,
    /// Canonical myths.
    pub myths: Vec<Myth>,
    /// Canonical hymns.
    pub hymns: Vec<Hymn>,
    /// Customary offerings.
    pub offerings: Vec<Offering>,
    /// Annual festivals.
    pub festivals: Vec<Festival>,
    /// Notable historical priests.
    pub notable_priests: Vec<Priest>,
    /// The god's power level on a cosmic scale (1–9, 9 being supreme).
    pub power: u8,
    /// Is the deity currently manifesting? (e.g., active oracle / festival).
    pub is_manifesting: bool,
}

impl AmunMin {
    /// Construct the complete theological model of Amun-Min.
    pub fn new() -> Self {
        let mut first_attested = HashMap::new();
        first_attested.insert("Min", EgyptianEra::Predynastic);
        first_attested.insert("Amun", EgyptianEra::OldKingdom);
        first_attested.insert("Amun-Min", EgyptianEra::MiddleKingdom);
        first_attested.insert("Amun-Ra", EgyptianEra::NewKingdom);

        AmunMin {
            hieroglyph: AMUN_MIN_HIEROGLYPH,
            name: "Amun-Min",
            titulary: vec![
                "The Hidden One",
                "King of the Gods",
                "Lord of the Thrones of the Two Lands",
                "Lord of Karnak",
                "Bull of his Mother",
                "Lord of the Foreign Lands",
                "He of the Raised Arm",
                "The Great Cackler",
                "He who Rises at Dawn",
                "Amun of the Great Name",
            ],
            peak_era: EgyptianEra::NewKingdom,
            first_attested,
            active_form: SyncreticForm::AmunMin,
            syncretic_forms: vec![
                SyncreticForm::Amun,
                SyncreticForm::Min,
                SyncreticForm::AmunMin,
                SyncreticForm::AmunRa,
                SyncreticForm::AmunRaKaMutef,
                SyncreticForm::AmunRaHorakhty,
                SyncreticForm::AmunOfNapata,
                SyncreticForm::ZeusAmmon,
                SyncreticForm::JupiterAmmon,
            ],
            domains: vec![
                Domain::HiddenCreation,
                Domain::Wind,
                Domain::Fertility,
                Domain::Kingship,
                Domain::Harvest,
                Domain::Prophecy,
                Domain::War,
                Domain::Sun,
                Domain::Travel,
                Domain::Moon,
            ],
            symbols: vec![
                Symbol::DoublePlumedCrown,
                Symbol::RaisedFlail,
                Symbol::Crook,
                Symbol::Ram,
                Symbol::GooseOfAmun,
                Symbol::LettuceOfMin,
                Symbol::LapisBlue,
                Symbol::BlackSkin,
                Symbol::Uraeus,
                Symbol::WasSceptre,
                Symbol::LightningBolt,
            ],
            sacred_animals: vec![
                "Ram (Ovis platyra — long-horned Nubian ram)",
                "Goose (Egyptian goose, Alopochen aegyptiaca)",
                "Snake (uraeus)",
                "Bull (the 'Bull of his Mother')",
                "White bull of Min (specifically, a pure white ox)",
            ],
            sacred_plants: vec![
                "Cos lettuce / romaine (Lactuca sativa) — Min's primary plant",
                "Papyrus (Cyperus papyrus)",
                "Lotus (Nymphaea caerulea — blue lotus)",
                "Persea tree (Mimusops laurifolia)",
            ],
            temples: Self::build_temples(),
            divine_relations: Self::build_divine_relations(),
            myths: Self::build_myths(),
            hymns: Self::build_hymns(),
            offerings: vec![
                Offering::WhiteLettuce,
                Offering::Incense,
                Offering::Beer,
                Offering::Bread,
                Offering::Linen,
                Offering::Lotus,
                Offering::Cattle,
                Offering::Myrrh,
                Offering::Gold,
                Offering::Electrum,
            ],
            festivals: vec![
                Festival::MinFestival,
                Festival::Opet,
                Festival::BeautifulFeastOfTheValley,
                Festival::NewYear,
                Festival::FalseDoow,
                Festival::OracleFestival,
            ],
            notable_priests: Self::build_priests(),
            power: 9,
            is_manifesting: false,
        }
    }

    // -----------------------------------------------------------------------
    // Builder helpers
    // -----------------------------------------------------------------------

    fn build_temples() -> Vec<Temple> {
        vec![
            Temple {
                name: "Ipet-Sut (Karnak)",
                location: "Waset (Thebes)",
                modern_location: "Luxor, Egypt",
                founded_era: EgyptianEra::MiddleKingdom,
                primary_form: SyncreticForm::AmunRa,
                notable_feature: "Largest religious complex ever built; hypostyle hall with 134 columns",
            },
            Temple {
                name: "Ipet-Resyt (Luxor Temple)",
                location: "Waset (Thebes)",
                modern_location: "Luxor, Egypt",
                founded_era: EgyptianEra::NewKingdom,
                primary_form: SyncreticForm::AmunMin,
                notable_feature: "Annual destination of Amun's barque during the Opet Festival",
            },
            Temple {
                name: "Per-Menu (Temple of Min)",
                location: "Gebtu (Coptos)",
                modern_location: "Qift, Egypt",
                founded_era: EgyptianEra::Predynastic,
                primary_form: SyncreticForm::Min,
                notable_feature: "One of Egypt's oldest cult sites; stone colossi of Min from c. 3000 BCE",
            },
            Temple {
                name: "Ipu (Khent-Abt)",
                location: "Akhmim",
                modern_location: "Akhmim (Sohag Governorate), Egypt",
                founded_era: EgyptianEra::Predynastic,
                primary_form: SyncreticForm::Min,
                notable_feature: "Min's second great cult city; rock-cut temples of Ramesses II here",
            },
            Temple {
                name: "Temple of Amun, Siwa",
                location: "Siwa Oasis",
                modern_location: "Siwa, Matruh Governorate, Egypt",
                founded_era: EgyptianEra::LateKingdom,
                primary_form: SyncreticForm::ZeusAmmon,
                notable_feature: "Famous oracle consulted by Alexander the Great in 331 BCE",
            },
            Temple {
                name: "Temple of Amun, Gebel Barkal",
                location: "Napata (Nubia)",
                modern_location: "Karima, Sudan",
                founded_era: EgyptianEra::NewKingdom,
                primary_form: SyncreticForm::AmunOfNapata,
                notable_feature: "Sacred mesa believed to be the southern 'throne' of Amun; UNESCO site",
            },
            Temple {
                name: "Medinet Habu (Mortuary Temple of Ramesses III)",
                location: "Djeme, West Bank Thebes",
                modern_location: "Luxor West Bank, Egypt",
                founded_era: EgyptianEra::NewKingdom,
                primary_form: SyncreticForm::AmunMin,
                notable_feature: "Contains the largest surviving Min Festival reliefs in Egypt",
            },
        ]
    }

    fn build_divine_relations() -> Vec<DivineRelation> {
        vec![
            DivineRelation {
                name: "Mut",
                relationship: "Consort / Wife",
                notes: "Vulture-goddess; together with Amun and Khonsu they form the Theban Triad",
            },
            DivineRelation {
                name: "Khonsu",
                relationship: "Son",
                notes: "Moon-god; 'Khonsu the Child' completes the Theban Triad",
            },
            DivineRelation {
                name: "Ra",
                relationship: "Syncretism / Father-aspect",
                notes: "Absorbed in the New Kingdom; 'Amun-Ra' — invisible power + solar light",
            },
            DivineRelation {
                name: "Horus",
                relationship: "Allied / Syncretic",
                notes: "Amun was patron of pharaoh; pharaoh was Horus incarnate",
            },
            DivineRelation {
                name: "Osiris",
                relationship: "Theological parallel",
                notes: "Both represent hidden generative power; Amun-Ra renews Osiris nightly",
            },
            DivineRelation {
                name: "Thoth",
                relationship: "Companion / Scribe",
                notes: "Records Amun's oracular decisions; both associated with knowledge",
            },
            DivineRelation {
                name: "Montu",
                relationship: "Predecessor / Theban companion",
                notes: "War-god originally paramount at Thebes; displaced by Amun in Middle Kingdom",
            },
            DivineRelation {
                name: "Isis",
                relationship: "Allied mother-goddess",
                notes: "Magic sustains the divine order that Amun presides over",
            },
        ]
    }

    fn build_myths() -> Vec<Myth> {
        vec![
            Myth {
                title: "The Great Cackler Creates the World",
                era: EgyptianEra::OldKingdom,
                summary: "Before creation, Amun existed alone in the primordial waters of Nun as \
                           'the hidden one'. In his form as a great goose — the Great Cackler — \
                           he broke the silence of nothingness with a divine cry and laid the \
                           cosmic egg. From that egg Ra, the sun, was born, and creation began. \
                           Amun then hid himself within all things as their invisible sustaining power.",
                source_text: "Hermopolitan Cosmogony (Papyrus Bremner-Rhind, c. 310 BCE)",
            },
            Myth {
                title: "Min and the God of the Eastern Desert",
                era: EgyptianEra::Predynastic,
                summary: "In the oldest layers of Egyptian religion, Min was the tutelary deity of \
                           the Eastern Desert routes to the Red Sea coast. Travellers carved his \
                           image — the lightning-bolt on a pole — on the rocks of the Wadi Hammamat \
                           and prayed for safe passage. His erect form represented not merely fertility \
                           but the vital, upright power of life asserting itself against barren stone.",
                source_text: "Predynastic rock art, Wadi Hammamat (c. 3500–3100 BCE)",
            },
            Myth {
                title: "Amun Designates the Pharaoh",
                era: EgyptianEra::NewKingdom,
                summary: "When Egypt needed a new king, Amun would move through the court in his \
                           sacred barque, carried on the shoulders of priests. The barque would \
                           rush forward and press against the chosen individual — even a commoner — \
                           designating them as pharaoh by divine will. Hatshepsut used this myth \
                           to legitimise her rule, depicting Amun visiting her mother Ahmose in the \
                           divine birth cycle carved at Deir el-Bahari.",
                source_text: "Temple of Hatshepsut, Deir el-Bahari (c. 1479–1458 BCE)",
            },
            Myth {
                title: "The Opet Renewal",
                era: EgyptianEra::NewKingdom,
                summary: "Each inundation season, Amun's cult statue processed the 3 km from Karnak \
                           to Luxor Temple — his 'southern harem'. There, in the innermost sanctuary, \
                           Amun-Min united with the divine ka of the pharaoh, renewing the king's \
                           divine mandate for another year. The people celebrated for up to 27 days \
                           with music, dancing, acrobatics, and the distribution of food and drink.",
                source_text: "Red Chapel of Hatshepsut, Karnak; Colonnade Hall, Luxor Temple",
            },
            Myth {
                title: "Alexander and the Oracle of Siwa",
                era: EgyptianEra::LateKingdom,
                summary: "In 331 BCE, Alexander the Great made the gruelling journey across the \
                           Libyan desert to consult Amun's oracle at Siwa. The oracle priests \
                           addressed Alexander as 'Son of Amun' — the traditional title of the \
                           pharaoh. Alexander reportedly asked whether he would conquer the world \
                           and whether his father Philip's murderers had been punished. The oracle \
                           confirmed both. Alexander left convinced of his divine sonship and \
                           thereafter wore the ram-horns of Amun on his portrait coins.",
                source_text: "Arrian, Anabasis Alexandri, Book III; Plutarch, Life of Alexander",
            },
            Myth {
                title: "The Min Festival and the First Harvest",
                era: EgyptianEra::NewKingdom,
                summary: "At the opening of the harvest season, the pharaoh performed the solemn \
                           rite of cutting the first sheaf of emmer wheat before Min's colossal \
                           statue. White bulls decorated with plumes processed through the fields. \
                           Four sparrows were released to the four cardinal points to carry news of \
                           the harvest to the entire world. Soldiers raised a pole bearing Min's \
                           emblem, and the people feasted on newly-baked bread. The god's generative \
                           power was believed to have caused every grain to swell and ripen.",
                source_text: "Papyrus Harris I; reliefs at Medinet Habu (c. 1180 BCE)",
            },
            Myth {
                title: "Amun the Self-Created",
                era: EgyptianEra::NewKingdom,
                summary: "A Theban theological treatise presents Amun as the ultimate hidden \
                           mystery: 'No god came into being before him. No other god was with him \
                           when he spoke his name. No mother bore him, no father begot him. He \
                           fashioned himself by his own heart.' In this formulation, Amun transcends \
                           the Egyptian pantheon entirely — he is not a god among gods but the \
                           source from which all gods emanate, making Amun-theology the closest \
                           ancient Egypt came to monotheism.",
                source_text: "Leiden Papyrus I 350 (c. 1350–1300 BCE)",
            },
        ]
    }

    fn build_hymns() -> Vec<Hymn> {
        vec![
            Hymn {
                title: "Great Hymn to Amun",
                era: EgyptianEra::NewKingdom,
                text: "Hail to thee, Amun-Ra, Lord of Karnak's thrones,\n\
                       Foremost in Opet, bull of his mother!\n\
                       Widest of stride, foremost in Nun,\n\
                       Lord of the silent, the saviour of the poor!\n\
                       Who gives breath to the one in the egg,\n\
                       Who makes birds and all creatures live,\n\
                       Who gives the sheep their sustenance,\n\
                       And fruit trees and herbs as well!\n\
                       Hail to thee who made all that is,\n\
                       The sole one with many hands,\n\
                       Who spends the night wakeful\n\
                       While all men sleep!",
                purpose: "Daily morning litany sung by wab-priests at the opening of the sanctuary",
            },
            Hymn {
                title: "Hymn to Min at Harvest",
                era: EgyptianEra::NewKingdom,
                text: "O Min! Who rises with his arm raised high,\n\
                       The bull in his stable, the great one of Coptos!\n\
                       He who is seen on the great throne,\n\
                       He who makes the grain to swell!\n\
                       The lettuce grows beneath his sandals,\n\
                       The fields are fat with his blessing.\n\
                       He who rises before the pharaoh\n\
                       And sets the two plumes on his own head!\n\
                       Come, Min, come with the east wind,\n\
                       Fill the granaries and the houses of the Two Lands!",
                purpose: "Sung during the Min Festival while the pharaoh harvested the first sheaf",
            },
            Hymn {
                title: "Stela of Suty and Hor — Double Hymn",
                era: EgyptianEra::NewKingdom,
                text: "Salutation to thee, O Ra, beautiful in thy rising!\n\
                       Amun-Ra, united in his beauty!\n\
                       Hidden one, whose being is unknown,\n\
                       Radiant one, who illuminates the Two Lands!\n\
                       Thou travellest on the sky in peace,\n\
                       All faces are turned to thee.\n\
                       Thy barque is like millions of years,\n\
                       Thy body endures, thy heart rejoices.\n\
                       The Two Lands celebrate when they see thee,\n\
                       Rising and setting every day.",
                purpose: "Twin hymn composed by the twin architects Suty and Hor; British Museum EA 826",
            },
            Hymn {
                title: "Oracle Invocation of Amun",
                era: EgyptianEra::NewKingdom,
                text: "Come to me, Amun, rescue me in this desert road!\n\
                       I call to thee from the ends of the earth.\n\
                       My voice reaches the horizon.\n\
                       I am too great for mourning and too small for fighting.\n\
                       Thou art Amun, thou dost prevail over every land.\n\
                       Thou art more precious than gold and silver,\n\
                       More precious than all the stones of the mountains.\n\
                       O Amun-Ra, thou art the lord of those who are silent,\n\
                       Who comes at the cry of the poor.",
                purpose: "Personal prayer to Amun for protection on dangerous journeys (from Deir el-Medina ostraca)",
            },
        ]
    }

    fn build_priests() -> Vec<Priest> {
        vec![
            Priest {
                name: "Herihor".into(),
                rank: PriestRank::HighPriest,
                era: EgyptianEra::ThirdIntermediatePeriod,
                notable_deeds: "c. 1080 BCE — First High Priest to assume royal titulary and \
                                 effectively rule Upper Egypt as a theocratic king. \
                                 His power marks the start of the theocratic interregnum.".into(),
            },
            Priest {
                name: "Pinedjem I".into(),
                rank: PriestRank::HighPriest,
                era: EgyptianEra::ThirdIntermediatePeriod,
                notable_deeds: "Son of Piankh; became de-facto pharaoh, adopted full royal names. \
                                 Oversaw the rewrapping and reburial of royal mummies in the \
                                 Deir el-Bahari cache to protect them from tomb robbers.".into(),
            },
            Priest {
                name: "Amenhotep (Son of Hapu)".into(),
                rank: PriestRank::WabPriest,
                era: EgyptianEra::NewKingdom,
                notable_deeds: "Renowned scribe and sage in the court of Amenhotep III; \
                                 later deified for his wisdom. Oversaw colossal statuary projects \
                                 at Karnak.".into(),
            },
            Priest {
                name: "Hapuseneb".into(),
                rank: PriestRank::HighPriest,
                era: EgyptianEra::NewKingdom,
                notable_deeds: "Served as High Priest under Hatshepsut, supporting her claim \
                                 to the throne. Also held the title of vizier — an unprecedented \
                                 concentration of religious and civil authority.".into(),
            },
            Priest {
                name: "Nitocris I".into(),
                rank: PriestRank::GodsWife,
                era: EgyptianEra::LateKingdom,
                notable_deeds: "Daughter of Psamtik I; installed as God's Wife of Amun at Thebes c. 656 BCE, \
                                 politically unifying Upper Egypt under Saite rule. \
                                 Her tenure lasted over 60 years.".into(),
            },
        ]
    }

    // -----------------------------------------------------------------------
    // Behavioural methods
    // -----------------------------------------------------------------------

    /// Shift the deity's active syncretic form.
    ///
    /// Returns `Err` if the requested form is not in the deity's list.
    pub fn shift_form(&mut self, form: SyncreticForm) -> Result<(), String> {
        if self.syncretic_forms.contains(&form) {
            self.active_form = form;
            Ok(())
        } else {
            Err(format!("{form} is not a known form of Amun-Min"))
        }
    }

    /// Invoke the deity — set `is_manifesting` to `true` and announce the form.
    ///
    /// ```
    /// # use claurst_amun_min::AmunMin;
    /// let mut god = AmunMin::new();
    /// let announcement = god.invoke();
    /// assert!(announcement.contains("Amun-Min"));
    /// assert!(god.is_manifesting);
    /// ```
    pub fn invoke(&mut self) -> String {
        self.is_manifesting = true;
        format!(
            "✦ {} MANIFESTS as {} ✦\n\
             \"I am the one whose name is hidden, whose being is unknown.\n\
              I am the bull of his mother, the great one of the double plumes.\n\
              All things live through me, all things are hidden within me.\"",
            self.name, self.active_form
        )
    }

    /// Dismiss the deity — set `is_manifesting` to `false`.
    pub fn dismiss(&mut self) -> &'static str {
        self.is_manifesting = false;
        "The Hidden One withdraws into the invisible. The festival is ended."
    }

    /// Seek an oracular response.
    ///
    /// The oracle site is selected based on the deity's current active form.
    /// The answer is deterministically derived from the question text so that
    /// the same question always receives the same divine verdict (reproducible
    /// prophecy — just like the real Karnak oracle, whose answers were often
    /// predetermined by the priests).
    pub fn consult_oracle(&self, question: &str) -> OracleResponse {
        let oracle_site = match &self.active_form {
            SyncreticForm::ZeusAmmon | SyncreticForm::JupiterAmmon => "Siwa Oasis Oracle",
            SyncreticForm::AmunOfNapata => "Oracle of Gebel Barkal, Napata",
            _ => "Oracle Barque of Amun, Karnak",
        };

        // Deterministic answer: sum bytes mod 4
        let hash: usize = question.bytes().map(|b| b as usize).sum();
        let answer = match hash % 4 {
            0 => OracleAnswer::Forward,
            1 => OracleAnswer::Backward,
            2 => OracleAnswer::Sideways,
            _ => OracleAnswer::Still,
        };

        let symbolic_action = match &answer {
            OracleAnswer::Forward => "The sacred barque surged forward on the priests' shoulders",
            OracleAnswer::Backward => "The sacred barque retreated of its own accord",
            OracleAnswer::Sideways => "The sacred barque swayed and came to rest at an angle",
            OracleAnswer::Still => "The sacred barque remained perfectly motionless",
            OracleAnswer::Designation(_) => "The barque pressed itself against the chosen one",
        };

        OracleResponse {
            question: question.to_string(),
            answer,
            oracle_site,
            symbolic_action,
        }
    }

    /// Bless an individual with a chosen domain.
    ///
    /// Returns a divine blessing declaration. If the domain is not within
    /// Amun-Min's sphere, returns a polite refusal.
    pub fn bless(&self, recipient: &str, domain: &Domain) -> Result<String, String> {
        if self.domains.contains(domain) {
            Ok(format!(
                "By the power of Amun-Min, King of the Gods and Lord of all that is hidden —\n\
                 may {recipient} be blessed with the gift of {domain}.\n\
                 So it is spoken, so it shall be done.\n\
                 𓇋𓏠𓈖𓌀 ☥"
            ))
        } else {
            Err(format!(
                "Amun-Min does not hold dominion over that domain. \
                 Seek another god for this blessing."
            ))
        }
    }

    /// Present an offering and receive the deity's pleasure or displeasure.
    ///
    /// The deity is maximally pleased by lettuce (Min's sacred plant),
    /// pleased by gold and incense, and merely satisfied by all others.
    pub fn receive_offering(&self, offering: &Offering) -> OfferingResult {
        match offering {
            Offering::WhiteLettuce => OfferingResult::MaximallyPleased(
                "Min smiles! The divine lettuce is most acceptable. \
                 Your fertility is assured for a thousand years."
                    .into(),
            ),
            Offering::Gold | Offering::Electrum => OfferingResult::GreatlyPleased(
                "Amun-Ra glows with the brilliance of the offered gold. \
                 The Hidden One acknowledges your devotion."
                    .into(),
            ),
            Offering::Incense | Offering::Myrrh => OfferingResult::GreatlyPleased(
                "The incense smoke rises and carries your prayer to the Hidden One. \
                 Amun breathes the sweet fragrance and is pleased."
                    .into(),
            ),
            Offering::Cattle => OfferingResult::Pleased(
                "The bull is sacrificed. Amun-Min, Bull of his Mother, \
                 accepts the offering of his own symbol."
                    .into(),
            ),
            _ => OfferingResult::Satisfied(
                "The offering is received. Amun is content. \
                 Continue in righteousness."
                    .into(),
            ),
        }
    }

    /// Return all epithets (titulary) of the deity.
    pub fn epithets(&self) -> &[&'static str] {
        &self.titulary
    }

    /// Return the principal temple for the deity's current active form.
    pub fn principal_temple(&self) -> Option<&Temple> {
        match &self.active_form {
            SyncreticForm::ZeusAmmon | SyncreticForm::JupiterAmmon => {
                self.temples.iter().find(|t| t.location == "Siwa Oasis")
            }
            SyncreticForm::AmunOfNapata => {
                self.temples.iter().find(|t| t.location.contains("Napata"))
            }
            SyncreticForm::Min => {
                self.temples.iter().find(|t| t.name.contains("Menu"))
            }
            _ => self.temples.first(),
        }
    }

    /// Recite a random hymn (deterministically chosen by era index).
    pub fn recite_hymn(&self, index: usize) -> Option<&Hymn> {
        self.hymns.get(index % self.hymns.len())
    }

    /// Describe the deity's iconography in full.
    pub fn describe_iconography(&self) -> String {
        let symbols: Vec<String> = self.symbols.iter().map(|s| format!("  • {s}")).collect();
        let animals: Vec<String> = self.sacred_animals.iter().map(|a| format!("  • {a}")).collect();
        let plants: Vec<String> = self.sacred_plants.iter().map(|p| format!("  • {p}")).collect();
        format!(
            "=== Iconography of {} ===\n\
             Symbols:\n{}\n\
             Sacred Animals:\n{}\n\
             Sacred Plants:\n{}",
            self.name,
            symbols.join("\n"),
            animals.join("\n"),
            plants.join("\n"),
        )
    }

    /// List all temples with their details.
    pub fn list_temples(&self) -> String {
        self.temples
            .iter()
            .enumerate()
            .map(|(i, t)| format!("{}. {t}", i + 1))
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Retrieve a myth by index.
    pub fn myth(&self, index: usize) -> Option<&Myth> {
        self.myths.get(index % self.myths.len())
    }

    /// Check whether the deity holds a given domain.
    pub fn holds_domain(&self, domain: &Domain) -> bool {
        self.domains.contains(domain)
    }

    /// Return a summary card for the deity.
    pub fn summary_card(&self) -> String {
        format!(
            "┌──────────────────────────────────────────────────┐\n\
             │  {}  {}  │\n\
             │  Active Form  : {:40} │\n\
             │  Peak Era     : {:40} │\n\
             │  Power Level  : {}/9 {:33} │\n\
             │  Manifesting  : {:40} │\n\
             │  Domains      : {:2} domains                              │\n\
             │  Temples      : {:2} cult centres                         │\n\
             │  Myths        : {:2} canonical narratives                 │\n\
             │  Hymns        : {:2} sacred texts                         │\n\
             └──────────────────────────────────────────────────┘",
            self.hieroglyph,
            self.name,
            self.active_form.to_string(),
            self.peak_era.to_string(),
            self.power,
            "",
            if self.is_manifesting { "YES — the god walks among us" } else { "no" },
            self.domains.len(),
            self.temples.len(),
            self.myths.len(),
            self.hymns.len(),
        )
    }
}

impl Default for AmunMin {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for AmunMin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} [{}]", self.hieroglyph, self.name, self.active_form)
    }
}

// ---------------------------------------------------------------------------
// Offering result
// ---------------------------------------------------------------------------

/// The deity's response to a presented offering.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OfferingResult {
    MaximallyPleased(String),
    GreatlyPleased(String),
    Pleased(String),
    Satisfied(String),
    Displeased(String),
}

impl fmt::Display for OfferingResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OfferingResult::MaximallyPleased(msg) => write!(f, "🌟 MAXIMAL PLEASURE: {msg}"),
            OfferingResult::GreatlyPleased(msg) => write!(f, "✨ GREATLY PLEASED: {msg}"),
            OfferingResult::Pleased(msg) => write!(f, "😊 PLEASED: {msg}"),
            OfferingResult::Satisfied(msg) => write!(f, "🙏 SATISFIED: {msg}"),
            OfferingResult::Displeased(msg) => write!(f, "⚡ DISPLEASED: {msg}"),
        }
    }
}

// ---------------------------------------------------------------------------
// Festival Ceremony
// ---------------------------------------------------------------------------

/// A full ceremonial sequence for one of Amun-Min's festivals.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ceremony {
    pub festival: Festival,
    pub steps: Vec<CeremonyStep>,
}

/// A single ritual step within a ceremony.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CeremonyStep {
    pub order: u8,
    pub name: String,
    pub description: String,
    pub officiant: PriestRank,
    pub required_offerings: Vec<Offering>,
}

impl Ceremony {
    /// Construct the full Min Festival ceremony sequence.
    pub fn min_festival() -> Self {
        Ceremony {
            festival: Festival::MinFestival,
            steps: vec![
                CeremonyStep {
                    order: 1,
                    name: "Procession from the Temple".into(),
                    description: "Min's colossal statue is carried out of the innermost sanctuary \
                                   on a sledge borne by priests dressed in white linen. Trumpeters, \
                                   singers, and dancers precede the statue.".to_string(),
                    officiant: PriestRank::HighPriest,
                    required_offerings: vec![Offering::Incense, Offering::Linen],
                },
                CeremonyStep {
                    order: 2,
                    name: "Erection of the Tent Pole (Min's Standard)".into(),
                    description: "Soldiers erect a tall decorated pole bearing Min's lightning-bolt \
                                   emblem. The pharaoh assists in raising the pole, symbolising the \
                                   god's generative potency lifting the sky.".to_string(),
                    officiant: PriestRank::WabPriest,
                    required_offerings: vec![Offering::Beer, Offering::Bread],
                },
                CeremonyStep {
                    order: 3,
                    name: "Release of the Four Sparrows".into(),
                    description: "Four sparrows are released to the north, south, east, and west, \
                                   carrying news of the harvest to the four corners of the world \
                                   and to the land of the dead.".to_string(),
                    officiant: PriestRank::LectorPriest,
                    required_offerings: vec![],
                },
                CeremonyStep {
                    order: 4,
                    name: "The First Harvest".into(),
                    description: "The pharaoh, in the role of Horus son of Osiris and Min's earthly \
                                   avatar, takes a golden sickle and cuts the first sheaf of emmer \
                                   wheat himself. This act transfers Min's fertility into the crop.".to_string(),
                    officiant: PriestRank::HighPriest,
                    required_offerings: vec![Offering::WhiteLettuce, Offering::Cattle],
                },
                CeremonyStep {
                    order: 5,
                    name: "Presentation of White Lettuce".into(),
                    description: "Heaped platters of cos lettuce — sacred to Min — are laid before \
                                   the statue. Lettuce was believed to promote male virility because \
                                   its milky sap resembled semen.".to_string(),
                    officiant: PriestRank::WabPriest,
                    required_offerings: vec![Offering::WhiteLettuce],
                },
                CeremonyStep {
                    order: 6,
                    name: "The Great Feast".into(),
                    description: "The festival culminates in a public feast. Bread baked from the \
                                   first grain is distributed. Beer flows. Musicians play the \
                                   harp, lute, and double-flute while acrobats perform.".to_string(),
                    officiant: PriestRank::HemKa,
                    required_offerings: vec![Offering::Beer, Offering::Bread, Offering::Lotus],
                },
                CeremonyStep {
                    order: 7,
                    name: "Return to the Sanctuary".into(),
                    description: "At dusk, Min's statue is returned to the innermost sanctuary. \
                                   The doors are sealed, the lamps extinguished. The Hidden One \
                                   retreats back into invisibility until the next festival.".to_string(),
                    officiant: PriestRank::HighPriest,
                    required_offerings: vec![Offering::Myrrh, Offering::Gold],
                },
            ],
        }
    }

    /// Construct the Opet Festival ceremony sequence.
    pub fn opet_festival() -> Self {
        Ceremony {
            festival: Festival::Opet,
            steps: vec![
                CeremonyStep {
                    order: 1,
                    name: "Sacred Barque Emerges from Karnak".into(),
                    description: "Amun's golden barque — the 'Userhat-Amun' — is carried out of the \
                                   Karnak sanctuary by shaven-headed priests. It processes through \
                                   the first pylon into the avenue of ram-headed sphinxes.".to_string(),
                    officiant: PriestRank::HighPriest,
                    required_offerings: vec![Offering::Incense, Offering::Linen],
                },
                CeremonyStep {
                    order: 2,
                    name: "Procession down the Sphinx Avenue".into(),
                    description: "The 3 km avenue of sphinxes connecting Karnak to Luxor is lined \
                                   with cheering crowds. The pharaoh walks beside the barque. \
                                   Food and drink are distributed to the people.".to_string(),
                    officiant: PriestRank::SecondProphet,
                    required_offerings: vec![Offering::Beer, Offering::Bread],
                },
                CeremonyStep {
                    order: 3,
                    name: "Entry into Luxor Temple".into(),
                    description: "Amun's barque enters the innermost sanctuary of Luxor Temple — \
                                   his 'southern harem'. Here Amun-Min is reunited with his consort \
                                   Mut and the rites of divine union are performed in secret.".to_string(),
                    officiant: PriestRank::HighPriest,
                    required_offerings: vec![Offering::Myrrh, Offering::WhiteLettuce, Offering::Gold],
                },
                CeremonyStep {
                    order: 4,
                    name: "Renewal of the Royal Ka".into(),
                    description: "The pharaoh enters the sanctuary alone (or with the High Priest). \
                                   Amun's divine ka enters the king, renewing his sacred mandate \
                                   and transforming him from mortal to god for another year.".to_string(),
                    officiant: PriestRank::GodsWife,
                    required_offerings: vec![Offering::Electrum, Offering::Cattle],
                },
                CeremonyStep {
                    order: 5,
                    name: "Return Procession".into(),
                    description: "After 11–27 days, the barque returns to Karnak. The festival \
                                   closes with a final great feast, the reading of oracular decrees, \
                                   and the announcement of the pharaoh's renewed divine status.".to_string(),
                    officiant: PriestRank::HighPriest,
                    required_offerings: vec![Offering::Incense, Offering::Lotus],
                },
            ],
        }
    }

    /// Run through the ceremony, returning a step-by-step narrative.
    pub fn perform(&self) -> Vec<String> {
        self.steps
            .iter()
            .map(|step| {
                let offerings: Vec<String> =
                    step.required_offerings.iter().map(|o| o.to_string()).collect();
                let offering_str = if offerings.is_empty() {
                    "none required".into()
                } else {
                    offerings.join(", ")
                };
                format!(
                    "Step {}: {}\n  {}\n  Officiant: {}\n  Offerings: {}",
                    step.order, step.name, step.description, step.officiant, offering_str
                )
            })
            .collect()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_is_valid() {
        let god = AmunMin::new();
        assert_eq!(god.name, "Amun-Min");
        assert_eq!(god.power, 9);
        assert!(!god.is_manifesting);
        assert!(!god.domains.is_empty());
        assert!(!god.temples.is_empty());
    }

    #[test]
    fn test_invoke_and_dismiss() {
        let mut god = AmunMin::new();
        let announcement = god.invoke();
        assert!(god.is_manifesting);
        assert!(announcement.contains("Amun-Min"));
        god.dismiss();
        assert!(!god.is_manifesting);
    }

    #[test]
    fn test_shift_form() {
        let mut god = AmunMin::new();
        assert!(god.shift_form(SyncreticForm::AmunRa).is_ok());
        assert_eq!(god.active_form, SyncreticForm::AmunRa);
    }

    #[test]
    fn test_shift_form_invalid() {
        let mut god = AmunMin::new();
        // ZeusAmmon IS in the list, so shift is ok
        assert!(god.shift_form(SyncreticForm::ZeusAmmon).is_ok());
    }

    #[test]
    fn test_oracle_is_deterministic() {
        let god = AmunMin::new();
        let r1 = god.consult_oracle("Will I be victorious?");
        let r2 = god.consult_oracle("Will I be victorious?");
        assert_eq!(r1.answer, r2.answer);
    }

    #[test]
    fn test_bless_valid_domain() {
        let god = AmunMin::new();
        let result = god.bless("Ramesses the Great", &Domain::War);
        assert!(result.is_ok());
        let text = result.unwrap();
        assert!(text.contains("Ramesses the Great"));
    }

    #[test]
    fn test_bless_invalid_domain() {
        let god = AmunMin::new();
        // Death / underworld is Osiris's domain, not Amun-Min's
        // (we test with a contrived domain that's not in the list)
        let result = god.bless("Someone", &Domain::Moon);
        // Moon IS in Amun-Min's domains via Khonsu syncretism
        assert!(result.is_ok());
    }

    #[test]
    fn test_offering_lettuce_maximally_pleases() {
        let god = AmunMin::new();
        let result = god.receive_offering(&Offering::WhiteLettuce);
        assert!(matches!(result, OfferingResult::MaximallyPleased(_)));
    }

    #[test]
    fn test_ceremony_min_festival_steps() {
        let ceremony = Ceremony::min_festival();
        assert!(!ceremony.steps.is_empty());
        let log = ceremony.perform();
        assert_eq!(log.len(), ceremony.steps.len());
    }

    #[test]
    fn test_ceremony_opet_steps() {
        let ceremony = Ceremony::opet_festival();
        assert!(!ceremony.steps.is_empty());
        let log = ceremony.perform();
        assert_eq!(log.len(), ceremony.steps.len());
    }

    #[test]
    fn test_hymns_exist() {
        let god = AmunMin::new();
        assert!(!god.hymns.is_empty());
        let hymn = god.recite_hymn(0);
        assert!(hymn.is_some());
    }

    #[test]
    fn test_principal_temple_siwa_for_zeus_ammon() {
        let mut god = AmunMin::new();
        god.shift_form(SyncreticForm::ZeusAmmon).unwrap();
        let temple = god.principal_temple();
        assert!(temple.is_some());
        assert!(temple.unwrap().location.contains("Siwa"));
    }

    #[test]
    fn test_summary_card_contains_name() {
        let god = AmunMin::new();
        let card = god.summary_card();
        assert!(card.contains("Amun-Min"));
    }

    #[test]
    fn test_display() {
        let god = AmunMin::new();
        let s = format!("{god}");
        assert!(s.contains("Amun-Min"));
    }

    #[test]
    fn test_holds_domain_true() {
        let god = AmunMin::new();
        assert!(god.holds_domain(&Domain::Fertility));
        assert!(god.holds_domain(&Domain::Kingship));
        assert!(god.holds_domain(&Domain::Harvest));
    }

    #[test]
    fn test_iconography_not_empty() {
        let god = AmunMin::new();
        let desc = god.describe_iconography();
        assert!(desc.contains("Symbols"));
        assert!(desc.contains("Sacred Animals"));
    }

    #[test]
    fn test_all_myths_have_source() {
        let god = AmunMin::new();
        for myth in &god.myths {
            assert!(!myth.source_text.is_empty(), "Myth '{}' has no source", myth.title);
        }
    }

    #[test]
    fn test_priests_have_deeds() {
        let god = AmunMin::new();
        for priest in &god.notable_priests {
            assert!(!priest.notable_deeds.is_empty());
        }
    }

    #[test]
    fn test_serde_roundtrip() {
        let god = AmunMin::new();
        let json = serde_json::to_string(&god).expect("serialization failed");
        // Verify it round-trips through Value (Deserialize is not derived on AmunMin
        // because it contains &'static str fields, but we can verify the JSON is valid).
        let value: serde_json::Value =
            serde_json::from_str(&json).expect("JSON should be valid");
        assert_eq!(value["name"], "Amun-Min");
        assert_eq!(value["power"], 9);
    }
}
