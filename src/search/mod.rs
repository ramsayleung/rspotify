use std::collections::HashMap;

use strum::Display;

#[derive(Debug, Display, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
pub enum SearchFilter {
    Album,
    Artist,
    Track,
    Year,
    Upc,
    #[strum(serialize = "tag:hipster")]
    TagHipster,
    #[strum(serialize = "tag:new")]
    TagNew,
    Isrc,
    Genre,
}

#[derive(Debug, Default)]
pub struct SearchQuery {
    no_filter_query: String,
    query_map: HashMap<SearchFilter, String>,
}

impl SearchQuery {
    pub fn any<T: Into<String>>(&mut self, str: T) -> &mut Self {
        self.no_filter_query = str.into();
        self
    }

    pub fn album<T: Into<String>>(&mut self, str: T) -> &mut Self {
        self.query_map.insert(SearchFilter::Album, str.into());
        self
    }

    pub fn artist<T: Into<String>>(&mut self, str: T) -> &mut Self {
        self.query_map.insert(SearchFilter::Artist, str.into());
        self
    }

    pub fn track<T: Into<String>>(&mut self, str: T) -> &mut Self {
        self.query_map.insert(SearchFilter::Track, str.into());
        self
    }

    pub fn year<T: Into<String>>(&mut self, str: T) -> &mut Self {
        self.query_map.insert(SearchFilter::Year, str.into());
        self
    }

    pub fn upc<T: Into<String>>(&mut self, str: T) -> &mut Self {
        self.query_map.insert(SearchFilter::Upc, str.into());
        self
    }

    pub fn tag_new(&mut self) -> &mut Self {
        self.query_map.insert(SearchFilter::TagNew, "".into());
        self
    }

    pub fn tag_hipster(&mut self) -> &mut Self {
        self.query_map.insert(SearchFilter::TagHipster, "".into());
        self
    }

    pub fn isrc<T: Into<String>>(&mut self, str: T) -> &mut Self {
        self.query_map.insert(SearchFilter::Isrc, str.into());
        self
    }

    pub fn genre<T: Into<String>>(&mut self, str: T) -> &mut Self {
        self.query_map.insert(SearchFilter::Genre, str.into());
        self
    }
}

impl From<&mut SearchQuery> for String {
    fn from(val: &mut SearchQuery) -> Self {
        let mut rep = val.no_filter_query.clone();
        rep.push(' ');
        rep.push_str(
            val.query_map
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
