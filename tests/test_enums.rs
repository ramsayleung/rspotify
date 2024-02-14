use rspotify::model::*;
use wasm_bindgen_test::*;

#[test]
#[wasm_bindgen_test]
fn test_include_external() {
    let audio = IncludeExternal::Audio;
    assert_eq!("audio", <&str>::from(audio));
}

#[test]
#[wasm_bindgen_test]
fn test_repeat_state() {
    let context = RepeatState::Context;
    assert_eq!(<&str>::from(context), "context");
}

#[test]
#[wasm_bindgen_test]
fn test_disallow_key() {
    let toggling_shuffle = DisallowKey::TogglingShuffle;
    assert_eq!(<&str>::from(toggling_shuffle), "toggling_shuffle");
}

#[test]
#[wasm_bindgen_test]
fn test_time_range() {
    let medium_range = TimeRange::MediumTerm;
    assert_eq!(<&str>::from(medium_range), "medium_term");
}

#[test]
#[wasm_bindgen_test]
fn test_date_precision() {
    let month = DatePrecision::Month;
    assert_eq!(<&str>::from(month), "month");
}

#[test]
#[wasm_bindgen_test]
fn test_album_type_convert_from_str() {
    let appears_on = AlbumType::AppearsOn;
    assert_eq!("appears_on", <&str>::from(appears_on));
}

#[test]
#[wasm_bindgen_test]
fn test_convert_search_type_from_str() {
    let search_type = SearchType::Artist;
    assert_eq!("artist", <&str>::from(search_type));
}

#[test]
#[wasm_bindgen_test]
fn test_type_convert_from_str() {
    let artist = Type::Artist;
    assert_eq!(<&str>::from(artist), "artist");
}

#[test]
#[wasm_bindgen_test]
fn test_additional_type() {
    let episode = AdditionalType::Episode;
    assert_eq!(<&str>::from(episode), "episode");
}

#[test]
#[wasm_bindgen_test]
fn test_current_playing_type() {
    let ad = CurrentlyPlayingType::Advertisement;
    assert_eq!(<&str>::from(ad), "ad");
}

#[test]
#[wasm_bindgen_test]
fn test_search_type() {
    let episode = SearchType::Episode;
    assert_eq!(<&str>::from(episode), "episode");
}

#[test]
#[wasm_bindgen_test]
fn test_convert_country_from_str() {
    let zimbabwe = Country::Zimbabwe;
    assert_eq!(<&str>::from(zimbabwe), "ZW");
}
