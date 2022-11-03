use std::collections::HashMap;

use rspotify_model::SearchFilter;

#[derive(Debug, Default)]
pub struct SearchQuery<'a> {
    no_filter_query: &'a str,
    query_map: HashMap<SearchFilter, &'a str>,
}

impl<'a> SearchQuery<'a> {
    pub fn any(&mut self, str: &'a str) -> &mut Self {
        self.no_filter_query = str;
        self
    }

    pub fn album(&mut self, str: &'a str) -> &mut Self {
        self.query_map.insert(SearchFilter::Album, str);
        self
    }

    pub fn artist(&mut self, str: &'a str) -> &mut Self {
        self.query_map.insert(SearchFilter::Artist, str);
        self
    }

    pub fn track(&mut self, str: &'a str) -> &mut Self {
        self.query_map.insert(SearchFilter::Track, str);
        self
    }

    pub fn year(&mut self, str: &'a str) -> &mut Self {
        self.query_map.insert(SearchFilter::Year, str);
        self
    }

    pub fn upc(&mut self, str: &'a str) -> &mut Self {
        self.query_map.insert(SearchFilter::Upc, str);
        self
    }

    pub fn tag_new(&mut self) -> &mut Self {
        self.query_map.insert(SearchFilter::TagNew, "");
        self
    }

    pub fn tag_hipster(&mut self) -> &mut Self {
        self.query_map.insert(SearchFilter::TagHipster, "");
        self
    }

    pub fn isrc(&mut self, str: &'a str) -> &mut Self {
        self.query_map.insert(SearchFilter::Isrc, str);
        self
    }

    pub fn genre(&mut self, str: &'a str) -> &mut Self {
        self.query_map.insert(SearchFilter::Genre, str);
        self
    }
}

impl From<&SearchQuery<'_>> for String {
    fn from(val: &SearchQuery) -> Self {
        let mut rep = val.no_filter_query.to_owned();
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
