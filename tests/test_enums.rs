use rspotify_async::model::*;

#[test]
fn test_include_external() {
    let audio = IncludeExternal::Audio;
    assert_eq!("audio", audio.as_ref());
}

#[test]
fn test_repeat_state() {
    let context = RepeatState::Context;
    assert_eq!(context.as_ref(), "context");
}

#[test]
fn test_disallow_key() {
    let toggling_shuffle = DisallowKey::TogglingShuffle;
    assert_eq!(toggling_shuffle.as_ref(), "toggling_shuffle");
}

#[test]
fn test_time_range() {
    let medium_range = TimeRange::MediumTerm;
    assert_eq!(medium_range.as_ref(), "medium_term");
}

#[test]
fn test_date_precision() {
    let month = DatePrecision::Month;
    assert_eq!(month.as_ref(), "month");
}

#[test]
fn test_album_type_convert_from_str() {
    let appears_on = AlbumType::AppearsOn;
    assert_eq!("appears_on", appears_on.as_ref());
}

#[test]
fn test_convert_search_type_from_str() {
    let search_type = SearchType::Artist;
    assert_eq!("artist", search_type.as_ref());
}

#[test]
fn test_type_convert_from_str() {
    let artist = Type::Artist;
    assert_eq!(artist.as_ref(), "artist");
}

#[test]
fn test_additional_type() {
    let episode = AdditionalType::Episode;
    assert_eq!(episode.as_ref(), "episode");
}

#[test]
fn test_current_playing_type() {
    let ad = CurrentlyPlayingType::Advertisement;
    assert_eq!(ad.as_ref(), "ad");
}

#[test]
fn test_search_type() {
    let episode = SearchType::Episode;
    assert_eq!(episode.as_ref(), "episode");
}

#[test]
fn test_convert_country_from_str() {
    let zimbabwe = Country::Zimbabwe;
    assert_eq!(zimbabwe.as_ref(), "ZW");
}
