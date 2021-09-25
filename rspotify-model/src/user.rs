//! All kinds of user object

use serde::{Deserialize, Serialize};

use std::collections::HashMap;

use crate::{Country, Followers, Image, SubscriptionLevel, UserId};

/// Public user object
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-publicuserobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct PublicUser {
    pub display_name: Option<String>,
    pub external_urls: HashMap<String, String>,
    pub followers: Option<Followers>,
    pub href: String,
    pub id: UserId,
    #[serde(default = "Vec::new")]
    pub images: Vec<Image>,
}

/// Private user object
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-privateuserobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct PrivateUser {
    pub country: Option<Country>,
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub external_urls: HashMap<String, String>,
    pub explicit_content: Option<ExplicitContent>,
    pub followers: Option<Followers>,
    pub href: String,
    pub id: UserId,
    pub images: Option<Vec<Image>>,
    pub product: Option<SubscriptionLevel>,
}

/// Explicit content setting object
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-explicitcontentsettingsobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExplicitContent {
    pub filter_enabled: bool,
    pub filter_locked: bool,
}
