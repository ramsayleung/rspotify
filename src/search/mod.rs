use std::collections::HashMap;
use std::marker::PhantomData;

use rspotify_model::{SearchFilter, SearchType};

pub struct Artists;
pub struct Albums;
pub struct Playlists;
pub struct Tracks;
pub struct Shows;
pub struct Episodes;

pub struct SearchQuery<T> {
    base_search: String,
    filters: HashMap<SearchFilter, String>,
    pub search_type: SearchType,
    _marker: PhantomData<T>,
}

impl YearSearchFilter for SearchQuery<Albums> {}
impl YearSearchFilter for SearchQuery<Artists> {}
impl YearSearchFilter for SearchQuery<Tracks> {}

impl ArtistSearchFilter for SearchQuery<Albums> {}
impl ArtistSearchFilter for SearchQuery<Artists> {}
impl ArtistSearchFilter for SearchQuery<Tracks> {}

impl AlbumSearchFilter for SearchQuery<Albums> {}
impl AlbumSearchFilter for SearchQuery<Tracks> {}

impl GenreSearchFilter for SearchQuery<Artists> {}
impl GenreSearchFilter for SearchQuery<Tracks> {}

impl TrackSearchFilter for SearchQuery<Tracks> {}

impl IsrcSearchFilter for SearchQuery<Tracks> {}

impl UpcSearchFilter for SearchQuery<Albums> {}

impl TagHipsterSearchFilter for SearchQuery<Albums> {}

impl TagNewSearchFilter for SearchQuery<Albums> {}

pub trait BaseSearchQuery {
    fn add_filter(&mut self, search_filter: SearchFilter, str: String);
}

impl<T> BaseSearchQuery for SearchQuery<T> {
    fn add_filter(&mut self, search_filter: SearchFilter, str: String) {
        self.filters.insert(search_filter, str);
    }
}

pub trait ArtistSearchFilter: BaseSearchQuery + Sized {
    fn artist<T: Into<String>>(mut self, str: T) -> Self {
        self.add_filter(SearchFilter::Artist, str.into());
        self
    }
}

pub trait AlbumSearchFilter: BaseSearchQuery + Sized {
    fn album<T: Into<String>>(mut self, str: T) -> Self {
        self.add_filter(SearchFilter::Album, str.into());
        self
    }
}

pub trait TrackSearchFilter: BaseSearchQuery + Sized {
    fn track<T: Into<String>>(mut self, str: T) -> Self {
        self.add_filter(SearchFilter::Track, str.into());
        self
    }
}

pub trait YearSearchFilter: BaseSearchQuery + Sized {
    fn year<T: Into<String>>(mut self, str: T) -> Self {
        self.add_filter(SearchFilter::Year, str.into());
        self
    }
}

pub trait UpcSearchFilter: BaseSearchQuery + Sized {
    fn upc<T: Into<String>>(mut self, str: T) -> Self {
        self.add_filter(SearchFilter::Upc, str.into());
        self
    }
}

pub trait GenreSearchFilter: BaseSearchQuery + Sized {
    fn genre<T: Into<String>>(mut self, str: T) -> Self {
        self.add_filter(SearchFilter::Genre, str.into());
        self
    }
}

pub trait TagHipsterSearchFilter: BaseSearchQuery + Sized {
    fn tag_hipster<T: Into<String>>(mut self) -> Self {
        self.add_filter(SearchFilter::TagHipster, "".into());
        self
    }
}

pub trait TagNewSearchFilter: BaseSearchQuery + Sized {
    fn tag_new<T: Into<String>>(mut self) -> Self {
        self.add_filter(SearchFilter::TagNew, "".into());
        self
    }
}

pub trait IsrcSearchFilter: BaseSearchQuery + Sized {
    fn isrc<T: Into<String>>(mut self, str: T) -> Self {
        self.add_filter(SearchFilter::Isrc, str.into());
        self
    }
}

impl<T> SearchQuery<T> {
    pub fn any(mut self, str: impl Into<String>) -> Self {
        self.base_search = str.into();
        self
    }
}

impl Default for SearchQuery<Playlists> {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for SearchQuery<Albums> {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for SearchQuery<Artists> {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for SearchQuery<Tracks> {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for SearchQuery<Shows> {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for SearchQuery<Episodes> {
    fn default() -> Self {
        Self::new()
    }
}

impl SearchQuery<Playlists> {
    pub fn new() -> Self {
        SearchQuery {
            base_search: String::default(),
            filters: HashMap::default(),
            search_type: SearchType::Playlist,
            _marker: PhantomData::default(),
        }
    }
}

impl SearchQuery<Albums> {
    pub fn new() -> Self {
        SearchQuery {
            base_search: String::default(),
            filters: HashMap::default(),
            search_type: SearchType::Album,
            _marker: PhantomData::default(),
        }
    }
}

impl SearchQuery<Artists> {
    pub fn new() -> Self {
        SearchQuery {
            base_search: String::default(),
            filters: HashMap::default(),
            search_type: SearchType::Artist,
            _marker: PhantomData::default(),
        }
    }
}

impl SearchQuery<Tracks> {
    pub fn new() -> Self {
        SearchQuery {
            base_search: String::default(),
            filters: HashMap::default(),
            search_type: SearchType::Track,
            _marker: PhantomData::default(),
        }
    }
}

impl SearchQuery<Shows> {
    pub fn new() -> Self {
        SearchQuery {
            base_search: String::default(),
            filters: HashMap::default(),
            search_type: SearchType::Show,
            _marker: PhantomData::default(),
        }
    }
}

impl SearchQuery<Episodes> {
    pub fn new() -> Self {
        SearchQuery {
            base_search: String::default(),
            filters: HashMap::default(),
            search_type: SearchType::Episode,
            _marker: PhantomData::default(),
        }
    }
}

impl<T> From<&SearchQuery<T>> for String {
    fn from(val: &SearchQuery<T>) -> Self {
        let mut rep = val.base_search.clone();
        rep.push(' ');
        rep.push_str(
            val.filters
                .iter()
                .map(|entry| match entry.0 {
                    SearchFilter::TagNew | SearchFilter::TagHipster => format!("{} ", entry.0),
                    _ => format!("{}:{} ", entry.0, entry.1),
                })
                .collect::<String>()
                .as_str(),
        );

        rep
    }
}

impl<T> From<SearchQuery<T>> for String {
    fn from(val: SearchQuery<T>) -> Self {
        (&val).into()
    }
}
