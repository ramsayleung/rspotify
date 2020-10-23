use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumString};

///ISO 3166-1 alpha-2 country code, [wiki about ISO 3166-1](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)
///Source from [country-list](https://datahub.io/core/country-list)
#[derive(
    Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug, EnumString, AsRefStr, Display,
)]
pub enum Country {
    #[strum(serialize = "AF")]
    Afghanistan,
    #[strum(serialize = "AX")]
    AlandIslands,
    #[strum(serialize = "AL")]
    Albania,
    #[strum(serialize = "DZ")]
    Algeria,
    #[strum(serialize = "AS")]
    AmericanSamoa,
    #[strum(serialize = "AD")]
    Andorra,
    #[strum(serialize = "AO")]
    Angola,
    #[strum(serialize = "AI")]
    Anguilla,
    #[strum(serialize = "AQ")]
    Antarctica,
    #[strum(serialize = "AG")]
    AntiguaAndBarbuda,
    #[strum(serialize = "AR")]
    Argentina,
    #[strum(serialize = "AM")]
    Armenia,
    #[strum(serialize = "AW")]
    Aruba,
    #[strum(serialize = "AU")]
    Australia,
    #[strum(serialize = "AT")]
    Austria,
    #[strum(serialize = "AZ")]
    Azerbaijan,
    #[strum(serialize = "BS")]
    Bahamas,
    #[strum(serialize = "BH")]
    Bahrain,
    #[strum(serialize = "BD")]
    Bangladesh,
    #[strum(serialize = "BB")]
    Barbados,
    #[strum(serialize = "BY")]
    Belarus,
    #[strum(serialize = "BE")]
    Belgium,
    #[strum(serialize = "BZ")]
    Belize,
    #[strum(serialize = "BJ")]
    Benin,
    #[strum(serialize = "BM")]
    Bermuda,
    #[strum(serialize = "BT")]
    Bhutan,
    #[strum(serialize = "BO")]
    BoliviaPlurinationalStateOf,
    #[strum(serialize = "BQ")]
    BonaireSintEustatiusAndSaba,
    #[strum(serialize = "BA")]
    BosniaAndHerzegovina,
    #[strum(serialize = "BW")]
    Botswana,
    #[strum(serialize = "BV")]
    BouvetIsland,
    #[strum(serialize = "BR")]
    Brazil,
    #[strum(serialize = "IO")]
    BritishIndianOceanTerritory,
    #[strum(serialize = "BN")]
    BruneiDarussalam,
    #[strum(serialize = "BG")]
    Bulgaria,
    #[strum(serialize = "BF")]
    BurkinaFaso,
    #[strum(serialize = "BI")]
    Burundi,
    #[strum(serialize = "KH")]
    Cambodia,
    #[strum(serialize = "CM")]
    Cameroon,
    #[strum(serialize = "CA")]
    Canada,
    #[strum(serialize = "CV")]
    CapeVerde,
    #[strum(serialize = "KY")]
    CaymanIslands,
    #[strum(serialize = "CF")]
    CentralAfricanRepublic,
    #[strum(serialize = "TD")]
    Chad,
    #[strum(serialize = "CL")]
    Chile,
    #[strum(serialize = "CN")]
    China,
    #[strum(serialize = "CX")]
    ChristmasIsland,
    #[strum(serialize = "CC")]
    CocosKeelingIslands,
    #[strum(serialize = "CO")]
    Colombia,
    #[strum(serialize = "KM")]
    Comoros,
    #[strum(serialize = "CG")]
    Congo,
    #[strum(serialize = "CD")]
    CongoTheDemocraticRepublicOfThe,
    #[strum(serialize = "CK")]
    CookIslands,
    #[strum(serialize = "CR")]
    CostaRica,
    #[strum(serialize = "CI")]
    CoteDivoire,
    #[strum(serialize = "HR")]
    Croatia,
    #[strum(serialize = "CU")]
    Cuba,
    #[strum(serialize = "CW")]
    Curacao,
    #[strum(serialize = "CY")]
    Cyprus,
    #[strum(serialize = "CZ")]
    CzechRepublic,
    #[strum(serialize = "DK")]
    Denmark,
    #[strum(serialize = "DJ")]
    Djibouti,
    #[strum(serialize = "DM")]
    Dominica,
    #[strum(serialize = "DO")]
    DominicanRepublic,
    #[strum(serialize = "EC")]
    Ecuador,
    #[strum(serialize = "EG")]
    Egypt,
    #[strum(serialize = "SV")]
    ElSalvador,
    #[strum(serialize = "GQ")]
    EquatorialGuinea,
    #[strum(serialize = "ER")]
    Eritrea,
    #[strum(serialize = "EE")]
    Estonia,
    #[strum(serialize = "ET")]
    Ethiopia,
    #[strum(serialize = "FK")]
    FalklandIslandsMalvinas,
    #[strum(serialize = "FO")]
    FaroeIslands,
    #[strum(serialize = "FJ")]
    Fiji,
    #[strum(serialize = "FI")]
    Finland,
    #[strum(serialize = "FR")]
    France,
    #[strum(serialize = "GF")]
    FrenchGuiana,
    #[strum(serialize = "PF")]
    FrenchPolynesia,
    #[strum(serialize = "TF")]
    FrenchSouthernTerritories,
    #[strum(serialize = "GA")]
    Gabon,
    #[strum(serialize = "GM")]
    Gambia,
    #[strum(serialize = "GE")]
    Georgia,
    #[strum(serialize = "DE")]
    Germany,
    #[strum(serialize = "GH")]
    Ghana,
    #[strum(serialize = "GI")]
    Gibraltar,
    #[strum(serialize = "GR")]
    Greece,
    #[strum(serialize = "GL")]
    Greenland,
    #[strum(serialize = "GD")]
    Grenada,
    #[strum(serialize = "GP")]
    Guadeloupe,
    #[strum(serialize = "GU")]
    Guam,
    #[strum(serialize = "GT")]
    Guatemala,
    #[strum(serialize = "GG")]
    Guernsey,
    #[strum(serialize = "GN")]
    Guinea,
    #[strum(serialize = "GW")]
    GuineaBissau,
    #[strum(serialize = "GY")]
    Guyana,
    #[strum(serialize = "HT")]
    Haiti,
    #[strum(serialize = "HM")]
    HeardIslandAndMcdonaldIslands,
    #[strum(serialize = "VA")]
    HolySeeVaticanCityState,
    #[strum(serialize = "HN")]
    Honduras,
    #[strum(serialize = "HK")]
    HongKong,
    #[strum(serialize = "HU")]
    Hungary,
    #[strum(serialize = "IS")]
    Iceland,
    #[strum(serialize = "IN")]
    India,
    #[strum(serialize = "ID")]
    Indonesia,
    #[strum(serialize = "IR")]
    IranIslamicRepublicOf,
    #[strum(serialize = "IQ")]
    Iraq,
    #[strum(serialize = "IE")]
    Ireland,
    #[strum(serialize = "IM")]
    IsleOfMan,
    #[strum(serialize = "IL")]
    Israel,
    #[strum(serialize = "IT")]
    Italy,
    #[strum(serialize = "JM")]
    Jamaica,
    #[strum(serialize = "JP")]
    Japan,
    #[strum(serialize = "JE")]
    Jersey,
    #[strum(serialize = "JO")]
    Jordan,
    #[strum(serialize = "KZ")]
    Kazakhstan,
    #[strum(serialize = "KE")]
    Kenya,
    #[strum(serialize = "KI")]
    Kiribati,
    #[strum(serialize = "KP")]
    KoreaDemocraticPeopleRepublicOf,
    #[strum(serialize = "KR")]
    KoreaRepublicOf,
    #[strum(serialize = "KW")]
    Kuwait,
    #[strum(serialize = "KG")]
    Kyrgyzstan,
    #[strum(serialize = "LA")]
    LaoPeopleDemocraticRepublic,
    #[strum(serialize = "LV")]
    Latvia,
    #[strum(serialize = "LB")]
    Lebanon,
    #[strum(serialize = "LS")]
    Lesotho,
    #[strum(serialize = "LR")]
    Liberia,
    #[strum(serialize = "LY")]
    Libya,
    #[strum(serialize = "LI")]
    Liechtenstein,
    #[strum(serialize = "LT")]
    Lithuania,
    #[strum(serialize = "LU")]
    Luxembourg,
    #[strum(serialize = "MO")]
    Macao,
    #[strum(serialize = "MK")]
    MacedoniaTheFormerYugoslavRepublicOf,
    #[strum(serialize = "MG")]
    Madagascar,
    #[strum(serialize = "MW")]
    Malawi,
    #[strum(serialize = "MY")]
    Malaysia,
    #[strum(serialize = "MV")]
    Maldives,
    #[strum(serialize = "ML")]
    Mali,
    #[strum(serialize = "MT")]
    Malta,
    #[strum(serialize = "MH")]
    MarshallIslands,
    #[strum(serialize = "MQ")]
    Martinique,
    #[strum(serialize = "MR")]
    Mauritania,
    #[strum(serialize = "MU")]
    Mauritius,
    #[strum(serialize = "YT")]
    Mayotte,
    #[strum(serialize = "MX")]
    Mexico,
    #[strum(serialize = "FM")]
    MicronesiaFederatedStatesOf,
    #[strum(serialize = "MD")]
    MoldovaRepublicOf,
    #[strum(serialize = "MC")]
    Monaco,
    #[strum(serialize = "MN")]
    Mongolia,
    #[strum(serialize = "ME")]
    Montenegro,
    #[strum(serialize = "MS")]
    Montserrat,
    #[strum(serialize = "MA")]
    Morocco,
    #[strum(serialize = "MZ")]
    Mozambique,
    #[strum(serialize = "MM")]
    Myanmar,
    #[strum(serialize = "NA")]
    Namibia,
    #[strum(serialize = "NR")]
    Nauru,
    #[strum(serialize = "NP")]
    Nepal,
    #[strum(serialize = "NL")]
    Netherlands,
    #[strum(serialize = "NC")]
    NewCaledonia,
    #[strum(serialize = "NZ")]
    NewZealand,
    #[strum(serialize = "NI")]
    Nicaragua,
    #[strum(serialize = "NE")]
    Niger,
    #[strum(serialize = "NG")]
    Nigeria,
    #[strum(serialize = "NU")]
    Niue,
    #[strum(serialize = "NF")]
    NorfolkIsland,
    #[strum(serialize = "MP")]
    NorthernMarianaIslands,
    #[strum(serialize = "NO")]
    Norway,
    #[strum(serialize = "OM")]
    Oman,
    #[strum(serialize = "PK")]
    Pakistan,
    #[strum(serialize = "PW")]
    Palau,
    #[strum(serialize = "PS")]
    PalestineStateOf,
    #[strum(serialize = "PA")]
    Panama,
    #[strum(serialize = "PG")]
    PapuaNewGuinea,
    #[strum(serialize = "PY")]
    Paraguay,
    #[strum(serialize = "PE")]
    Peru,
    #[strum(serialize = "PH")]
    Philippines,
    #[strum(serialize = "PN")]
    Pitcairn,
    #[strum(serialize = "PL")]
    Poland,
    #[strum(serialize = "PT")]
    Portugal,
    #[strum(serialize = "PR")]
    PuertoRico,
    #[strum(serialize = "QA")]
    Qatar,
    #[strum(serialize = "RE")]
    Reunion,
    #[strum(serialize = "RO")]
    Romania,
    #[strum(serialize = "RU")]
    RussianFederation,
    #[strum(serialize = "RW")]
    Rwanda,
    #[strum(serialize = "BL")]
    SaintBarthelemy,
    #[strum(serialize = "SH")]
    SaintHelenaAscensionAndTristanDaCunha,
    #[strum(serialize = "KN")]
    SaintKittsAndNevis,
    #[strum(serialize = "LC")]
    SaintLucia,
    #[strum(serialize = "MF")]
    SaintMartinFrenchPart,
    #[strum(serialize = "PM")]
    SaintPierreAndMiquelon,
    #[strum(serialize = "VC")]
    SaintVincentAndTheGrenadines,
    #[strum(serialize = "WS")]
    Samoa,
    #[strum(serialize = "SM")]
    SanMarino,
    #[strum(serialize = "ST")]
    SaoTomeAndPrincipe,
    #[strum(serialize = "SA")]
    SaudiArabia,
    #[strum(serialize = "SN")]
    Senegal,
    #[strum(serialize = "RS")]
    Serbia,
    #[strum(serialize = "SC")]
    Seychelles,
    #[strum(serialize = "SL")]
    SierraLeone,
    #[strum(serialize = "SG")]
    Singapore,
    #[strum(serialize = "SX")]
    SintMaartenDutchPart,
    #[strum(serialize = "SK")]
    Slovakia,
    #[strum(serialize = "SI")]
    Slovenia,
    #[strum(serialize = "SB")]
    SolomonIslands,
    #[strum(serialize = "SO")]
    Somalia,
    #[strum(serialize = "ZA")]
    SouthAfrica,
    #[strum(serialize = "GS")]
    SouthGeorgiaAndTheSouthSandwichIslands,
    #[strum(serialize = "SS")]
    SouthSudan,
    #[strum(serialize = "ES")]
    Spain,
    #[strum(serialize = "LK")]
    SriLanka,
    #[strum(serialize = "SD")]
    Sudan,
    #[strum(serialize = "SR")]
    Suriname,
    #[strum(serialize = "SJ")]
    SvalbardAndJanMayen,
    #[strum(serialize = "SZ")]
    Swaziland,
    #[strum(serialize = "SE")]
    Sweden,
    #[strum(serialize = "CH")]
    Switzerland,
    #[strum(serialize = "SY")]
    SyrianArabRepublic,
    #[strum(serialize = "TW")]
    TaiwanProvinceOfChina,
    #[strum(serialize = "TJ")]
    Tajikistan,
    #[strum(serialize = "TZ")]
    TanzaniaUnitedRepublicOf,
    #[strum(serialize = "TH")]
    Thailand,
    #[strum(serialize = "TL")]
    TimorLeste,
    #[strum(serialize = "TG")]
    Togo,
    #[strum(serialize = "TK")]
    Tokelau,
    #[strum(serialize = "TO")]
    Tonga,
    #[strum(serialize = "TT")]
    TrinidadAndTobago,
    #[strum(serialize = "TN")]
    Tunisia,
    #[strum(serialize = "TR")]
    Turkey,
    #[strum(serialize = "TM")]
    Turkmenistan,
    #[strum(serialize = "TC")]
    TurksAndCaicosIslands,
    #[strum(serialize = "TV")]
    Tuvalu,
    #[strum(serialize = "UG")]
    Uganda,
    #[strum(serialize = "UA")]
    Ukraine,
    #[strum(serialize = "AE")]
    UnitedArabEmirates,
    #[strum(serialize = "GB")]
    UnitedKingdom,
    #[strum(serialize = "US")]
    UnitedStates,
    #[strum(serialize = "UM")]
    UnitedStatesMinorOutlyingIslands,
    #[strum(serialize = "UY")]
    Uruguay,
    #[strum(serialize = "UZ")]
    Uzbekistan,
    #[strum(serialize = "VU")]
    Vanuatu,
    #[strum(serialize = "VE")]
    VenezuelaBolivarianRepublicOf,
    #[strum(serialize = "VN")]
    VietNam,
    #[strum(serialize = "VG")]
    VirginIslandsBritish,
    #[strum(serialize = "VI")]
    VirginIslandsUS,
    #[strum(serialize = "WF")]
    WallisAndFutuna,
    #[strum(serialize = "EH")]
    WesternSahara,
    #[strum(serialize = "YE")]
    Yemen,
    #[strum(serialize = "ZM")]
    Zambia,
    #[strum(serialize = "ZW")]
    Zimbabwe,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_convert_country_from_str() {
        let country = Country::from_str("JP");
        assert_eq!(country.unwrap(), Country::Japan);
        let unknown_country = Country::from_str("not exist enum");
        assert!(unknown_country.is_err());
        let zimbabwe = Country::Zimbabwe;
        assert_eq!(zimbabwe.to_string(), "ZW".to_string());
        assert_eq!(zimbabwe.as_ref(), "ZW");
    }
}
