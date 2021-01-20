use serde::{Deserialize, Serialize};
use strum::ToString;

/// ISO 3166-1 alpha-2 country code, from
/// [country-list](https://datahub.io/core/country-list)
///
/// [Reference](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug, ToString)]
pub enum Country {
    #[strum(serialize = "AF")]
    #[serde(rename = "AF")]
    Afghanistan,
    #[strum(serialize = "AX")]
    #[serde(rename = "AX")]
    AlandIslands,
    #[strum(serialize = "AL")]
    #[serde(rename = "AL")]
    Albania,
    #[strum(serialize = "DZ")]
    #[serde(rename = "DZ")]
    Algeria,
    #[strum(serialize = "AS")]
    #[serde(rename = "AS")]
    AmericanSamoa,
    #[strum(serialize = "AD")]
    #[serde(rename = "AD")]
    Andorra,
    #[strum(serialize = "AO")]
    #[serde(rename = "AO")]
    Angola,
    #[strum(serialize = "AI")]
    #[serde(rename = "AI")]
    Anguilla,
    #[strum(serialize = "AQ")]
    #[serde(rename = "AQ")]
    Antarctica,
    #[strum(serialize = "AG")]
    #[serde(rename = "AG")]
    AntiguaAndBarbuda,
    #[strum(serialize = "AR")]
    #[serde(rename = "AR")]
    Argentina,
    #[strum(serialize = "AM")]
    #[serde(rename = "AM")]
    Armenia,
    #[strum(serialize = "AW")]
    #[serde(rename = "AW")]
    Aruba,
    #[strum(serialize = "AU")]
    #[serde(rename = "AU")]
    Australia,
    #[strum(serialize = "AT")]
    #[serde(rename = "AT")]
    Austria,
    #[strum(serialize = "AZ")]
    #[serde(rename = "AZ")]
    Azerbaijan,
    #[strum(serialize = "BS")]
    #[serde(rename = "BS")]
    Bahamas,
    #[strum(serialize = "BH")]
    #[serde(rename = "BH")]
    Bahrain,
    #[strum(serialize = "BD")]
    #[serde(rename = "BD")]
    Bangladesh,
    #[strum(serialize = "BB")]
    #[serde(rename = "BB")]
    Barbados,
    #[strum(serialize = "BY")]
    #[serde(rename = "BY")]
    Belarus,
    #[strum(serialize = "BE")]
    #[serde(rename = "BE")]
    Belgium,
    #[strum(serialize = "BZ")]
    #[serde(rename = "BZ")]
    Belize,
    #[strum(serialize = "BJ")]
    #[serde(rename = "BJ")]
    Benin,
    #[strum(serialize = "BM")]
    #[serde(rename = "BM")]
    Bermuda,
    #[strum(serialize = "BT")]
    #[serde(rename = "BT")]
    Bhutan,
    #[strum(serialize = "BO")]
    #[serde(rename = "BO")]
    BoliviaPlurinationalStateOf,
    #[strum(serialize = "BQ")]
    #[serde(rename = "BQ")]
    BonaireSintEustatiusAndSaba,
    #[strum(serialize = "BA")]
    #[serde(rename = "BA")]
    BosniaAndHerzegovina,
    #[strum(serialize = "BW")]
    #[serde(rename = "BW")]
    Botswana,
    #[strum(serialize = "BV")]
    #[serde(rename = "BV")]
    BouvetIsland,
    #[strum(serialize = "BR")]
    #[serde(rename = "BR")]
    Brazil,
    #[strum(serialize = "IO")]
    #[serde(rename = "IO")]
    BritishIndianOceanTerritory,
    #[strum(serialize = "BN")]
    #[serde(rename = "BN")]
    BruneiDarussalam,
    #[strum(serialize = "BG")]
    #[serde(rename = "BG")]
    Bulgaria,
    #[strum(serialize = "BF")]
    #[serde(rename = "BF")]
    BurkinaFaso,
    #[strum(serialize = "BI")]
    #[serde(rename = "BI")]
    Burundi,
    #[strum(serialize = "KH")]
    #[serde(rename = "KH")]
    Cambodia,
    #[strum(serialize = "CM")]
    #[serde(rename = "CM")]
    Cameroon,
    #[strum(serialize = "CA")]
    #[serde(rename = "CA")]
    Canada,
    #[strum(serialize = "CV")]
    #[serde(rename = "CV")]
    CapeVerde,
    #[strum(serialize = "KY")]
    #[serde(rename = "KY")]
    CaymanIslands,
    #[strum(serialize = "CF")]
    #[serde(rename = "CF")]
    CentralAfricanRepublic,
    #[strum(serialize = "TD")]
    #[serde(rename = "TD")]
    Chad,
    #[strum(serialize = "CL")]
    #[serde(rename = "CL")]
    Chile,
    #[strum(serialize = "CN")]
    #[serde(rename = "CN")]
    China,
    #[strum(serialize = "CX")]
    #[serde(rename = "CX")]
    ChristmasIsland,
    #[strum(serialize = "CC")]
    #[serde(rename = "CC")]
    CocosKeelingIslands,
    #[strum(serialize = "CO")]
    #[serde(rename = "CO")]
    Colombia,
    #[strum(serialize = "KM")]
    #[serde(rename = "KM")]
    Comoros,
    #[strum(serialize = "CG")]
    #[serde(rename = "CG")]
    Congo,
    #[strum(serialize = "CD")]
    #[serde(rename = "CD")]
    CongoTheDemocraticRepublicOfThe,
    #[strum(serialize = "CK")]
    #[serde(rename = "CK")]
    CookIslands,
    #[strum(serialize = "CR")]
    #[serde(rename = "CR")]
    CostaRica,
    #[strum(serialize = "CI")]
    #[serde(rename = "CI")]
    CoteDivoire,
    #[strum(serialize = "HR")]
    #[serde(rename = "HR")]
    Croatia,
    #[strum(serialize = "CU")]
    #[serde(rename = "CU")]
    Cuba,
    #[strum(serialize = "CW")]
    #[serde(rename = "CW")]
    Curacao,
    #[strum(serialize = "CY")]
    #[serde(rename = "CY")]
    Cyprus,
    #[strum(serialize = "CZ")]
    #[serde(rename = "CZ")]
    CzechRepublic,
    #[strum(serialize = "DK")]
    #[serde(rename = "DK")]
    Denmark,
    #[strum(serialize = "DJ")]
    #[serde(rename = "DJ")]
    Djibouti,
    #[strum(serialize = "DM")]
    #[serde(rename = "DM")]
    Dominica,
    #[strum(serialize = "DO")]
    #[serde(rename = "DO")]
    DominicanRepublic,
    #[strum(serialize = "EC")]
    #[serde(rename = "EC")]
    Ecuador,
    #[strum(serialize = "EG")]
    #[serde(rename = "EG")]
    Egypt,
    #[strum(serialize = "SV")]
    #[serde(rename = "SV")]
    ElSalvador,
    #[strum(serialize = "GQ")]
    #[serde(rename = "GQ")]
    EquatorialGuinea,
    #[strum(serialize = "ER")]
    #[serde(rename = "ER")]
    Eritrea,
    #[strum(serialize = "EE")]
    #[serde(rename = "EE")]
    Estonia,
    #[strum(serialize = "ET")]
    #[serde(rename = "ET")]
    Ethiopia,
    #[strum(serialize = "FK")]
    #[serde(rename = "FK")]
    FalklandIslandsMalvinas,
    #[strum(serialize = "FO")]
    #[serde(rename = "FO")]
    FaroeIslands,
    #[strum(serialize = "FJ")]
    #[serde(rename = "FJ")]
    Fiji,
    #[strum(serialize = "FI")]
    #[serde(rename = "FI")]
    Finland,
    #[strum(serialize = "FR")]
    #[serde(rename = "FR")]
    France,
    #[strum(serialize = "GF")]
    #[serde(rename = "GF")]
    FrenchGuiana,
    #[strum(serialize = "PF")]
    #[serde(rename = "PF")]
    FrenchPolynesia,
    #[strum(serialize = "TF")]
    #[serde(rename = "TF")]
    FrenchSouthernTerritories,
    #[strum(serialize = "GA")]
    #[serde(rename = "GA")]
    Gabon,
    #[strum(serialize = "GM")]
    #[serde(rename = "GM")]
    Gambia,
    #[strum(serialize = "GE")]
    #[serde(rename = "GE")]
    Georgia,
    #[strum(serialize = "DE")]
    #[serde(rename = "DE")]
    Germany,
    #[strum(serialize = "GH")]
    #[serde(rename = "GH")]
    Ghana,
    #[strum(serialize = "GI")]
    #[serde(rename = "GI")]
    Gibraltar,
    #[strum(serialize = "GR")]
    #[serde(rename = "GR")]
    Greece,
    #[strum(serialize = "GL")]
    #[serde(rename = "GL")]
    Greenland,
    #[strum(serialize = "GD")]
    #[serde(rename = "GD")]
    Grenada,
    #[strum(serialize = "GP")]
    #[serde(rename = "GP")]
    Guadeloupe,
    #[strum(serialize = "GU")]
    #[serde(rename = "GU")]
    Guam,
    #[strum(serialize = "GT")]
    #[serde(rename = "GT")]
    Guatemala,
    #[strum(serialize = "GG")]
    #[serde(rename = "GG")]
    Guernsey,
    #[strum(serialize = "GN")]
    #[serde(rename = "GN")]
    Guinea,
    #[strum(serialize = "GW")]
    #[serde(rename = "GW")]
    GuineaBissau,
    #[strum(serialize = "GY")]
    #[serde(rename = "GY")]
    Guyana,
    #[strum(serialize = "HT")]
    #[serde(rename = "HT")]
    Haiti,
    #[strum(serialize = "HM")]
    #[serde(rename = "HM")]
    HeardIslandAndMcdonaldIslands,
    #[strum(serialize = "VA")]
    #[serde(rename = "VA")]
    HolySeeVaticanCityState,
    #[strum(serialize = "HN")]
    #[serde(rename = "HN")]
    Honduras,
    #[strum(serialize = "HK")]
    #[serde(rename = "HK")]
    HongKong,
    #[strum(serialize = "HU")]
    #[serde(rename = "HU")]
    Hungary,
    #[strum(serialize = "IS")]
    #[serde(rename = "IS")]
    Iceland,
    #[strum(serialize = "IN")]
    #[serde(rename = "IN")]
    India,
    #[strum(serialize = "ID")]
    #[serde(rename = "ID")]
    Indonesia,
    #[strum(serialize = "IR")]
    #[serde(rename = "IR")]
    IranIslamicRepublicOf,
    #[strum(serialize = "IQ")]
    #[serde(rename = "IQ")]
    Iraq,
    #[strum(serialize = "IE")]
    #[serde(rename = "IE")]
    Ireland,
    #[strum(serialize = "IM")]
    #[serde(rename = "IM")]
    IsleOfMan,
    #[strum(serialize = "IL")]
    #[serde(rename = "IL")]
    Israel,
    #[strum(serialize = "IT")]
    #[serde(rename = "IT")]
    Italy,
    #[strum(serialize = "JM")]
    #[serde(rename = "JM")]
    Jamaica,
    #[strum(serialize = "JP")]
    #[serde(rename = "JP")]
    Japan,
    #[strum(serialize = "JE")]
    #[serde(rename = "JE")]
    Jersey,
    #[strum(serialize = "JO")]
    #[serde(rename = "JO")]
    Jordan,
    #[strum(serialize = "KZ")]
    #[serde(rename = "KZ")]
    Kazakhstan,
    #[strum(serialize = "KE")]
    #[serde(rename = "KE")]
    Kenya,
    #[strum(serialize = "KI")]
    #[serde(rename = "KI")]
    Kiribati,
    #[strum(serialize = "KP")]
    #[serde(rename = "KP")]
    KoreaDemocraticPeopleRepublicOf,
    #[strum(serialize = "KR")]
    #[serde(rename = "KR")]
    KoreaRepublicOf,
    #[strum(serialize = "KW")]
    #[serde(rename = "KW")]
    Kuwait,
    #[strum(serialize = "KG")]
    #[serde(rename = "KG")]
    Kyrgyzstan,
    #[strum(serialize = "LA")]
    #[serde(rename = "LA")]
    LaoPeopleDemocraticRepublic,
    #[strum(serialize = "LV")]
    #[serde(rename = "LV")]
    Latvia,
    #[strum(serialize = "LB")]
    #[serde(rename = "LB")]
    Lebanon,
    #[strum(serialize = "LS")]
    #[serde(rename = "LS")]
    Lesotho,
    #[strum(serialize = "LR")]
    #[serde(rename = "LR")]
    Liberia,
    #[strum(serialize = "LY")]
    #[serde(rename = "LY")]
    Libya,
    #[strum(serialize = "LI")]
    #[serde(rename = "LI")]
    Liechtenstein,
    #[strum(serialize = "LT")]
    #[serde(rename = "LT")]
    Lithuania,
    #[strum(serialize = "LU")]
    #[serde(rename = "LU")]
    Luxembourg,
    #[strum(serialize = "MO")]
    #[serde(rename = "MO")]
    Macao,
    #[strum(serialize = "MK")]
    #[serde(rename = "MK")]
    MacedoniaTheFormerYugoslavRepublicOf,
    #[strum(serialize = "MG")]
    #[serde(rename = "MG")]
    Madagascar,
    #[strum(serialize = "MW")]
    #[serde(rename = "MW")]
    Malawi,
    #[strum(serialize = "MY")]
    #[serde(rename = "MY")]
    Malaysia,
    #[strum(serialize = "MV")]
    #[serde(rename = "MV")]
    Maldives,
    #[strum(serialize = "ML")]
    #[serde(rename = "ML")]
    Mali,
    #[strum(serialize = "MT")]
    #[serde(rename = "MT")]
    Malta,
    #[strum(serialize = "MH")]
    #[serde(rename = "MH")]
    MarshallIslands,
    #[strum(serialize = "MQ")]
    #[serde(rename = "MQ")]
    Martinique,
    #[strum(serialize = "MR")]
    #[serde(rename = "MR")]
    Mauritania,
    #[strum(serialize = "MU")]
    #[serde(rename = "MU")]
    Mauritius,
    #[strum(serialize = "YT")]
    #[serde(rename = "YT")]
    Mayotte,
    #[strum(serialize = "MX")]
    #[serde(rename = "MX")]
    Mexico,
    #[strum(serialize = "FM")]
    #[serde(rename = "FM")]
    MicronesiaFederatedStatesOf,
    #[strum(serialize = "MD")]
    #[serde(rename = "MD")]
    MoldovaRepublicOf,
    #[strum(serialize = "MC")]
    #[serde(rename = "MC")]
    Monaco,
    #[strum(serialize = "MN")]
    #[serde(rename = "MN")]
    Mongolia,
    #[strum(serialize = "ME")]
    #[serde(rename = "ME")]
    Montenegro,
    #[strum(serialize = "MS")]
    #[serde(rename = "MS")]
    Montserrat,
    #[strum(serialize = "MA")]
    #[serde(rename = "MA")]
    Morocco,
    #[strum(serialize = "MZ")]
    #[serde(rename = "MZ")]
    Mozambique,
    #[strum(serialize = "MM")]
    #[serde(rename = "MM")]
    Myanmar,
    #[strum(serialize = "NA")]
    #[serde(rename = "NA")]
    Namibia,
    #[strum(serialize = "NR")]
    #[serde(rename = "NR")]
    Nauru,
    #[strum(serialize = "NP")]
    #[serde(rename = "NP")]
    Nepal,
    #[strum(serialize = "NL")]
    #[serde(rename = "NL")]
    Netherlands,
    #[strum(serialize = "NC")]
    #[serde(rename = "NC")]
    NewCaledonia,
    #[strum(serialize = "NZ")]
    #[serde(rename = "NZ")]
    NewZealand,
    #[strum(serialize = "NI")]
    #[serde(rename = "NI")]
    Nicaragua,
    #[strum(serialize = "NE")]
    #[serde(rename = "NE")]
    Niger,
    #[strum(serialize = "NG")]
    #[serde(rename = "NG")]
    Nigeria,
    #[strum(serialize = "NU")]
    #[serde(rename = "NU")]
    Niue,
    #[strum(serialize = "NF")]
    #[serde(rename = "NF")]
    NorfolkIsland,
    #[strum(serialize = "MP")]
    #[serde(rename = "MP")]
    NorthernMarianaIslands,
    #[strum(serialize = "NO")]
    #[serde(rename = "NO")]
    Norway,
    #[strum(serialize = "OM")]
    #[serde(rename = "OM")]
    Oman,
    #[strum(serialize = "PK")]
    #[serde(rename = "PK")]
    Pakistan,
    #[strum(serialize = "PW")]
    #[serde(rename = "PW")]
    Palau,
    #[strum(serialize = "PS")]
    #[serde(rename = "PS")]
    PalestineStateOf,
    #[strum(serialize = "PA")]
    #[serde(rename = "PA")]
    Panama,
    #[strum(serialize = "PG")]
    #[serde(rename = "PG")]
    PapuaNewGuinea,
    #[strum(serialize = "PY")]
    #[serde(rename = "PY")]
    Paraguay,
    #[strum(serialize = "PE")]
    #[serde(rename = "PE")]
    Peru,
    #[strum(serialize = "PH")]
    #[serde(rename = "PH")]
    Philippines,
    #[strum(serialize = "PN")]
    #[serde(rename = "PN")]
    Pitcairn,
    #[strum(serialize = "PL")]
    #[serde(rename = "PL")]
    Poland,
    #[strum(serialize = "PT")]
    #[serde(rename = "PT")]
    Portugal,
    #[strum(serialize = "PR")]
    #[serde(rename = "PR")]
    PuertoRico,
    #[strum(serialize = "QA")]
    #[serde(rename = "QA")]
    Qatar,
    #[strum(serialize = "RE")]
    #[serde(rename = "RE")]
    Reunion,
    #[strum(serialize = "RO")]
    #[serde(rename = "RO")]
    Romania,
    #[strum(serialize = "RU")]
    #[serde(rename = "RU")]
    RussianFederation,
    #[strum(serialize = "RW")]
    #[serde(rename = "RW")]
    Rwanda,
    #[strum(serialize = "BL")]
    #[serde(rename = "BL")]
    SaintBarthelemy,
    #[strum(serialize = "SH")]
    #[serde(rename = "SH")]
    SaintHelenaAscensionAndTristanDaCunha,
    #[strum(serialize = "KN")]
    #[serde(rename = "KN")]
    SaintKittsAndNevis,
    #[strum(serialize = "LC")]
    #[serde(rename = "LC")]
    SaintLucia,
    #[strum(serialize = "MF")]
    #[serde(rename = "MF")]
    SaintMartinFrenchPart,
    #[strum(serialize = "PM")]
    #[serde(rename = "PM")]
    SaintPierreAndMiquelon,
    #[strum(serialize = "VC")]
    #[serde(rename = "VC")]
    SaintVincentAndTheGrenadines,
    #[strum(serialize = "WS")]
    #[serde(rename = "WS")]
    Samoa,
    #[strum(serialize = "SM")]
    #[serde(rename = "SM")]
    SanMarino,
    #[strum(serialize = "ST")]
    #[serde(rename = "ST")]
    SaoTomeAndPrincipe,
    #[strum(serialize = "SA")]
    #[serde(rename = "SA")]
    SaudiArabia,
    #[strum(serialize = "SN")]
    #[serde(rename = "SN")]
    Senegal,
    #[strum(serialize = "RS")]
    #[serde(rename = "RS")]
    Serbia,
    #[strum(serialize = "SC")]
    #[serde(rename = "SC")]
    Seychelles,
    #[strum(serialize = "SL")]
    #[serde(rename = "SL")]
    SierraLeone,
    #[strum(serialize = "SG")]
    #[serde(rename = "SG")]
    Singapore,
    #[strum(serialize = "SX")]
    #[serde(rename = "SX")]
    SintMaartenDutchPart,
    #[strum(serialize = "SK")]
    #[serde(rename = "SK")]
    Slovakia,
    #[strum(serialize = "SI")]
    #[serde(rename = "SI")]
    Slovenia,
    #[strum(serialize = "SB")]
    #[serde(rename = "SB")]
    SolomonIslands,
    #[strum(serialize = "SO")]
    #[serde(rename = "SO")]
    Somalia,
    #[strum(serialize = "ZA")]
    #[serde(rename = "ZA")]
    SouthAfrica,
    #[strum(serialize = "GS")]
    #[serde(rename = "GS")]
    SouthGeorgiaAndTheSouthSandwichIslands,
    #[strum(serialize = "SS")]
    #[serde(rename = "SS")]
    SouthSudan,
    #[strum(serialize = "ES")]
    #[serde(rename = "ES")]
    Spain,
    #[strum(serialize = "LK")]
    #[serde(rename = "LK")]
    SriLanka,
    #[strum(serialize = "SD")]
    #[serde(rename = "SD")]
    Sudan,
    #[strum(serialize = "SR")]
    #[serde(rename = "SR")]
    Suriname,
    #[strum(serialize = "SJ")]
    #[serde(rename = "SJ")]
    SvalbardAndJanMayen,
    #[strum(serialize = "SZ")]
    #[serde(rename = "SZ")]
    Swaziland,
    #[strum(serialize = "SE")]
    #[serde(rename = "SE")]
    Sweden,
    #[strum(serialize = "CH")]
    #[serde(rename = "CH")]
    Switzerland,
    #[strum(serialize = "SY")]
    #[serde(rename = "SY")]
    SyrianArabRepublic,
    #[strum(serialize = "TW")]
    #[serde(rename = "TW")]
    TaiwanProvinceOfChina,
    #[strum(serialize = "TJ")]
    #[serde(rename = "TJ")]
    Tajikistan,
    #[strum(serialize = "TZ")]
    #[serde(rename = "TZ")]
    TanzaniaUnitedRepublicOf,
    #[strum(serialize = "TH")]
    #[serde(rename = "TH")]
    Thailand,
    #[strum(serialize = "TL")]
    #[serde(rename = "TL")]
    TimorLeste,
    #[strum(serialize = "TG")]
    #[serde(rename = "TG")]
    Togo,
    #[strum(serialize = "TK")]
    #[serde(rename = "TK")]
    Tokelau,
    #[strum(serialize = "TO")]
    #[serde(rename = "TO")]
    Tonga,
    #[strum(serialize = "TT")]
    #[serde(rename = "TT")]
    TrinidadAndTobago,
    #[strum(serialize = "TN")]
    #[serde(rename = "TN")]
    Tunisia,
    #[strum(serialize = "TR")]
    #[serde(rename = "TR")]
    Turkey,
    #[strum(serialize = "TM")]
    #[serde(rename = "TM")]
    Turkmenistan,
    #[strum(serialize = "TC")]
    #[serde(rename = "TC")]
    TurksAndCaicosIslands,
    #[strum(serialize = "TV")]
    #[serde(rename = "TV")]
    Tuvalu,
    #[strum(serialize = "UG")]
    #[serde(rename = "UG")]
    Uganda,
    #[strum(serialize = "UA")]
    #[serde(rename = "UA")]
    Ukraine,
    #[strum(serialize = "AE")]
    #[serde(rename = "AE")]
    UnitedArabEmirates,
    #[strum(serialize = "GB")]
    #[serde(rename = "GB")]
    UnitedKingdom,
    #[strum(serialize = "US")]
    #[serde(rename = "US")]
    UnitedStates,
    #[strum(serialize = "UM")]
    #[serde(rename = "UM")]
    UnitedStatesMinorOutlyingIslands,
    #[strum(serialize = "UY")]
    #[serde(rename = "UY")]
    Uruguay,
    #[strum(serialize = "UZ")]
    #[serde(rename = "UZ")]
    Uzbekistan,
    #[strum(serialize = "VU")]
    #[serde(rename = "VU")]
    Vanuatu,
    #[strum(serialize = "VE")]
    #[serde(rename = "VE")]
    VenezuelaBolivarianRepublicOf,
    #[strum(serialize = "VN")]
    #[serde(rename = "VN")]
    VietNam,
    #[strum(serialize = "VG")]
    #[serde(rename = "VG")]
    VirginIslandsBritish,
    #[strum(serialize = "VI")]
    #[serde(rename = "VI")]
    VirginIslandsUS,
    #[strum(serialize = "WF")]
    #[serde(rename = "WF")]
    WallisAndFutuna,
    #[strum(serialize = "EH")]
    #[serde(rename = "EH")]
    WesternSahara,
    #[strum(serialize = "YE")]
    #[serde(rename = "YE")]
    Yemen,
    #[strum(serialize = "ZM")]
    #[serde(rename = "ZM")]
    Zambia,
    #[strum(serialize = "ZW")]
    #[serde(rename = "ZW")]
    Zimbabwe,
}
