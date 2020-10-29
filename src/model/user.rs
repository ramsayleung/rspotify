//! All kinds of user object
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use std::collections::HashMap;

use super::image::Image;
use crate::model::Type;
/// [Public user object](https://developer.spotify.com/documentation/web-api/reference/object-model/#user-object-public)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PublicUser {
    pub display_name: Option<String>,
    pub external_urls: HashMap<String, String>,
    pub followers: Option<HashMap<String, Option<Value>>>,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    #[serde(rename = "type")]
    pub _type: Type,
    pub uri: String,
}

/// [Private user object](https://developer.spotify.com/documentation/web-api/reference/users-profile/get-current-users-profile)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PrivateUser {
    pub birthdate: Option<NaiveDate>,
    pub country: Option<String>,
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub external_urls: HashMap<String, String>,
    pub followers: Option<HashMap<String, Option<Value>>>,
    pub href: String,
    pub id: String,
    pub images: Option<Vec<Image>>,
    #[serde(rename = "type")]
    pub _type: Type,
    pub uri: String,
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
}
