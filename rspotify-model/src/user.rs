//! All kinds of user object

use serde::{Deserialize, Serialize};

use std::collections::HashMap;

use crate::{Country, Followers, Image, SubscriptionLevel, UserId};

/// Public user object
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct PublicUser {
    pub display_name: Option<String>,
    pub external_urls: HashMap<String, String>,
    #[deprecated(
        since = "0.16.0",
        note = "Spotify has removed this field. See https://github.com/ramsayleung/rspotify/issues/550"
    )]
    pub followers: Option<Followers>,
    pub href: String,
    pub id: UserId<'static>,
    #[serde(default = "Vec::new")]
    pub images: Vec<Image>,
}

/// Private user object
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct PrivateUser {
    #[deprecated(
        since = "0.16.0",
        note = "Spotify has removed this field. See https://github.com/ramsayleung/rspotify/issues/550"
    )]
    pub country: Option<Country>,
    pub display_name: Option<String>,
    #[deprecated(
        since = "0.16.0",
        note = "Spotify has removed this field. See https://github.com/ramsayleung/rspotify/issues/550"
    )]
    pub email: Option<String>,
    pub external_urls: HashMap<String, String>,
    #[deprecated(
        since = "0.16.0",
        note = "Spotify has removed this field. See https://github.com/ramsayleung/rspotify/issues/550"
    )]
    pub explicit_content: Option<ExplicitContent>,
    #[deprecated(
        since = "0.16.0",
        note = "Spotify has removed this field. See https://github.com/ramsayleung/rspotify/issues/550"
    )]
    pub followers: Option<Followers>,
    pub href: String,
    pub id: UserId<'static>,
    pub images: Option<Vec<Image>>,
    #[deprecated(
        since = "0.16.0",
        note = "Spotify has removed this field. See https://github.com/ramsayleung/rspotify/issues/550"
    )]
    pub product: Option<SubscriptionLevel>,
}

/// Explicit content setting object
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct ExplicitContent {
    pub filter_enabled: bool,
    pub filter_locked: bool,
}
