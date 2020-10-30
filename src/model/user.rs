//! All kinds of user object
use serde::{Deserialize, Serialize};
use serde_json::Value;

use std::collections::HashMap;

use super::image::Image;
use crate::model::{Country, SubscriptionLevel, Type};
/// [Public user object](https://developer.spotify.com/web-api/object-model/#user-object-public)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PublicUser {
    pub display_name: Option<String>,
    pub external_urls: HashMap<String, String>,
    pub followers: Option<HashMap<String, Option<Value>>>,
    pub href: String,
    pub id: String,
    #[serde(default = "Vec::new")]
    pub images: Vec<Image>,
    #[serde(rename = "type")]
    pub _type: Type,
    pub uri: String,
}

/// [Private user object](https://developer.spotify.com/documentation/web-api/reference/users-profile/get-current-users-profile)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PrivateUser {
    pub country: Option<Country>,
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub external_urls: HashMap<String, String>,
    pub explicit_content: Option<ExplicitContent>,
    pub followers: Option<HashMap<String, Option<Value>>>,
    pub href: String,
    pub id: String,
    pub images: Option<Vec<Image>>,
    pub product: Option<SubscriptionLevel>,
    #[serde(rename = "type")]
    pub _type: Type,
    pub uri: String,
}

/// [Explicit content setting object](https://developer.spotify.com/documentation/web-api/reference/object-model/#explicit-content-settings-object)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExplicitContent {
    pub filter_enabled: bool,
    pub filter_locked: bool,
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_public_user() {
        let json_str = r#"
        {
            "display_name": "Ronald Pompa",
            "external_urls": {
                "spotify": "https://open.spotify.com/user/wizzler"
            },
            "followers": {
                "href": null,
                "total": 4050
            },
            "href": "https://api.spotify.com/v1/users/wizzler",
            "id": "wizzler",
            "images": [
                {
                "height": null,
                "url": "https://i.scdn.co/image/ab6775700000ee85b5d374d281b9e510eda15fdf",
                "width": null
                }
            ],
            "type": "user",
            "uri": "spotify:user:wizzler"
        }
        "#;
        let user: PublicUser = serde_json::from_str(&json_str).unwrap();
        assert_eq!(user.id, "wizzler".to_string());
    }
    fn test_private_user() {
        let json_str = r#"
        {
            "country": "US",
            "display_name": "Sergey",
            "email": "vixatew967@top-email.org",
            "explicit_content": {
              "filter_enabled": false,
              "filter_locked": false
            },
            "external_urls": {
              "spotify": "https://open.spotify.com/user/waq5aexykhm6nlv0cnwdieng0"
            },
            "followers": {
              "href": null,
              "total": 0
            },
            "href": "https://api.spotify.com/v1/users/waq5aexykhm6nlv0cnwdieng0",
            "id": "waq5aexykhm6nlv0cnwdieng0",
            "images": [],
            "product": "open",
            "type": "user",
            "uri": "spotify:user:waq5aexykhm6nlv0cnwdieng0"
          } 
        "#;
        let private_user: PrivateUser = serde_json::from_str(&json_str).unwrap();
        assert_eq!(private_user.country.unwrap(), Country::UnitedStates);
    }
}
