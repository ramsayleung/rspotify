//! All enums for rspotify
use std::error;
use std::fmt;
use std::str::FromStr;
#[derive(Clone, Debug)]
pub struct Error {
    kind: ErrorKind,
}
impl Error {
    pub(crate) fn new(kind: ErrorKind) -> Error {
        Error { kind }
    }

    /// Return the kind of this error.
    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}
/// The kind of an error that can occur.
#[derive(Clone, Debug)]
pub enum ErrorKind {
    /// This error occurs when no proper enum was found.
    NoEnum(String),
}
impl error::Error for Error {
    fn description(&self) -> &str {
        match self.kind {
            ErrorKind::NoEnum(_) => "no proper enum was found",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ErrorKind::NoEnum(ref s) => write!(f, "can't find proper enum of `{:?}`", s),
        }
    }
}
/// Album type - ‘album’, ‘single’, ‘appears_on’, ‘compilation’
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum AlbumType {
    Album,
    Single,
    AppearsOn,
    Compilation,
}
impl FromStr for AlbumType {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "album" => Ok(AlbumType::Album),
            "single" => Ok(AlbumType::Single),
            "appears_on" => Ok(AlbumType::AppearsOn),
            "compilation" => Ok(AlbumType::Compilation),
            _ => Err(Error::new(ErrorKind::NoEnum(s.to_owned()))),
        }
    }
}
impl AlbumType {
    pub fn as_str(&self) -> &str {
        match *self {
            AlbumType::Album => "album",
            AlbumType::Single => "single",
            AlbumType::AppearsOn => "appears_on",
            AlbumType::Compilation => "compilation",
        }
    }
}
#[test]
fn test_album_type_convert_from_str() {
    let album_type = AlbumType::from_str("album");
    assert_eq!(album_type.unwrap(), AlbumType::Album);
    let empty_type = AlbumType::from_str("not exist album");
    assert_eq!(empty_type.is_err(), true);
}

///  Type: ‘artist’, ‘album’,‘track’, ‘playlist’, 'show' or 'episode'
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Type {
    Artist,
    Album,
    Track,
    Playlist,
    User,
    Show,
    Episode,
}
impl Type {
    pub fn as_str(&self) -> &str {
        match *self {
            Type::Album => "album",
            Type::Artist => "artist",
            Type::Track => "track",
            Type::Playlist => "playlist",
            Type::User => "user",
            Type::Show => "show",
            Type::Episode => "episode",
        }
    }
}
impl FromStr for Type {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "artist" => Ok(Type::Artist),
            "album" => Ok(Type::Album),
            "track" => Ok(Type::Track),
            "playlist" => Ok(Type::Playlist),
            "user" => Ok(Type::User),
            "show" => Ok(Type::Show),
            "episode" => Ok(Type::Episode),
            _ => Err(Error::new(ErrorKind::NoEnum(s.to_owned()))),
        }
    }
}

#[test]
fn test_type_convert_from_str() {
    let _type = Type::from_str("album");
    assert_eq!(_type.unwrap(), Type::Album);

    let empty_type = Type::from_str("not_exist_type");
    assert_eq!(empty_type.is_err(), true);
}

/// additional_typs: track, episode
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum AdditionalType {
    Track,
    Episode,
}
impl AdditionalType {
    pub fn as_str(&self) -> &str {
        match *self {
            AdditionalType::Track => "track",
            AdditionalType::Episode => "episode",
        }
    }
}
impl FromStr for AdditionalType {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "track" => Ok(AdditionalType::Track),
            "episode" => Ok(AdditionalType::Episode),
            _ => Err(Error::new(ErrorKind::NoEnum(s.to_owned()))),
        }
    }
}
/// currently_playing_type: track, episode, ad, unknown.
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum CurrentlyPlayingType {
    Track,
    Episode,
    Advertisement,
    Unknown,
}
impl CurrentlyPlayingType {
    pub fn as_str(&self) -> &str {
        match *self {
            CurrentlyPlayingType::Track => "track",
            CurrentlyPlayingType::Episode => "episode",
            CurrentlyPlayingType::Advertisement => "ad",
            CurrentlyPlayingType::Unknown => "unknown",
        }
    }
}
impl FromStr for CurrentlyPlayingType {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "track" => Ok(CurrentlyPlayingType::Track),
            "episode" => Ok(CurrentlyPlayingType::Episode),
            "ad" => Ok(CurrentlyPlayingType::Advertisement),
            "unknown" => Ok(CurrentlyPlayingType::Unknown),
            _ => Err(Error::new(ErrorKind::NoEnum(s.to_owned()))),
        }
    }
}

/// disallow: interrupting_playback, pausing, resuming, seeking, skipping_next, skipping_prev, toggling_repeat_context, toggling_shuffle, toggling_repeat_track, transferring_playback
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug, Hash)]
#[serde(rename_all = "snake_case")]
pub enum DisallowKey {
    InterruptingPlayback,
    Pausing,
    Resuming,
    Seeking,
    SkippingNext,
    SkippingPrev,
    TogglingRepeatContext,
    TogglingShuffle,
    TogglingRepeatTrack,
    TransferringPlayback,
}
impl DisallowKey {
    pub fn as_str(&self) -> &str {
        match *self {
            DisallowKey::InterruptingPlayback => "interrupting_playback",
            DisallowKey::Pausing => "pausing",
            DisallowKey::Resuming => "resuming",
            DisallowKey::Seeking => "seeking",
            DisallowKey::SkippingNext => "skipping_next",
            DisallowKey::SkippingPrev => "skipping_prev",
            DisallowKey::TogglingRepeatContext => "toggling_repeat_context",
            DisallowKey::TogglingShuffle => "toggling_shuffle",
            DisallowKey::TogglingRepeatTrack => "toggling_repeat_track",
            DisallowKey::TransferringPlayback => "transferring_playback",
        }
    }
}
impl FromStr for DisallowKey {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "interrupting_playback" => Ok(DisallowKey::InterruptingPlayback),
            "pausing" => Ok(DisallowKey::Pausing),
            "resuming" => Ok(DisallowKey::Resuming),
            "seeking" => Ok(DisallowKey::Seeking),
            "skipping_next" => Ok(DisallowKey::SkippingNext),
            "skipping_prev" => Ok(DisallowKey::SkippingPrev),
            "toggling_repeat_context" => Ok(DisallowKey::TogglingRepeatContext),
            "toggling_shuffle" => Ok(DisallowKey::TogglingShuffle),
            "toggling_repeat_track" => Ok(DisallowKey::TogglingRepeatTrack),
            "transferring_playback" => Ok(DisallowKey::TransferringPlayback),
            _ => Err(Error::new(ErrorKind::NoEnum(s.to_owned()))),
        }
    }
}

/// time range: long-term, medium-term, short-term
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum TimeRange {
    LongTerm,
    MediumTerm,
    ShortTerm,
}

impl TimeRange {
    pub fn as_str(&self) -> &str {
        match *self {
            TimeRange::LongTerm => "long_term",
            TimeRange::MediumTerm => "medium_term",
            TimeRange::ShortTerm => "short_term",
        }
    }
}

impl FromStr for TimeRange {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "long_term" => Ok(TimeRange::LongTerm),
            "medium_term" => Ok(TimeRange::MediumTerm),
            "short_term" => Ok(TimeRange::ShortTerm),
            _ => Err(Error::new(ErrorKind::NoEnum(s.to_owned()))),
        }
    }
}

#[test]
fn test_convert_time_range_from_str() {
    let time_range = TimeRange::from_str("long_term");
    assert_eq!(time_range.unwrap(), TimeRange::LongTerm);
    let empty_range = TimeRange::from_str("not exist enum");
    assert_eq!(empty_range.is_err(), true);
}
///ISO 3166-1 alpha-2 country code, [wiki about ISO 3166-1](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)
///Source from [country-list](https://datahub.io/core/country-list)
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug)]
pub enum Country {
    Afghanistan,
    AlandIslands,
    Albania,
    Algeria,
    AmericanSamoa,
    Andorra,
    Angola,
    Anguilla,
    Antarctica,
    AntiguaAndBarbuda,
    Argentina,
    Armenia,
    Aruba,
    Australia,
    Austria,
    Azerbaijan,
    Bahamas,
    Bahrain,
    Bangladesh,
    Barbados,
    Belarus,
    Belgium,
    Belize,
    Benin,
    Bermuda,
    Bhutan,
    BoliviaPlurinationalStateOf,
    BonaireSintEustatiusAndSaba,
    BosniaAndHerzegovina,
    Botswana,
    BouvetIsland,
    Brazil,
    BritishIndianOceanTerritory,
    BruneiDarussalam,
    Bulgaria,
    BurkinaFaso,
    Burundi,
    Cambodia,
    Cameroon,
    Canada,
    CapeVerde,
    CaymanIslands,
    CentralAfricanRepublic,
    Chad,
    Chile,
    China,
    ChristmasIsland,
    CocosKeelingIslands,
    Colombia,
    Comoros,
    Congo,
    CongoTheDemocraticRepublicOfThe,
    CookIslands,
    CostaRica,
    CoteDivoire,
    Croatia,
    Cuba,
    Curacao,
    Cyprus,
    CzechRepublic,
    Denmark,
    Djibouti,
    Dominica,
    DominicanRepublic,
    Ecuador,
    Egypt,
    ElSalvador,
    EquatorialGuinea,
    Eritrea,
    Estonia,
    Ethiopia,
    FalklandIslandsMalvinas,
    FaroeIslands,
    Fiji,
    Finland,
    France,
    FrenchGuiana,
    FrenchPolynesia,
    FrenchSouthernTerritories,
    Gabon,
    Gambia,
    Georgia,
    Germany,
    Ghana,
    Gibraltar,
    Greece,
    Greenland,
    Grenada,
    Guadeloupe,
    Guam,
    Guatemala,
    Guernsey,
    Guinea,
    GuineaBissau,
    Guyana,
    Haiti,
    HeardIslandAndMcdonaldIslands,
    HolySeeVaticanCityState,
    Honduras,
    HongKong,
    Hungary,
    Iceland,
    India,
    Indonesia,
    IranIslamicRepublicOf,
    Iraq,
    Ireland,
    IsleOfMan,
    Israel,
    Italy,
    Jamaica,
    Japan,
    Jersey,
    Jordan,
    Kazakhstan,
    Kenya,
    Kiribati,
    KoreaDemocraticPeopleRepublicOf,
    KoreaRepublicOf,
    Kuwait,
    Kyrgyzstan,
    LaoPeopleDemocraticRepublic,
    Latvia,
    Lebanon,
    Lesotho,
    Liberia,
    Libya,
    Liechtenstein,
    Lithuania,
    Luxembourg,
    Macao,
    MacedoniaTheFormerYugoslavRepublicOf,
    Madagascar,
    Malawi,
    Malaysia,
    Maldives,
    Mali,
    Malta,
    MarshallIslands,
    Martinique,
    Mauritania,
    Mauritius,
    Mayotte,
    Mexico,
    MicronesiaFederatedStatesOf,
    MoldovaRepublicOf,
    Monaco,
    Mongolia,
    Montenegro,
    Montserrat,
    Morocco,
    Mozambique,
    Myanmar,
    Namibia,
    Nauru,
    Nepal,
    Netherlands,
    NewCaledonia,
    NewZealand,
    Nicaragua,
    Niger,
    Nigeria,
    Niue,
    NorfolkIsland,
    NorthernMarianaIslands,
    Norway,
    Oman,
    Pakistan,
    Palau,
    PalestineStateOf,
    Panama,
    PapuaNewGuinea,
    Paraguay,
    Peru,
    Philippines,
    Pitcairn,
    Poland,
    Portugal,
    PuertoRico,
    Qatar,
    Reunion,
    Romania,
    RussianFederation,
    Rwanda,
    SaintBarthelemy,
    SaintHelenaAscensionAndTristanDaCunha,
    SaintKittsAndNevis,
    SaintLucia,
    SaintMartinFrenchPart,
    SaintPierreAndMiquelon,
    SaintVincentAndTheGrenadines,
    Samoa,
    SanMarino,
    SaoTomeAndPrincipe,
    SaudiArabia,
    Senegal,
    Serbia,
    Seychelles,
    SierraLeone,
    Singapore,
    SintMaartenDutchPart,
    Slovakia,
    Slovenia,
    SolomonIslands,
    Somalia,
    SouthAfrica,
    SouthGeorgiaAndTheSouthSandwichIslands,
    SouthSudan,
    Spain,
    SriLanka,
    Sudan,
    Suriname,
    SvalbardAndJanMayen,
    Swaziland,
    Sweden,
    Switzerland,
    SyrianArabRepublic,
    TaiwanProvinceOfChina,
    Tajikistan,
    TanzaniaUnitedRepublicOf,
    Thailand,
    TimorLeste,
    Togo,
    Tokelau,
    Tonga,
    TrinidadAndTobago,
    Tunisia,
    Turkey,
    Turkmenistan,
    TurksAndCaicosIslands,
    Tuvalu,
    Uganda,
    Ukraine,
    UnitedArabEmirates,
    UnitedKingdom,
    UnitedStates,
    UnitedStatesMinorOutlyingIslands,
    Uruguay,
    Uzbekistan,
    Vanuatu,
    VenezuelaBolivarianRepublicOf,
    VietNam,
    VirginIslandsBritish,
    VirginIslandsUS,
    WallisAndFutuna,
    WesternSahara,
    Yemen,
    Zambia,
    Zimbabwe,
}
impl Country {
    pub fn as_str(&self) -> &str {
        match *self {
            Country::Afghanistan => "AF",
            Country::AlandIslands => "AX",
            Country::Albania => "AL",
            Country::Algeria => "DZ",
            Country::AmericanSamoa => "AS",
            Country::Andorra => "AD",
            Country::Angola => "AO",
            Country::Anguilla => "AI",
            Country::Antarctica => "AQ",
            Country::AntiguaAndBarbuda => "AG",
            Country::Argentina => "AR",
            Country::Armenia => "AM",
            Country::Aruba => "AW",
            Country::Australia => "AU",
            Country::Austria => "AT",
            Country::Azerbaijan => "AZ",
            Country::Bahamas => "BS",
            Country::Bahrain => "BH",
            Country::Bangladesh => "BD",
            Country::Barbados => "BB",
            Country::Belarus => "BY",
            Country::Belgium => "BE",
            Country::Belize => "BZ",
            Country::Benin => "BJ",
            Country::Bermuda => "BM",
            Country::Bhutan => "BT",
            Country::BoliviaPlurinationalStateOf => "BO",
            Country::BonaireSintEustatiusAndSaba => "BQ",
            Country::BosniaAndHerzegovina => "BA",
            Country::Botswana => "BW",
            Country::BouvetIsland => "BV",
            Country::Brazil => "BR",
            Country::BritishIndianOceanTerritory => "IO",
            Country::BruneiDarussalam => "BN",
            Country::Bulgaria => "BG",
            Country::BurkinaFaso => "BF",
            Country::Burundi => "BI",
            Country::Cambodia => "KH",
            Country::Cameroon => "CM",
            Country::Canada => "CA",
            Country::CapeVerde => "CV",
            Country::CaymanIslands => "KY",
            Country::CentralAfricanRepublic => "CF",
            Country::Chad => "TD",
            Country::Chile => "CL",
            Country::China => "CN",
            Country::ChristmasIsland => "CX",
            Country::CocosKeelingIslands => "CC",
            Country::Colombia => "CO",
            Country::Comoros => "KM",
            Country::Congo => "CG",
            Country::CongoTheDemocraticRepublicOfThe => "CD",
            Country::CookIslands => "CK",
            Country::CostaRica => "CR",
            Country::CoteDivoire => "CI",
            Country::Croatia => "HR",
            Country::Cuba => "CU",
            Country::Curacao => "CW",
            Country::Cyprus => "CY",
            Country::CzechRepublic => "CZ",
            Country::Denmark => "DK",
            Country::Djibouti => "DJ",
            Country::Dominica => "DM",
            Country::DominicanRepublic => "DO",
            Country::Ecuador => "EC",
            Country::Egypt => "EG",
            Country::ElSalvador => "SV",
            Country::EquatorialGuinea => "GQ",
            Country::Eritrea => "ER",
            Country::Estonia => "EE",
            Country::Ethiopia => "ET",
            Country::FalklandIslandsMalvinas => "FK",
            Country::FaroeIslands => "FO",
            Country::Fiji => "FJ",
            Country::Finland => "FI",
            Country::France => "FR",
            Country::FrenchGuiana => "GF",
            Country::FrenchPolynesia => "PF",
            Country::FrenchSouthernTerritories => "TF",
            Country::Gabon => "GA",
            Country::Gambia => "GM",
            Country::Georgia => "GE",
            Country::Germany => "DE",
            Country::Ghana => "GH",
            Country::Gibraltar => "GI",
            Country::Greece => "GR",
            Country::Greenland => "GL",
            Country::Grenada => "GD",
            Country::Guadeloupe => "GP",
            Country::Guam => "GU",
            Country::Guatemala => "GT",
            Country::Guernsey => "GG",
            Country::Guinea => "GN",
            Country::GuineaBissau => "GW",
            Country::Guyana => "GY",
            Country::Haiti => "HT",
            Country::HeardIslandAndMcdonaldIslands => "HM",
            Country::HolySeeVaticanCityState => "VA",
            Country::Honduras => "HN",
            Country::HongKong => "HK",
            Country::Hungary => "HU",
            Country::Iceland => "IS",
            Country::India => "IN",
            Country::Indonesia => "ID",
            Country::IranIslamicRepublicOf => "IR",
            Country::Iraq => "IQ",
            Country::Ireland => "IE",
            Country::IsleOfMan => "IM",
            Country::Israel => "IL",
            Country::Italy => "IT",
            Country::Jamaica => "JM",
            Country::Japan => "JP",
            Country::Jersey => "JE",
            Country::Jordan => "JO",
            Country::Kazakhstan => "KZ",
            Country::Kenya => "KE",
            Country::Kiribati => "KI",
            Country::KoreaDemocraticPeopleRepublicOf => "KP",
            Country::KoreaRepublicOf => "KR",
            Country::Kuwait => "KW",
            Country::Kyrgyzstan => "KG",
            Country::LaoPeopleDemocraticRepublic => "LA",
            Country::Latvia => "LV",
            Country::Lebanon => "LB",
            Country::Lesotho => "LS",
            Country::Liberia => "LR",
            Country::Libya => "LY",
            Country::Liechtenstein => "LI",
            Country::Lithuania => "LT",
            Country::Luxembourg => "LU",
            Country::Macao => "MO",
            Country::MacedoniaTheFormerYugoslavRepublicOf => "MK",
            Country::Madagascar => "MG",
            Country::Malawi => "MW",
            Country::Malaysia => "MY",
            Country::Maldives => "MV",
            Country::Mali => "ML",
            Country::Malta => "MT",
            Country::MarshallIslands => "MH",
            Country::Martinique => "MQ",
            Country::Mauritania => "MR",
            Country::Mauritius => "MU",
            Country::Mayotte => "YT",
            Country::Mexico => "MX",
            Country::MicronesiaFederatedStatesOf => "FM",
            Country::MoldovaRepublicOf => "MD",
            Country::Monaco => "MC",
            Country::Mongolia => "MN",
            Country::Montenegro => "ME",
            Country::Montserrat => "MS",
            Country::Morocco => "MA",
            Country::Mozambique => "MZ",
            Country::Myanmar => "MM",
            Country::Namibia => "NA",
            Country::Nauru => "NR",
            Country::Nepal => "NP",
            Country::Netherlands => "NL",
            Country::NewCaledonia => "NC",
            Country::NewZealand => "NZ",
            Country::Nicaragua => "NI",
            Country::Niger => "NE",
            Country::Nigeria => "NG",
            Country::Niue => "NU",
            Country::NorfolkIsland => "NF",
            Country::NorthernMarianaIslands => "MP",
            Country::Norway => "NO",
            Country::Oman => "OM",
            Country::Pakistan => "PK",
            Country::Palau => "PW",
            Country::PalestineStateOf => "PS",
            Country::Panama => "PA",
            Country::PapuaNewGuinea => "PG",
            Country::Paraguay => "PY",
            Country::Peru => "PE",
            Country::Philippines => "PH",
            Country::Pitcairn => "PN",
            Country::Poland => "PL",
            Country::Portugal => "PT",
            Country::PuertoRico => "PR",
            Country::Qatar => "QA",
            Country::Reunion => "RE",
            Country::Romania => "RO",
            Country::RussianFederation => "RU",
            Country::Rwanda => "RW",
            Country::SaintBarthelemy => "BL",
            Country::SaintHelenaAscensionAndTristanDaCunha => "SH",
            Country::SaintKittsAndNevis => "KN",
            Country::SaintLucia => "LC",
            Country::SaintMartinFrenchPart => "MF",
            Country::SaintPierreAndMiquelon => "PM",
            Country::SaintVincentAndTheGrenadines => "VC",
            Country::Samoa => "WS",
            Country::SanMarino => "SM",
            Country::SaoTomeAndPrincipe => "ST",
            Country::SaudiArabia => "SA",
            Country::Senegal => "SN",
            Country::Serbia => "RS",
            Country::Seychelles => "SC",
            Country::SierraLeone => "SL",
            Country::Singapore => "SG",
            Country::SintMaartenDutchPart => "SX",
            Country::Slovakia => "SK",
            Country::Slovenia => "SI",
            Country::SolomonIslands => "SB",
            Country::Somalia => "SO",
            Country::SouthAfrica => "ZA",
            Country::SouthGeorgiaAndTheSouthSandwichIslands => "GS",
            Country::SouthSudan => "SS",
            Country::Spain => "ES",
            Country::SriLanka => "LK",
            Country::Sudan => "SD",
            Country::Suriname => "SR",
            Country::SvalbardAndJanMayen => "SJ",
            Country::Swaziland => "SZ",
            Country::Sweden => "SE",
            Country::Switzerland => "CH",
            Country::SyrianArabRepublic => "SY",
            Country::TaiwanProvinceOfChina => "TW",
            Country::Tajikistan => "TJ",
            Country::TanzaniaUnitedRepublicOf => "TZ",
            Country::Thailand => "TH",
            Country::TimorLeste => "TL",
            Country::Togo => "TG",
            Country::Tokelau => "TK",
            Country::Tonga => "TO",
            Country::TrinidadAndTobago => "TT",
            Country::Tunisia => "TN",
            Country::Turkey => "TR",
            Country::Turkmenistan => "TM",
            Country::TurksAndCaicosIslands => "TC",
            Country::Tuvalu => "TV",
            Country::Uganda => "UG",
            Country::Ukraine => "UA",
            Country::UnitedArabEmirates => "AE",
            Country::UnitedKingdom => "GB",
            Country::UnitedStates => "US",
            Country::UnitedStatesMinorOutlyingIslands => "UM",
            Country::Uruguay => "UY",
            Country::Uzbekistan => "UZ",
            Country::Vanuatu => "VU",
            Country::VenezuelaBolivarianRepublicOf => "VE",
            Country::VietNam => "VN",
            Country::VirginIslandsBritish => "VG",
            Country::VirginIslandsUS => "VI",
            Country::WallisAndFutuna => "WF",
            Country::WesternSahara => "EH",
            Country::Yemen => "YE",
            Country::Zambia => "ZM",
            Country::Zimbabwe => "ZW",
        }
    }
}
impl FromStr for Country {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AF" => Ok(Country::Afghanistan),
            "AX" => Ok(Country::AlandIslands),
            "AL" => Ok(Country::Albania),
            "DZ" => Ok(Country::Algeria),
            "AS" => Ok(Country::AmericanSamoa),
            "AD" => Ok(Country::Andorra),
            "AO" => Ok(Country::Angola),
            "AI" => Ok(Country::Anguilla),
            "AQ" => Ok(Country::Antarctica),
            "AG" => Ok(Country::AntiguaAndBarbuda),
            "AR" => Ok(Country::Argentina),
            "AM" => Ok(Country::Armenia),
            "AW" => Ok(Country::Aruba),
            "AU" => Ok(Country::Australia),
            "AT" => Ok(Country::Austria),
            "AZ" => Ok(Country::Azerbaijan),
            "BS" => Ok(Country::Bahamas),
            "BH" => Ok(Country::Bahrain),
            "BD" => Ok(Country::Bangladesh),
            "BB" => Ok(Country::Barbados),
            "BY" => Ok(Country::Belarus),
            "BE" => Ok(Country::Belgium),
            "BZ" => Ok(Country::Belize),
            "BJ" => Ok(Country::Benin),
            "BM" => Ok(Country::Bermuda),
            "BT" => Ok(Country::Bhutan),
            "BO" => Ok(Country::BoliviaPlurinationalStateOf),
            "BQ" => Ok(Country::BonaireSintEustatiusAndSaba),
            "BA" => Ok(Country::BosniaAndHerzegovina),
            "BW" => Ok(Country::Botswana),
            "BV" => Ok(Country::BouvetIsland),
            "BR" => Ok(Country::Brazil),
            "IO" => Ok(Country::BritishIndianOceanTerritory),
            "BN" => Ok(Country::BruneiDarussalam),
            "BG" => Ok(Country::Bulgaria),
            "BF" => Ok(Country::BurkinaFaso),
            "BI" => Ok(Country::Burundi),
            "KH" => Ok(Country::Cambodia),
            "CM" => Ok(Country::Cameroon),
            "CA" => Ok(Country::Canada),
            "CV" => Ok(Country::CapeVerde),
            "KY" => Ok(Country::CaymanIslands),
            "CF" => Ok(Country::CentralAfricanRepublic),
            "TD" => Ok(Country::Chad),
            "CL" => Ok(Country::Chile),
            "CN" => Ok(Country::China),
            "CX" => Ok(Country::ChristmasIsland),
            "CC" => Ok(Country::CocosKeelingIslands),
            "CO" => Ok(Country::Colombia),
            "KM" => Ok(Country::Comoros),
            "CG" => Ok(Country::Congo),
            "CD" => Ok(Country::CongoTheDemocraticRepublicOfThe),
            "CK" => Ok(Country::CookIslands),
            "CR" => Ok(Country::CostaRica),
            "CI" => Ok(Country::CoteDivoire),
            "HR" => Ok(Country::Croatia),
            "CU" => Ok(Country::Cuba),
            "CW" => Ok(Country::Curacao),
            "CY" => Ok(Country::Cyprus),
            "CZ" => Ok(Country::CzechRepublic),
            "DK" => Ok(Country::Denmark),
            "DJ" => Ok(Country::Djibouti),
            "DM" => Ok(Country::Dominica),
            "DO" => Ok(Country::DominicanRepublic),
            "EC" => Ok(Country::Ecuador),
            "EG" => Ok(Country::Egypt),
            "SV" => Ok(Country::ElSalvador),
            "GQ" => Ok(Country::EquatorialGuinea),
            "ER" => Ok(Country::Eritrea),
            "EE" => Ok(Country::Estonia),
            "ET" => Ok(Country::Ethiopia),
            "FK" => Ok(Country::FalklandIslandsMalvinas),
            "FO" => Ok(Country::FaroeIslands),
            "FJ" => Ok(Country::Fiji),
            "FI" => Ok(Country::Finland),
            "FR" => Ok(Country::France),
            "GF" => Ok(Country::FrenchGuiana),
            "PF" => Ok(Country::FrenchPolynesia),
            "TF" => Ok(Country::FrenchSouthernTerritories),
            "GA" => Ok(Country::Gabon),
            "GM" => Ok(Country::Gambia),
            "GE" => Ok(Country::Georgia),
            "DE" => Ok(Country::Germany),
            "GH" => Ok(Country::Ghana),
            "GI" => Ok(Country::Gibraltar),
            "GR" => Ok(Country::Greece),
            "GL" => Ok(Country::Greenland),
            "GD" => Ok(Country::Grenada),
            "GP" => Ok(Country::Guadeloupe),
            "GU" => Ok(Country::Guam),
            "GT" => Ok(Country::Guatemala),
            "GG" => Ok(Country::Guernsey),
            "GN" => Ok(Country::Guinea),
            "GW" => Ok(Country::GuineaBissau),
            "GY" => Ok(Country::Guyana),
            "HT" => Ok(Country::Haiti),
            "HM" => Ok(Country::HeardIslandAndMcdonaldIslands),
            "VA" => Ok(Country::HolySeeVaticanCityState),
            "HN" => Ok(Country::Honduras),
            "HK" => Ok(Country::HongKong),
            "HU" => Ok(Country::Hungary),
            "IS" => Ok(Country::Iceland),
            "IN" => Ok(Country::India),
            "ID" => Ok(Country::Indonesia),
            "IR" => Ok(Country::IranIslamicRepublicOf),
            "IQ" => Ok(Country::Iraq),
            "IE" => Ok(Country::Ireland),
            "IM" => Ok(Country::IsleOfMan),
            "IL" => Ok(Country::Israel),
            "IT" => Ok(Country::Italy),
            "JM" => Ok(Country::Jamaica),
            "JP" => Ok(Country::Japan),
            "JE" => Ok(Country::Jersey),
            "JO" => Ok(Country::Jordan),
            "KZ" => Ok(Country::Kazakhstan),
            "KE" => Ok(Country::Kenya),
            "KI" => Ok(Country::Kiribati),
            "KP" => Ok(Country::KoreaDemocraticPeopleRepublicOf),
            "KR" => Ok(Country::KoreaRepublicOf),
            "KW" => Ok(Country::Kuwait),
            "KG" => Ok(Country::Kyrgyzstan),
            "LA" => Ok(Country::LaoPeopleDemocraticRepublic),
            "LV" => Ok(Country::Latvia),
            "LB" => Ok(Country::Lebanon),
            "LS" => Ok(Country::Lesotho),
            "LR" => Ok(Country::Liberia),
            "LY" => Ok(Country::Libya),
            "LI" => Ok(Country::Liechtenstein),
            "LT" => Ok(Country::Lithuania),
            "LU" => Ok(Country::Luxembourg),
            "MO" => Ok(Country::Macao),
            "MK" => Ok(Country::MacedoniaTheFormerYugoslavRepublicOf),
            "MG" => Ok(Country::Madagascar),
            "MW" => Ok(Country::Malawi),
            "MY" => Ok(Country::Malaysia),
            "MV" => Ok(Country::Maldives),
            "ML" => Ok(Country::Mali),
            "MT" => Ok(Country::Malta),
            "MH" => Ok(Country::MarshallIslands),
            "MQ" => Ok(Country::Martinique),
            "MR" => Ok(Country::Mauritania),
            "MU" => Ok(Country::Mauritius),
            "YT" => Ok(Country::Mayotte),
            "MX" => Ok(Country::Mexico),
            "FM" => Ok(Country::MicronesiaFederatedStatesOf),
            "MD" => Ok(Country::MoldovaRepublicOf),
            "MC" => Ok(Country::Monaco),
            "MN" => Ok(Country::Mongolia),
            "ME" => Ok(Country::Montenegro),
            "MS" => Ok(Country::Montserrat),
            "MA" => Ok(Country::Morocco),
            "MZ" => Ok(Country::Mozambique),
            "MM" => Ok(Country::Myanmar),
            "NA" => Ok(Country::Namibia),
            "NR" => Ok(Country::Nauru),
            "NP" => Ok(Country::Nepal),
            "NL" => Ok(Country::Netherlands),
            "NC" => Ok(Country::NewCaledonia),
            "NZ" => Ok(Country::NewZealand),
            "NI" => Ok(Country::Nicaragua),
            "NE" => Ok(Country::Niger),
            "NG" => Ok(Country::Nigeria),
            "NU" => Ok(Country::Niue),
            "NF" => Ok(Country::NorfolkIsland),
            "MP" => Ok(Country::NorthernMarianaIslands),
            "NO" => Ok(Country::Norway),
            "OM" => Ok(Country::Oman),
            "PK" => Ok(Country::Pakistan),
            "PW" => Ok(Country::Palau),
            "PS" => Ok(Country::PalestineStateOf),
            "PA" => Ok(Country::Panama),
            "PG" => Ok(Country::PapuaNewGuinea),
            "PY" => Ok(Country::Paraguay),
            "PE" => Ok(Country::Peru),
            "PH" => Ok(Country::Philippines),
            "PN" => Ok(Country::Pitcairn),
            "PL" => Ok(Country::Poland),
            "PT" => Ok(Country::Portugal),
            "PR" => Ok(Country::PuertoRico),
            "QA" => Ok(Country::Qatar),
            "RE" => Ok(Country::Reunion),
            "RO" => Ok(Country::Romania),
            "RU" => Ok(Country::RussianFederation),
            "RW" => Ok(Country::Rwanda),
            "BL" => Ok(Country::SaintBarthelemy),
            "SH" => Ok(Country::SaintHelenaAscensionAndTristanDaCunha),
            "KN" => Ok(Country::SaintKittsAndNevis),
            "LC" => Ok(Country::SaintLucia),
            "MF" => Ok(Country::SaintMartinFrenchPart),
            "PM" => Ok(Country::SaintPierreAndMiquelon),
            "VC" => Ok(Country::SaintVincentAndTheGrenadines),
            "WS" => Ok(Country::Samoa),
            "SM" => Ok(Country::SanMarino),
            "ST" => Ok(Country::SaoTomeAndPrincipe),
            "SA" => Ok(Country::SaudiArabia),
            "SN" => Ok(Country::Senegal),
            "RS" => Ok(Country::Serbia),
            "SC" => Ok(Country::Seychelles),
            "SL" => Ok(Country::SierraLeone),
            "SG" => Ok(Country::Singapore),
            "SX" => Ok(Country::SintMaartenDutchPart),
            "SK" => Ok(Country::Slovakia),
            "SI" => Ok(Country::Slovenia),
            "SB" => Ok(Country::SolomonIslands),
            "SO" => Ok(Country::Somalia),
            "ZA" => Ok(Country::SouthAfrica),
            "GS" => Ok(Country::SouthGeorgiaAndTheSouthSandwichIslands),
            "SS" => Ok(Country::SouthSudan),
            "ES" => Ok(Country::Spain),
            "LK" => Ok(Country::SriLanka),
            "SD" => Ok(Country::Sudan),
            "SR" => Ok(Country::Suriname),
            "SJ" => Ok(Country::SvalbardAndJanMayen),
            "SZ" => Ok(Country::Swaziland),
            "SE" => Ok(Country::Sweden),
            "CH" => Ok(Country::Switzerland),
            "SY" => Ok(Country::SyrianArabRepublic),
            "TW" => Ok(Country::TaiwanProvinceOfChina),
            "TJ" => Ok(Country::Tajikistan),
            "TZ" => Ok(Country::TanzaniaUnitedRepublicOf),
            "TH" => Ok(Country::Thailand),
            "TL" => Ok(Country::TimorLeste),
            "TG" => Ok(Country::Togo),
            "TK" => Ok(Country::Tokelau),
            "TO" => Ok(Country::Tonga),
            "TT" => Ok(Country::TrinidadAndTobago),
            "TN" => Ok(Country::Tunisia),
            "TR" => Ok(Country::Turkey),
            "TM" => Ok(Country::Turkmenistan),
            "TC" => Ok(Country::TurksAndCaicosIslands),
            "TV" => Ok(Country::Tuvalu),
            "UG" => Ok(Country::Uganda),
            "UA" => Ok(Country::Ukraine),
            "AE" => Ok(Country::UnitedArabEmirates),
            "GB" => Ok(Country::UnitedKingdom),
            "US" => Ok(Country::UnitedStates),
            "UM" => Ok(Country::UnitedStatesMinorOutlyingIslands),
            "UY" => Ok(Country::Uruguay),
            "UZ" => Ok(Country::Uzbekistan),
            "VU" => Ok(Country::Vanuatu),
            "VE" => Ok(Country::VenezuelaBolivarianRepublicOf),
            "VN" => Ok(Country::VietNam),
            "VG" => Ok(Country::VirginIslandsBritish),
            "VI" => Ok(Country::VirginIslandsUS),
            "WF" => Ok(Country::WallisAndFutuna),
            "EH" => Ok(Country::WesternSahara),
            "YE" => Ok(Country::Yemen),
            "ZM" => Ok(Country::Zambia),
            "ZW" => Ok(Country::Zimbabwe),
            _ => Err(Error::new(ErrorKind::NoEnum(s.to_owned()))),
        }
    }
}
#[test]
fn test_convert_country_from_str() {
    let country = Country::from_str("JP");
    assert_eq!(country.unwrap(), Country::Japan);
    let unknown_country = Country::from_str("not exist enum");
    assert_eq!(unknown_country.is_err(), true);
}

///repeat state: track, context or off.
/// - track will repeat the current track.
/// - context will repeat the current context.
/// - off will turn repeat off.
#[derive(Clone, Debug, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RepeatState {
    Off,
    Track,
    Context,
}
impl RepeatState {
    pub fn as_str(&self) -> &str {
        match *self {
            RepeatState::Off => "off",
            RepeatState::Track => "track",
            RepeatState::Context => "context",
        }
    }
}
impl FromStr for RepeatState {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "off" => Ok(RepeatState::Off),
            "track" => Ok(RepeatState::Track),
            "context" => Ok(RepeatState::Context),
            _ => Err(Error::new(ErrorKind::NoEnum(s.to_owned()))),
        }
    }
}

#[test]
fn test_convert_repeat_state_from_str() {
    let repeat_state = RepeatState::from_str("off");
    assert_eq!(repeat_state.unwrap(), RepeatState::Off);
    let unknown_state = RepeatState::from_str("not exist enum");
    assert_eq!(unknown_state.is_err(), true);
}
/// Type for include_external: audio
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum IncludeExternal {
    Audio,
}
impl IncludeExternal {
    pub fn as_str(&self) -> &str {
        match *self {
            IncludeExternal::Audio => "audio",
        }
    }
}
impl FromStr for IncludeExternal {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "audio" => Ok(IncludeExternal::Audio),
            _ => Err(Error::new(ErrorKind::NoEnum(s.to_owned()))),
        }
    }
}

/// Type for search: artist, album, track, playlist, show, episode
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum SearchType {
    Artist,
    Album,
    Track,
    Playlist,
    Show,
    Episode,
}

impl SearchType {
    pub fn as_str(&self) -> &str {
        match *self {
            SearchType::Album => "album",
            SearchType::Artist => "artist",
            SearchType::Track => "track",
            SearchType::Playlist => "playlist",
            SearchType::Show => "show",
            SearchType::Episode => "episode",
        }
    }
}
impl FromStr for SearchType {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "artist" => Ok(SearchType::Artist),
            "album" => Ok(SearchType::Album),
            "track" => Ok(SearchType::Track),
            "playlist" => Ok(SearchType::Playlist),
            "show" => Ok(SearchType::Show),
            "episode" => Ok(SearchType::Episode),
            _ => Err(Error::new(ErrorKind::NoEnum(s.to_owned()))),
        }
    }
}

#[test]
fn test_convert_search_type_from_str() {
    let search_type = SearchType::from_str("artist");
    assert_eq!(search_type.unwrap(), SearchType::Artist);
    let unknown_search_type = SearchType::from_str("unknown_search_type");
    assert_eq!(unknown_search_type.is_err(), true);
}

/// Device Type: computer, smartphone, speaker, TV, etc.
/// See the [Spotify developer
/// docs](https://developer.spotify.com/documentation/web-api/reference/player/get-a-users-available-devices/#device-types)
/// for more information, or in case we are missing a device type here.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DeviceType {
    Computer,
    Tablet,
    Smartphone,
    Speaker,
    TV,
    AVR,
    STB,
    AudioDongle,
    GameConsole,
    CastVideo,
    CastAudio,
    Automobile,
    Unknown,
}
