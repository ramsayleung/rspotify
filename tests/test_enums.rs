use rspotify::model::*;

#[test]
fn test_include_external() {
    let audio = IncludeExternal::Audio;
    assert_eq!("audio".to_string(), audio.to_string());
}
#[test]
fn test_repeat_state() {
    let context = RepeatState::Context;
    assert_eq!(context.to_string(), "context".to_string());
}

#[test]
fn test_disallow_key() {
    let toggling_shuffle = DisallowKey::TogglingShuffle;
    assert_eq!(toggling_shuffle.to_string(), "toggling_shuffle".to_string());
}

#[test]
fn test_time_range() {
    let medium_range = TimeRange::MediumTerm;
    assert_eq!(medium_range.to_string(), "medium_term".to_string());
}
#[test]
fn test_date_precision() {
    let month = DatePrecision::Month;
    assert_eq!(month.to_string(), "month".to_string());
}

#[test]
fn test_album_type_convert_from_str() {
    let appears_on = AlbumType::AppearsOn;
    assert_eq!("appears_on".to_string(), appears_on.to_string());
}
#[test]
fn test_convert_search_type_from_str() {
    let search_type = SearchType::Artist;
    assert_eq!("artist".to_string(), search_type.to_string());
}

#[test]
fn test_type_convert_from_str() {
    let artist = Type::Artist;
    assert_eq!(artist.to_string(), "artist".to_string());
}
#[test]
fn test_additional_type() {
    let episode = AdditionalType::Episode;
    assert_eq!(episode.to_string(), "episode".to_string());
}
#[test]
fn test_current_playing_type() {
    let ad = CurrentlyPlayingType::Advertisement;
    assert_eq!(ad.to_string(), "ad".to_string());
}
#[test]
fn test_search_type() {
    let episode = SearchType::Episode;
    assert_eq!(episode.to_string(), "episode".to_string());
}

#[test]
fn test_convert_country_from_str() {
    let zimbabwe = Country::Zimbabwe;
    assert_eq!(zimbabwe.to_string(), "ZW".to_string());
}
